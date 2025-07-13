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

cargo install --git https://github.com/mohe2015/wasm-bindgen --branch emscripten-fixes wasm-bindgen-cli

# I think with this dlmalloc we will overwrite the memory used by rust? so we still need to manually specify the malloc etc function just not all functions?
em++ -g -O3 -lc -lc++ -lc++abi -ldlmalloc -r libfoo.cpp -o libfoo.o && emar rcs libfoo.a libfoo.o && wasm-pack build --dev --target web && wasm2wat pkg/rust_wasm_cpp_bg.wasm | grep "(import "

wasm-objdump -x libfoo.o | grep explicit_name

wasm2wat pkg/rust_wasm_cpp_bg.wasm > rust_wasm_cpp.wat

https://users.rust-lang.org/t/merging-a-rust-wasm-bindgen-wasm-module-and-an-emscripten-c-module/112786
https://github.com/rustwasm/wasm-bindgen/pull/4443

python3 -m http.server
```
