// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::{LogType, VERSION};
use crate::luma::application::{Application, GOT_SIGNAL};

use sdl2::event::Event;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;

impl Application {
	pub fn run(&mut self) {
		eprintln!();
		eprintln!("luma {}.{}", VERSION.major, VERSION.minor);
		eprintln!("Copyright 2021-2023 Gabriel Jensen.");
		eprintln!();

		self.load();

		let mut event_pump = self.sdl.event_pump().expect("unable to get event pump");

		eprintln!("starting emulation at {:#010X}",self.registers[0xF] - 0x8);

		'main_loop: loop {
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

			// Decode opcode:
			let opcode = self.read_word(self.registers[0xF] - 0x8);
			self.decode(opcode);

			// Continue:
			self.registers[0xF] += 0x4;
			self.log(LogType::Continue(self.registers[0xF]));

			sleep(Duration::from_secs(0x1));
		}
	}
}
