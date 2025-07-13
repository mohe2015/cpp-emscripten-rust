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
pub unsafe extern "C" fn __wasi_fd_write(fd: u32, mut iovs: *const Ciovec, iovs_len: u32, nwritten: *mut u32) -> u16 {
    for i in 0..iovs_len {
        log(std::str::from_utf8(std::slice::from_raw_parts((*iovs).buf, (*iovs).len.try_into().unwrap())).unwrap());
        *nwritten += (*iovs).len;
        iovs = iovs.add(1);
    }
    0
}

// https://github.com/Spxg/sqlite-wasm-rs/blob/e151a2c230a8c826e5d500af607bb37b8ec2c467/sqlite-wasm-rs/src/shim.rs#L187
const ALIGN: usize = std::mem::size_of::<usize>() * 2;

#[no_mangle]
pub unsafe extern "C" fn __libc_malloc(size: usize) -> *mut u8 {
    malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align_unchecked(size + ALIGN, ALIGN);
    let ptr = std::alloc::alloc(layout);

    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    *ptr.cast::<usize>() = size;

    ptr.add(ALIGN)
}

#[no_mangle]
pub unsafe extern "C" fn __libc_free(ptr: *mut u8) {
    free(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    let ptr = ptr.sub(ALIGN);
    let size = *(ptr.cast::<usize>());

    let layout = std::alloc::Layout::from_size_align_unchecked(size + ALIGN, ALIGN);
    std::alloc::dealloc(ptr, layout);
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    let ptr = ptr.sub(ALIGN);
    let size = *(ptr.cast::<usize>());

    let layout = std::alloc::Layout::from_size_align_unchecked(size + ALIGN, ALIGN);
    let ptr = std::alloc::realloc(ptr, layout, new_size + ALIGN);

    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    *ptr.cast::<usize>() = new_size;

    ptr.add(ALIGN)
}

#[no_mangle]
pub unsafe extern "C" fn __libc_calloc(num: usize, size: usize) -> *mut u8 {
    calloc(num, size)
}

#[no_mangle]
pub unsafe extern "C" fn calloc(num: usize, size: usize) -> *mut u8 {
    let total = num * size;
    let ptr = malloc(total);
    if !ptr.is_null() {
        std::ptr::write_bytes(ptr, 0, total);
    }
    ptr
}

#[link(name = "foo", kind = "static")]
unsafe extern "C" {
    fn experiment();
}

#[wasm_bindgen]
pub fn main() {
    unsafe { experiment() };
}