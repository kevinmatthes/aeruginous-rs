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

/// Prefer a certain value to a fallback.
pub trait Prioritise {
  /// Prefer a certain value to a fallback.
  ///
  /// The instance this method is called on will be used as a fallback in case
  /// that the given value does not fulfill certain criteria to be considered
  /// preferable.
  #[must_use]
  fn prioritise(&self, other: Self) -> Self;
}

impl<T: Clone> Prioritise for Option<T> {
  fn prioritise(&self, other: Self) -> Self {
    other
      .as_ref()
      .map_or_else(|| (*self).clone(), |value| Some(value.clone()))
  }
}

#[cfg(test)]
mod option_t {
  use crate::Prioritise;

  #[test]
  fn prioritise() {
    assert_eq!(Some(23).prioritise(Some(42)), Some(42));
    assert_eq!(None.prioritise(Some(42)), Some(42));
    assert_eq!(Some(23).prioritise(None), Some(23));
    assert_eq!(None::<i32>.prioritise(None), None);
  }
}

/******************************************************************************/
