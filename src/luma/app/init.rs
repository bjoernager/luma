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

use crate::luma::{SCREEN_SIZE, VERSION};
use crate::luma::app::App;
use crate::luma::configuration::Configuration;

use sdl2::pixels::Color;
use sdl2::render::BlendMode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

impl App {
	pub fn init(configuration: Configuration) -> Result<Self, String> {
		let got_terminate = Arc::new(AtomicBool::new(false));

		match ctrlc::set_handler({
			let got_terminate = got_terminate.clone();
			move || got_terminate.store(true, Ordering::Relaxed)
		}) {
			Err(..) => return Err("unable to set signal handler".to_string()),
			_       => {},
		};

		let sdl = match sdl2::init() {
			Ok( sdl) => sdl,
			Err(..)  => return Err("unable to initialise sdl2".to_string()),
		};

		let sdl_video = match sdl.video() {
			Ok( video) => video,
			Err(..)    => return Err("unable to initialise video".to_string()),
		};

		let window_title = format!("Luma {:X}.{:X}", VERSION.0, VERSION.1);

		let mut window_builder = sdl_video.window(&window_title, SCREEN_SIZE.0 as u32 * configuration.scale, SCREEN_SIZE.1 as u32 * configuration.scale);
		window_builder.position_centered();

		let window = match window_builder.build() {
			Ok( window) => window,
			Err(..)     => return Err("unable to open window".to_string()),
		};

		let mut canvas = match window.into_canvas().build() {
			Ok( canvas) => canvas,
			Err(..)     => return Err("unable to build canvas".to_string()),
		};

		canvas.set_blend_mode(BlendMode::Blend);

		let clear_colour = Color::RGB(0x00, 0x00, 0x00);
		canvas.set_draw_color(clear_colour);
		canvas.clear();
		canvas.present();

		return Ok(App {
			bootloader: configuration.bootloader,
			image:      configuration.image,

			scale: configuration.scale,

			got_terminate: got_terminate,

			sdl:    sdl,
			canvas: canvas,
		});
	}
}
