<div align="center">
  <h1><code>wasm-lsp-server</code></h1>
  <p>
    <strong>A language server implementation for WebAssembly</strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <a href="https://wasm-lsp.github.io/wasm-lsp-server/wasm_language_server"><img
        src="https://img.shields.io/badge/docs-latest-blueviolet?logo=Read-the-docs&logoColor=white"
        /></a>
    <a href="https://github.com/wasm-lsp/wasm-lsp-server/actions"><img
        src="https://github.com/wasm-lsp/wasm-lsp-server/workflows/main/badge.svg"
        /></a>
    <a href="https://codecov.io/gh/wasm-lsp/wasm-lsp-server"><img
        src="https://codecov.io/gh/wasm-lsp/wasm-lsp-server/branches/main/graph/badge.svg"
        /></a>
  </p>
</div>

## Status

The server is still in an early state. It is usable but many advanced features have not yet been implemented.

## Usage

The server has not yet had a stable release. You can build and install it locally if you would like to experiment with it in the meantime.

### Installing the Server

#### Prebuilt Binaries

The easiest way to install the server is to grab one of the prebuilt binaries under [releases](https://github.com/wasm-lsp/wasm-lsp-server/releases).

#### Building from Source

First ensure you have the [rust toolchain](https://rustup.rs/) installed, then proceed as follows:

```bash
git clone https://github.com/wasm-lsp/wasm-lsp-server
cd wasm-lsp-server
cargo xtask init
cargo xtask install
```

##### Selecting the Async Runtime

The server is runtime agnostic and can be configured to run on [`async-std`](https://github.com/async-rs/async-std), [`futures`](https://github.com/rust-lang/futures-rs), [`smol`](https://github.com/smol-rs/smol), or [`tokio`](https://github.com/tokio-rs/tokio).

The table below describes how to select a runtime. The `tokio` runtime is selected by default.

| runtime     | command                                   |
| ----------- | ----------------------------------------- |
| `async-std` | `cargo xtask install --runtime=async-std` |
| `futures`   | `cargo xtask install --runtime=futures`   |
| `smol`      | `cargo xtask install --runtime=smol`      |
| `tokio`     | `cargo xtask install --runtime=tokio`     |

### Installing the Client Extension

Once the server is installed you can install the Visual Studio Code [client extension](https://github.com/wasm-lsp/vscode-wasm).

## Supported Document Types

| extension | supported | kind                                                                                                             |
| :-------: | --------- | ---------------------------------------------------------------------------------------------------------------- |
|  `.wat`   | 🗹         | [WebAssembly module definition](https://github.com/WebAssembly/spec/tree/master/interpreter#s-expression-syntax) |
|  `.wast`  | 🗹         | [WebAssembly script](https://github.com/WebAssembly/spec/tree/master/interpreter#scripts)                        |

## Supported WebAssembly Proposals

The server also supports parsing WebAssembly modules that use the following features:

#### Phase 4 (Standardization)

- 🗹 [bulk-memory-operations](https://github.com/WebAssembly/bulk-memory-operations)
- 🗹 [reference-types](https://github.com/WebAssembly/reference-types)

#### Phase 3 (Implementation)

- 🗹 [annotations](https://github.com/WebAssembly/annotations)
- 🗹 [multi-memory](https://github.com/WebAssembly/multi-memory)
- 🗹 [simd](https://github.com/WebAssembly/simd)

#### Phase 2 (Specification)

- 🗹 [exception-handling](https://github.com/WebAssembly/exception-handling)
- 🗹 [function-references](https://github.com/WebAssembly/function-references)
- 🗹 [threads](https://github.com/WebAssembly/threads)

#### Phase 1 (Proposal)

Nothing planned.

#### Phase 0 (Pre-Proposal)

Nothing planned.

## Language Server Feature Support

- 🗹 document parsing via [wasm tree-sitter grammars](https://github.com/wasm-lsp/tree-sitter-wasm)
- 🗹 document symbol provider
- 🗹 syntax error diagnostics provider
- 🗹 incremental document synchronization

## Language Server Feature Roadmap

- ☐ code action provider
- ☐ code lens provider
- ☐ completion provider
- ☐ definition provider
- ☐ document formatting (full and ranged) provider
- ☐ document highlight provider
- ☐ hover provider
- ☐ references provider
- ☐ workspace symbol provider
- ☐ semantic tokens provider
- ☐ signature help provider
- ☐ document validation
- ☐ integration with existing wasm toolchains
- ☐ implementation of the [Debug Adapter Protocol](https://microsoft.github.io/debug-adapter-protocol/)
