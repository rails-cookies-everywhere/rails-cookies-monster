# Rails Cookies Monster

## Overview
Rails Cookies Monster is a comprehensive testing suite designed to validate Rails cookies decryption across different programming languages and frameworks. It helps ensure that your non-Rails applications can correctly decrypt and verify Rails-encrypted cookies.

It uses Docker under the hood to run out-of-the-box Rails apps and capture their cookies.

## Quick Start
```shell
# Clone and run with default settings
git clone https://github.com/rails-cookies-everywhere/rails-cookies-monster
cd rails-cookies-monster
cargo run "8.0.1"
```

## Requirements
- Rust toolchain (1.70 or later recommended)
- Orbstack/Docker (This is developed on an Orbstack system).
- Git

## Installation
Binaries for platforms will come when the app is more stable. For now, run from source:
```shell
# Clone the repository
git clone https://github.com/rails-cookies-everywhere/rails-cookies-monster
cd rails-cookies-monster

# Build the project
cargo run "8.0.1"
```

# Static cookies
If you just want to test against pre-computed cookies, you can check the `cookies` directory.

These cookies were built with the following environment variables:
```shell
ENV SECRET_KEY_BASE="rails-cookies-everywhere"
ENV CANARY_VALUE="correct-horse-battery-staple"
```
The `CANARY_VALUE` is the string that must be found after decrypting the cookies.

# Usage
```shell
# Set up specific cookie configuration
export SECRET_KEY_BASE="rails-cookies-all-around"
export CANARY_VALUE="you must find this string in the decoded cookie"
# Run against a specific Rails version
cargo run "8.0.1"
```

# Development Status

Currently implemented features:
- [x] Takes a version requirement as argument using [semver](https://github.com/dtolnay/semver).
- [x] Checks if the docker images exist to avoid rebuilding them.
- [x] Build the associated docker image (needs to have the `rails-base` image as a base).
- [x] Runs the docker image and prints the cookies.
- [x] Process versions in parallel.
- [x] Use the Docker socket to build the images instead of CLI.
- [ ] Use the Docker socket to run the container.
- [x] Pass the cookies to a [rust cookies parser library](https://github.com/rails-cookies-everywhere/rails-cookies-rust).
- [x] Check the cookie against the canary value.
- [ ] Do more with the cookies, either pass them to a FFI or a binary?

## Planned Features
- [ ] Parameter `--generate-static-cookies` to generate cookies for use in a test suite.
- [ ] Parameter `--use-static-cookies` to use pre-generated cookies for test.
- [ ] Parameter `--test-command` to define a command that will be used to test the cookies.
- [ ] Keep an entry available for `LD_PRELOAD` so a test library can be plugged straight away.

Examples:
```shell
$ rcm --versions "^7.0.0" --generate-static-cookies
$ rcm --versions "8.*" --versions "^7.1" --test-command 'bun lib-in-javscript.js'
$ LD_PRELOAD=compiled_from_zig.so rcm --use-static-cookies --versions '*'
```

# Contributing
Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

# License
See the [LICENSE](LICENSE) file for details.
