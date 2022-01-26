cmake_minimum_required(
	VERSION
	3.22
)
project(
	luma
	VERSION
	24
	DESCRIPTION
	"Reference emulator for the luma platform."
	HOMEPAGE_URL
	"https://mandelbrot.dk/luma"
	LANGUAGES
	C
)
set(
	CMAKE_C_STANDARD
	11
)

# Disable in-souce builds:
if(
	"${PROJECT_BINARY_DIR}"
	STREQUAL
	"${PROJECT_SOURCE_DIR}"
)
   message(
	   FATAL_ERROR
	   "Building in the source tree is not allowed."
	)
endif()

# Dependencies:
find_package(
	SDL2
	REQUIRED
)

# Include directories:
include_directories(
	"${PROJECT_SOURCE_DIR}/include"
	"${SDL2_INCLUDE_DIRS}"
)

# Enable compiler warnings:
if(
	MSVC
)
	add_compile_options(
		"/W4"
		"/WX"
	)
else()
	add_compile_options(
		"-pedantic-errors"
		"-Wfatal-errors"
		"-Wall"
		"-Wconversion"
		"-Werror"
		"-Wextra"
		"-Wunreachable-code"
		"-Wunused"
	)
	if(
		"${CMAKE_CXX_COMPILER_ID}"
		STREQUAL
		"Clang"
	)
		add_compile_options(
			"-Wnewline-eof"
		)
	endif()
endif()

# Set compiler optimisations level:
if(
	CMAKE_BUILD_TYPE
	MATCHES
	Release
)
	message(
		STATUS
		"Setting optimisation level..."
	)
	if(
		MSVC
	)
		add_compile_options(
			"/Os"
		)
	else()
		add_compile_options(
			"-Os"
		)
	endif()
endif()

# Sources:
add_executable(
	luma
	"luma/src/luma_abrt.c"
	"luma/src/luma_checkEvts.c"
	"luma/src/luma_dat.c"
	"luma/src/luma_drwVram.c"
	"luma/src/luma_initDat.c"
	"luma/src/luma_initWin.c"
	"luma/src/luma_ldBank.c"
	"luma/src/luma_ldBootlder.c"
	"luma/src/luma_ldRom.c"
	"luma/src/luma_mem.c"
	"luma/src/luma_memDump.c"
	"luma/src/luma_proc.c"
	"luma/src/luma_setByte.c"
	"luma/src/luma_setDbl.c"
	"luma/src/main.c"
)

# Link Libraries:
target_link_libraries(
	luma
	${SDL2_LIBRARIES}
)
