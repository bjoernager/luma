// Copyright 2021-2023 Gabriel Jensen.

use std::sync::atomic::AtomicBool;

pub mod drop;
pub mod end;
pub mod ini;
pub mod new;
pub mod prspar;
pub mod run;

pub struct App {
	btl: String,
	img: String,
}

pub static mut GOTSIG: AtomicBool = AtomicBool::new(false);
