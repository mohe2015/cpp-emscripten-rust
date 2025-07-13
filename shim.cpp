#include <stdint.h>
#include <stddef.h>

// https://github.com/Spxg/sqlite-wasm-rs/blob/e151a2c230a8c826e5d500af607bb37b8ec2c467/sqlite-wasm-rs/shim/wasm-shim.c#L27C10-L27C27
// https://github.com/emscripten-core/emscripten/blob/4182f94222db892e16961fbbfd8097c0797d30c4/system/include/wasi/api.h#L2206

typedef struct __wasi_ciovec_t {
    const uint8_t * buf;
    size_t buf_len;
} __wasi_ciovec_t;

extern "C" uint16_t MOVEDTORUST__wasi_fd_write(
    uint32_t fd,

    /**
     * List of scatter/gather vectors from which to retrieve data.
     */
    const __wasi_ciovec_t *iovs,

    /**
     * The length of the array pointed to by `iovs`.
     */
    size_t iovs_len,

    /**
     * The number of bytes written.
     */
    size_t *nwritten
) {
    return 0;
}
