// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::application::Application;

impl Drop for Application {
	fn drop(&mut self) {
		self.end(0x0,None);
	}
}
