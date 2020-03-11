# f3 Rust Quickstart
> A template for building Rust applications for STM32F303DISCOVERY (`f3` for short) development board

This is based on the [cortex-m-quickstart][original-template] project.

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31 toolchain or newer.

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Using this template

1. Instantiate the template.

``` console
$ cargo generate --git https://github.com/tomasz-rozanski/f3-rust-quickstart
```

You will be asked for a project name:
``` console
 Project Name: <name>
 Creating project called `name`...
 Done! New project created /tmp/name

$ cd <name>
```

2. Build the application

``` console
$ cargo build
```

[original-template]: https://github.com/rust-embedded/cortex-m-quickstart.git