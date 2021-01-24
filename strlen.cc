# include <luma/main.hh>
int luma::strlen(char const * str) {
	int len = 0x0;
	while(str[len] != '\0') {
		++len;
	}
	return len;
}
