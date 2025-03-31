# Maintainer: Adam Subora <adam.subora@proton.me>
pkgname=randlas
pkgver=1.0.0.r3.gd05779e
pkgrel=1
pkgdesc="Generate customizable but random lidar files."
url="https://github.com/asub-sandwich/randlas"
license=("MIT")
arch=("x86_64")
makedepends=("cargo")

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin" "target/release/$pkgname"
}

