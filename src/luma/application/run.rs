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

use crate::luma::VERSION;
use crate::luma::application::{Application, GOT_SIGNAL};

use sdl2::event::Event;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;

impl Application {
	pub fn run(&mut self) {
		eprintln!();
		eprintln!("luma {:X}.{:X}", VERSION.major, VERSION.minor);
		eprintln!("Copyright 2021-2023 Gabriel Jensen.");
		eprintln!();

		self.load();

		let mut event_pump = self.sdl.event_pump().expect("unable to get event pump");

		'main_loop: for cycle in 0x0..u64::MAX {
			// Check if we have recieved a signal:
			if unsafe { GOT_SIGNAL.load(Ordering::Relaxed) } {
				eprintln!("got interrupt");
				break;
			}

			// Iterate over events:
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit {..} => break 'main_loop,
					_                => {},
				}
			}

			if cfg!(debug_assertions) { eprintln!("({cycle})"); }

			(self.device.decode)(&mut self.device);

			sleep(Duration::from_secs(0x1));
		}
	}
}
