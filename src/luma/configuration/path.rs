// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::configuration::Configuration;

use std::env::var;

impl Configuration {
	pub(super) fn path() -> String {
		return match var("HOME") {
			Ok( path) => path,
			Err(_)    => panic!("unable to get home directory"),
		} + "/.luma.toml";
	}
}
