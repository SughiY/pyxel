use std::ffi::CStr;
use std::ptr::null_mut;

#[repr(C)]
pub struct Font {
    pub(crate) inner: pyxel::SharedFont,
}

#[no_mangle]
pub extern "C" fn font_new(filename: *const i8) -> *mut Font {
    let c_str = unsafe { CStr::from_ptr(filename) };
    match c_str.to_str() {
        Ok(str_slice) => {
            return Box::into_raw(Box::new(Font {
                inner: pyxel::Font::new(str_slice),
            }));
        }
        Err(_) => {
            return null_mut();
        }
    }
}

#[no_mangle]
pub extern "C" fn font_text_width(font_ptr: *mut Font, s: *const i8) -> i32 {
    if font_ptr.is_null() {
        return -1;
    }
    let c_str = unsafe { CStr::from_ptr(s) };
    match c_str.to_str() {
        Ok(str_slice) => unsafe {
            return (*font_ptr).inner.lock().text_width(str_slice);
        },
        Err(_) => {
            return -1;
        }
    }
}

#[no_mangle]
pub extern "C" fn font_free(font_ptr: *mut Font) {
    if !font_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(font_ptr);
        }
    }
}