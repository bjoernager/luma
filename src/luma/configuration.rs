// Copyright 2021-2023 Gabriel Jensen.

pub mod create;
pub mod load;
pub mod new;
pub mod overwrite;
pub mod path;

#[derive(Clone)]
pub struct Configuration {
	pub bootloader: String,
	pub image:      String,
	pub scale:      u32,
}
