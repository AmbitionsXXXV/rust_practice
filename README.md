## [pi_prev](https://github.com/AmbitionsXXXV/rust_practice/blob/main/pi_prev/src/main.rs)

```shell
cargo run -p pi_prev
```

## [simple_http_cli](https://github.com/AmbitionsXXXV/rust_practice/blob/main/simple_http_cli/src/main.rs)

```shell
cargo run -p simple_http_cli -- --url <a real website url>
```

## [rust_httpie](https://github.com/AmbitionsXXXV/rust_practice/tree/main/rust_httpie)

```shell
cargo run -p rust_httpie -- <http method(only support get&post)> <url>

# or

cargo build -p rust_httpie --release

cd target/release/rust_httpie
./rust_httpie <http method(only support get&post)> <url>
```