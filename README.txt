LUMA

Copyright 2021-2023 Gabriel Jensen.

luma is an emulator for the AGB - Game Boy Advance - platform.

USAGE

luma [image] [bootloader]

Invoke the emulator via the 'luma' command.

CONFIGURATION

The emulator tries to read the configuration file at '${HOME}/.luma.ini'. If 
this file is found, the following fields are read (if present):

luma:
 - version:    (Uint)   The version of the configuration file (0)

device:
 - bootloader: (String) The path to the bootloader file (home-relative)
 - image:      (String) The path to the image file (home-relative)

video:
 - scale:      (Uint)   The scale modifier applied to the screen (min 1; max (2^32-1))

These settings are overwritten by terminal parameters (see USAGE).

COMPATIBILITY

Currently, the emulator only supports the b{cond}{l}.w instruction. Improved 
support is, of course, planned.
