# Toothpaste Factory (TFF)

## Formatting & Linting

```
cargo +nightly fmt
cargo clippy --all --all-targets --release
```


```
cargo run -p tpf-anals --example count
```

```
cargo run -p 
```

## Config file example

```toml
[package]
package-code = "C:/tpf/tpf/package"
package-resources = "C:/tpf/package"

[monitor]
main = 0
sub = 1
```