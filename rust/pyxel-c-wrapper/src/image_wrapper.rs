use std::ffi::CStr;
use std::ptr::null_mut;

#[repr(C)]
pub struct Image {
    pub(crate) inner: pyxel::SharedImage,
}

#[no_mangle]
pub extern "C" fn image_new(width: u32, height: u32) -> *mut Image {
    Box::into_raw(Box::new(Image {
        inner: pyxel::Image::new(width, height),
    }))
}

#[no_mangle]
pub extern "C" fn image_from(filename: *const i8, incl_u8s: bool) -> *mut Image {
    let c_str = unsafe { CStr::from_ptr(filename) };
    match c_str.to_str() {
        Ok(str_slice) => Box::into_raw(Box::new(Image {
            inner: pyxel::Image::from_image(str_slice, Some(incl_u8s)),
        })),
        Err(_) => null_mut(),
    }
}

#[no_mangle]
pub fn image_width(image_ptr: *mut Image) -> u32 {
    if !image_ptr.is_null() {
        unsafe { (*image_ptr).inner.lock().width() }
    } else {
        0
    }
}

#[no_mangle]
pub fn image_height(image_ptr: *mut Image) -> u32 {
    if !image_ptr.is_null() {
        unsafe { (*image_ptr).inner.lock().height() }
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn image_data_ptr(image_ptr: *mut Image) -> *mut u8 {
    if image_ptr.is_null() {
        return null_mut();
    }
    unsafe {
        let mut image = (*image_ptr).inner.lock();
        image.data_ptr()
    }
}

#[no_mangle]
pub extern "C" fn image_data_length(image_ptr: *mut Image) -> usize {
    if image_ptr.is_null() {
        return 0;
    }
    unsafe {
        let image = (*image_ptr).inner.lock();
        (image.width() * image.height()) as usize
    }
}

// #[no_mangle]
// pub extern "C" fn set(x: i32, y: i32, data:) {

#[no_mangle]
pub extern "C" fn image_load(image_ptr: *mut Image, x:i32, y:i32, filename: *const i8, incl_colors: u32) -> i32 {
    if image_ptr.is_null() {
        return -1;
    }
    let c_str = unsafe { CStr::from_ptr(filename) };
    match c_str.to_str() {
        Ok(str_slice) => {
            unsafe {
                (*image_ptr).inner.lock().load(x, y, str_slice,
                    if incl_colors == 0 { None } else { Some(true) }
                    );
            }
            0
        }
        Err(_) => -1,
    }
}


#[no_mangle]
pub extern "C" fn image_save(image_ptr: *mut Image, filename: *const i8, scale: u32) -> i32 {
    if image_ptr.is_null() {
        return -1;
    }
    let c_str = unsafe { CStr::from_ptr(filename) };
    match c_str.to_str() {
        Ok(str_slice) => {
            unsafe {
                (*image_ptr).inner.lock().save(str_slice, scale);
            }
            0
        }
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn image_clip(image_ptr: *mut Image, x: f64, y: f64, w: f64, h: f64) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().clip(x, y, w, h);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_clip0(image_ptr: *mut Image) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().clip0();
        }
    }
}

#[no_mangle]
pub extern "C" fn image_camera(image_ptr: *mut Image, x: f64, y: f64) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().camera(x, y);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_camera0(image_ptr: *mut Image) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().camera0();
        }
    }
}

#[no_mangle]
pub extern "C" fn image_pal(image_ptr: *mut Image, src: u8, dst: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().pal(src, dst);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_pal0(image_ptr: *mut Image) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().pal0();
        }
    }
}

#[no_mangle]
pub extern "C" fn image_dither(image_ptr: *mut Image, alpha: f32) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().dither(alpha);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_cls(image_ptr: *mut Image, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().cls(col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_pget(image_ptr: *mut Image, x: f64, y: f64) -> u8 {
    if !image_ptr.is_null() {
        unsafe { (*image_ptr).inner.lock().pget(x, y) }
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn image_pset(image_ptr: *mut Image, x: f64, y: f64, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().pset(x, y, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_line(image_ptr: *mut Image, x1: f64, y1: f64, x2: f64, y2: f64, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().line(x1, y1, x2, y2, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_rect(image_ptr: *mut Image, x: f64, y: f64, w: f64, h: f64, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().rect(x, y, w, h, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_rectb(image_ptr: *mut Image, x: f64, y: f64, w: f64, h: f64, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().rectb(x, y, w, h, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_circ(image_ptr: *mut Image, x: f64, y: f64, r: f64, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().circ(x, y, r, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_circb(image_ptr: *mut Image, x: f64, y: f64, r: f64, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().circb(x, y, r, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_elli(image_ptr: *mut Image, x: f64, y: f64, w: f64, h: f64, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().elli(x, y, w, h, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_ellib(image_ptr: *mut Image, x: f64, y: f64, w: f64, h: f64, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().ellib(x, y, w, h, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_tri(
    image_ptr: *mut Image,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    col: u8,
) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().tri(x1, y1, x2, y2, x3, y3, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_trib(
    image_ptr: *mut Image,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    col: u8,
) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().trib(x1, y1, x2, y2, x3, y3, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_fill(image_ptr: *mut Image, x: f64, y: f64, col: u8) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().fill(x, y, col);
        }
    }
}

#[no_mangle]
pub extern "C" fn image_blt(
    image_ptr: *mut Image,
    x: f64,
    y: f64,
    img: *mut Image,
    u: f64,
    v: f64,
    w: f64,
    h: f64,
    colkey: i32,
    rotate: f64,
    scale: f64,
) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().blt(
                x,
                y,
                (*img).inner.clone(),
                u,
                v,
                w,
                h,
                if colkey < 0 { None } else { Some(colkey as u8) },
                if rotate == 0.0 { None } else { Some(rotate) },
                if scale == 1.0 { None } else { Some(scale) },
            );
        }
    }
}

// #[no_mangle]
// #[pyo3(signature = (x, y, tm, u, v, w, h, colkey=None, rotate=None, scale=None))]
// pub fn bltm(
//     &self,
//     x: f64,
//     y: f64,
//     tm: Bound<'_, PyAny>,
//     u: f64,
//     v: f64,
//     w: f64,
//     h: f64,
//     colkey: Option<pyxel::Color>,
//     rotate: Option<f64>,
//     scale: Option<f64>,
// ) -> PyResult<()> {
//     cast_pyany! {
//         tm,
//         (u32, {
//             let tilemap = pyxel().tilemaps.lock()[tm as usize].clone();
//             self.inner.lock().bltm(x, y, tilemap, u, v, w, h, colkey, rotate, scale);
//         }),
//         (Tilemap, { self.inner.lock().bltm(x, y, tm.inner, u, v, w, h, colkey, rotate, scale); })
//     }
//     Ok(())
// }

#[no_mangle]
pub extern "C" fn image_text(image_ptr: *mut Image, x: f64, y: f64, s: *const i8, col: u8) {
    if !image_ptr.is_null() {
        let c_str = unsafe { CStr::from_ptr(s) };
        if let Ok(str_slice) = c_str.to_str() {
            unsafe {
                (*image_ptr).inner.lock().text(x, y, str_slice, col, None);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn image_free(image_ptr: *mut Image) {
    if !image_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(image_ptr);
        }
    }
}