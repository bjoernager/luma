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

use crate::shifter::Shifter;
use crate::state::State;

impl State {
	#[must_use]
	pub fn shifter_value(&self, shifter: Shifter) -> (u32, bool) {
		use Shifter::*;

		let c = self.read_cpsr() & 0b00100000000000000000000000000000 != 0x0;

		let (value, carry) = match shifter {
			Immediate(imm, rot) => {
				let result = (imm as u32).rotate_right(rot as u32);
				let c      = if rot == 0x0 { c } else { result & 0b10000000000000000000000000000000 != 0x0 };

				(result, c)
			},

			ArithmeticShiftRightImmediate(rm, imm) => {
				let rm_value = self.read_register(rm);

				let result = (rm_value as i32).wrapping_shr(imm as u32) as u32;
				let c      = false; // TODO

				(result, c)
			},

			ArithmeticShiftRightRegister(rm, rs) => {
				let rm_value = self.read_register(rm);
				let rs_value = self.read_register(rs);

				let result = (rm_value as i32).wrapping_shr(rs_value) as u32;
				let c      = false; // TODO

				(result, c)
			},

			LogicalShiftLeftImmediate(rm, imm) => {
				let rm_value = self.read_register(rm);

				let result = rm_value.wrapping_shl(imm as u32);
				let c      = false; // TODO

				(result, c)
			},

			LogicalShiftLeftRegister(rm, rs) => {
				let rm_value = self.read_register(rm);
				let rs_value = self.read_register(rs);

				let result = rm_value.wrapping_shl(rs_value);
				let c      = false; // TODO

				(result, c)
			},

			LogicalShiftRightImmediate(rm, imm) => {
				let rm_value = self.read_register(rm);

				let result = rm_value.wrapping_shr(imm as u32);
				let c      = false; // TODO

				(result, c)
			},

			LogicalShiftRightRegister(rm, rs) => {
				let rm_value = self.read_register(rm);
				let rs_value = self.read_register(rs);

				let result = rm_value.wrapping_shr(rs_value);
				let c      = false; // TODO

				(result, c)
			},

			RotateRightImmediate(rm, imm) => {
				let rm_value = self.read_register(rm);

				let result = rm_value.rotate_right(imm as u32);
				let c      = false; // TODO

				(result, c)
			},

			RotateRightRegister(rm, rs) => {
				let rm_value = self.read_register(rm);
				let rs_value = self.read_register(rs);

				let result = rm_value.rotate_right(rs_value);
				let c      = false; // TODO

				(result, c)
			},

			RotateRightExtend(rm) => {
				let rm_value = self.read_register(rm);

				let result = 0x0_u32; // TODO
				let c      = rm_value & 0b00000000000000000000000000000001 != 0x0;

				(result, c)
			},
		};

		return (value, carry);
	}
}
