# include <fcntl.h>
# include <luma/main.hh>
# include <unistd.h>
void luma::msg(int pipe,char const * msg) {
	if(::write(pipe,msg,luma::strlen(msg)) > 0x0) {
	}
}
