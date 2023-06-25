// Copyright 2021-2023 Gabriel Jensen.

use std::sync::atomic::AtomicBool;

pub mod drop;
pub mod end;
pub mod initialise;
pub mod new;
pub mod parse_parameters;
pub mod run;

pub struct Application {
	bootloader: String,
	image: String,
}

pub static mut GOT_SIGNAL: AtomicBool = AtomicBool::new(false);
