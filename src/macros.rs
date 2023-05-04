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
///
/// Getter methods usually only return either a reference to or a copy of the
/// corresponding field without any further action on the data.  Their
/// definition is, thus, a repetitive task which can be automated by some
/// technologies.  This macro aims to provide simple and convenient semantics to
/// do so.
///
/// # Copies of Values
///
/// This macro offers two modes for getter methods which shall retrieve a copy
/// the respective fields:  `@cp` as well as `@fn @cp`.  The difference between
/// these two is that the former one will create a new `impl` block for the
/// methods whilst the latter one requires the existence of such a block to put
/// the generated methods into.
///
/// The following example illustrates the generation of getter methods within a
/// completely new block.  The Rust Documentation System will generate a section
/// on its own for this `impl` block.
///
/// ```rust
/// use aeruginous::getters;
///
/// struct Example {
///   a: i32,
///   b: f64,
/// }
///
/// getters!(@cp Example {
///   a: i32,
///   b: f64
/// });
///
/// let example = Example {a: 42, b: 23.0};
///
/// assert_eq!(example.a(), 42);
/// assert_eq!(example.b(), 23.0);
/// ```
///
/// In case of further methods for the same struct, such a break in the
/// documentation might be rather unaesthetic.  For this use case, there is the
/// second mode of this macro:  `@fn @cp`.  Users can decide to have their getter
/// methods rendered into an already existing `impl` block, as shown by the
/// following example.
///
/// ```rust
/// use aeruginous::getters;
///
/// struct Example {
///   a: i32,
///   b: f64,
/// }
///
/// impl Example {
///   getters!(@fn @cp
///     a: i32,
///     b: f64
///   );
///
///   pub fn function() -> i32 {
///     42
///   }
/// }
///
/// let example = Example {a: 42, b: 23.0};
///
/// assert_eq!(example.a(), 42);
/// assert_eq!(example.b(), 23.0);
/// assert_eq!(Example::function(), 42);
/// ```
///
/// # References to Values
///
/// More complex data often does not implement the [`Copy`] trait.  Thus, when
/// creating a getter for such a field, returning a reference to the information
/// should be the preferred solution.  This macro also offers modes for these
/// cases.
///
/// Again, first of all, here is an example for the creation of an entirely new
/// `impl` block to store the methods in.
///
/// ```rust
/// use aeruginous::getters;
///
/// struct Example {
///   a: String,
///   b: Vec<i32>,
/// }
///
/// getters!(@ref Example {
///   a: String,
///   b: Vec<i32>
/// });
///
/// let example = Example {a: String::from("string"), b: vec![1, 2, 3]};
///
/// assert_eq!(example.a(), "string");
/// assert_eq!(example.b(), &vec![1, 2, 3]);
/// ```
///
/// Furthermore, the generation of getter methods returning references within
/// already existing `impl` blocks works analogously to the copying case.
///
/// ```rust
/// use aeruginous::getters;
///
/// struct Example {
///   a: String,
///   b: Vec<i32>,
/// }
///
/// impl Example {
///   getters!(@fn @ref
///     a: String,
///     b: Vec<i32>
///   );
///
///   pub fn function() -> i32 {
///     42
///   }
/// }
///
/// let example = Example {a: String::from("string"), b: vec![1, 2, 3]};
///
/// assert_eq!(example.a(), "string");
/// assert_eq!(example.b(), &vec![1, 2, 3]);
/// assert_eq!(Example::function(), 42);
/// ```
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
