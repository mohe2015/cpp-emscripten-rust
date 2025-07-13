use wasm_bindgen::prelude::*;
use js_sys::{Date, Number};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


// https://github.com/emscripten-core/emscripten/blob/4182f94222db892e16961fbbfd8097c0797d30c4/system/include/wasi/api.h#L1829
#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_close(fd: u32) -> u16 {
    panic!();
}

pub struct Ciovec {
    buf: *const u8,
    len: u32
}

#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_write(fd: u32, mut iovs: *const Ciovec, iovs_len: u32, nwritten: *mut u32) -> u16 {
    for i in 0..iovs_len {
        log(std::str::from_utf8(std::slice::from_raw_parts((*iovs).buf, (*iovs).len.try_into().unwrap())).unwrap());
        *nwritten += (*iovs).len;
        iovs = iovs.add(1);
    }
    0
}

// https://github.com/emscripten-core/emscripten/blob/4182f94222db892e16961fbbfd8097c0797d30c4/system/include/wasi/api.h#L2064
#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_read(fd: u32, mut iovs: *const Ciovec, iovs_len: u32, nread: *mut u32) -> u16 {
    panic!();
}

type __wasi_filedelta_t = i64;
type __wasi_whence_t = u8;
type __wasi_filesize_t = u64;
type __wasi_errno_t = u16;
type __wasi_size_t = u32; // maybe wrong

// https://github.com/emscripten-core/emscripten/blob/4182f94222db892e16961fbbfd8097c0797d30c4/system/include/wasi/api.h#L2150
#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_seek(fd: u32, mut offset: __wasi_filedelta_t, whence: __wasi_whence_t, newoffset: *const __wasi_filesize_t) -> u16 {
    panic!();
}

/// https://github.com/emscripten-core/emscripten/blob/df69e2ccc287beab6f580f33b33e6b5692f5d20b/system/lib/libc/emscripten_internal.h#L45
///
/// https://github.com/sqlite/sqlite-wasm/blob/7c1b309c3bd07d8e6d92f82344108cebbd14f161/sqlite-wasm/jswasm/sqlite3-bundler-friendly.mjs#L3460
#[no_mangle]
pub unsafe extern "C" fn _tzset_js(
    timezone: *mut std::os::raw::c_long,
    daylight: *mut std::os::raw::c_int,
    std_name: *mut std::os::raw::c_char,
    dst_name: *mut std::os::raw::c_char,
) {
    unsafe fn set_name(name: String, dst: *mut std::os::raw::c_char) {
        for (idx, byte) in name.bytes().enumerate() {
            *dst.add(idx) = byte as _;
        }
        *dst.add(name.len()) = 0;
    }

    fn extract_zone(timezone_offset: f64) -> String {
        let sign = if timezone_offset >= 0.0 { '-' } else { '+' };
        let offset = timezone_offset.abs();
        let hours = format!("{:02}", (offset / 60.0).floor() as i32);
        let minutes = format!("{:02}", (offset % 60.0) as i32);
        format!("UTC{sign}{hours}{minutes}")
    }

    let current_year = Date::new_0().get_full_year();
    let winter = Date::new_with_year_month_day(current_year, 0, 1);
    let summer = Date::new_with_year_month_day(current_year, 6, 1);
    let winter_offset = winter.get_timezone_offset();
    let summer_offset = summer.get_timezone_offset();

    let std_timezone_offset = winter_offset.max(summer_offset);
    *timezone = (std_timezone_offset * 60.0) as _;
    *daylight = i32::from(winter_offset != summer_offset);

    let winter_name = extract_zone(winter_offset);
    let summer_name = extract_zone(summer_offset);

    if summer_offset < winter_offset {
        set_name(winter_name, std_name);
        set_name(summer_name, dst_name);
    } else {
        set_name(winter_name, dst_name);
        set_name(summer_name, std_name);
    }
}

/// https://github.com/emscripten-core/emscripten/blob/df69e2ccc287beab6f580f33b33e6b5692f5d20b/system/lib/libc/emscripten_internal.h#L29
#[no_mangle]
pub unsafe extern "C" fn _abort_js() {
    std::process::abort();
}

#[no_mangle]
pub unsafe extern "C" fn __cxa_throw() {
    panic!();
}

// https://github.com/emscripten-core/emscripten/blob/4182f94222db892e16961fbbfd8097c0797d30c4/system/include/wasi/api.h#L1701
#[no_mangle]
pub unsafe extern "C" fn __wasi_environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> __wasi_errno_t {
    // this is probably wrong
    return 0;
}

// https://github.com/emscripten-core/emscripten/blob/4182f94222db892e16961fbbfd8097c0797d30c4/system/include/wasi/api.h#L1714
#[no_mangle]
pub unsafe extern "C" fn __wasi_environ_sizes_get(argc: *mut __wasi_size_t, argv_buf_size: *mut __wasi_size_t) -> __wasi_errno_t {
    *argc = 0;
    *argv_buf_size = 0;
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn emscripten_resize_heap() {
    panic!();
}

#[link(name = "foo", kind = "static")]
unsafe extern "C" {
    fn experiment();
}

#[wasm_bindgen]
pub fn main() {
    unsafe { experiment() };
}