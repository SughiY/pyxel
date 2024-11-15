use std::mem::transmute;
use std::ptr::null_mut;

use pyxel::Pyxel;

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
pub extern "C" fn circ(x: f64, y: f64, r: f64, col: u8) {
    pyxel().circ(x, y, r, col);
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