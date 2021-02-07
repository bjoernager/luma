# include <luma/main.hh>
char const * luma::app_t::kernelstr(luma::kernel_t kernel) noexcept {
	char const * str = "";
	switch(kernel) {
	default:
		str = "Unknown";
		break;
	case luma::kernel_t::darwinos:
		str = "Dawin OS";
		break;
	case luma::kernel_t::dragonflybsd:
		str = "DragonFly BSD";
		break;
	case luma::kernel_t::freebsd:
		str = "FreeBSD";
		break;
	case luma::kernel_t::hurd:
		str = "Hurd";
		break;
	case luma::kernel_t::linux:
		str = "Linux";
		break;
	case luma::kernel_t::minix:
		str = "MINIX";
		break;
	case luma::kernel_t::netbsd:
		str = "NetBSD";
		break;
	case luma::kernel_t::openbsd:
		str = "OpenBSD";
		break;
	}
	return str;
}
