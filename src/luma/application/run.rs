// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::{Application, GOT_SIGNAL};
use crate::luma::VERSION;

use sdl2::event::Event;
use std::sync::atomic::Ordering;

impl Application {
	pub fn run(&mut self) {
		eprintln!("luma {}.{}", VERSION.major, VERSION.minor);

		self.parse_parameters();
		self.load();

		let mut event_pump = self.sdl.event_pump().expect("unable to get event pump");

		eprintln!("starting emulation at 0x{:08X}",self.registers[0xF] - 0x8);

		'main_loop: loop {
			// Check if we have recieved a signal:
			if unsafe { GOT_SIGNAL.load(Ordering::Relaxed) } {
				eprintln!("got interrupt");
				break;
			}

			for event in event_pump.poll_iter() {
				match event {
					Event::Quit {..} => break 'main_loop,
					_                => {},
				}
			}

			// Decode opcode:
			let opcode = self.read_word(self.registers[0xF] - 0x8);
			self.decode(opcode);

			// Continue:
			self.registers[0xF] += 0x4;
		}
	}
}
