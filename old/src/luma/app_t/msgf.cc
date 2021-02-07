# include <cstddef>
# include <luma/main.hh>
# include <unistd.h>
void luma::app_t::msgf(int pipe,char const * msg,std::size_t count) {
	if(::write(pipe,msg,count) < 0x0) {
		// ???
	}
}
