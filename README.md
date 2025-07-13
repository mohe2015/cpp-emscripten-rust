```
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
cd ..

# https://github.com/rustwasm/wasm-bindgen/issues/4446

# https://emscripten.org/docs/compiling/Building-Projects.html#emscripten-linker-output-files

emcc -O3 -lc -r libfoo.cpp shim.cpp -o libfoo.o && emar rcs libfoo.a libfoo.o && emcc libfoo.o -o libfoo.js && wasm-pack build --target web && wasm2wat pkg/rust_wasm_cpp_bg.wasm | grep "(import "

https://users.rust-lang.org/t/merging-a-rust-wasm-bindgen-wasm-module-and-an-emscripten-c-module/112786
https://github.com/rustwasm/wasm-bindgen/pull/4443


python3 -m http.server
```
