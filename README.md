# Rails Cookies Monster

A complete test suite for Rails Cookies decryption in other languages/frameworks.

# Static cookies
If you just want to test against pre-computed cookies, you can check the `cookies` directory.

These cookies were built with the following environment variables:
```shell
ENV SECRET_KEY_BASE="rails-cookies-everywhere"
ENV CANARY_VALUE="correct-horse-battery-staple"
```
The `CANARY_VALUE` is the string you must find after decrypting the cookies.

# Rust test suite launcher

Currently, the rails-cookie-monster binary's to-do list:
- [x] Takes a single version as argument (format: `MAJOR.MINOR.PATCH`).
- [x] Checks if the docker image exists to avoid rebuilding it.
- [x] Build the associated docker image (needs to have the `rails-base` image as a base).
- [x] Runs the docker image and prints the cookies.
- [ ] Accept multiple versions in input and process them in parallel.
- [ ] Use the Docker socket to build the images instead of CLI.
- [ ] Use the Docker socket to run the container.
- [x] Pass the cookies to a [rust cookies parser library](https://github.com/rails-cookies-everywhere/rails-cookies-rust).
- [x] Check the cookie against the canary value.
- [ ] Do more with the cookies, either pass them to a FFI or a binary?

Type of commands I'd like to support later:
```shell
$ rcm --versions 8.*,^7.1 --test-command 'bun input-for-my-lib-in-js.js'
$ LD_PRELOAD=compiled_from_zig.so rcm --static --versions '*'
```

# Current automated stuff you can do:

```shell
SECRET_KEY_BASE=rails-cookies-all-around CANARY_VALUE="you must find this string in the decoded cookie" cargo run 8.0.0
```

# Current HOWTO before automatization
## 1. List all Rails releases
```shell
git tag | grep -v -E 'rc|alpha|beta|RC|PR|pre' | sort -V
```

## 2. Build a base Dockerfile with Rails
```shell
docker build -t rails-base -f Dockerfile.base .
```

## 3. Build a version-specific images
```shell 
docker build -t rails:v8.0.1 --build-arg RAILS_VERSION_TAG=v8.0.1 .
```

## 4. Get the cookies
### Run the server
```shell
docker run -p 3000:3000 rails:v8.0.1
```
### Query the server and extract the cookies
```shell
curl localhost:3000 -v 2>&1 | grep set-cookie | cut -d' ' -f3- | cut -d';' -f1
```

## 5. Check the cookies
```rust
todo!()
```
