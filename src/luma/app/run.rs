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

use crate::luma::{PALETTE_LENGTH, VIDEO_LENGTH};
use crate::luma::app::App;
use crate::luma::cpu::Cpu;
use crate::luma::state::State;

use std::time::Instant;

impl App {
	pub fn run(mut self) -> Result<(), String> {
		let mut state = State::new();

		self.load(&mut state)?;

		let cpu = Cpu::new(state);

		let mut cpu = cpu.boot();

		let mut video_buffer:   Vec::<u8>  = vec![0x0; VIDEO_LENGTH as usize];
		let mut palette_buffer: Vec::<u16> = vec![0x0; (PALETTE_LENGTH / 0x2) as usize];

		'main_loop: loop {
			let frame_start = Instant::now();

			if self.check_events()? { break 'main_loop };

			cpu.dump_video(  &mut video_buffer[..]);
			cpu.dump_palette(&mut palette_buffer[..]);
			self.draw_video(&video_buffer[..], &palette_buffer[..]);

			self.sync_video(frame_start);
		}

		cpu.kill()?;

		return Ok(());
	}
}
