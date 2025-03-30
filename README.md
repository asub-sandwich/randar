# Randlas

A tool to generate completely random lidar files from the command line, written completely in rust. 

## Description

You can choose the LAS version and format, set the lateral extent and z-range, 
and the percent of points classified as ground. This was developed to aid in 
testing of various lidar formats and compression efficiency tests.

### Usage

```
Usage: randlas [OPTIONS] <OUTPUT>

Arguments:
  <OUTPUT>  Output filename. Format is determined by extension

Options:
  -n, --num-points <NUM_POINTS>  Number of points in cloud [default: 10000]
  -m, --minor <MINOR>            LAS Minor Version [default: 4]
  -f, --format <FORMAT>          LAS Point Format (0 - 10) [default: 1]
  -g, --ground <GROUND>          Percent (0 - 100) of points classified as Ground [default: 90]
  -s, --surface                  Make Z-values surface-like
  --hills <HILLS>                Number of hills, otherwise random. Used with surface
  -x, --x <X> <X>                W-E extent (meters) [default: 0 1000]
  -y, --y <Y> <Y>                S-N extent (meters) [default: 0 1000]
  -z, --z <Z> <Z>                Z extent (meters) [default: 0 100]
```

## Getting Started

### Deps

* Tested on Arch Linux, Debian, and Windows 11

### Installation

* Pre-built binaries
    + Debian
    + Statically linked linux binary
    + Windows

* Install from crates.io:

```bash
cargo install randlas
```

* Install from source:

```bash
git clone https://asub-sandwich/randlas.git

cd randlas

cargo build --release
```

## Future Plans / Known Issues

1. Currently, only LAS and LAZ files can be generated. I would like to add ASCII and G-Zipped ASCII files, but those are currently being made by using RapidLasso's las2txt(64) binary after generation of las files. 

2. More control over surface generation

3. Binary is not yet portable due to the use of the rand crate.

4. LAS formats (4,5,9,10) with waveforms are buggy at best, usually unreadable. I wanted to include the waveforms inside the LAS file, but support for this is not great across software. LASTools, PDAL and ArcGIS do not support this feature at least, and tbh, I'm not sure how much these formats are used anyway, especially within the Geomorph community. If there is a need, I will look into it more.

### Additonal Disclaimers

1. This generates lidar files similar to what would be taken with Aerial Lidar Scanners, that is, evenly distributed across an area. Simulation of terrestrial lidar scanners would be interesting.

2. My programming is almost entirely self taught; if you find an issue or find that I have violated a custom, please let me know! 

## Authors

Adam Subora <adam.subora@proton.me>

## Version History

* 0.1.1
    * Changed Progress Bar from TQDM to Indicatif

* 0.1.0
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE.md file for details
