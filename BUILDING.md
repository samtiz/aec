# Building

```console
git clone https://github.com/thewh1teagle/aec --recursive
cd aec-rs
cargo build
```

Useful website for comparesion:

https://fjiang9.github.io/NKF-AEC/

## Publish crates

```console
cd crates/aec-rs-sys
cargo publish
cd ../../
cargo publish
```

## Build for Android

You must install NDK from Android Studio settings.

```console
rustup target add aarch64-linux-android
cargo install cargo-ndk
export NDK_HOME="$HOME/Library/Android/sdk/ndk/$(ls -1 $HOME/Library/Android/sdk/ndk | sort | tail -n 1)"
export CMAKE_TOOLCHAIN_FILE="$NDK_HOME/build/cmake/android.toolchain.cmake"
cargo ndk -t arm64-v8a build --release -p libaec
```

## Build for IOS

Install Xcode command line tools

```console
xcode-select --install
```

Install toolchain

```console
# IOS
rustup target add aarch64-apple-ios
# Intel chip emulator
rustup target add x86_64-apple-ios
# Apple chip emulator
rustup target add aarch64-apple-ios-sim
```

Build

```console
cargo build --release --target aarch64-apple-ios
```

## Build for wasm

Use [wasm-pack](https://rustwasm.github.io/docs/wasm-pack) with [emscripten.org](https://emscripten.org)

```console
brew install emscripten
rustup target add wasm32-unknown-emscripten
cargo build --release --target wasm32-unknown-emscripten
CC=emcc AR=emar wasm-pack build
```

## Build pyaec (Python)

Use [uv](https://astral.sh/blog/uv)

```console
cargo build -p libaec --release
cp -rf ../target/release/libaec.dylib src/pyaec/
WHEEL_TAG="py3-none-macosx_11_0_arm64" uv build
```

Publish

```console
export UV_PUBLISH_TOKEN="..."
uv publish
```

Installing once published

```console
uv run --with pyaec --refresh-package pyaec --no-project -- python -c "import pyaec"
```

---

## Build for Glass

This guide explains how to build the `aec.js` and `aec.wasm` modules required by the Glass desktop application.

### Prerequisites

Ensure you have Rust, Cargo, and the Emscripten SDK (`emcc`) installed and activated in your current shell environment. The sections above provide guidance on installing these tools.

### Steps

1.  **Compile Rust and C dependencies to static libraries:**

    This command compiles the Rust wrapper and the underlying `speexdsp` C library for the `wasm32-unknown-emscripten` target, which prepares the code for use in web environments. Navigate to the root of the `aec` submodule directory and run:

    ```console
    cargo build --release --target wasm32-unknown-emscripten
    ```

2.  **Prepare a directory for linking:**

    For a clean build, it's best to create a dedicated directory for the final linking stage. This command sequence creates a `link` directory and copies the necessary static library artifacts (`.a` files) into it.

    ```console
    cd target/wasm32-unknown-emscripten/release
    mkdir -p link
    cd link
    cp ../libaec_rs.a .
    cp ../build/aec-rs-sys-*/out/lib/libspeexdsp.a .
    ```
    *Note: The wildcard `*` in `aec-rs-sys-*` handles the unique hash generated during the build process.*

3.  **Link static libraries into a single JS module using Emscripten:**

    Now, use `emcc` to link the two static libraries (`libaec_rs.a` and `libspeexdsp.a`) into a final, self-contained JavaScript module.

    The `-sSINGLE_FILE=1` flag is critical here: it embeds the WebAssembly binary directly into the JavaScript file as a Base64 string, eliminating the need to manage a separate `.wasm` file.

    ```console
    emcc libaec_rs.a libspeexdsp.a -O3 \
      -sSINGLE_FILE=1 \
      -sENVIRONMENT=web,worker \
      -sMODULARIZE=1 \
      -sEXPORT_NAME="createAecModule" \
      -sEXPORTED_FUNCTIONS='["_AecNew","_AecCancelEcho","_AecDestroy","_malloc","_free"]' \
      -sEXPORTED_RUNTIME_METHODS='["ccall","cwrap"]' \
      -o aec.js
    ```
    This command will generate a single, self-contained `aec.js` file in the `link` directory.

4.  **Copy the final artifact to the Glass project:**

    Finally, copy the generated `aec.js` file from the `link` directory to the main `Glass` application's assets folder.

    ```console
    cp aec.js ../../../../../src/assets/
    ```

After completing these steps, the `Glass` application will have the latest custom-built version of the AEC module.
