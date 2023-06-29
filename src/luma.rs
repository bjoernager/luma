// Copyright 2021-2023 Gabriel Jensen.

pub mod application;
pub mod configuration;

pub struct VersionType<T> {
	major: T,
	minor: T,
}

pub const VERSION: VersionType::<u32> = VersionType::<u32> {
	major: 0x0,
	minor: 0x23,
};

pub enum TrapKind {
	BadAlignment( u32, u32),
	InvalidOpcode(u32, u32),
	OutOfBounds(  u32),
}

pub const CONFIGURATION_VERSION: u32 = 0x0;

pub const MEMORY_SIZE: usize = 0x0E010000;

pub const BOOTLOADER_SIZE: usize = 0x00004000;
pub const IMAGE_SIZE:      usize = 0x02000000;

pub const SCREEN_WIDTH:  u8 = 0xF0;
pub const SCREEN_HEIGHT: u8 = 0xA0;
