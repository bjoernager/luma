// Copyright 2021-2023 Gabriel Jensen.

use crate::luma::LogType;
use crate::luma::application::Application;

impl Application {
	pub fn log(&mut self, log_type: LogType) {
		let type_string = match log_type {
			LogType::Branch(       _, _) => "branch  ",
			LogType::Continue(     _)    => "continue",
			LogType::Link(         _)    => "link    ",
			LogType::Load(         _, _) => "load    ",
			LogType::MoveRegister( _, _) => "move    ",
			LogType::MoveImmediate(_, _) => "move    ",
			LogType::Store(        _, _) => "store   ",
		};

		let message = match log_type {
			LogType::Branch(       offset,      address)   => format!("pc{offset:+} => {address:#010X}"),
			LogType::Continue(     address)                => format!("pc => {address:#010X}"),
			LogType::Link(         address)                => format!("lr => {address:#010X}"),
			LogType::Load(         register,    address)   => format!("r{register} => {address:#010X}"),
			LogType::MoveRegister( destination, source)    => format!("r{destination} => r{source}"),
			LogType::MoveImmediate(register,    immediate) => format!("r{register} => {immediate:#X}"),
			LogType::Store(        address,     register)  => format!("{address:#010X} => r{register}"),
		};

		eprintln!("{type_string} : {message}");
	}
}
