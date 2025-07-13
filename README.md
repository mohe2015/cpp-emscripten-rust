```
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
cd ..

# https://github.com/rustwasm/wasm-bindgen/issues/4446
# https://github.com/rustwasm/wasm-bindgen/issues/3315
# https://github.com/rustwasm/wasm-bindgen/issues/2673

# https://emscripten.org/docs/compiling/Building-Projects.html#emscripten-linker-output-files

em++ -O3 -lc -lc++ -r libfoo.cpp -o libfoo.o && emar rcs libfoo.a libfoo.o && wasm-pack build --target web && wasm2wat pkg/rust_wasm_cpp_bg.wasm | grep "(import "

em++ -O3 -lc -lc++ -lc++abi -r libfoo.cpp -o libfoo.o && emar rcs libfoo.a libfoo.o && wasm-pack build --target web || wasm2wat /home/moritz/Documents/rust-wasm-cpp/target/wasm32-unknown-unknown/release/rust_wasm_cpp.wasm | grep "(import " | wc -l

https://users.rust-lang.org/t/merging-a-rust-wasm-bindgen-wasm-module-and-an-emscripten-c-module/112786
https://github.com/rustwasm/wasm-bindgen/pull/4443


python3 -m http.server
```
