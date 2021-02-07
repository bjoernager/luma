# include <luma/main.hh>
# include <unistd.h>
[[noreturn]] void luma::app_t::exit() noexcept {
	::_exit(0x0);
}
