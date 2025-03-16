# Maintainer: Adam Subora <adam.subora@proton.me>
pkgname=randlas
pkgver=0.1.1
pkgrel=1
pkgdesc="A program to generate random lidar files."
url="https://github.com/asub-sandwich/randlas"
license=("MIT")
arch=("x86_64")
makedepends=("cargo")

pkgver() {
    (git describe --long --tags || echo "$pkgver") | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
    return 0
}

package() {
    cd ..
    usrdir="$pkgdir/usr"
    mkdir -p $usrdir
    cargo install --no-track --path . --root "$usrdir"
}

