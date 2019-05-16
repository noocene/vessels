use crate::graphics_2d::{Rasterizable, Color};
use crate::path::{Fill, GradientStop, Path, Texture};
use std::sync::{Arc, RwLock};
use lcms2::{Intent, PixelFormat, Transform};
use libc::c_void;

use glutin::Window;

#[cfg(target_os = "macos")]
mod cm_backing {
    use libc::c_void;
    use std::ffi::CString;
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
            std::slice::from_raw_parts(
                profile_data_bytes as *const u8,
                profile_data_length as usize,
            )
        })
    }
}

unsafe impl Send for Profile {}
unsafe impl Sync for Profile {}

struct ProfileState {
    display_profile: lcms2::Profile,
    srgb_profile: lcms2::Profile,
}

#[derive(Clone)]
pub(crate) struct Profile {
    state: Arc<RwLock<ProfileState>>
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
            })),
        })
    }
    pub(crate) fn from_window(window: &Window) -> Result<Profile, ()> {
        #[cfg(target_os = "macos")]
        return Profile::from_window_macos(window);
        #[cfg(not(target_os = "macos"))]
        Err(())
    }
    pub(crate) fn transform(&self, color: Color) -> Color {
        let state = self.state.read().unwrap();
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
        Color::rgba(
            source_pixels[0][0],
            source_pixels[0][1],
            source_pixels[0][2],
            source_pixels[0][3],
        )
    }
    pub(crate) fn transform_texture(&self, texture: Texture) -> Texture {
        match texture {
            Texture::Solid(color) => Texture::Solid(self.transform(color)),
            Texture::LinearGradient(mut gradient) => {
                gradient.stops = gradient.stops.iter().map(|stop| GradientStop {
                    offset: stop.offset,
                    color: self.transform(stop.color),
                }).collect();
                Texture::LinearGradient(gradient)
            },
            Texture::RadialGradient(mut gradient) => {
                gradient.stops = gradient.stops.iter().map(|stop| GradientStop {
                    offset: stop.offset,
                    color: self.transform(stop.color),
                }).collect();
                Texture::RadialGradient(gradient)
            },
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
                stroke: path.stroke.map(|mut stroke| {
                    stroke.content = self.transform_texture(stroke.content);
                    stroke
                }),
                fill: path.fill.map(|fill| Fill {
                    content: self.transform_texture(fill.content)
                }),
                shadows: path.shadows.iter().map(|shadow| {
                    let mut corrected_shadow = shadow.clone();
                    corrected_shadow.color = self.transform(shadow.color);
                    corrected_shadow
                }).collect(),
                closed: path.closed,
            })) 
        }
    }
}
