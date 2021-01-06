# wasm-language-server-fuzz

The `wasm-language-server-fuzz` subcrate is used for fuzzing the WebAssembly language server.

## Usage

First, you need to have `cargo-fuzz` installed.

### Listing the fuzz targets

List the different fuzz targets with the following command:

```bash
cargo fuzz list
```

### Running the fuzz target

Run the fuzz target with the following command:

```bash
cargo +nightly fuzz run FUZZ_TARGET_NAME 1> /dev/null
```

Currently this only works on linux targets.
