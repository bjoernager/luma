// Copyright 2021-2023 Gabriel Jensen.

pub mod emu;
pub mod end;
pub mod ini;
pub mod new;
pub mod prspar;
pub mod run;
pub mod trp;

pub struct App {
	btl: String,
	img: String,
	mem: *mut u8,
}
