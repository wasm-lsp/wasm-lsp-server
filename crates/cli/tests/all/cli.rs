const BIN: &str = env!("CARGO_BIN_EXE_wasm-language-server");

mod invoke {
    use crate::cli::BIN;
    use assert_cmd::Command;

    #[test]
    fn server() -> anyhow::Result<()> {
        Command::cargo_bin(BIN)?.assert().success();
        Ok(())
    }

    mod flag {
        use crate::cli::BIN;
        use assert_cmd::Command;

        #[test]
        fn help() -> anyhow::Result<()> {
            Command::cargo_bin(BIN)?.arg("--help").assert().success();
            Ok(())
        }

        #[test]
        fn version() -> anyhow::Result<()> {
            Command::cargo_bin(BIN)?.arg("--version").assert().success();
            Ok(())
        }
    }

    mod server {
        mod stdio {
            use crate::cli::BIN;
            use assert_cmd::Command;
            use wasm_language_server_testing as testing;

            #[test]
            fn initialize() -> anyhow::Result<()> {
                let request = testing::lsp::initialize::request().to_string();
                let response = testing::lsp::initialize::response().to_string();
                let stdin = format!("Content-Length: {}\r\n\r\n{}", request.len(), request);
                let stdout = format!("Content-Length: {}\r\n\r\n{}", response.len(), response);
                Command::cargo_bin(BIN)?.write_stdin(stdin).assert().stdout(stdout);
                Ok(())
            }

            // FIXME: this test hangs with smol
            #[cfg(feature = "runtime-tokio")]
            #[test]
            fn newline() -> anyhow::Result<()> {
                use predicates::prelude::*;
                #[rustfmt::skip]
                const STDOUT: &str = "Content-Length: 75\r\n\r\n{\"jsonrpc\":\"2.0\",\"error\":{\"code\":-32700,\"message\":\"Parse error\"},\"id\":null}";
                #[rustfmt::skip]
                const STDERR: &str = "ERROR lspower::transport] failed to decode message: missing required `Content-Length` header\n";
                Command::cargo_bin(BIN)?
                    .write_stdin("\n")
                    .assert()
                    .stdout(STDOUT)
                    .stderr(predicate::str::ends_with(STDERR));
                Ok(())
            }
        }
    }
}
