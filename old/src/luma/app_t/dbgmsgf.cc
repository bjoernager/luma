# include <luma/main.hh>
void luma::app_t::dbgmsgf(char const * msg) {
	if constexpr(debug) {
		this->msgferr(msg);
	}
}
