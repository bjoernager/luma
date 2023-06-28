// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::{MEMORY_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::luma::application::{Application, GOT_SIGNAL};
use crate::luma::configuration::Configuration;

extern crate libc;
extern crate sdl2;

use libc::{c_int, sighandler_t, SIGINT, signal, SIGTERM};
use std::alloc::{alloc_zeroed, Layout};
use std::mem::{size_of, transmute};
use std::sync::atomic::Ordering;

fn signal_handler(sig: c_int) {
	unsafe {
		signal(sig, transmute::<fn(c_int), sighandler_t>(signal_handler));

		GOT_SIGNAL.store(true, Ordering::Relaxed);
	}
}

impl Application {
	pub fn initialise(configuration: &Configuration) -> Application {
		eprintln!("initialising");

		unsafe {
			signal(SIGINT,  transmute::<fn(c_int), sighandler_t>(signal_handler));
			signal(SIGTERM, transmute::<fn(c_int), sighandler_t>(signal_handler));
		}

		let sdl       = sdl2::init().expect("unable to initialise sdl2");
		let sdl_video = sdl.video().expect("unable to initialise sdl2");

		let window = sdl_video.window("luma", SCREEN_WIDTH as u32 * configuration.scale, SCREEN_HEIGHT as u32 * configuration.scale).position_centered().build().unwrap();

		let memory = unsafe { alloc_zeroed(Layout::new::<[u32; MEMORY_SIZE / size_of::<u32>()]>()) };
		if memory.is_null() { panic!("unable to allocate memory buffer") }

		eprintln!("allocated memory buffer at 0x{:0X}", memory as usize);

		return Application {
			configuration: configuration.clone(),
			sdl:           sdl,
			sdl_video:     sdl_video,
			window:        window,
			memory:        memory,
			registers:     [
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x00000000,
				0x08000008,
			],
			psr:           0b00000000000000000000000000001111,
		};
	}
}
