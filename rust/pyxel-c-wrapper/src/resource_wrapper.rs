use std::ffi::CStr;
use crate::pyxel_singleton::pyxel;

#[no_mangle]
pub extern "C" fn pyxel_load(
    filename: *const i8,
    excl_images: bool,
    excl_tilemaps: bool,
    excl_sounds: bool,
    excl_musics: bool,
    incl_colors: bool,
    incl_channels: bool,
    incl_tones: bool,
) {
    let c_str = unsafe { CStr::from_ptr(filename) };
    match c_str.to_str() {
        Ok(str_slice) => {
            pyxel().load(
                str_slice,
                Some(excl_images),
                Some(excl_tilemaps),
                Some(excl_sounds),
                Some(excl_musics),
                Some(incl_colors),
                Some(incl_channels),
                Some(incl_tones),
            );
        }
        Err(_) => {
            return;
        }
    }
}

#[no_mangle]
pub extern "C" fn save(
    filename: *const i8,
    excl_images: bool,
    excl_tilemaps: bool,
    excl_sounds: bool,
    excl_musics: bool,
    incl_colors: bool,
    incl_channels: bool,
    incl_tones: bool,
) {
    let c_str = unsafe { CStr::from_ptr(filename) };
    match c_str.to_str() {
        Ok(str_slice) => {
            pyxel().save(
                str_slice,
                Some(excl_images),
                Some(excl_tilemaps),
                Some(excl_sounds),
                Some(excl_musics),
                Some(incl_colors),
                Some(incl_channels),
                Some(incl_tones),
            );
        }
        Err(_) => {
            return;
        }
    }
}

#[no_mangle]
pub extern "C" fn screenshot(scale: u32) {
    pyxel().screenshot(
        if scale == 0 { None } else { Some(scale) }
    );
}

#[no_mangle]
pub extern "C" fn screencast(scale: u32) {
    pyxel().screencast(
        if scale == 0 { None } else { Some(scale) }
    );
}

#[no_mangle]
pub extern "C" fn reset_screencast() {
    pyxel().reset_screencast();
}

#[no_mangle]
pub extern "C" fn user_data_dir(vendor_name: *const i8, app_name: *const i8) -> *const i8{
    let vender_name_c_str = unsafe { CStr::from_ptr(vendor_name) };
    let app_name_c_str = unsafe { CStr::from_ptr(app_name) };
    match vender_name_c_str.to_str() {
        Ok(vender_name_str_slice) => {
            match app_name_c_str.to_str() {
                Ok(app_name_str_slice) => {
                    pyxel().user_data_dir(vender_name_str_slice, app_name_str_slice).as_ptr() as *const i8
                }
                Err(_) => "".as_ptr() as *const i8,
            }
        }
        Err(_) => "".as_ptr() as *const i8,
    }
}