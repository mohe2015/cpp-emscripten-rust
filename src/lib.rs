use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Ciovec {
    buf: *const u8,
    len: u32
}

#[no_mangle]
pub unsafe extern "C" fn __wasi_fd_write(fd: u32, iovs: *const Ciovec, iovs_len: u32, nwritten: *mut u32) -> u16 {
    log(std::str::from_utf8(std::slice::from_raw_parts((*iovs).buf, (*iovs).len.try_into().unwrap())).unwrap());
    *nwritten = (*iovs).len;
    0
}

#[link(name = "foo", kind = "static")]
unsafe extern "C" {
    fn experiment();
}

#[wasm_bindgen]
pub fn main() {
    unsafe { experiment() };
}