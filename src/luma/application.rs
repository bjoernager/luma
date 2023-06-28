// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::configuration::Configuration;

extern crate sdl2;

use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::Window;
use std::sync::atomic::AtomicBool;

pub mod bootloader;
pub mod decode;
pub mod drop;
pub mod end;
pub mod image;
pub mod initialise;
pub mod load;
pub mod read;
pub mod run;
pub mod trap;

pub struct Application {
	configuration: Configuration,
	sdl:           Sdl,
	sdl_video:     VideoSubsystem,
	window:        Window,
	memory:        *mut u8,
	registers:     [u32; 0x10],
	psr:           u32,
}

pub static mut GOT_SIGNAL: AtomicBool = AtomicBool::new(false);
