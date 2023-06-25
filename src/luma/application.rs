// Copyright 2021-2023 Gabriel Jensen.

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
pub mod parse_parameters;
pub mod read;
pub mod run;
pub mod trap;

pub enum TrapKind {
	BadAlignment,
	InvalidOpcode,
	OutOfBounds,
}

pub struct Application {
	bootloader: String,
	image:      String,
	sdl:        Sdl,
	sdl_video:  VideoSubsystem,
	window:     Window,
	memory:     *mut u8,
	registers:  [u32; 0x10],
	psr:        u32,
}

pub static mut GOT_SIGNAL: AtomicBool = AtomicBool::new(false);
