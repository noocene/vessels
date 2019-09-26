use crate::graphics::{
    canvas::Rasterizable,
    path::{Fill, GradientStop, Path, Texture},
    LDRColor,
};
use lcms2::{Intent, PixelFormat, Transform};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

use glutin::Window;

use libc::c_void;

#[cfg(target_os = "macos")]
mod cm_backing {
    use libc::c_void;
    use std::{ffi::CString, slice};
    extern "C" {
        fn objc_msgSend(id: *const c_void, sel: *const c_void, ...) -> *const c_void;
        fn sel_registerName(name: *const i8) -> *const c_void;
    }
    fn msg_send(object: *const c_void, selector: &'_ str) -> Option<*const c_void> {
        let sel_string = CString::new(selector).unwrap();
        let ptr = unsafe {
            let selector = sel_registerName(sel_string.as_ref().as_ptr());
            objc_msgSend(object, selector)
        };
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }
    pub(crate) fn get_profile_data<'a>(window: *const c_void) -> Result<&'a [u8], ()> {
        let color_space = msg_send(window, "colorSpace").ok_or_else(|| ())?;
        let profile_data = msg_send(color_space, "ICCProfileData").ok_or_else(|| ())?;
        let profile_data_length = msg_send(profile_data, "length").ok_or_else(|| ())?;
        let profile_data_bytes = msg_send(profile_data, "bytes").ok_or_else(|| ())?;

        Ok(unsafe {
            slice::from_raw_parts(
                profile_data_bytes as *const u8,
                profile_data_length as usize,
            )
        })
    }
}

#[cfg(target_os = "windows")]
mod cm_backing {
    use libc::c_void;
    use std::fs;
    extern "C" {
        fn GetDC(window_handle: *const c_void) -> *const c_void;
        fn GetICMProfileW(
            device_context_handle: *const c_void,
            buffer_size: *const u32,
            buffer: *mut u16,
        ) -> bool;
    }
    pub(crate) fn get_profile_data<'a>(window: *const c_void) -> Result<Vec<u8>, ()> {
        let dc = unsafe { GetDC(window) };
        let mut size: u32 = 0;
        unsafe { GetICMProfileW(dc, &mut size as *const u32, 0x0 as *mut u16) };
        let mut buf: Vec<u16> = Vec::with_capacity(size as usize);
        if !unsafe { GetICMProfileW(dc, &(buf.capacity() as u32) as *const u32, buf.as_mut_ptr()) }
        {
            return Err(());
        }
        unsafe {
            buf.set_len(size as usize);
        }
        let path = String::from_utf16_lossy(&buf);
        let path = path.trim_end_matches(char::from(0));
        fs::read(path).map_err(|_| ())
    }
}

unsafe impl Sync for Profile {}
unsafe impl Send for Profile {}

struct ProfileState {
    display_profile: lcms2::Profile,
    srgb_profile: lcms2::Profile,
    color_cache: HashMap<LDRColor, LDRColor>,
    color_cache_queue: VecDeque<LDRColor>,
}

#[derive(Clone)]
pub(crate) struct Profile {
    state: Arc<RwLock<ProfileState>>,
}

impl Profile {
    #[cfg(target_os = "macos")]
    fn from_window_macos(window: &Window) -> Result<Profile, ()> {
        use glutin::os::macos::WindowExt;
        let os_window = window.get_nswindow();
        let display_profile =
            lcms2::Profile::new_icc(cm_backing::get_profile_data(os_window as *const c_void)?)
                .map_err(|_| ())?;
        Ok(Profile {
            state: Arc::new(RwLock::new(ProfileState {
                display_profile,
                srgb_profile: lcms2::Profile::new_srgb(),
                color_cache: HashMap::with_capacity(10),
                color_cache_queue: VecDeque::with_capacity(10),
            })),
        })
    }
    #[cfg(target_os = "windows")]
    fn from_window_windows(window: &Window) -> Result<Profile, ()> {
        use glutin::os::windows::WindowExt;
        let os_window = window.get_hwnd();
        let display_profile =
            lcms2::Profile::new_icc(&cm_backing::get_profile_data(os_window)?).map_err(|_| ())?;
        Ok(Profile {
            state: Arc::new(RwLock::new(ProfileState {
                display_profile,
                srgb_profile: lcms2::Profile::new_srgb(),
                color_cache: HashMap::with_capacity(10),
                color_cache_queue: VecDeque::with_capacity(10),
            })),
        })
    }
    pub(crate) fn from_window(window: &Window) -> Result<Profile, ()> {
        #[cfg(target_os = "macos")]
        return Profile::from_window_macos(window);
        #[cfg(target_os = "windows")]
        return Profile::from_window_windows(window);
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        Err(())
    }
    pub(crate) fn transform(&self, color: LDRColor) -> LDRColor {
        let state = self.state.read().unwrap();
        if let Some(transformed_color) = state.color_cache.get(&color) {
            return transformed_color.clone();
        }
        let t = Transform::new(
            &state.srgb_profile,
            PixelFormat::RGBA_8,
            &state.display_profile,
            PixelFormat::RGBA_8,
            Intent::Perceptual,
        )
        .unwrap();
        let source_pixels: &mut [[u8; 4]] = &mut [[color.r, color.g, color.b, color.a]];
        t.transform_in_place(source_pixels);
        let transformed_color = LDRColor::rgba(
            source_pixels[0][0],
            source_pixels[0][1],
            source_pixels[0][2],
            source_pixels[0][3],
        );
        drop(state);
        let mut state = self.state.write().unwrap();
        if state.color_cache_queue.len() >= state.color_cache_queue.capacity() {
            let rm_color = state.color_cache_queue.pop_front().unwrap();
            state.color_cache.remove(&rm_color).unwrap();
        }
        state.color_cache_queue.push_back(color);
        state.color_cache.insert(color, transformed_color);
        transformed_color
    }
    pub(crate) fn transform_texture(&self, texture: Texture) -> Texture {
        match texture {
            Texture::Solid(color) => Texture::Solid(self.transform(color)),
            Texture::LinearGradient(mut gradient) => {
                gradient.stops = gradient
                    .stops
                    .iter()
                    .map(|stop| GradientStop {
                        offset: stop.offset,
                        color: self.transform(stop.color),
                    })
                    .collect();
                Texture::LinearGradient(gradient)
            }
            Texture::RadialGradient(mut gradient) => {
                gradient.stops = gradient
                    .stops
                    .iter()
                    .map(|stop| GradientStop {
                        offset: stop.offset,
                        color: self.transform(stop.color),
                    })
                    .collect();
                Texture::RadialGradient(gradient)
            }
            texture => texture,
        }
    }
    pub(crate) fn transform_content(&self, content: Rasterizable) -> Rasterizable {
        match content {
            Rasterizable::Text(mut text) => Rasterizable::Text({
                text.color = self.transform(text.color);
                text
            }),
            Rasterizable::Path(path) => Rasterizable::Path(Box::new(Path {
                segments: path.segments,
                clip_segments: path.clip_segments,
                stroke: path.stroke.map(|mut stroke| {
                    stroke.content = self.transform_texture(stroke.content);
                    stroke
                }),
                fill: path.fill.map(|fill| Fill {
                    content: self.transform_texture(fill.content),
                }),
                shadows: path
                    .shadows
                    .iter()
                    .map(|shadow| {
                        let mut corrected_shadow = shadow.clone();
                        corrected_shadow.color = self.transform(shadow.color);
                        corrected_shadow
                    })
                    .collect(),
                closed: path.closed,
            })),
        }
    }
}
