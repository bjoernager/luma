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

impl Configuration {
	pub fn new() -> Configuration {
		let mut configuration = Configuration {
			bootloader: "bootloader.bin".to_string(),
			image:      "image.agb".to_string(),
			scale:      0x1,
		};

		configuration.load();
		configuration.overwrite();

		return configuration;
	}
}
