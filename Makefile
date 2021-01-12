CC=clang
CCFLAGS+=-Iinclude -std=c2x -Wall -Wextra -Wpedantic -Werror
CCFLAGS+=-O2
LIBS=-lgmp -lmpfr -lxcb -lvulkan
SRC=\
	include/luma/stdlibsock.h \
	include/luma/stdlibsock/gfx.h \
	main.c \
	stdlibsock/gfx/crtwin.c \
	stdlibsock/gfx/destwin.c
luma-testa: Makefile $(SRC)
	$(CC) $(CCFLAGS) $(LIBS) $(SRC)
.PHONY: clean
clean:
	rm luma *.gch
