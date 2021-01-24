# include <fcntl.h>
# include <luma/main.hh>
# include <unistd.h>
void luma::msgerr(char const * msg) {
	int pipe = ::open("/dev/stderr",O_WRONLY);
	return luma::msg(pipe,msg);
}
