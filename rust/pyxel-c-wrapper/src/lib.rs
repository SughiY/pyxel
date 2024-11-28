use std::ffi::CStr;
use std::mem::transmute;
use std::ptr::null_mut;

use pyxel::{Pyxel, PyxelCallback, KEY_UNKNOWN};

static mut PYXEL: *mut Pyxel = null_mut();

fn pyxel() -> &'static mut Pyxel {
    unsafe {
        if PYXEL.is_null() {
            panic!("Pyxel not initialized");
        } else {
            &mut *PYXEL
        }
    }
}

fn set_pyxel_instance(pyxel: Pyxel) {
    unsafe {
        PYXEL = transmute::<Box<Pyxel>, *mut Pyxel>(Box::new(pyxel));
    }
}

#[no_mangle]
pub extern "C" fn pyxel_init(
    width: u32,
    height: u32,
    title: *const i8,
    fps: u32,
    quit_key: u32,
    display_scale: u32,
    capture_scale: u32,
    capture_sec: u32,
) -> i32 {
    set_pyxel_instance(pyxel::init(
        width,
        height,
        if (title as *const u8).is_null() {
            None
        } else {
            Some(unsafe { std::ffi::CStr::from_ptr(title).to_str().unwrap() })
        },
        if fps == 0 { None } else { Some(fps) },
        if quit_key == KEY_UNKNOWN {
            None
        } else {
            Some(quit_key)
        },
        if display_scale == 0 {
            None
        } else {
            Some(display_scale)
        },
        if capture_scale == 0 {
            None
        } else {
            Some(capture_scale)
        },
        if capture_sec == 0 {
            None
        } else {
            Some(capture_sec)
        },
    ));
    0
}

#[no_mangle]
pub extern "C" fn pyxel_flip() {
    pyxel().flip();
}

#[no_mangle]
pub extern "C" fn pyxel_quit() {
    pyxel().quit();
}

#[no_mangle]
pub extern "C" fn pyxel_title(title: *const i8) {
    pyxel().title(unsafe { std::ffi::CStr::from_ptr(title).to_str().unwrap() });
}

#[no_mangle]
pub extern "C" fn pyxel_perf_monitor(enabled: bool) {
    pyxel().perf_monitor(enabled);
}

#[no_mangle]
pub extern "C" fn pyxel_mouse(visible: bool) {
    pyxel().mouse(visible);
}

#[no_mangle]
pub extern "C" fn pyxel_mouse_x() -> i32 {
    pyxel().mouse_x
}

#[no_mangle]
pub extern "C" fn pyxel_mouse_y() -> i32 {
    pyxel().mouse_y
}

#[no_mangle]
pub extern "C" fn pyxel_mouse_wheel() -> i32 {
    pyxel().mouse_wheel
}

#[no_mangle]
pub extern "C" fn pyxel_btn(key: u32) -> bool {
    pyxel().btn(key)
}

#[no_mangle]
pub extern "C" fn pyxel_btnp(key: u32, hold: u32, period: u32) -> bool {
    pyxel().btnp(
        key,
        if hold == 0 { None } else { Some(hold) },
        if period == 0 { None } else { Some(period) },
    )
}

#[no_mangle]
pub extern "C" fn pyxel_btnr(key: u32) -> bool {
    pyxel().btnr(key)
}

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
pub extern "C" fn pyxel_input_text() -> *const i8 {
    pyxel().input_text.as_ptr() as *const i8
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
pub extern "C" fn pyxel_show() {
    pyxel().show();
}

#[no_mangle]
pub extern "C" fn pyxel_run(update_fn: extern "C" fn(), draw_fn: extern "C" fn()) {
    struct CCallback {
        update: extern "C" fn(),
        draw: extern "C" fn(),
    }

    impl PyxelCallback for CCallback {
        fn update(&mut self, _pyxel: &mut Pyxel) {
            (self.update)();
        }

        fn draw(&mut self, _pyxel: &mut Pyxel) {
            (self.draw)();
        }
    }

    pyxel().run(CCallback {
        update: update_fn,
        draw: draw_fn,
    });
}

#[repr(C)]
pub struct Font {
    /// cbindgen:ignore
    pub(crate) inner: pyxel::SharedFont,
}

impl Font {
    /// cbindgen:ignore
    pub fn wrap(inner: pyxel::SharedFont) -> Self {
        Self { inner }
    }
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

#[no_mangle]
pub extern "C" fn pyxel_load(
    filename: *const i8,
    excl_images: bool,
    excl_tilemaps: bool,
    excl_sounds: bool,
    excl_musics: bool,
    incl_u8s: bool,
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
                Some(incl_u8s),
                Some(incl_channels),
                Some(incl_tones),
            );
        }
        Err(_) => {
            return;
        }
    }
}

#[repr(C)]
pub struct Image {
    pub(crate) inner: pyxel::SharedImage,
}

impl Image {
    pub fn wrap(inner: pyxel::SharedImage) -> Self {
        Self { inner }
    }
}

#[no_mangle]
pub extern "C" fn pyxel_screen() -> *mut Image {
    Box::into_raw(Box::new(Image {
        inner: pyxel().screen.clone(),
    }))
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
pub extern "C" fn image_free(image_ptr: *mut Image) {
    if !image_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(image_ptr);
        }
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
pub extern "C" fn image_dither(image_ptr: *mut Image, alpha: f32) {
    if !image_ptr.is_null() {
        unsafe {
            (*image_ptr).inner.lock().dither(alpha);
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