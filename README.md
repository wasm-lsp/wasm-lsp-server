<div align="center">
  <h1><code>wasm-language-server</code></h1>
  <p>
    <strong>A language server implementation for WebAssembly</strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <a href="https://wasm-lsp.github.io/wasm-language-server/wasm_language_server"><img
        src="https://img.shields.io/badge/docs-latest-blueviolet?logo=Read-the-docs&logoColor=white"
        /></a>
    <a href="https://github.com/wasm-lsp/wasm-language-server/actions"><img
        src="https://github.com/wasm-lsp/wasm-language-server/workflows/main/badge.svg"
        /></a>
  </p>
</div>

## Status

The server is still in an early state. It is usable but many advanced features have not yet been implemented.

## Usage

The server has not yet had a stable release. You can build and install it locally if you would like to experiment with it in the meantime.

### Installing the Server

First ensure you have the [rust toolchain](https://rustup.rs/) installed, then proceed as follows:

```bash
git clone --recursive https://github.com/wasm-lsp/wasm-language-server
cd wasm-language-server
git submodule update --init --recursive
cd crates/server
cargo install --path .
```

### Installing the Client Extension

Once the server is installed you can install the Visual Studio Code [client extension](https://github.com/wasm-lsp/vscode-wasm).

## Supported Document Types

| supported | extension | kind |
|:---------:|-----------|------|
| ☑ | `.wat` | [WebAssembly module definition](https://github.com/WebAssembly/spec/tree/master/interpreter#s-expression-syntax) |
| ☑ | `.wast` | [WebAssembly script](https://github.com/WebAssembly/spec/tree/master/interpreter#scripts) |
| soon | `.wit` | [WebAssembly module type](https://github.com/WebAssembly/module-types/blob/master/proposals/module-types/Overview.md) |
| soon | `.witx` | [WebAssembly API description](https://github.com/WebAssembly/WASI/blob/57744f48ec7d4e211d1542d1f56746b5cc1cf6a9/meetings/2019/WASI-09-12.md#meeting-notes) |

## Features

- [x] document parsing via [wasm tree-sitter grammars](https://github.com/wasm-lsp/)
- [x] document symbol provider
- [x] syntax error diagnostics provider


## Roadmap

- [ ] incremental document synchronization
- [ ] document validation
- [ ] code action provider
- [ ] code lens provider
- [ ] completion provider
- [ ] definition provider
- [ ] document formatting (full and ranged) provider
- [ ] document highlight provider
- [ ] hover provider
- [ ] references provider
- [ ] workspace symbol provider
- [ ] semantic tokens provider
- [ ] integration with existing wasm toolchains
- [ ] implementation of the [Debug Adapter Protocol](https://microsoft.github.io/debug-adapter-protocol/)
