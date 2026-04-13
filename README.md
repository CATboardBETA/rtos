# (Currently Unnamed) Operating System

## AI Policy

No AI was used in the development of this project. Contributions using AI will be rejected. Do not try to circumvent this policy.

## Contributing

Reading [CONTRIBUTING.md](CONTRIBUTING.md), and raise an issue! In case you missed it there but see it here, read [STYLE.md](STYLE.md)

If you need help, you can message me @ `catboardbeta` on Discord.

If you want to make direct code/asset contributions, fork the repository and create a PR. Don't forget to add yourself to [CONTRIBUTORS.md](CONTRIBUTORS.md).

## Building instructions

Make sure you have `xorriso`, `qemu`, `limine`, and `rust` in your `PATH`. You'll need to install the `x86_64-none-none.json` and `aarch64-none-none.json`  targets from the `runner` directory. Currently, these are copies of the `-unknown-none` equivalents, but may be changed in the future (likely with an actual os name).

The only command to actually run it is
```sh
cargo r # for x86_64
# or
cargo r --features aarch # for aarch64
```

This runs the `runner` package, which is supplied an iso from its build script.
