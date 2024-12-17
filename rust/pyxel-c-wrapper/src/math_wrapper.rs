use crate::pyxel_singleton::pyxel;

#[no_mangle]
pub extern "C" fn ceil(x: f64) -> i32 {
    pyxel().ceil(x)
}

#[no_mangle]
pub extern "C" fn floor(x: f64) -> i32 {
    pyxel().floor(x)
}

#[no_mangle]
pub extern "C" fn sgn(x: f64) -> i32 {
    pyxel().sgn(x)
}

#[no_mangle]
pub extern "C" fn sqrt(x: f64) -> f64 {
    pyxel().sqrt(x)
}

#[no_mangle]
pub extern "C" fn sin(deg: f64) -> f64 {
    pyxel().sin(deg)
}

#[no_mangle]
pub extern "C" fn cos(deg: f64) -> f64 {
    pyxel().cos(deg)
}

#[no_mangle]
pub extern "C" fn atan2(y: f64, x: f64) -> f64 {
    pyxel().atan2(y, x)
}

#[no_mangle]
pub extern "C" fn rseed(seed: u32) {
    pyxel().rseed(seed);
}

#[no_mangle]
pub extern "C" fn rndi(a: i32, b: i32) -> i32 {
    pyxel().rndi(a, b)
}

#[no_mangle]
pub extern "C" fn rndf(a: f64, b: f64) -> f64 {
    pyxel().rndf(a, b)
}

#[no_mangle]
pub extern "C" fn nseed(seed: u32) {
    pyxel().nseed(seed);
}

#[no_mangle]
pub extern "C" fn noise(x: f64, y: f64, z: f64) -> f64 {
    pyxel().noise(x, y, z)
}

