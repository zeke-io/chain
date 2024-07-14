# Installation

## Windows

You can find an installer for each version in [Releases](https://github.com/zeke-io/crafty/releases).

## Manual Installation (from source)

### Requirements

- Rust & Cargo (at least version 1.70)
- [cargo-wix](https://crates.io/crates/cargo-wix) (Windows)
- [WiX v3](https://wixtoolset.org/docs/wix3/) (Windows)

### Build the project

1. Clone the repository
2. Open a terminal inside the cloned repository
    ```bash
    cd crafty
    ```
3. Build the project
    ```bash
    cargo build --release
    ```

You can find the binaries at `target/release`.

### Build the installer (Windows)

To build the Crafty installer for Windows, you need to run:

```bash
cargo wix
```

*If wix is not in your system's PATH, you can add the `-b` argument followed by the path to the `bin` folder where `wix`
and `candle` are located.*

You can find the installer at `target/wix`.
