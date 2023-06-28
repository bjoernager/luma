// Copyright 2021-2023 Gabriel Jensen.

mod luma;

use crate::luma::application::Application;
use crate::luma::configuration::Configuration;

fn main() {
	let configuration = Configuration::new();

	let mut application = Application::initialise(&configuration);
	application.run();
}
