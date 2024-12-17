use crate::pyxel_singleton::pyxel;

#[no_mangle]
pub extern "C" fn btn(key: u32) -> bool {
    pyxel().btn(key)
}

#[no_mangle]
pub extern "C" fn btnp(key: u32, hold: u32, repeat: u32) -> bool {
    pyxel().btnp(
        key,
        if hold == 0 { None } else { Some(hold) },
        if repeat == 0 { None } else { Some(repeat) },
    )
}

#[no_mangle]
pub extern "C" fn btnr(key: u32) -> bool {
    pyxel().btnr(key)
}

#[no_mangle]
pub extern "C" fn btnv(key: u32) -> i32 {
    pyxel().btnv(key)
}

#[no_mangle]
pub extern "C" fn mouse(visible: bool) {
    pyxel().mouse(visible);
}

#[no_mangle]
pub extern "C" fn warp_mouse(x: f64, y: f64) {
    pyxel().warp_mouse(x, y);
}
