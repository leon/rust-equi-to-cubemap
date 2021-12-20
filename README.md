# Equirectangular to cube side converter

takes a 360° equirectangular image, and converts it to all cube sides.
each cube side will be `image_width / 4` in size

## Setup

Make sure you are running the latest rust version

```
rustup update
```

## Test

Will output the cube sides in `out/` directory

```bash
cargo test
```

## Build and test release

Will output cube sides in same directory as original

```bash
cargo build --release
./target/release/equi_to_cubemap assets/pano3072.jpg
```
