- LUMA

Copyright 2021-2023 Gabriel Jensen.

This program is free software: you can redistribute it and/or modify it under 
the terms of the GNU General Public License as published by the Free Software 
Foundation, either version 3 of the License, or (at your option) any later 
version.

This program is distributed in the hope that it will be useful, but WITHOUT 
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS 
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with 
this program. If not, see <https://www.gnu.org/licenses/>. 

- ABOUT

luma is an emulator for the AGB - Game Boy Advance - platform.

- USAGE

luma [image] [bootloader]

Invoke the emulator via the 'luma' command.

- CONFIGURATION

The emulator tries to read the configuration file at '${HOME}/.luma.toml'. If 
this file is found, the following fields are read (if present):

luma:
 * version:    (Integer) The version of the configuration file (0)

device:
 * bootloader: (String)  The path to the bootloader file (home-relative)
 * image:      (String)  The path to the image file (home-relative)

video:
 * scale:      (Integer) The scale modifier applied to the screen (min 1; max (2^32-1))

These settings are overwritten by terminal parameters (see USAGE).

- COMPATIBILITY

Currently, the emulator supports the following ARM instructions only. Others 
will be skipped.

 * b{cond}{l}
 * bx         Rm
 * ldr{cond}  Rn,  +/-offset
 * mov{cond}  Rd,  Rn
 * mov{cons}s r15, Rn
 * str{cond}  Rn,  +/-offset

When the virtual processor boots, the default mode is the sys mode. As no 
supported instruction can change this mode, this is also the only mode for now.

The entire memory space (0x00000000 to 0x0E010000, inclusive) is available, 
however, no I/O-mapped addresses are currently functional.

Improved support is, of course, planned.
