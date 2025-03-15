# Randar

A tool to generate completely random lidar files from the command line, written completely in rust. 

## Description

You can choose the LAS version and format, set the lateral extent and z-range, 
and the percent of points classified as ground. This was developed to aid in 
testing of various lidar formats and compression efficiency tests.

### Usage

```
Usage: randar [OPTIONS] <OUTPUT>

Arguments:
  <OUTPUT>  Output filename. Format is determined by extension

Options:
  -n, --num-points <NUM_POINTS>  Number of points in cloud [default: 10000]
  -m, --minor <MINOR>            LAS Minor Version [default: 4]
  -f, --format <FORMAT>          LAS Point Format (0 - 10) [default: 1]
  -g, --ground <GROUND>          Percent (0 - 100) of points classified as Ground [default: 90]
  -s, --surface                  Make Z-values surface-like
  -h, --hills <HILLS>            Number of hills, otherwise random. Used with surface
  -x, --x <X> <X>                W-E extent (meters) [default: 0 1000]
  -y, --y <Y> <Y>                S-N extent (meters) [default: 0 1000]
  -z, --z <Z> <Z>                Z extent (meters) [default: 0 100]
```

## Getting Started

### Deps

* las2txt (From RapidLasso's LASTools) (Optional)
* Tested on Arch Linux, should be able to be built on any OS via Cargo

### Building

Upon request, binaries can be built for distrobution, until then, build with cargo!

```bash
git clone https://asub-sandwich/randar.git

cd randar

cargo build --release
```

## Future Plans / Known Issues

1. Currently, only LAS and LAZ files can be generated. I would like to add ASCII and G-Zipped ASCII files, but those are currently being made by using RapidLasso's las2txt binary after generation of las files. 

2. More control over surface generation

3. Binary is not yet portable due to the use of the rand crate, but can be built on any system.

## Authors

Adam Subora <adam.subora@proton.me>

## Version History

* 0.1.0
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE.md file for details
