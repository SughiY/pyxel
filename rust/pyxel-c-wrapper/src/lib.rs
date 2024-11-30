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
pub extern "C" fn pyxel_frame_count() -> u32 {
    pyxel().frame_count
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

// Export constants
#[no_mangle]
pub static PYXEL_NUM_COLORS:u32 = pyxel::NUM_COLORS;
#[no_mangle]
pub static PYXEL_NUM_IMAGES:u32 = pyxel::NUM_IMAGES; 
#[no_mangle]
pub static PYXEL_IMAGE_SIZE:u32 = pyxel::IMAGE_SIZE;
#[no_mangle]
pub static PYXEL_NUM_TILEMAPS:u32 = pyxel::NUM_TILEMAPS;
#[no_mangle]
pub static PYXEL_TILEMAP_SIZE:u32 = pyxel::TILEMAP_SIZE; 
#[no_mangle]
pub static PYXEL_TILE_SIZE:u32 = pyxel::TILE_SIZE;

#[no_mangle]
pub static PYXEL_COLOR_BLACK:u8 = pyxel::COLOR_BLACK;
#[no_mangle]
pub static PYXEL_COLOR_NAVY:u8 = pyxel::COLOR_NAVY;
#[no_mangle]
pub static PYXEL_COLOR_PURPLE:u8 = pyxel::COLOR_PURPLE; 
#[no_mangle]
pub static PYXEL_COLOR_GREEN:u8 = pyxel::COLOR_GREEN;
#[no_mangle]
pub static PYXEL_COLOR_BROWN:u8 = pyxel::COLOR_BROWN;
#[no_mangle]
pub static PYXEL_COLOR_DARK_BLUE:u8 = pyxel::COLOR_DARK_BLUE;
#[no_mangle]
pub static PYXEL_COLOR_LIGHT_BLUE:u8 = pyxel::COLOR_LIGHT_BLUE;
#[no_mangle]
pub static PYXEL_COLOR_WHITE:u8 = pyxel::COLOR_WHITE;
#[no_mangle]
pub static PYXEL_COLOR_RED:u8 = pyxel::COLOR_RED;
#[no_mangle]
pub static PYXEL_COLOR_ORANGE:u8 = pyxel::COLOR_ORANGE;
#[no_mangle]
pub static PYXEL_COLOR_YELLOW:u8 = pyxel::COLOR_YELLOW;
#[no_mangle]
pub static PYXEL_COLOR_LIME:u8 = pyxel::COLOR_LIME;
#[no_mangle]
pub static PYXEL_COLOR_CYAN:u8 = pyxel::COLOR_CYAN;
#[no_mangle]
pub static PYXEL_COLOR_GRAY:u8 = pyxel::COLOR_GRAY;
#[no_mangle]
pub static PYXEL_COLOR_PINK:u8 = pyxel::COLOR_PINK;
#[no_mangle]
pub static PYXEL_COLOR_PEACH:u8 = pyxel::COLOR_PEACH;

#[no_mangle]
pub static PYXEL_FONT_WIDTH:u32 = pyxel::FONT_WIDTH;
#[no_mangle]
pub static PYXEL_FONT_HEIGHT:u32 = pyxel::FONT_HEIGHT;

#[no_mangle]
pub static PYXEL_NUM_CHANNELS:u32 = pyxel::NUM_CHANNELS;
#[no_mangle]
pub static PYXEL_NUM_TONES:u32 = pyxel::NUM_TONES;
#[no_mangle]
pub static PYXEL_NUM_SOUNDS:u32 = pyxel::NUM_SOUNDS;
#[no_mangle]
pub static PYXEL_NUM_MUSICS:u32 = pyxel::NUM_MUSICS;

#[no_mangle]
pub static PYXEL_TONE_TRIANGLE:u32 = pyxel::TONE_TRIANGLE;
#[no_mangle]
pub static PYXEL_TONE_SQUARE:u32 = pyxel::TONE_SQUARE;
#[no_mangle]
pub static PYXEL_TONE_PULSE:u32 = pyxel::TONE_PULSE; 
#[no_mangle]
pub static PYXEL_TONE_NOISE:u32 = pyxel::TONE_NOISE;

#[no_mangle]
pub static PYXEL_EFFECT_NONE:u8 = pyxel::EFFECT_NONE;
#[no_mangle]
pub static PYXEL_EFFECT_SLIDE:u8 = pyxel::EFFECT_SLIDE;
#[no_mangle]
pub static PYXEL_EFFECT_VIBRATO:u8 = pyxel::EFFECT_VIBRATO;
#[no_mangle]
pub static PYXEL_EFFECT_FADEOUT:u8 = pyxel::EFFECT_FADEOUT;
#[no_mangle]
pub static PYXEL_EFFECT_HALF_FADEOUT:u8 = pyxel::EFFECT_HALF_FADEOUT;
#[no_mangle]
pub static PYXEL_EFFECT_QUARTER_FADEOUT:u8 = pyxel::EFFECT_QUARTER_FADEOUT;

// Key constants
#[no_mangle]
pub static PYXEL_KEY_UNKNOWN:u32 = pyxel::KEY_UNKNOWN;
#[no_mangle]
pub static PYXEL_KEY_RETURN:u32 = pyxel::KEY_RETURN;
#[no_mangle]
pub static PYXEL_KEY_ESCAPE:u32 = pyxel::KEY_ESCAPE;
#[no_mangle]
pub static PYXEL_KEY_BACKSPACE:u32 = pyxel::KEY_BACKSPACE;
#[no_mangle]
pub static PYXEL_KEY_TAB:u32 = pyxel::KEY_TAB;
#[no_mangle]
pub static PYXEL_KEY_SPACE:u32 = pyxel::KEY_SPACE;
#[no_mangle]
pub static PYXEL_KEY_EXCLAIM:u32 = pyxel::KEY_EXCLAIM;
#[no_mangle]
pub static PYXEL_KEY_QUOTEDBL:u32 = pyxel::KEY_QUOTEDBL;
#[no_mangle]
pub static PYXEL_KEY_HASH:u32 = pyxel::KEY_HASH;
#[no_mangle]
pub static PYXEL_KEY_PERCENT:u32 = pyxel::KEY_PERCENT;
#[no_mangle]
pub static PYXEL_KEY_DOLLAR:u32 = pyxel::KEY_DOLLAR;
#[no_mangle]
pub static PYXEL_KEY_AMPERSAND:u32 = pyxel::KEY_AMPERSAND;
#[no_mangle]
pub static PYXEL_KEY_QUOTE:u32 = pyxel::KEY_QUOTE;
#[no_mangle]
pub static PYXEL_KEY_LEFTPAREN:u32 = pyxel::KEY_LEFTPAREN;
#[no_mangle]
pub static PYXEL_KEY_RIGHTPAREN:u32 = pyxel::KEY_RIGHTPAREN;
#[no_mangle]
pub static PYXEL_KEY_ASTERISK:u32 = pyxel::KEY_ASTERISK;
#[no_mangle]
pub static PYXEL_KEY_PLUS:u32 = pyxel::KEY_PLUS;
#[no_mangle]
pub static PYXEL_KEY_COMMA:u32 = pyxel::KEY_COMMA;
#[no_mangle]
pub static PYXEL_KEY_MINUS:u32 = pyxel::KEY_MINUS;
#[no_mangle]
pub static PYXEL_KEY_PERIOD:u32 = pyxel::KEY_PERIOD;
#[no_mangle]
pub static PYXEL_KEY_SLASH:u32 = pyxel::KEY_SLASH;

// Number keys
#[no_mangle]
pub static PYXEL_KEY_0:u32 = pyxel::KEY_0;
#[no_mangle]
pub static PYXEL_KEY_1:u32 = pyxel::KEY_1;
#[no_mangle]
pub static PYXEL_KEY_2:u32 = pyxel::KEY_2;
#[no_mangle]
pub static PYXEL_KEY_3:u32 = pyxel::KEY_3;
#[no_mangle]
pub static PYXEL_KEY_4:u32 = pyxel::KEY_4;
#[no_mangle]
pub static PYXEL_KEY_5:u32 = pyxel::KEY_5;
#[no_mangle]
pub static PYXEL_KEY_6:u32 = pyxel::KEY_6;
#[no_mangle]
pub static PYXEL_KEY_7:u32 = pyxel::KEY_7;
#[no_mangle]
pub static PYXEL_KEY_8:u32 = pyxel::KEY_8;
#[no_mangle]
pub static PYXEL_KEY_9:u32 = pyxel::KEY_9;

// Symbol keys 
#[no_mangle]
pub static PYXEL_KEY_COLON:u32 = pyxel::KEY_COLON;
#[no_mangle]
pub static PYXEL_KEY_SEMICOLON:u32 = pyxel::KEY_SEMICOLON;
#[no_mangle]
pub static PYXEL_KEY_LESS:u32 = pyxel::KEY_LESS;
#[no_mangle]
pub static PYXEL_KEY_EQUALS:u32 = pyxel::KEY_EQUALS;
#[no_mangle]
pub static PYXEL_KEY_GREATER:u32 = pyxel::KEY_GREATER;
#[no_mangle]
pub static PYXEL_KEY_QUESTION:u32 = pyxel::KEY_QUESTION;
#[no_mangle]
pub static PYXEL_KEY_AT:u32 = pyxel::KEY_AT;
#[no_mangle]
pub static PYXEL_KEY_LEFTBRACKET:u32 = pyxel::KEY_LEFTBRACKET;
#[no_mangle]
pub static PYXEL_KEY_BACKSLASH:u32 = pyxel::KEY_BACKSLASH;
#[no_mangle]
pub static PYXEL_KEY_RIGHTBRACKET:u32 = pyxel::KEY_RIGHTBRACKET;
#[no_mangle]
pub static PYXEL_KEY_CARET:u32 = pyxel::KEY_CARET;
#[no_mangle]
pub static PYXEL_KEY_UNDERSCORE:u32 = pyxel::KEY_UNDERSCORE;
#[no_mangle]
pub static PYXEL_KEY_BACKQUOTE:u32 = pyxel::KEY_BACKQUOTE;

// Letter keys
#[no_mangle]
pub static PYXEL_KEY_A:u32 = pyxel::KEY_A;
#[no_mangle]
pub static PYXEL_KEY_B:u32 = pyxel::KEY_B;
#[no_mangle]
pub static PYXEL_KEY_C:u32 = pyxel::KEY_C;
#[no_mangle]
pub static PYXEL_KEY_D:u32 = pyxel::KEY_D;
#[no_mangle]
pub static PYXEL_KEY_E:u32 = pyxel::KEY_E;
#[no_mangle]
pub static PYXEL_KEY_F:u32 = pyxel::KEY_F;
#[no_mangle]
pub static PYXEL_KEY_G:u32 = pyxel::KEY_G;
#[no_mangle]
pub static PYXEL_KEY_H:u32 = pyxel::KEY_H;
#[no_mangle]
pub static PYXEL_KEY_I:u32 = pyxel::KEY_I;
#[no_mangle]
pub static PYXEL_KEY_J:u32 = pyxel::KEY_J;
#[no_mangle]
pub static PYXEL_KEY_K:u32 = pyxel::KEY_K;
#[no_mangle]
pub static PYXEL_KEY_L:u32 = pyxel::KEY_L;
#[no_mangle]
pub static PYXEL_KEY_M:u32 = pyxel::KEY_M;
#[no_mangle]
pub static PYXEL_KEY_N:u32 = pyxel::KEY_N;
#[no_mangle]
pub static PYXEL_KEY_O:u32 = pyxel::KEY_O;
#[no_mangle]
pub static PYXEL_KEY_P:u32 = pyxel::KEY_P;
#[no_mangle]
pub static PYXEL_KEY_Q:u32 = pyxel::KEY_Q;
#[no_mangle]
pub static PYXEL_KEY_R:u32 = pyxel::KEY_R;
#[no_mangle]
pub static PYXEL_KEY_S:u32 = pyxel::KEY_S;
#[no_mangle]
pub static PYXEL_KEY_T:u32 = pyxel::KEY_T;
#[no_mangle]
pub static PYXEL_KEY_U:u32 = pyxel::KEY_U;
#[no_mangle]
pub static PYXEL_KEY_V:u32 = pyxel::KEY_V;
#[no_mangle]
pub static PYXEL_KEY_W:u32 = pyxel::KEY_W;
#[no_mangle]
pub static PYXEL_KEY_X:u32 = pyxel::KEY_X;
#[no_mangle]
pub static PYXEL_KEY_Y:u32 = pyxel::KEY_Y;
#[no_mangle]
pub static PYXEL_KEY_Z:u32 = pyxel::KEY_Z;

// Special keys
#[no_mangle]
pub static PYXEL_KEY_CAPSLOCK:u32 = pyxel::KEY_CAPSLOCK;
#[no_mangle]
pub static PYXEL_KEY_F1:u32 = pyxel::KEY_F1;
#[no_mangle]
pub static PYXEL_KEY_F2:u32 = pyxel::KEY_F2;
#[no_mangle]
pub static PYXEL_KEY_F3:u32 = pyxel::KEY_F3;
#[no_mangle]
pub static PYXEL_KEY_F4:u32 = pyxel::KEY_F4;
#[no_mangle]
pub static PYXEL_KEY_F5:u32 = pyxel::KEY_F5;
#[no_mangle]
pub static PYXEL_KEY_F6:u32 = pyxel::KEY_F6;
#[no_mangle]
pub static PYXEL_KEY_F7:u32 = pyxel::KEY_F7;
#[no_mangle]
pub static PYXEL_KEY_F8:u32 = pyxel::KEY_F8;
#[no_mangle]
pub static PYXEL_KEY_F9:u32 = pyxel::KEY_F9;
#[no_mangle]
pub static PYXEL_KEY_F10:u32 = pyxel::KEY_F10;
#[no_mangle]
pub static PYXEL_KEY_F11:u32 = pyxel::KEY_F11;
#[no_mangle]
pub static PYXEL_KEY_F12:u32 = pyxel::KEY_F12;

#[no_mangle]
pub static PYXEL_KEY_PRINTSCREEN:u32 = pyxel::KEY_PRINTSCREEN;
#[no_mangle]
pub static PYXEL_KEY_SCROLLLOCK:u32 = pyxel::KEY_SCROLLLOCK;
#[no_mangle]
pub static PYXEL_KEY_PAUSE:u32 = pyxel::KEY_PAUSE;
#[no_mangle]
pub static PYXEL_KEY_INSERT:u32 = pyxel::KEY_INSERT;
#[no_mangle]
pub static PYXEL_KEY_HOME:u32 = pyxel::KEY_HOME;
#[no_mangle]
pub static PYXEL_KEY_PAGEUP:u32 = pyxel::KEY_PAGEUP;
#[no_mangle]
pub static PYXEL_KEY_DELETE:u32 = pyxel::KEY_DELETE;
#[no_mangle]
pub static PYXEL_KEY_END:u32 = pyxel::KEY_END;
#[no_mangle]
pub static PYXEL_KEY_PAGEDOWN:u32 = pyxel::KEY_PAGEDOWN;
#[no_mangle]
pub static PYXEL_KEY_RIGHT:u32 = pyxel::KEY_RIGHT;
#[no_mangle]
pub static PYXEL_KEY_LEFT:u32 = pyxel::KEY_LEFT;
#[no_mangle]
pub static PYXEL_KEY_DOWN:u32 = pyxel::KEY_DOWN;
#[no_mangle]
pub static PYXEL_KEY_UP:u32 = pyxel::KEY_UP;
#[no_mangle]
pub static PYXEL_KEY_NUMLOCKCLEAR:u32 = pyxel::KEY_NUMLOCKCLEAR;
#[no_mangle]
pub static PYXEL_KEY_KP_DIVIDE:u32 = pyxel::KEY_KP_DIVIDE;
#[no_mangle]
pub static PYXEL_KEY_KP_MULTIPLY:u32 = pyxel::KEY_KP_MULTIPLY;
#[no_mangle]
pub static PYXEL_KEY_KP_MINUS:u32 = pyxel::KEY_KP_MINUS;
#[no_mangle]
pub static PYXEL_KEY_KP_PLUS:u32 = pyxel::KEY_KP_PLUS;
#[no_mangle]
pub static PYXEL_KEY_KP_ENTER:u32 = pyxel::KEY_KP_ENTER;
#[no_mangle]
pub static PYXEL_KEY_KP_1:u32 = pyxel::KEY_KP_1;
#[no_mangle]
pub static PYXEL_KEY_KP_2:u32 = pyxel::KEY_KP_2;
#[no_mangle]
pub static PYXEL_KEY_KP_3:u32 = pyxel::KEY_KP_3;
#[no_mangle]
pub static PYXEL_KEY_KP_4:u32 = pyxel::KEY_KP_4;
#[no_mangle]
pub static PYXEL_KEY_KP_5:u32 = pyxel::KEY_KP_5;
#[no_mangle]
pub static PYXEL_KEY_KP_6:u32 = pyxel::KEY_KP_6;
#[no_mangle]
pub static PYXEL_KEY_KP_7:u32 = pyxel::KEY_KP_7;
#[no_mangle]
pub static PYXEL_KEY_KP_8:u32 = pyxel::KEY_KP_8;
#[no_mangle]
pub static PYXEL_KEY_KP_9:u32 = pyxel::KEY_KP_9;
#[no_mangle]
pub static PYXEL_KEY_KP_0:u32 = pyxel::KEY_KP_0;
#[no_mangle]
pub static PYXEL_KEY_KP_PERIOD:u32 = pyxel::KEY_KP_PERIOD;
#[no_mangle]
pub static PYXEL_KEY_APPLICATION:u32 = pyxel::KEY_APPLICATION;
#[no_mangle]
pub static PYXEL_KEY_POWER:u32 = pyxel::KEY_POWER;
#[no_mangle]
pub static PYXEL_KEY_KP_EQUALS:u32 = pyxel::KEY_KP_EQUALS;
#[no_mangle]
pub static PYXEL_KEY_F13:u32 = pyxel::KEY_F13;
#[no_mangle]
pub static PYXEL_KEY_F14:u32 = pyxel::KEY_F14;
#[no_mangle]
pub static PYXEL_KEY_F15:u32 = pyxel::KEY_F15;
#[no_mangle]
pub static PYXEL_KEY_F16:u32 = pyxel::KEY_F16;
#[no_mangle]
pub static PYXEL_KEY_F17:u32 = pyxel::KEY_F17;
#[no_mangle]
pub static PYXEL_KEY_F18:u32 = pyxel::KEY_F18;
#[no_mangle]
pub static PYXEL_KEY_F19:u32 = pyxel::KEY_F19;
#[no_mangle]
pub static PYXEL_KEY_F20:u32 = pyxel::KEY_F20;
#[no_mangle]
pub static PYXEL_KEY_F21:u32 = pyxel::KEY_F21;
#[no_mangle]
pub static PYXEL_KEY_F22:u32 = pyxel::KEY_F22;
#[no_mangle]
pub static PYXEL_KEY_F23:u32 = pyxel::KEY_F23;
#[no_mangle]
pub static PYXEL_KEY_F24:u32 = pyxel::KEY_F24;
#[no_mangle]
pub static PYXEL_KEY_EXECUTE:u32 = pyxel::KEY_EXECUTE;
#[no_mangle]
pub static PYXEL_KEY_HELP:u32 = pyxel::KEY_HELP;
#[no_mangle]
pub static PYXEL_KEY_MENU:u32 = pyxel::KEY_MENU;
#[no_mangle]
pub static PYXEL_KEY_SELECT:u32 = pyxel::KEY_SELECT;
#[no_mangle]
pub static PYXEL_KEY_STOP:u32 = pyxel::KEY_STOP;
#[no_mangle]
pub static PYXEL_KEY_AGAIN:u32 = pyxel::KEY_AGAIN;
#[no_mangle]
pub static PYXEL_KEY_UNDO:u32 = pyxel::KEY_UNDO;
#[no_mangle]
pub static PYXEL_KEY_CUT:u32 = pyxel::KEY_CUT;
#[no_mangle]
pub static PYXEL_KEY_COPY:u32 = pyxel::KEY_COPY;
#[no_mangle]
pub static PYXEL_KEY_PASTE:u32 = pyxel::KEY_PASTE;
#[no_mangle]
pub static PYXEL_KEY_FIND:u32 = pyxel::KEY_FIND;
#[no_mangle]
pub static PYXEL_KEY_MUTE:u32 = pyxel::KEY_MUTE;
#[no_mangle]
pub static PYXEL_KEY_VOLUMEUP:u32 = pyxel::KEY_VOLUMEUP;
#[no_mangle]
pub static PYXEL_KEY_VOLUMEDOWN:u32 = pyxel::KEY_VOLUMEDOWN;
#[no_mangle]
pub static PYXEL_KEY_KP_COMMA:u32 = pyxel::KEY_KP_COMMA;
#[no_mangle]
pub static PYXEL_KEY_KP_EQUALSAS400:u32 = pyxel::KEY_KP_EQUALSAS400;
#[no_mangle]
pub static PYXEL_KEY_ALTERASE:u32 = pyxel::KEY_ALTERASE;
#[no_mangle]
pub static PYXEL_KEY_SYSREQ:u32 = pyxel::KEY_SYSREQ;
#[no_mangle]
pub static PYXEL_KEY_CANCEL:u32 = pyxel::KEY_CANCEL;
#[no_mangle]
pub static PYXEL_KEY_CLEAR:u32 = pyxel::KEY_CLEAR;
#[no_mangle]
pub static PYXEL_KEY_PRIOR:u32 = pyxel::KEY_PRIOR;
#[no_mangle]
pub static PYXEL_KEY_RETURN2:u32 = pyxel::KEY_RETURN2;
#[no_mangle]
pub static PYXEL_KEY_SEPARATOR:u32 = pyxel::KEY_SEPARATOR;
#[no_mangle]
pub static PYXEL_KEY_OUT:u32 = pyxel::KEY_OUT;
#[no_mangle]
pub static PYXEL_KEY_OPER:u32 = pyxel::KEY_OPER;
#[no_mangle]
pub static PYXEL_KEY_CLEARAGAIN:u32 = pyxel::KEY_CLEARAGAIN;
#[no_mangle]
pub static PYXEL_KEY_CRSEL:u32 = pyxel::KEY_CRSEL;
#[no_mangle]
pub static PYXEL_KEY_EXSEL:u32 = pyxel::KEY_EXSEL;
#[no_mangle]
pub static PYXEL_KEY_KP_00:u32 = pyxel::KEY_KP_00;
#[no_mangle]
pub static PYXEL_KEY_KP_000:u32 = pyxel::KEY_KP_000;
#[no_mangle]
pub static PYXEL_KEY_THOUSANDSSEPARATOR:u32 = pyxel::KEY_THOUSANDSSEPARATOR;
#[no_mangle]
pub static PYXEL_KEY_DECIMALSEPARATOR:u32 = pyxel::KEY_DECIMALSEPARATOR;
#[no_mangle]
pub static PYXEL_KEY_CURRENCYUNIT:u32 = pyxel::KEY_CURRENCYUNIT;
#[no_mangle]
pub static PYXEL_KEY_CURRENCYSUBUNIT:u32 = pyxel::KEY_CURRENCYSUBUNIT;
#[no_mangle]
pub static PYXEL_KEY_KP_LEFTPAREN:u32 = pyxel::KEY_KP_LEFTPAREN;
#[no_mangle]
pub static PYXEL_KEY_KP_RIGHTPAREN:u32 = pyxel::KEY_KP_RIGHTPAREN;
#[no_mangle]
pub static PYXEL_KEY_KP_LEFTBRACE:u32 = pyxel::KEY_KP_LEFTBRACE;
#[no_mangle]
pub static PYXEL_KEY_KP_RIGHTBRACE:u32 = pyxel::KEY_KP_RIGHTBRACE;
#[no_mangle]
pub static PYXEL_KEY_KP_TAB:u32 = pyxel::KEY_KP_TAB;
#[no_mangle]
pub static PYXEL_KEY_KP_BACKSPACE:u32 = pyxel::KEY_KP_BACKSPACE;
#[no_mangle]
pub static PYXEL_KEY_KP_A:u32 = pyxel::KEY_KP_A;
#[no_mangle]
pub static PYXEL_KEY_KP_B:u32 = pyxel::KEY_KP_B;
#[no_mangle]
pub static PYXEL_KEY_KP_C:u32 = pyxel::KEY_KP_C;
#[no_mangle]
pub static PYXEL_KEY_KP_D:u32 = pyxel::KEY_KP_D;
#[no_mangle]
pub static PYXEL_KEY_KP_E:u32 = pyxel::KEY_KP_E;
#[no_mangle]
pub static PYXEL_KEY_KP_F:u32 = pyxel::KEY_KP_F;
#[no_mangle]
pub static PYXEL_KEY_KP_XOR:u32 = pyxel::KEY_KP_XOR;
#[no_mangle]
pub static PYXEL_KEY_KP_POWER:u32 = pyxel::KEY_KP_POWER;
#[no_mangle]
pub static PYXEL_KEY_KP_PERCENT:u32 = pyxel::KEY_KP_PERCENT;
#[no_mangle]
pub static PYXEL_KEY_KP_LESS:u32 = pyxel::KEY_KP_LESS;
#[no_mangle]
pub static PYXEL_KEY_KP_GREATER:u32 = pyxel::KEY_KP_GREATER;
#[no_mangle]
pub static PYXEL_KEY_KP_AMPERSAND:u32 = pyxel::KEY_KP_AMPERSAND;
#[no_mangle]
pub static PYXEL_KEY_KP_DBLAMPERSAND:u32 = pyxel::KEY_KP_DBLAMPERSAND;
#[no_mangle]
pub static PYXEL_KEY_KP_VERTICALBAR:u32 = pyxel::KEY_KP_VERTICALBAR;
#[no_mangle]
pub static PYXEL_KEY_KP_DBLVERTICALBAR:u32 = pyxel::KEY_KP_DBLVERTICALBAR;
#[no_mangle]
pub static PYXEL_KEY_KP_COLON:u32 = pyxel::KEY_KP_COLON;
#[no_mangle]
pub static PYXEL_KEY_KP_HASH:u32 = pyxel::KEY_KP_HASH;
#[no_mangle]
pub static PYXEL_KEY_KP_SPACE:u32 = pyxel::KEY_KP_SPACE;
#[no_mangle]
pub static PYXEL_KEY_KP_AT:u32 = pyxel::KEY_KP_AT;
#[no_mangle]
pub static PYXEL_KEY_KP_EXCLAM:u32 = pyxel::KEY_KP_EXCLAM;
#[no_mangle]
pub static PYXEL_KEY_KP_MEMSTORE:u32 = pyxel::KEY_KP_MEMSTORE;
#[no_mangle]
pub static PYXEL_KEY_KP_MEMRECALL:u32 = pyxel::KEY_KP_MEMRECALL;
#[no_mangle]
pub static PYXEL_KEY_KP_MEMCLEAR:u32 = pyxel::KEY_KP_MEMCLEAR;
#[no_mangle]
pub static PYXEL_KEY_KP_MEMADD:u32 = pyxel::KEY_KP_MEMADD;
#[no_mangle]
pub static PYXEL_KEY_KP_MEMSUBTRACT:u32 = pyxel::KEY_KP_MEMSUBTRACT;
#[no_mangle]
pub static PYXEL_KEY_KP_MEMMULTIPLY:u32 = pyxel::KEY_KP_MEMMULTIPLY;
#[no_mangle]
pub static PYXEL_KEY_KP_MEMDIVIDE:u32 = pyxel::KEY_KP_MEMDIVIDE;
#[no_mangle]
pub static PYXEL_KEY_KP_PLUSMINUS:u32 = pyxel::KEY_KP_PLUSMINUS;
#[no_mangle]
pub static PYXEL_KEY_KP_CLEAR:u32 = pyxel::KEY_KP_CLEAR;
#[no_mangle]
pub static PYXEL_KEY_KP_CLEARENTRY:u32 = pyxel::KEY_KP_CLEARENTRY;
#[no_mangle]
pub static PYXEL_KEY_KP_BINARY:u32 = pyxel::KEY_KP_BINARY;
#[no_mangle]
pub static PYXEL_KEY_KP_OCTAL:u32 = pyxel::KEY_KP_OCTAL;
#[no_mangle]
pub static PYXEL_KEY_KP_DECIMAL:u32 = pyxel::KEY_KP_DECIMAL;
#[no_mangle]
pub static PYXEL_KEY_KP_HEXADECIMAL:u32 = pyxel::KEY_KP_HEXADECIMAL;
#[no_mangle]
pub static PYXEL_KEY_LCTRL:u32 = pyxel::KEY_LCTRL;
#[no_mangle]
pub static PYXEL_KEY_LSHIFT:u32 = pyxel::KEY_LSHIFT;
#[no_mangle]
pub static PYXEL_KEY_LALT:u32 = pyxel::KEY_LALT;
#[no_mangle]
pub static PYXEL_KEY_LGUI:u32 = pyxel::KEY_LGUI;
#[no_mangle]
pub static PYXEL_KEY_RCTRL:u32 = pyxel::KEY_RCTRL;
#[no_mangle]
pub static PYXEL_KEY_RSHIFT:u32 = pyxel::KEY_RSHIFT;
#[no_mangle]
pub static PYXEL_KEY_RALT:u32 = pyxel::KEY_RALT;
#[no_mangle]
pub static PYXEL_KEY_RGUI:u32 = pyxel::KEY_RGUI;
#[no_mangle]
pub static PYXEL_KEY_MODE:u32 = pyxel::KEY_MODE;
#[no_mangle]
pub static PYXEL_KEY_AUDIONEXT:u32 = pyxel::KEY_AUDIONEXT;
#[no_mangle]
pub static PYXEL_KEY_AUDIOPREV:u32 = pyxel::KEY_AUDIOPREV;
#[no_mangle]
pub static PYXEL_KEY_AUDIOSTOP:u32 = pyxel::KEY_AUDIOSTOP;
#[no_mangle]
pub static PYXEL_KEY_AUDIOPLAY:u32 = pyxel::KEY_AUDIOPLAY;
#[no_mangle]
pub static PYXEL_KEY_AUDIOMUTE:u32 = pyxel::KEY_AUDIOMUTE;
#[no_mangle]
pub static PYXEL_KEY_MEDIASELECT:u32 = pyxel::KEY_MEDIASELECT;
#[no_mangle]
pub static PYXEL_KEY_WWW:u32 = pyxel::KEY_WWW;
#[no_mangle]
pub static PYXEL_KEY_MAIL:u32 = pyxel::KEY_MAIL;
#[no_mangle]
pub static PYXEL_KEY_CALCULATOR:u32 = pyxel::KEY_CALCULATOR;
#[no_mangle]
pub static PYXEL_KEY_COMPUTER:u32 = pyxel::KEY_COMPUTER;
#[no_mangle]
pub static PYXEL_KEY_AC_SEARCH:u32 = pyxel::KEY_AC_SEARCH;
#[no_mangle]
pub static PYXEL_KEY_AC_HOME:u32 = pyxel::KEY_AC_HOME;
#[no_mangle]
pub static PYXEL_KEY_AC_BACK:u32 = pyxel::KEY_AC_BACK;
#[no_mangle]
pub static PYXEL_KEY_AC_FORWARD:u32 = pyxel::KEY_AC_FORWARD;
#[no_mangle]
pub static PYXEL_KEY_AC_STOP:u32 = pyxel::KEY_AC_STOP;
#[no_mangle]
pub static PYXEL_KEY_AC_REFRESH:u32 = pyxel::KEY_AC_REFRESH;
#[no_mangle]
pub static PYXEL_KEY_AC_BOOKMARKS:u32 = pyxel::KEY_AC_BOOKMARKS;
#[no_mangle]
pub static PYXEL_KEY_BRIGHTNESSDOWN:u32 = pyxel::KEY_BRIGHTNESSDOWN;
#[no_mangle]
pub static PYXEL_KEY_BRIGHTNESSUP:u32 = pyxel::KEY_BRIGHTNESSUP;
#[no_mangle]
pub static PYXEL_KEY_DISPLAYSWITCH:u32 = pyxel::KEY_DISPLAYSWITCH;
#[no_mangle]
pub static PYXEL_KEY_KBDILLUMTOGGLE:u32 = pyxel::KEY_KBDILLUMTOGGLE;
#[no_mangle]
pub static PYXEL_KEY_KBDILLUMDOWN:u32 = pyxel::KEY_KBDILLUMDOWN;
#[no_mangle]
pub static PYXEL_KEY_KBDILLUMUP:u32 = pyxel::KEY_KBDILLUMUP;
#[no_mangle]
pub static PYXEL_KEY_EJECT:u32 = pyxel::KEY_EJECT;
#[no_mangle]
pub static PYXEL_KEY_SLEEP:u32 = pyxel::KEY_SLEEP;
#[no_mangle]
pub static PYXEL_KEY_APP1:u32 = pyxel::KEY_APP1;
#[no_mangle]
pub static PYXEL_KEY_APP2:u32 = pyxel::KEY_APP2;
#[no_mangle]
pub static PYXEL_KEY_AUDIOREWIND:u32 = pyxel::KEY_AUDIOREWIND;
#[no_mangle]
pub static PYXEL_KEY_AUDIOFASTFORWARD:u32 = pyxel::KEY_AUDIOFASTFORWARD;
#[no_mangle]
pub static PYXEL_KEY_NONE:u32 = pyxel::KEY_NONE;
#[no_mangle]
pub static PYXEL_KEY_SHIFT:u32 = pyxel::KEY_SHIFT;
#[no_mangle]
pub static PYXEL_KEY_CTRL:u32 = pyxel::KEY_CTRL;
#[no_mangle]
pub static PYXEL_KEY_ALT:u32 = pyxel::KEY_ALT;
#[no_mangle]
pub static PYXEL_KEY_GUI:u32 = pyxel::KEY_GUI;

#[no_mangle]
pub static PYXEL_MOUSE_POS_X:u32 = pyxel::MOUSE_POS_X;
#[no_mangle]
pub static PYXEL_MOUSE_POS_Y:u32 = pyxel::MOUSE_POS_Y;
#[no_mangle]
pub static PYXEL_MOUSE_WHEEL_X:u32 = pyxel::MOUSE_WHEEL_X;
#[no_mangle]
pub static PYXEL_MOUSE_WHEEL_Y:u32 = pyxel::MOUSE_WHEEL_Y;
#[no_mangle]
pub static PYXEL_MOUSE_BUTTON_LEFT:u32 = pyxel::MOUSE_BUTTON_LEFT;
#[no_mangle]
pub static PYXEL_MOUSE_BUTTON_MIDDLE:u32 = pyxel::MOUSE_BUTTON_MIDDLE;
#[no_mangle]
pub static PYXEL_MOUSE_BUTTON_RIGHT:u32 = pyxel::MOUSE_BUTTON_RIGHT;
#[no_mangle]
pub static PYXEL_MOUSE_BUTTON_X1:u32 = pyxel::MOUSE_BUTTON_X1;
#[no_mangle]
pub static PYXEL_MOUSE_BUTTON_X2:u32 = pyxel::MOUSE_BUTTON_X2;

#[no_mangle]
pub static PYXEL_GAMEPAD1_AXIS_LEFTX:u32 = pyxel::GAMEPAD1_AXIS_LEFTX;
#[no_mangle]
pub static PYXEL_GAMEPAD1_AXIS_LEFTY:u32 = pyxel::GAMEPAD1_AXIS_LEFTY;
#[no_mangle]
pub static PYXEL_GAMEPAD1_AXIS_RIGHTX:u32 = pyxel::GAMEPAD1_AXIS_RIGHTX;
#[no_mangle]
pub static PYXEL_GAMEPAD1_AXIS_RIGHTY:u32 = pyxel::GAMEPAD1_AXIS_RIGHTY;
#[no_mangle]
pub static PYXEL_GAMEPAD1_AXIS_TRIGGERLEFT:u32 = pyxel::GAMEPAD1_AXIS_TRIGGERLEFT;
#[no_mangle]
pub static PYXEL_GAMEPAD1_AXIS_TRIGGERRIGHT:u32 = pyxel::GAMEPAD1_AXIS_TRIGGERRIGHT;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_A:u32 = pyxel::GAMEPAD1_BUTTON_A;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_B:u32 = pyxel::GAMEPAD1_BUTTON_B;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_X:u32 = pyxel::GAMEPAD1_BUTTON_X;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_Y:u32 = pyxel::GAMEPAD1_BUTTON_Y;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_BACK:u32 = pyxel::GAMEPAD1_BUTTON_BACK;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_GUIDE:u32 = pyxel::GAMEPAD1_BUTTON_GUIDE;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_START:u32 = pyxel::GAMEPAD1_BUTTON_START;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_LEFTSTICK:u32 = pyxel::GAMEPAD1_BUTTON_LEFTSTICK;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_RIGHTSTICK:u32 = pyxel::GAMEPAD1_BUTTON_RIGHTSTICK;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_LEFTSHOULDER:u32 = pyxel::GAMEPAD1_BUTTON_LEFTSHOULDER;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_RIGHTSHOULDER:u32 = pyxel::GAMEPAD1_BUTTON_RIGHTSHOULDER;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_DPAD_UP:u32 = pyxel::GAMEPAD1_BUTTON_DPAD_UP;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_DPAD_DOWN:u32 = pyxel::GAMEPAD1_BUTTON_DPAD_DOWN;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_DPAD_LEFT:u32 = pyxel::GAMEPAD1_BUTTON_DPAD_LEFT;
#[no_mangle]
pub static PYXEL_GAMEPAD1_BUTTON_DPAD_RIGHT:u32 = pyxel::GAMEPAD1_BUTTON_DPAD_RIGHT;

#[no_mangle]
pub static PYXEL_GAMEPAD2_AXIS_LEFTX:u32 = pyxel::GAMEPAD2_AXIS_LEFTX;
#[no_mangle]
pub static PYXEL_GAMEPAD2_AXIS_LEFTY:u32 = pyxel::GAMEPAD2_AXIS_LEFTY;
#[no_mangle]
pub static PYXEL_GAMEPAD2_AXIS_RIGHTX:u32 = pyxel::GAMEPAD2_AXIS_RIGHTX;
#[no_mangle]
pub static PYXEL_GAMEPAD2_AXIS_RIGHTY:u32 = pyxel::GAMEPAD2_AXIS_RIGHTY;
#[no_mangle]
pub static PYXEL_GAMEPAD2_AXIS_TRIGGERLEFT:u32 = pyxel::GAMEPAD2_AXIS_TRIGGERLEFT;
#[no_mangle]
pub static PYXEL_GAMEPAD2_AXIS_TRIGGERRIGHT:u32 = pyxel::GAMEPAD2_AXIS_TRIGGERRIGHT;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_A:u32 = pyxel::GAMEPAD2_BUTTON_A;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_B:u32 = pyxel::GAMEPAD2_BUTTON_B;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_X:u32 = pyxel::GAMEPAD2_BUTTON_X;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_Y:u32 = pyxel::GAMEPAD2_BUTTON_Y;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_BACK:u32 = pyxel::GAMEPAD2_BUTTON_BACK;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_GUIDE:u32 = pyxel::GAMEPAD2_BUTTON_GUIDE;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_START:u32 = pyxel::GAMEPAD2_BUTTON_START;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_LEFTSTICK:u32 = pyxel::GAMEPAD2_BUTTON_LEFTSTICK;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_RIGHTSTICK:u32 = pyxel::GAMEPAD2_BUTTON_RIGHTSTICK;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_LEFTSHOULDER:u32 = pyxel::GAMEPAD2_BUTTON_LEFTSHOULDER;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_RIGHTSHOULDER:u32 = pyxel::GAMEPAD2_BUTTON_RIGHTSHOULDER;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_DPAD_UP:u32 = pyxel::GAMEPAD2_BUTTON_DPAD_UP;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_DPAD_DOWN:u32 = pyxel::GAMEPAD2_BUTTON_DPAD_DOWN;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_DPAD_LEFT:u32 = pyxel::GAMEPAD2_BUTTON_DPAD_LEFT;
#[no_mangle]
pub static PYXEL_GAMEPAD2_BUTTON_DPAD_RIGHT:u32 = pyxel::GAMEPAD2_BUTTON_DPAD_RIGHT;

#[no_mangle]
pub static PYXEL_GAMEPAD3_AXIS_LEFTX:u32 = pyxel::GAMEPAD3_AXIS_LEFTX;
#[no_mangle]
pub static PYXEL_GAMEPAD3_AXIS_LEFTY:u32 = pyxel::GAMEPAD3_AXIS_LEFTY;
#[no_mangle]
pub static PYXEL_GAMEPAD3_AXIS_RIGHTX:u32 = pyxel::GAMEPAD3_AXIS_RIGHTX;
#[no_mangle]
pub static PYXEL_GAMEPAD3_AXIS_RIGHTY:u32 = pyxel::GAMEPAD3_AXIS_RIGHTY;
#[no_mangle]
pub static PYXEL_GAMEPAD3_AXIS_TRIGGERLEFT:u32 = pyxel::GAMEPAD3_AXIS_TRIGGERLEFT;
#[no_mangle]
pub static PYXEL_GAMEPAD3_AXIS_TRIGGERRIGHT:u32 = pyxel::GAMEPAD3_AXIS_TRIGGERRIGHT;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_A:u32 = pyxel::GAMEPAD3_BUTTON_A;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_B:u32 = pyxel::GAMEPAD3_BUTTON_B;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_X:u32 = pyxel::GAMEPAD3_BUTTON_X;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_Y:u32 = pyxel::GAMEPAD3_BUTTON_Y;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_BACK:u32 = pyxel::GAMEPAD3_BUTTON_BACK;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_GUIDE:u32 = pyxel::GAMEPAD3_BUTTON_GUIDE;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_START:u32 = pyxel::GAMEPAD3_BUTTON_START;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_LEFTSTICK:u32 = pyxel::GAMEPAD3_BUTTON_LEFTSTICK;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_RIGHTSTICK:u32 = pyxel::GAMEPAD3_BUTTON_RIGHTSTICK;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_LEFTSHOULDER:u32 = pyxel::GAMEPAD3_BUTTON_LEFTSHOULDER;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_RIGHTSHOULDER:u32 = pyxel::GAMEPAD3_BUTTON_RIGHTSHOULDER;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_DPAD_UP:u32 = pyxel::GAMEPAD3_BUTTON_DPAD_UP;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_DPAD_DOWN:u32 = pyxel::GAMEPAD3_BUTTON_DPAD_DOWN;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_DPAD_LEFT:u32 = pyxel::GAMEPAD3_BUTTON_DPAD_LEFT;
#[no_mangle]
pub static PYXEL_GAMEPAD3_BUTTON_DPAD_RIGHT:u32 = pyxel::GAMEPAD3_BUTTON_DPAD_RIGHT;

#[no_mangle]
pub static PYXEL_GAMEPAD4_AXIS_LEFTX:u32 = pyxel::GAMEPAD4_AXIS_LEFTX;
#[no_mangle]
pub static PYXEL_GAMEPAD4_AXIS_LEFTY:u32 = pyxel::GAMEPAD4_AXIS_LEFTY;
#[no_mangle]
pub static PYXEL_GAMEPAD4_AXIS_RIGHTX:u32 = pyxel::GAMEPAD4_AXIS_RIGHTX;
#[no_mangle]
pub static PYXEL_GAMEPAD4_AXIS_RIGHTY:u32 = pyxel::GAMEPAD4_AXIS_RIGHTY;
#[no_mangle]
pub static PYXEL_GAMEPAD4_AXIS_TRIGGERLEFT:u32 = pyxel::GAMEPAD4_AXIS_TRIGGERLEFT;
#[no_mangle]
pub static PYXEL_GAMEPAD4_AXIS_TRIGGERRIGHT:u32 = pyxel::GAMEPAD4_AXIS_TRIGGERRIGHT;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_A:u32 = pyxel::GAMEPAD4_BUTTON_A;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_B:u32 = pyxel::GAMEPAD4_BUTTON_B;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_X:u32 = pyxel::GAMEPAD4_BUTTON_X;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_Y:u32 = pyxel::GAMEPAD4_BUTTON_Y;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_BACK:u32 = pyxel::GAMEPAD4_BUTTON_BACK;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_GUIDE:u32 = pyxel::GAMEPAD4_BUTTON_GUIDE;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_START:u32 = pyxel::GAMEPAD4_BUTTON_START;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_LEFTSTICK:u32 = pyxel::GAMEPAD4_BUTTON_LEFTSTICK;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_RIGHTSTICK:u32 = pyxel::GAMEPAD4_BUTTON_RIGHTSTICK;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_LEFTSHOULDER:u32 = pyxel::GAMEPAD4_BUTTON_LEFTSHOULDER;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_RIGHTSHOULDER:u32 = pyxel::GAMEPAD4_BUTTON_RIGHTSHOULDER;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_DPAD_UP:u32 = pyxel::GAMEPAD4_BUTTON_DPAD_UP;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_DPAD_DOWN:u32 = pyxel::GAMEPAD4_BUTTON_DPAD_DOWN;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_DPAD_LEFT:u32 = pyxel::GAMEPAD4_BUTTON_DPAD_LEFT;
#[no_mangle]
pub static PYXEL_GAMEPAD4_BUTTON_DPAD_RIGHT:u32 = pyxel::GAMEPAD4_BUTTON_DPAD_RIGHT;