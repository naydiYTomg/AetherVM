# AetherVM

AetherVM is a custom-built virtual machine designed to execute bytecode compiled from the Omnia programming language. It is optimized for performance, portability, and extensibility, with plans to support additional languages in the future, including both custom and existing ones.

This repository also includes a **bytecode-to-machine-code translator/compiler**, allowing for efficient conversion of AetherVM bytecode into native virtual machine code.
## Key Features

- **High Performance**: Engineered to execute bytecode with minimal overhead and maximum efficiency.
- **Portability**: Lightweight and cross-platform design for various devices and operating systems.
- **Multi-language Support**: Future-proof architecture designed to accommodate multiple programming languages.
- **Optimized for Omnia**: Fully supports the unique features and capabilities of the Omnia language.
- **Extensibility**: Modular design allows for easy integration of new features and languages.
- **Debugging Tools**: Built-in support for bytecode inspection and runtime debugging.

## Installation
First of all, you must have the [Rust](https://www.rust-lang.org/) programming language installed on your device, or the **cargo** tool separately

AetherVM is currently under active development. There are two ways to use the virtual machine:

### 1. Building from source:
1. Clone the repository:
    ```bash
    git clone https://github.com/naydiYTomg/AetherVM.git
    cd aethervm
    ```
2. Build the virtual machine:
    ```bash
    cargo build --release
    ```
3. Enjoy AetherVM

    You can find binary in `./target/release/`
### 2. Download built binary:
1. Download latest release:

   Go to [Releases](https://github.com/naydiYTomg/AetherVM/releases) page and download latest
2. Enjoy AetherVM

## Usage
First of all, you need to add AetherVM binary to PATH.

Simple usage:
```bash
aethervm run your_bytecode_file.abc
```
To see all available commands and their descriptions, simply type `aethervm`

## Development Roadmap
- Full implementation of the Omnia bytecode specification.
- Support for additional programming languages, both custom and existing.
- Optimized just-in-time (JIT) compilation for enhanced performance.
- Advanced debugging and profiling tools.
- Expanded platform support for a wider range of architectures.
## Contributing
We welcome contributions to our project! To get started:
1. Read the CONTRIBUTING.md guide
2. Explore open issues in [Issues](https://github.com/naydiYTomg/AetherVM/issues)
3. Submit a Pull Request with your changes

# AetherVM - the heart of your language ecosystem.