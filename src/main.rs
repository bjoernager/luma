// Copyright 2021-2023 Gabriel Jensen.

mod luma;

use crate::luma::application::Application;

fn main() {
	let mut application = Application::new();
	application.run();
}
