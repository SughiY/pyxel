use crate::pyxel_singleton::pyxel;

#[no_mangle]
pub extern "C" fn pyxel_frame_count() -> u32 {
    pyxel().frame_count
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
pub extern "C" fn pyxel_input_text() -> *const i8 {
    pyxel().input_text.as_ptr() as *const i8
}