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
	Affero General Public License along with Luma. If not, 
	see <https://www.gnu.org/licenses/>. 
*/

use crate::luma::configuration::Configuration;

use std::env::args;

impl Configuration {
	pub(super) fn overwrite(&mut self) {
		eprintln!("overwritting settings");

		let parameters: Vec<String> = args().collect();
		let number                  = parameters.len();

		if number >= 0x2 { self.image = parameters[0x1].clone() }

		if number >= 0x3 { self.bootloader = parameters[0x2].clone() }

		if number > 0x3 { panic!("invalid number of parameters ({number})") }
	}
}
