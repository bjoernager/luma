/*
	Copyright 2021, 2022 Gabriel Jensen

	This file is part of luma.

	luma is free software: you can redistribute it and/or modify it under the
	terms of the GNU Affero General Public License as published by the Free
	Software Foundation, either version 3 of the License, or (at your
	option) any later version.

	luma is distributed in the hope that it will be useful, but WITHOUT ANY
	WARRANTY; without even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
	License for more details.

	You should have received a copy of the GNU Affero General Public License
	along with luma. If not, see <https://www.gnu.org/licenses/>.
*/

#include "luma.h"

#include <SDL2/SDL.h>
#include <stdbool.h>

bool luma_checkEvts(void) {
	SDL_Event evt;
	while (SDL_PollEvent(&evt)) {
		switch (evt.type) {
		default:
			break;
		case SDL_QUIT:
			return true;
		}
	}
	return false;
}
