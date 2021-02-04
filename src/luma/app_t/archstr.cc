# include <luma/main.hh>
char const * luma::app_t::archstr(luma::arch_t arch) noexcept {
	char const * str = "";
	switch(arch) {
	default:
		str = "Unknown";
		break;
	case luma::arch_t::aarch64:
		str = "ARM64/AArch64";
		break;
	case luma::arch_t::amd64:
		str = "AMD64/x86-64";
		break;
	case luma::arch_t::ia64:
		str = "IA-64";
		break;
	case luma::arch_t::ppc64:
		str = "PPC64";
		break;
	}
	return str;
}
