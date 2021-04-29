# Maintainer: Gabriel Jensen
pkgname=luma
pkgver=20.0.0
pkgrel=1
pkgdesc="luma programming language - runtime environment"
arch=("any")
url="https://mandelbrot.dk/luma/luma"
license=("AGPL3")
makedepends=("git")
source=("git+https://mandelbrot.dk/luma/luma.git")
sha512sums=("SKIP")
build() {
	cd "$srcdir/$pkgname"
	make -j$(nproc)
}
package() {
	cd "$srcdir/$pkgname"
	make DESTDIR="$pkgdir/usr" install
}
