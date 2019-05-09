use crate::graphics_2d::Color;
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

unsafe impl Sync for Profile {}

pub(crate) struct Profile {
    display_profile: lcms2::Profile,
    srgb_profile: lcms2::Profile,
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
            display_profile,
            srgb_profile: lcms2::Profile::new_srgb(),
        })
    }
    pub(crate) fn from_window(window: &Window) -> Result<Profile, ()> {
        #[cfg(target_os = "macos")]
        return Profile::from_window_macos(window);
        #[cfg(not(target_os = "macos"))]
        Err(())
    }
    pub(crate) fn transform(&self, color: Color) -> Color {
        let t = Transform::new(
            &self.srgb_profile,
            PixelFormat::RGBA_8,
            &self.display_profile,
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
}
