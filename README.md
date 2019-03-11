# WinDivvun

The speller service for Windows.

## Compile

1. Install the Rust toolchain with rustup.rs - use `nightly-i686-pc-window-msvc` or you will have a bad time.
2. Install Visual Studio with the C++ Compiler and the Windows 10 SDK enabled, or just the Windows build tools.
3. Install rust nightly for Windows https://github.com/rust-lang-nursery/rustup.rs/#other-installation-methods
4. Copy .env_sample to .env and customize the Sentry URL
5. Open up the x86 compilation Visual Studio command line and run `cargo build`.
