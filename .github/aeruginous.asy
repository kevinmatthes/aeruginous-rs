/*********************** GNU General Public License 3.0 ***********************\
|                                                                              |
|  Copyright (C) 2023 Kevin Matthes                                            |
|                                                                              |
|  This program is free software: you can redistribute it and/or modify        |
|  it under the terms of the GNU General Public License as published by        |
|  the Free Software Foundation, either version 3 of the License, or           |
|  (at your option) any later version.                                         |
|                                                                              |
|  This program is distributed in the hope that it will be useful,             |
|  but WITHOUT ANY WARRANTY; without even the implied warranty of              |
|  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the               |
|  GNU General Public License for more details.                                |
|                                                                              |
|  You should have received a copy of the GNU General Public License           |
|  along with this program.  If not, see <https://www.gnu.org/licenses/>.      |
|                                                                              |
\******************************************************************************/

// Settings.
settings.outformat = "svg";
size (50);

// Gear.
for (int i = 0; i < 42; ++i)
    filldraw (
        rotate (i * 360 / 42) * (
            (0,1.4) -- (-.05,1.3) -- (.05,1.3) -- cycle
        )
    );

for (int i = 0; i < 5; ++i)
    draw (rotate (72 * i) * circle ((0,1.12), .1), linewidth (1));

draw (circle ((0,0), 1), linewidth (1.5));
draw (circle ((0,0), 1.25), linewidth (1.8));

// Text.
label ("\Huge Æ", (0,0));

/******************************************************************************/
