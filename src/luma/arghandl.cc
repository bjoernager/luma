# include <cstdio>
# include <luma.hh>
# include <string>
using namespace std::literals::string_literals;
void luma::arghandl(int const argc,char const * * argv) {
	for(int pos = 0x1;pos < argc; ++pos) {
		std::string arg = argv[pos];
		if(pos == 0x1) {
			this->lumafile = arg;
		}
	}
}
