- LUMA

Copyright 2021-2023 Gabriel Jensen.

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version.

This program is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
details.

You should have received a copy of the GNU Affero General Public License along
with this program. If not, see <https://www.gnu.org/licenses/>.

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

 * b{cond}{l} Immediate24
 * bx{cond}   Rm
 * ldr{cond}  Rd,  [Rn, Immediate12]
 * mov{cond}  Rd,  Rn
 * mov{cond}  Rd,  #Immediate8
 * mov{cons}s r15, Rn
 * str{cond}  Rd,  [Rn, Immediate12]

Moreover, the following Thumb instructions are supported:

 * b          Immediate11
 * b{cond}    Immediate8
 * bl         Immediate24
 * bx         Rm
 * ldr        Rd,  [Rn,  Immediate5]
 * ldr        Rd,  [Rn,  Rm]
 * ldr        Rd,  [r13, Immediate8]
 * ldr        Rd,  [r15, Immediate8]
 * lsl        Rd,  Rm,   Immediate5
 * lsr        Rd,  Rm,   Immediate5
 * mov        Rd,  Rn
 * movs       Rd,  Immediate8
 * movs       Rd,  Rn
 * pop        Registers
 * push       Registers
 * strh       Rd,  [Rn,  Immediate5]
 * svc        Immediate8

When the virtual processor boots, the default mode is the sys mode. This can be
changed using the 'svc Immediate8' (Thumb) or 'svc Immediate24' (ARM)
instructions, which changes this to the svc mode.

The entire memory space (0x00000000 to 0x0E010000, exclusive) is available,
however, no I/O-mapped addresses are currently functional.

Improved support is, of course, planned.
