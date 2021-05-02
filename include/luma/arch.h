/*
	Copyright 2021 Gabriel Jensen

	This file is part of luma.

	luma is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or	(at your option) any later version.

	luma is distributed in the hope that it will be useful,	but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

	See the	GNU Affero General Public License for more details.

	You should have received a copy of the GNU Affero General Public License along with luma.

	If not, see <https://www.gnu.org/licenses/>.
*/
# if !defined(LUMA_HDR_ARCH)
# define LUMA_HDR_ARCH
enum luma_arch {
	// Null
	luma_arch_null,
	// Keywords
	luma_arch_lab,
	luma_arch_next,
	// Instructions
	luma_arch_add,
	luma_arch_call,
	luma_arch_cp,
	luma_arch_div,
	luma_arch_ex,
	luma_arch_fac,
	luma_arch_get,
	luma_arch_hello,
	luma_arch_luma,
	luma_arch_mult,
	luma_arch_pow,
	luma_arch_root,
	luma_arch_sub,
	// Numbers
	luma_arch_zero,
	luma_arch_one,
	luma_arch_two,
	luma_arch_three,
	luma_arch_four,
	luma_arch_five,
	luma_arch_six,
	luma_arch_seven,
	luma_arch_eight,
	luma_arch_nine,
	luma_arch_dek,
	luma_arch_el,
};
# endif
