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

use crate::luma::{decode_colour, SCREEN_SIZE};
use crate::luma::app::App;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

impl App {
	pub fn draw_video(&mut self, video: &[u8], agb_palette: &[u16]) {
		// TO-DO: Honour video mode.

		let mut palette: [Color; 0x100] = [Color::RGB(0x00, 0x00, 0x00); 0x100];

		for (index, value) in (&agb_palette[0x0..0x100]).into_iter().enumerate() {
			let colour = decode_colour(*value);

			palette[index] = colour;
		}

		for pixel_y in 0x0..SCREEN_SIZE.1 {
			for pixel_x in 0x0..SCREEN_SIZE.0 {
				let pixel = pixel_y as usize * SCREEN_SIZE.0 as usize + pixel_x as usize;

				let value  = video[pixel];
				let colour = palette[value as usize];
				self.canvas.set_draw_color(colour);

				let square = Rect::new(
					(pixel_x as u32 * self.scale) as i32,
					(pixel_y as u32 * self.scale) as i32,
					self.scale,
					self.scale,
				);
				self.canvas.fill_rect(square).unwrap();
			}
		}

		self.canvas.present();
	}
}
