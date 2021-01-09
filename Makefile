CCFLAGS+=-Iinclude -std=c18 -Wall -Wextra -Wpedantic
CCFLAGS+=-O2
LIBS=-lxcb
SRC=\
	main.c \
	stdlibsock/gfx/crtwin.c
luma: $(SRC)
	$(CC) $(CCFLAGS) $(LIBS) $(SRC) -o $@
.PHONY: clean
clean:
	rm luma *.o
