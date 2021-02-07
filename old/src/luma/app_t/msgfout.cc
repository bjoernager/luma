# include <luma/main.hh>
void luma::app_t::msgfout(char const * msg) {
	this->msgf(this->stdout,msg,this->strlen(msg));
}
