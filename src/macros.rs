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

/// Implement getter methods for the given struct fields.
#[macro_export]
macro_rules! getters {
  ( @cp $strct:ty { $( $field:ident : $return:ty ),+ } ) => {
    $crate::implement! {
      $strct;
      $crate::getters! {
        @fn @cp $(
          $field : $return
        ),+
      }
    }
  };

  ( @fn @cp $( $field:ident : $return:ty ),+ ) => {
    $(
      $crate::getters! {
        @header $field = (
          pub const fn $field(&self) -> $return {
            self.$field
          }
        )
      }
    )+
  };

  ( @fn @ref $( $field:ident : $return:ty ),+ ) => {
    $(
      $crate::getters! {
        @header $field = (
          pub const fn $field(&self) -> &$return {
            &self.$field
          }
        )
      }
    )+
  };

  ( @header $field:ident = ( $function:item ) ) => {
    #[doc = concat!("Retrieve [`Self::", stringify!($field), "`].")]
    #[must_use]
    $function
  };

  ( @ref $strct:ty { $( $field:ident : $return:ty ),+ } ) => {
    $crate::implement! {
      $strct;
      $crate::getters! {
        @fn @ref $(
          $field : $return
        ),+
      }
    }
  };
}

/// Create an `impl` block for the given struct.
///
/// Despite this macro being primarily intended for the definition of further
/// macros, it can be also applied in production anyway as the following
/// example illustrates.
///
/// ```rust
/// use aeruginous::implement;
///
/// struct Example;
///
/// implement!(Example;
///   pub fn function() -> i32 {
///     42
///   }
/// );
///
/// assert_eq!(Example::function(), 42);
/// ```
#[macro_export]
macro_rules! implement {
  ( $T:ty ; $( $function:item ),+ ) => {
    impl $T {
      $(
        $function
      )+
    }
  };
}

/******************************************************************************/
