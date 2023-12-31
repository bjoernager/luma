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

use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub mod check_events;
pub mod draw_video;
pub mod init;
pub mod load;
pub mod main;
pub mod run;
pub mod sync_video;

pub struct App {
	bootloader: String,
	image:      String,

	scale: u32,

	got_terminate: Arc::<AtomicBool>,

	sdl:    Sdl,
	canvas: WindowCanvas,
}
