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

use crate::luma::CONFIGURATION_VERSION;
use crate::luma::configuration::Configuration;

use std::fs::write;

impl Configuration {
	pub(super) fn create(&mut self) {
		let configuration_path = Configuration::path();

		eprintln!("creating configuration at {configuration_path}");

		let default_configuration = format!(
			"# This is the default configuration for the\n\
			 # Luma emulator.\n\
			 \n\
			 [luma]\n\
			 version = {CONFIGURATION_VERSION}\n\
			 \n\
			 [device]\n\
			 #bootloader = \"\"\n\
			 #image = \"\"\n\
			 \n\
			 [video]\n\
			 scale = 1\n"
		);

		write(configuration_path, default_configuration).unwrap();
	}
}
