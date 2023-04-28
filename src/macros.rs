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

/// Implement getters for certain struct fields.
#[macro_export]
macro_rules! get {
  ( @context $strct:ty { $( $field:ident = ( $function:item ) ),+ } ) => {
    $crate::implement! { $strct;
      $(
        #[doc = concat!("Retrieve [`Self::", stringify!($field), "`].")]
        #[must_use]
        $function
      ),+
    }
  };
  ( @cp $strct:ty { $( $field:ident : $return:ty ),+ } ) => {
    $crate::get! {
      @context $strct {
        $(
          $field = (
            pub const fn $field(&self) -> $return {
              self.$field
            }
          )
        ),+
      }
    }
  };
  ( @ref $strct:ty { $( $field:ident : $return:ty ),+ } ) => {
    $crate::get! {
      @context $strct {
        $(
          $field = (
            pub const fn $field(&self) -> &$return {
              &self.$field
            }
          )
        ),+
      }
    }
  };
}

/// Create an `impl` block for the given struct.
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
