# Peter Shirley's "Raytracing in One Weekend", written in Rust.

## Building:
```shell
cargo build --release
```

## Running:
```shell
cargo run > image.ppm
```
This will produce an "image.ppm" file, which can be opened with IrfanView on Windows.

---
Things that could be added when the three books are implemented:
- Actix actors support
- CUDA and/or HIP backend
- Vulkan/OpenGL display
- IMGUI user interface to modify parameters
- Arbitrary mesh support
- OBJ or FBX (ugh) support. glTF is probably a better option
- Spectral rendering
- Full path-tracing
- Full PBR
- Better BRDF