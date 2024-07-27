build:
  mkdir -p ./build/release/native
  cargo build --release --no-default-features
  cp ./target/release/re-cycle* ./build/release/native/
  rm ./build/release/native/re-cycle.d
  rm -rf ./build/release/native/assets
  cp -R ./assets ./build/release/native/

build-debug:
  mkdir -p ./build/debug/native
  cargo build 
  cp ./target/debug/re-cycle* ./build/debug/native/
  rm ./build/debug/native/re-cycle.d
  rm -rf ./build/debug/native/assets
  cp -R ./assets ./build/debug/native/
  cp ./target/debug/bevy_dylib* ./build/debug/native
  
build-web:
  mkdir -p ./build/release/web
  cargo build --release --target wasm32-unknown-unknown --no-default-features
  wasm-bindgen --no-typescript --target web \
    --out-dir ./build/release/web \
    --out-name "re-cycle" \
    ./target/wasm32-unknown-unknown/release/re-cycle.wasm
  cp ./index.html ./build/release/web/index.html
  cp ./restart-audio-context.js ./build/release/web/restart-audio-context.js
  mkdir -p ./build/release/web/assets/
  rm -rf ./build/release/web/assets
  cp -R ./assets* ./build/release/web/

build-web-debug:
  mkdir -p ./build/debug/web
  cargo build  --target wasm32-unknown-unknown --no-default-features
  wasm-bindgen --no-typescript --target web \
    --out-dir ./build/debug/web \
    --out-name "re-cycle" \
    ./target/wasm32-unknown-unknown/debug/re-cycle.wasm
  cp ./index.html ./build/debug/web/index.html
  mkdir -p ./build/debug/web/assets/
  mkdir -p ./build/release/web/assets/
  rm -rf ./build/debug/web/assets
  cp -R ./assets* ./build/debug/web/
  cp ./restart-audio-context.js ./build/debug/web/restart-audio-context.js
