# fere

A 3d rendering engine focused on realtime rendering.

## Formatting & Linting

```
cargo +nightly fmt
cargo clippy --all --all-targets --release
```

## Examples
 
To run example,

```
cargo run --release -p fere-examples --example grid 
```

## Video

```
./ffmpeg.exe -i tcp://127.0.0.1:5555 -vcodec libx264 outfile.mkv
```