# include <luma/main.hh>
# include <unistd.h>
void luma::app_t::msgf(int pipe,char const * msg) {
	if(::write(pipe,msg,this->strlen(msg)) < 0x0) {
		// ???
	}
}
