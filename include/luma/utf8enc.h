/*
	Copyright 2021 Gabriel Jensen

	This file is part of luma.

	luma is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or	(at your option) any later version.

	luma is distributed in the hope that it will be useful,	but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

	See the	GNU Affero General Public License for more details.

	You should have received a copy of the GNU Affero General Public License along with luma.

	If not, see <https://www.gnu.org/licenses/>.
*/
# if !defined(LUMA_HDR_UTF8ENC)
# define LUMA_HDR_UTF8ENC
# include <stddef.h>
# include <stdint.h>
extern int luma_utf8enc(uint32_t * codeps,uint8_t * * utf,size_t * outszptr);
# endif
