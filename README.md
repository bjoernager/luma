# Luma

luma is an emulator for the AGB&mdash;Game Boy Advance platform.

# Usage

```
luma [configuration]
```

Invoke the emulator via the `luma` command.

## Configuration

The emulator tries to read the configuration file at `${HOME}/.luma.toml`. If successful, the following fields are read (all must be present):

`luma`:
 * `version`:    The configuration format (currently 0)

`device`:
 * `bootloader`: The path to the bootloader file
 * `image`:      The path to the image file

`video`:
 * `scale`:      The scale modifier applied to the screen (1-4294967295)

If a path is parsed as a terminal parameter, the configuration at that location is read instead.

# Compatibility

Currently, the emulator has limited support for the Arm instruction set. All of the instructions used in the provided test program are &ndash; however &ndash; implemented.

The entire memory space (`0x00000000` to `0x0E00FFFF`) is available, however, no I/O-mapped addresses are currently functional.

Improved support is, of course, planned.

# Copyright & License

Copyright 2021-2023 Gabriel Bjørnager Jensen.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
