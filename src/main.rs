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

extern crate ctrlc;
extern crate sdl2;
extern crate toml;

mod luma;

use crate::luma::VERSION;
use crate::luma::app::App;
use crate::luma::configuration::Configuration;

use std::env::{args, var};
use std::process::exit;

fn main() {
	eprintln!();
	eprintln!("\u{1B}[1mluma\u{1B}[0m {:X}.{:X}", VERSION.0, VERSION.1);
	eprintln!("Copyright 2021-2023 \u{1B}[1mGabriel Bj\u{F8}rnager Jensen\u{1B}[0m.");
	eprintln!();

	let path = if let Some(path) = args().nth(0x1) { path }
	else                                           { default_configuration_path() };

	let configuration = match Configuration::load(&path) {
		Ok( configuration) => configuration,
		Err(message)       => panic!("unable to load configuration: {message}"),
	};

	let app = match App::init(configuration) {
		Ok( app)     => app,
		Err(message) => panic!("unable to initialise application: {message}"),
	};

	let result = app.run();

	if let Err(ref message) = result { eprintln!("\u{1B}[1m\u{1B}[91merror\u{1B}[0m: {message}") };

	exit(match result {
		Ok( ..) => 0x0,
		Err(..) => 0x1,
	});
}

fn default_configuration_path() -> String {
	let home = match var("HOME") {
		Ok( path) => path,
		Err(..)   => "/".to_string(),
	};

	let path = home + "/.luma.toml";
	return path;
}
