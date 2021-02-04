# include <luma/main.hh>
void luma::app_t::msgferr(char const * msg) {
	this->msgf(this->stderr,msg);
}
