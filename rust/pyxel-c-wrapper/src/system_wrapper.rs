#[cfg(not(target_os = "emscripten"))]
use sysinfo::{Pid, System};

use pyxel::{Pyxel, PyxelCallback, KEY_UNKNOWN};
use crate::pyxel_singleton::{pyxel, set_pyxel_instance};

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

#[no_mangle]
pub extern "C" fn pyxel_show() {
    pyxel().show();
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
pub extern "C" fn pyxel_integer_scale(enabled: bool) {
    pyxel().integer_scale(enabled);
}

#[no_mangle]
pub extern "C" fn pyxel_screen_mode(scr: u32) {
    pyxel().screen_mode(scr);
}

#[no_mangle]
pub extern "C" fn pyxel_fullscreen(enabled: bool) {
    pyxel().fullscreen(enabled);
}

#[cfg(not(target_os = "emscripten"))]
#[no_mangle]
pub extern "C" fn process_exists(pid: u32) -> bool {
    let system = System::new_all();
    system.process(Pid::from_u32(pid)).is_some()
}