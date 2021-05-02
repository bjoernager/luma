# 1↊

* Remove memory leaks.
* Require C17 instead of C2x.
* Create license notices in source files.
* Create install and uninstall targets in Makefile.

# 19

* Improve Makefile.
* Improve UTF-8 encoder.
* Complete UTF-8 decoder.
* Create basic print function.

# 18

* Complete UTF-8 encoder.
* Fix #1.

# 17

* Reformat changelog to Markdown.
* Completely rework codebase (multiple times, in C, C++, Objective-C and Rust). Finally decide on C.
* Split project into three seperate projects: *libluma* (API), *luma* (interpreter), and *luma-docs* (documentation).
* Merge with *libluma*.
* Create language sample.
* Use STDC functions instead of POSIX where possible.
* Create functions for decoding and encoding UTF-8.
* Don't include entire changelog in commit message.

# 16

* Remove build artifacts.

# 15

* Compile "luma" instead of "luma.bin".
* Get input file via arguments passed to executable.

# 14

* Reformat README.html to Markdown.

# 13

* Add "changelog.html" to keep track of changes.
* Remove deprecated gfx library files.
* Fix PGKBUILD version not considering version 0.
* Add new language example.
* Begin rewrite of entire codebase.<\li>
* Move old codebase into the "old" folder.
* Begin creation Luma stdlib API.
* Build "luma.bin" file instead of "luma.elf".

# 12

* revert .gitignore styling
* reorganize source code structure in filesystem
* remove gfxlib in favour of language-bindings to underlying libraries
* adjust compiler optimization flags
* improve c++ stdlib replacement
* create cmd argument handler
* unite core functions in class with app data (replaces luma::dat) for easier access (no friends needed, "this->" instead of "luma::dat.")
* reformat README into HTML (temporary change, will be reformated again in later commit)

# 11

* create the arch_t and kernel_t types
* use char const * instead of std::string
* use custom function instead of std::cerr and std::cout
* replace as many stdlib function with custom-made ones

# 10

* redo .gitignore ifle
* clean up Makefile
* create PKGBUILD file
* create dedicated folder for language examples
* try to avoid macros where possible
* remove C relics
* create semi-working Vulkan test

# ↋

* readd x support but only for non-linux systems (may change in the future)
* fix makefile cxxflags

# ↊

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
