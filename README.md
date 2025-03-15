# Randar

A tool to generate completely random lidar files from the command line, written completely in rust. 

## Description

You can choose the LAS version and format, set the lateral extent and z-range, 
and the percent of points classified as ground. This was developed to aid in 
testing of various lidar formats and compression efficiency tests.

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

## Authors

Adam Subora <adam.subora@proton.me>

## Version History

* 0.1.0
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE.md file for details
