pkgname=window-renicer
pkgver=0.1
pkgrel=1
makedepends=(cargo)
source=("git+https://github.com/simophin/window-renicer.git")
arch=('any')

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
    install -Dm0755 -t "$pkgdir/usr/lib/systemd/user" "window-renicer.service"
    install -Dm0755 -t "$pkgdir/usr/share/kwin/scripts/" kwin-script/window-renicer
}