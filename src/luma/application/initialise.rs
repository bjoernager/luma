// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::{Application, GOT_SIGNAL};

extern crate libc;

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
	pub fn initialise(&mut self) {
		eprintln!("initialising");

		unsafe {
			signal(SIGINT,  transmute::<fn(c_int), sighandler_t>(signal_handler));
			signal(SIGTERM, transmute::<fn(c_int), sighandler_t>(signal_handler));
		}
	}
}
