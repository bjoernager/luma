# include <fcntl.h>
# include <luma/main.hh>
# include <unistd.h>
void luma::msgout(char const * msg) {
	int pipe = ::open("/dev/stdout",O_WRONLY);
	return luma::msg(pipe,msg);
}
