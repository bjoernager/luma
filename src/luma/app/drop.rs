// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::app::App;

impl Drop for App {
	fn drop(&mut self) {
		self.end(0x0,None);
	}
}
