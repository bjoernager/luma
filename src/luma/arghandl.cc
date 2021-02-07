# include <cstdio>
# include <luma.hh>
# include <string>
# include <vector>
using namespace std::literals::string_literals;
void luma::arghandl(int const argc,char const * * argv) {
	std::vector<std::basic_string<char>> args;
	for(int pos = 0x1;pos < argc; ++pos) {
		args.push_back(argv[pos]);
	}
	for(std::basic_string<char> arg : args) {
		std::printf("Got argument \"%s\".\u000A",arg.c_str());
	}
}
