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

use crate::luma::app::App;

use sdl2::event::Event;
use std::sync::atomic::Ordering;

impl App {
	pub fn check_events(&mut self) -> Result<bool, String> {
		// Return true if we should quit.

		let mut event_pump = match self.sdl.event_pump() {
			Ok(pump) => pump,
			_        => return Err("unable to get event pump".to_string()),
		};

		if self.got_terminate.load(Ordering::Relaxed) {
			eprintln!("got terminate");
			return Ok(true)
		};

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => return Ok(true),
				_                => {},
			};
		}

		return Ok(false);
	}
}
