// Copyright 2021-2023 Gabriel Jensen.

pub mod application;

pub struct VersionType<T> {
	major: T,
	minor: T,
}

pub const VERSION: VersionType::<u32> = VersionType::<u32> {
	major: 0x0,
	minor: 0x21,
};

pub const MEMORY_SIZE: usize = 0x0E010000;

pub const BOOTLOADER_SIZE: usize = 0x00004000;
pub const IMAGE_SIZE: usize = 0x02000000;
