// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::app::{App, GOTSIG};

extern crate libc;

use libc::{c_int, sighandler_t, SIGINT, signal, SIGTERM};
use std::mem::transmute;
use std::sync::atomic::Ordering;

fn sighnd(sig: c_int) {
	unsafe {
		signal(sig, transmute::<fn(c_int), sighandler_t>(sighnd));

		GOTSIG.store(true, Ordering::Relaxed);
	}
}

impl App {
	pub fn ini(&mut self) {
		eprintln!("initialising");

		unsafe {
			signal(SIGINT,  transmute::<fn(c_int), sighandler_t>(sighnd));
			signal(SIGTERM, transmute::<fn(c_int), sighandler_t>(sighnd));
		}
	}
}
