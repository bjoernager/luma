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

use std::thread::sleep;
use std::time::{Duration, Instant};

impl App {
	pub fn sync_video(&self, frame_start: Instant) {
		// Courtesy of TASVideos: <https://tasvideos.org/PlatformFramerates>
		// 59.7275005696058 Hz

		const FRAME_DURATION: u64 = 0xFF7932;
		let frame_duration        = Duration::from_nanos(FRAME_DURATION);

		let remaining = match frame_duration.checked_sub(frame_start.elapsed()) {
			Some(value) => value,
			None        => Duration::from_secs(0x0),
		};

		sleep(remaining);
	}
}
