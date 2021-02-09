# include <cstdio>
# include <cstdlib>
# include <luma.hh>
# include <fcntl.h>
# include <sys/stat.h>
# include <unistd.h>
[[noreturn]] luma::luma(int const argc,char const * * argv) {
	this->arghandl(argc,argv);
	std::printf("Will open \"%s\".\u000A",this->lumafile.c_str());
	int file      = ::open(this->lumafile.c_str(),O_RDONLY);
	char tok      = '\0';
	char word[0x1000];
	int wordpos = 0x0;
	struct ::stat fst;
	::fstat(file,&fst);
	printf("Size of file is %ld characters.\u000A",fst.st_size);
	::sleep(0x1);
	for(int filepos = 0x0;filepos < fst.st_size;++filepos) {
		if(::read(file,&tok,0x1) < 0x0) {
			std::printf("Error reading file.\u000A");
			::_exit(EXIT_FAILURE);
		}
		if(tok == '\u000A' || tok == '\u0020') {
			if(wordpos == 0x0) continue;
			word[wordpos] = '\0';
			std::printf("The word is \"%s\".\u000A",word);
			word[0x0] = '\0';
			wordpos   = 0x0;
		}
		else {
			word[wordpos] = tok;
			std::printf("Got character '%c' and set word[%d] to it.\u000A",tok,wordpos);
			++wordpos;
		}
	}
	::_exit(EXIT_SUCCESS);
}
