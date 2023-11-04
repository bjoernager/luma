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

macro_rules! test_predicate {
	($cpsr: expr, $predicate: expr) => {{
		// True if predicate applies.

		// Code Id.       Predicates
		// 0    eq        Z==1
		// 1    ne        Z==0
		// 2    cs, hs    C==1
		// 3    cc, lo    C==0
		// 4    mi        N==1
		// 5    pl        N==0
		// 6    vs        V==1
		// 7    vc        V==0
		// 8    hi        C==1 && Z==0
		// 9    ls        C==0 && Z==1
		// A    ge        N==V
		// B    lt        N!=V
		// C    gt        Z==0 && N==V
		// D    le        Z==1 && N!=V
		// E    al        true
		// F    nv        false
		//
		// Note that nv is always invalid on ARMv4.

		let v = $cpsr & 0b00010000000000000000000000000000 != 0x0;
		let c = $cpsr & 0b00100000000000000000000000000000 != 0x0;
		let z = $cpsr & 0b01000000000000000000000000000000 != 0x0;
		let n = $cpsr & 0b10000000000000000000000000000000 != 0x0;

		match $predicate {
			0x0 => z,
			0x1 => !z,
			0x2 => c,
			0x3 => !c,
			0x4 => n,
			0x5 => !n,
			0x6 => v,
			0x7 => !v,
			0x8 => c && !z,
			0x9 => !c && z,
			0xA => n == v,
			0xB => n != v,
			0xC => !z && n == v,
			0xD => z && n != v,
			0xE => false,
			0xF => true,
			_   => unreachable!(),
		}
	}}
}
pub(crate) use test_predicate;
