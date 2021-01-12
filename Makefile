CC=clang
CFLAGS+=-Iinclude -std=c2x -Wall -Wextra -Wpedantic -Werror
CFLAGS+=-O3
LIBS=-lgmp -lmpfr -lxcb -lvulkan
HDR=\
	include/luma/stdlibsock/gfx.h \
	include/luma/main.h \
	include/luma/stdlibsock.h
SRC=\
	stdlibsock/gfx/crtwin.c \
	stdlibsock/gfx/destwin.c \
	main.c
OBJ=$(SRC:.c=.o)
luma.elf: $(OBJ)
	$(CC) $(CCFLAGS) $(LIBS) $(OBJ) -o $@
$(OBJ): Makefile $(HDR) $(SRC)
.PHONY: clean
clean:
	rm $(OBJ)
