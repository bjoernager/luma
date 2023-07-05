/*
	Copyright 2021-2023 Gabriel Jensen.

	This file is part of Luma.

	Luma is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	Affero General Public License as published by
	the Free Software Foundation, either version 3
	of the License, or (at your option) any later
	version.

	Luma is distributed in the hope that it will be
	useful, but WITHOUT ANY WARRANTY; without even
	the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU
	Affero General Public License along with Luma.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::luma::{SCREEN_SIZE};
use crate::luma::application::{Application, GOT_SIGNAL};
use crate::luma::configuration::Configuration;
use crate::luma::device::Device;

extern crate libc;
extern crate sdl2;

use libc::{c_int, sighandler_t, SIGINT, signal, SIGTERM};
use std::mem::transmute;
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

		let window = sdl_video.window("luma", SCREEN_SIZE.width as u32 * configuration.scale, SCREEN_SIZE.height as u32 * configuration.scale).position_centered().build().unwrap();

		return Application {
			configuration: configuration.clone(),
			sdl:           sdl,
			sdl_video:     sdl_video,
			window:        window,
			device:        Device::new(),
		};
	}
}
