# [Glow](https://github.com/grovesNL/glow) + [SPIRV-Cross](https://github.com/grovesNL/spirv_cross) Example

Based on [glow example](https://github.com/grovesNL/glow/tree/master/examples/hello).

# How to Build

## Native (OpenGL)

To run with `OpenGL 4.1`:

```shell
cargo run --features=gl
```

## Web (WebGL)

To run with `WebGL 2`:

```shell
cargo web start --features webgl --target wasm32-unknown-unknown
```
