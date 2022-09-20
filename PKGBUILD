pkgname=window-renicer
pkgver=0.1
pkgrel=1
makedepends=(cargo)
source=("git+https://github.com/simophin/window-renicer.git")
arch=('any')
md5sums=('SKIP')

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cd $pkgname && cargo build --frozen --release
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "$srcdir/$pkgname/target/release/$pkgname"
    install -Dm0755 -t "$pkgdir/usr/lib/systemd/user" "$srcdir/$pkgname/window-renicer.service"
    install -Dm0755 -t "$pkgdir/usr/share/kwin/scripts/" "$srcdir/$pkgname/kwin-script/window-renicer"
}