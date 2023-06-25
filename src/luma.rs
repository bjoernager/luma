// Copyright 2021-2023 Gabriel Jensen.

pub mod application;
pub mod emulator;

pub const VERSION: u32 = 0x20;

pub const MEMORY_SIZE: usize = 0x0E010000;

pub const BOOTLOADER_SIZE: usize = 0x00004000;
pub const IMAGE_SIZE: usize = 0x02000000;
