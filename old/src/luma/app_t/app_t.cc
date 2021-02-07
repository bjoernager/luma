# include <fcntl.h>
# include <iostream>
# include <luma/main.hh>
# include <unistd.h>
luma::app_t::app_t(int const argc,char const * * argv) {
	this->stderr = ::open("/dev/stderr",O_WRONLY);
	this->stdout = ::open("/dev/stdout",O_WRONLY);
	if(argc < 0x2) {
		this->msgferr("Missing argument \"file\".\n");
		this->msgferr("Use \"--help\" for help.\n");
		this->exit();
	}
	//else {
	//	for(int i = 0x0;i < argc;++i) {
	//		this->arghandl(argv[i]);
	//	}
	//}
	this->msgfout(this->archstr(this->arch));
	this->msgfout(this->kernelstr(this->kernel));
	if(!::access(argv[0x1],R_OK)) {
	 	int lumafile = ::open(argv[0x1],O_RDONLY);
		for(int line = 0x0,pos = 0x0;;++pos) {
			char * tok;
			::read(lumafile,&tok,0x1);
			this->msgfout("Here?\n");
			std::cout << line << ":" << pos << "=" << "\u000A";
			int pipe = ::open("/dev/stdout",O_WRONLY);
			this->msgf(pipe,tok,0x4);
			if(this->strcmp(tok,"\u000A") == 0x0) {
				++line;
			}
			else if(this->strcmp(tok,"\u0061") == 0x0) {
				this->msgfout("\'a\' detected!");
			}
		}
		/*
		char const * toks[] = {
			"\u0009", // Horizontal Tabulation
			"\u000A", // New Line (Nl)
			"\u0020", // Space
			"\u0023", // Number Sign
			"\u0028", // Left Parenthesis
			"\u0029", // Right Parenthesis
			"\u002B", // Plus Sign
			"\u003C", // Less-Than Sign
			"\u003D", // Equals Sign
			"\u003E", // Greater-Than Sign
			"\u0061", // Latin Small Letter a
			"\u0062", // Latin Small Letter b
			"\u0063", // Latin Small Letter c
			"\u0064", // Latin Small Letter d
			"\u0065", // Latin Small Letter e
			"\u0066", // Latin Small Letter f
			"\u0067", // Latin Small Letter g
			"\u0068", // Latin Small Letter h
			"\u0069", // Latin Small Letter i
			"\u006a", // Latin Small Letter j
			"\u006B", // Latin Small Letter k
			"\u006C", // Latin Small Letter l
			"\u006D", // Latin Small Letter m
			"\u006E", // Latin Small Letter n
			"\u006F", // Latin Small Letter o
			"\u0070", // Latin Small Letter p
			"\u0071", // Latin Small Letter q
			"\u0072", // Latin Small Letter r
			"\u0073", // Latin Small Letter s
			"\u0074", // Latin Small Letter t
			"\u0075", // Latin Small Letter u
			"\u0076", // Latin Small Letter v
			"\u0077", // Latin Small Letter w
			"\u0078", // Latin Small Letter x
			"\u0079", // Latin Small Letter y
			"\u007A", // Latin Small Letter z
			"\u00D7", // Multiplication Sign
			"\u00F7", // Division Sign
			"\u201C", // Left Double Quotation Mark
			"\u201D", // Right Double Quotation Mark
			"\u218A", // Turned Digit Two
			"\u218B", // Turned Digit Three
			"\u2212", // Minus Sign
			"\u2217", // Asterisk Operator
			"\u2260", // Not Equal To
			"\u2264", // Less-Than or Equal To
			"\u2265", // Greater-Than or Equal To
		};
		*/
	}
	else {
		this->msgferr("The file doesn\'t exist.\n");
		this->exit();
	}
}
luma::app_t::~app_t() {
	this->exit();
}
