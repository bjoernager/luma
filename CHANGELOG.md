# 0.2E

* Update readme
* Implement remaining data-processing instructions with register operands in Thumb
* Extend memory writes according to region
* Fix colour decoding
* Improve logging
* Update code structure
* Implement read-only memory

# 0.2D

* Reformat changelog in Markdown
* Add test program (including build script)
* Update readme (make Markdown)
* Rewrite and restructure project
* Update optimisation flags
* Depend on ctrlc, sdl2, and toml
* Set window title according to image
* Draw video memory
* Update naming convention
* Update gitignore
* Never hang on trap
* Run CPU on seperate thread
* Rework logs

# 0.2C

* Fill window according to first palette entry;

# 0.2B

* Update messages
* Fix lsr and lsl
* Fix str

# 0.2A

* Implement Thumb:
  * ldr  Rd, [Rn, Immediate5]
  * ldr  Rd, [Rn, Rm]
  * ldr  Rd, [pc, Immediate8]
  * ldr  Rd, [sp, Immediate8]
  * lsl  Rd, Rm,  Immediate5
  * svc  Immediate8
  * push Registers
  * pop  Registers
  * strh Rd, [Rn, Immediate5]
  * lsr  Rd, Rm,  Immediate5
* Update readme
* Rework instruction functions
* Improve comments
* Attach license file
* Initialise SP
* Optimise sign-extensions
* Bump dependency versions

# 0.29

* Implement Thumb:
  * mov Rd, Rm;
  * movs Rd, immediate8;
  * movs Rd, Rm;
* Update readme;
* Rename condition method to check_condition;

# 0.28

* Fix wrong license in readme;

# 0.27

* Add support for Thumb:
  * bx;
  * b{cond};
  * b;
* Fix bx;
* Rework log method;
* Improve comments;
* Update readme;
* Only survive invalid opcode traps;

# 0.26

* Support bx;
* Fix ldr|str;
* Remove logging in release builds;
* Fix version number being in decimal;
* Update readme;

# 0.25

* Support load and store instructions;
* Update messages;
* Implement some move instructions;
* Update log function;
* Add equivalent write functions;
* Fix conditional execution;
* Fix branch;
* Add device helper structure;
* Update trap print;
* Reenable overflow checks;
* License under AGPL3;
* Fix default configuration not being made;
* Update readme;
* Add spsr registers;

# 0.24

* Bump dependency versions;
* Optimise b{cond}{l}.w decoder;
* Update messages;
* Remove Application::end method;
* Add comments;
* Add function for logging;
* Use TOML for configuration;
* Depend on Serde;
* Update readme;
* Remove unused Configuration::create;

# 0.23

* Update manifest;
* Update trap function (make better use of enumerations);

# 0.22

* Survive traps;
* Use hexadecimal version numbers;
* Rewrite readme into ASCII-text;
* Don't default image;
* Add configuration file;

# 0.21

* Update version constant to include minor versions;
* Use SDL2 for windowing;
* Combine Application and Emulator structures;
* Rename opcode method to decode;

# 0.20

* Support bl;
* Update register format;
* Unify trap functions;
* Add memory read helper functions;
* Update naming convention;

# 0.1F

* Update trap function;
* Add emulator helper structure;
* Support conditional instructions;
* Set signal handlers;

# 0.1E

* Repurpose project for emulating the AGB;
* Make changelog plain-text (rename to CHANGELOG.txt);
* Write in Rust;
* Update gitignore;
* Use Git tagging;
* Update versioning: major.minor;

# 1D

* Fix logs being forced disabled.

# 1C

* Depend on SDL2.
* Remove include directory path.
* Add new instruction: DRW, CPP, STP.
* Implement more instruction: DRW, CPP, STP.
* Fix logger for CPD.
* Create window for visualising VRAM.
* Remove speed limiter.
* Unify all global variables into a struct.
* Fix luma_setDbl.
* Improve some loggers.
* Create new test program.

# 1B

* Implement more instructions.
* Update project description.
* Fix UB in signal handler.
* Require C11 instead of C99.
* Add more instructions.
* Remove sound buffer.

# 1A

* Fix version number being out of date.

# 19

* Remove old readme.
* Update memory model.
* Fix ROM loader loading ROM into wrong address.
* Update bootloader.
* Fix bootloader loaded as bank 0 (should be 1).
* Add new instructions.
* Implement more instructions.
* Rename opcode LDB to BNK.
* Rewrite instruction interpreter.
* Writes in ROM no longer succeed.
* Create SIGINT handler.

# 18

* Move all UTF-8 related code into a seperate project, *u8c*.
* Rewrite project.
* Require C99 instead of C17.
* Reformat the readme into HTML.

# 17

* Create *bin* folder in destination directory when installing.

# 16

* Remove memory leaks.
* Require C17 instead of C2x.
* Create license notices in source files.
* Create install and uninstall targets in Makefile.

# 15

* Improve Makefile.
* Improve UTF-8 encoder.
* Complete UTF-8 decoder.
* Create basic print function.

# 14

* Complete UTF-8 encoder.
* Fix #1.

# 13

* Reformat changelog to Markdown.
* Completely rework codebase (multiple times, in C, C++, Objective-C and Rust). Finally decide on C.
* Split project into three seperate projects: *libluma* (API), *luma* (interpreter), and *luma-docs* (documentation).
* Merge with *libluma*.
* Create language sample.
* Use STDC functions instead of POSIX where possible.
* Create functions for decoding and encoding UTF-8.
* Don't include entire changelog in commit message.

# 12

* Remove build artifacts.

# 11

* Compile "luma" instead of "luma.bin".
* Get input file via arguments passed to executable.

# 10

* Reformat README.html to Markdown.

# F

* Add "changelog.html" to keep track of changes.
* Remove deprecated gfx library files.
* Fix PGKBUILD version not considering version 0.
* Add new language example.
* Begin rewrite of entire codebase.<\li>
* Move old codebase into the "old" folder.
* Begin creation Luma stdlib API.
* Build "luma.bin" file instead of "luma.elf".

# E

* revert .gitignore styling
* reorganize source code structure in filesystem
* remove gfxlib in favour of language-bindings to underlying libraries
* adjust compiler optimization flags
* improve c++ stdlib replacement
* create cmd argument handler
* unite core functions in class with app data (replaces luma::dat) for easier access (no friends needed, "this->" instead of "luma::dat.")
* reformat README into HTML (temporary change, will be reformated again in later commit)

# D

* create the arch_t and kernel_t types
* use char const * instead of std::string
* use custom function instead of std::cerr and std::cout
* replace as many stdlib function with custom-made ones

# C

* redo .gitignore ifle
* clean up Makefile
* create PKGBUILD file
* create dedicated folder for language examples
* try to avoid macros where possible
* remove C relics
* create semi-working Vulkan test

# B

* readd x support but only for non-linux systems (may change in the future)
* fix makefile cxxflags

# A

* drop x support
* move codebase to c++
* rework makefile
* create simple wayland demo

# 9

* quick commit before dropping x support

# 8

* create license file
* makefile optimizations
* added readme
* added stdc version checking
* x connection handling to a different file

# 7

* make Makefile check for Makefile changes when linking not compiling

# 6

* make Makefile check for Makefile changes when making
* make a gamble and enable -O3

# 5

* fix error when compiling crtwin.c
* make makefile super nice
* update .gitignore

# 4

* change compiler to clang
* improve makefile
* create luma example file
* expanded stdlib

# 3

* readd .gitignore

# 2

* create makefile
* create simple xcb demo

# 1

* create foundation for stdlib socket

# 0

* first
