# Let's Build a Renderer

This repo contains the accompanying source files for the `Let's Build a Renderer` book.

## Dependencies

This project requires the [Rust programming language](https://www.rust-lang.org/).

The easiest way to get access to the dependencies is to install [the Vulkan SDK](https://vulkan.lunarg.com/sdk/home). This provides access to the Vulkan Configurator, glslangvalidator, the debug layers, and other useful tools for working with Vulkan.

The latest version of `GCC` and `CMake` will also be required for ffi bindings to C/C++ libraries.

## Development Environment Setup

Using [vscode](https://code.visualstudio.com/) and [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) with the rust-analyzer [vscode extension](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) is recommended. However, any Rust development environment you are comfortable with will work.

The [official Rust book](https://doc.rust-lang.org/book/) is a great resource if you are new to Rust.

## Running

To run a chapter's code, checkout the branch for that chapter and run it with cargo:

```bash
git checkout chapter1
cargo run --release
```
