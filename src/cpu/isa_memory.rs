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

use crate::log;
use crate::cpu::{Cpu, take_state_mut};

impl Cpu {
	pub(super) fn isa_load_halfword(&mut self, rd: u8, rn: u8, imm: i16) {
		take_state_mut!(state, self);

		log!("ldrh r{rd}, [r{rn}, #{imm:#X}]");

		let rn_value = state.read_register(rn);

		let target = rn_value.wrapping_add_signed(imm as i32);

		let result = state.read_halfword(target) as u32;
		state.write_register(rd, result);
	}

	pub(super) fn isa_load_immediate_offset(&mut self, rd: u8, rn: u8, imm: i16) {
		take_state_mut!(state, self);

		let rn_value = state.read_register(rn);

		let target = rn_value.wrapping_add_signed(imm as i32);

		log!("ldr r{rd}, [r{rn}, #{imm:#X}] @ {target:#010X}");

		let result = state.read_word(target);
		state.write_register(rd, result);
	}

	pub(super) fn isa_load_pc(&mut self, rd: u8, imm: u16) {
		// Slightly different from load_immediate_offset
		// due to the target being forced word-aligned.

		take_state_mut!(state, self);

		let rn_value = state.read_register(0xF) & 0b11111111111111111111111111111100;

		let target = rn_value.wrapping_add(imm as u32);

		log!("ldr r{rd}, [pc, #{imm:#X}] @ {target:#010X}");

		let result = state.read_word(target);
		state.write_register(rd, result);
	}

	pub(super) fn isa_store_byte_immediate_offset(&mut self, rd: u8, rn: u8, imm: i16) {
		log!("strb r{rd}, [r{rn}, #{imm:#X}]");

		take_state_mut!(state, self);

		let rn_value = state.read_register(rn);

		let target = rn_value.wrapping_add_signed(imm as i32);

		let result = state.read_register(rd) as u8;
		state.write_byte(target, result);
	}

	pub(super) fn isa_store_byte_register_offset(&mut self, rd: u8, rn: u8, rm: u8) {
		log!("strb r{rd}, [r{rn}, r{rm}]");

		take_state_mut!(state, self);

		let rn_value = state.read_register(rn);
		let rm_value = state.read_register(rm);

		let target = rn_value.wrapping_add(rm_value);

		let result = state.read_register(rd) as u8;
		state.write_byte(target, result);
	}

	pub(super) fn isa_store_halfword(&mut self, rd: u8, rn: u8, imm: i16) {
		log!("strh r{rd}, [r{rn}, #{imm:#X}]");

		take_state_mut!(state, self);

		let rn_value = state.read_register(rn);

		let target = rn_value.wrapping_add_signed(imm as i32);

		let result = state.read_register(rd) as u16;
		state.write_halfword(target, result);
	}

	pub(super) fn isa_store_immediate_offset(&mut self, rd: u8, rn: u8, imm: i16) {
		log!("str r{rd}, [r{rn}, #{imm:#X}]");

		take_state_mut!(state, self);

		let rn_value = state.read_register(rn);

		let target = rn_value.wrapping_add_signed(imm as i32);

		let result = state.read_register(rd);
		state.write_word(target, result);
	}
}
