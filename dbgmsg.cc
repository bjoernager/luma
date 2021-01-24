# include <luma/main.hh>
void luma::dbgmsg(char const * msg) {
	if constexpr(debug) {
		luma::msgerr(msg);
	}
}
