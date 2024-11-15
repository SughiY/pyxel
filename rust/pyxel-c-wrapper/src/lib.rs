use std::mem::transmute;
use std::ptr::null_mut;

use pyxel::{Pyxel, PyxelCallback};

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
pub extern "C" fn cls_c(col: u8) {
    pyxel().cls(col);
}

#[no_mangle]
pub extern "C" fn circ_c(x: f64, y: f64, r: f64, col: u8) {
    pyxel().circ(x, y, r, col);
}

#[no_mangle]
pub extern "C" fn circb_c(x: f64, y: f64, r: f64, col: u8) {
    pyxel().circb(x, y, r, col);
}

#[no_mangle]
pub extern "C" fn rect_c(x: f64, y: f64, w: f64, h:f64, col: u8) {
    pyxel().rect(x, y, w, h, col);
}

#[no_mangle]
pub extern "C" fn pyxel_init(
    width: u32,
    height: u32,
) -> i32 {
    set_pyxel_instance(pyxel::init(
        width,
        height,
        None,
        None,
        None,
        None,
        None,
        None,
    ));
    0
}

#[no_mangle]
pub extern "C" fn pyxel_show() {
    pyxel().show();
}

#[no_mangle]
pub extern "C" fn pyxel_run(
    update_fn: extern "C" fn(),
    draw_fn: extern "C" fn(),
) {
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