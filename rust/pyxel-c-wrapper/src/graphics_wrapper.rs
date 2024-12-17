use crate::pyxel_singleton::pyxel;
use crate::image_wrapper::Image;

#[no_mangle]
pub extern "C" fn pyxel_camera(x: f64, y: f64) {
    pyxel().camera(x, y);
}

#[no_mangle]
pub extern "C" fn pyxel_dither(alpha: f32) {
    pyxel().dither(alpha);
}

#[no_mangle]
pub extern "C" fn pyxel_pget(x: f64, y: f64) -> u8 {
    pyxel().pget(x, y)
}

#[no_mangle]
pub extern "C" fn pyxel_pset(x: f64, y: f64, col: u8) {
    pyxel().pset(x, y, col);
}

#[no_mangle]
pub extern "C" fn pyxel_camera0() {
    pyxel().camera0();
}

#[no_mangle]
pub extern "C" fn pyxel_pal(col1: u8, col2: u8) {
    pyxel().pal(col1, col2);
}

#[no_mangle]
pub extern "C" fn pyxel_pal0() {
    pyxel().pal0();
}

#[no_mangle]
pub extern "C" fn pyxel_clip(x: f64, y: f64, w: f64, h: f64) {
    pyxel().clip(x, y, w, h);
}

#[no_mangle]
pub extern "C" fn pyxel_clip0() {
    pyxel().clip0();
}

#[no_mangle]
pub extern "C" fn pyxel_line(x1: f64, y1: f64, x2: f64, y2: f64, col: u8) {
    pyxel().line(x1, y1, x2, y2, col);
}

#[no_mangle]
pub extern "C" fn pyxel_cls(col: u8) {
    pyxel().cls(col);
}

#[no_mangle]
pub extern "C" fn pyxel_circ(x: f64, y: f64, r: f64, col: u8) {
    pyxel().circ(x, y, r, col);
}

#[no_mangle]
pub extern "C" fn pyxel_circb(x: f64, y: f64, r: f64, col: u8) {
    pyxel().circb(x, y, r, col);
}

#[no_mangle]
pub extern "C" fn pyxel_elli(x: f64, y: f64, w: f64, h: f64, col: u8) {
    pyxel().elli(x, y, w, h, col);
}

#[no_mangle]
pub extern "C" fn pyxel_ellib(x: f64, y: f64, w: f64, h: f64, col: u8) {
    pyxel().ellib(x, y, w, h, col);
}

#[no_mangle]
pub extern "C" fn pyxel_fill(x: f64, y: f64, col: u8) {
    pyxel().fill(x, y, col);
}

#[no_mangle]
pub extern "C" fn pyxel_rect(x: f64, y: f64, w: f64, h: f64, col: u8) {
    pyxel().rect(x, y, w, h, col);
}

#[no_mangle]
pub extern "C" fn pyxel_rectb(x: f64, y: f64, w: f64, h: f64, col: u8) {
    pyxel().rectb(x, y, w, h, col);
}

#[no_mangle]
pub extern "C" fn pyxel_tri(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: u8) {
    pyxel().tri(x1, y1, x2, y2, x3, y3, col);
}

#[no_mangle]
pub extern "C" fn pyxel_trib(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, col: u8) {
    pyxel().trib(x1, y1, x2, y2, x3, y3, col);
}

#[no_mangle]
pub extern "C" fn pyxel_blt(
    x: f64,
    y: f64,
    img_id: u32,
    u: f64,
    v: f64,
    w: f64,
    h: f64,
    colkey: i32,
    rotate: f64,
    scale: f64,
) {
    pyxel().blt(
        x,
        y,
        img_id,
        u,
        v,
        w,
        h,
        if colkey < 0 { None } else { Some(colkey as u8) },
        if rotate == 0.0 { None } else { Some(rotate) },
        if scale == 1.0 { None } else { Some(scale) },
    );
}

#[no_mangle]
pub extern "C" fn pyxel_blt_screen(
    x: f64,
    y: f64,
    img_ptr: *mut Image,
    u: f64,
    v: f64,
    w: f64,
    h: f64,
    colkey: i32,
    rotate: f64,
    scale: f64,
) {
    if let Some(img_ptr) = unsafe { img_ptr.as_ref() } {
        pyxel().screen.lock().blt(
            x,
            y,
            (*img_ptr).inner.clone(),
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

#[no_mangle]
pub extern "C" fn pyxel_bltm(
    x: f64,
    y: f64,
    img_id: u32,
    u: f64,
    v: f64,
    w: f64,
    h: f64,
    colkey: i32,
    rotate: f64,
    scale: f64,
) {
    pyxel().bltm(
        x,
        y,
        img_id,
        u,
        v,
        w,
        h,
        if colkey < 0 { None } else { Some(colkey as u8) },
        if rotate == 0.0 { None } else { Some(rotate) },
        if scale == 1.0 { None } else { Some(scale) },
    );
}

#[no_mangle]
pub extern "C" fn pyxel_text(x: f64, y: f64, s: *const i8, col: u8) {
    pyxel().text(
        x,
        y,
        unsafe { std::ffi::CStr::from_ptr(s).to_str().unwrap() },
        col,
        None,
    );
}

#[no_mangle]
pub extern "C" fn pyxel_screen() -> *mut Image {
    Box::into_raw(Box::new(Image {
        inner: pyxel().screen.clone(),
    }))
}