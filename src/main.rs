// Copyright 2021-2023 Gabriel Jensen.

mod luma;

use crate::luma::app::App;

fn main() {
	let mut app = App::new();
	app.run();
}
