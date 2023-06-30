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

/// Write a coloured error message to [`std::io::Stderr`] without a line feed.
///
/// # Examples
///
/// ```rust
/// use aeruginous::ceprint;
/// use sysexits::Result;
///
/// fn function() -> Result<()> {
///   ceprint!("Green message."!Green);
///   ceprint!("Green"!Green, " and white message.");
///
///   Ok(())
/// }
/// ```
///
/// # Errors
///
/// See [`crate::ColourMessage`].
#[macro_export]
macro_rules! ceprint {
  ( $message:literal ! $colour:ident ) => {
    {
      use $crate::ColourMessage;

      $message.colour_message(
        anstyle::AnsiColor::$colour,
        &mut std::io::stderr()
      )?;
    }
  };

  ( $message:literal ! $colour:ident , $( $suffix:tt )+ ) => {
    $crate::ceprint!($message ! $colour);
    eprint!( $( $suffix )+ );
  };
}

/// Write a coloured error message to [`std::io::Stderr`].
///
/// # Examples
///
/// ```rust
/// use aeruginous::ceprintln;
/// use sysexits::Result;
///
/// fn function() -> Result<()> {
///   ceprintln!("Green message."!Green);
///   ceprintln!("Green"!Green, " and white message.");
///
///   Ok(())
/// }
/// ```
///
/// # Errors
///
/// See [`crate::ColourMessage`].
#[macro_export]
macro_rules! ceprintln {
  ( $message:literal ! $colour:ident ) => {
    $crate::ceprint!($message ! $colour);
    eprintln!();
  };

  ( $message:literal ! $colour:ident , $( $suffix:tt )+ ) => {
    $crate::ceprint!($message ! $colour);
    eprintln!( $( $suffix )+ );
  };
}

/// Join coloured part and suffix by a space character.
#[macro_export]
macro_rules! ceprintlns {
  ( $message:literal ! $colour:ident , $( $suffix:tt )+ ) => {
    $crate::ceprint!($message ! $colour);
    eprint!(" ");
    eprintln!( $( $suffix )+ );
  };
}

/// Implement certain common traits for enums.
///
/// # `Display`
///
/// This mode will implement [`std::fmt::Display`] for the given enum.
///
/// ```rust
/// use aeruginous::enum_trait;
///
/// enum E {
///   A,
///   B,
/// }
///
/// enum_trait!(E { A -> "a", B -> "b" });
///
/// assert_eq!(&E::A.to_string(), "a");
/// assert_eq!(&E::B.to_string(), "b");
/// ```
///
/// # `FromStr`
///
/// This mode will implement [`std::str::FromStr`] for the given enum.
///
/// ```rust
/// use aeruginous::enum_trait;
/// use std::str::FromStr;
///
/// #[derive(Debug, PartialEq)]
/// enum E {
///   A,
///   B,
/// }
///
/// enum_trait!(E { A <- "a", B <- "b" });
///
/// assert_eq!(E::from_str("a").unwrap(), E::A);
/// assert_eq!(E::from_str("b").unwrap(), E::B);
/// assert!(E::from_str("?").is_err());
/// ```
/// # `Display` and `FromStr`
///
/// To implement both the [`std::fmt::Display`] and [`std::str::FromStr`] traits
/// at once, this macro offers a shortcut mode.
///
/// ```rust
/// use aeruginous::enum_trait;
/// use std::str::FromStr;
///
/// #[derive(Debug, PartialEq)]
/// enum E {
///   A,
///   B,
/// }
///
/// enum_trait!(E { A <-> "a", B <-> "b" });
///
/// assert_eq!(&E::A.to_string(), "a");
/// assert_eq!(&E::B.to_string(), "b");
/// assert_eq!(E::from_str("a").unwrap(), E::A);
/// assert_eq!(E::from_str("b").unwrap(), E::B);
/// assert!(E::from_str("?").is_err());
/// ```
#[macro_export]
macro_rules! enum_trait {
  ( $E:ty { $( $V:ident <-> $s:literal ),+ } ) => {
    $crate::enum_trait! {
      $E {
        $(
          $V -> $s
        ),+
      }
    }

    $crate::enum_trait! {
      $E {
        $(
          $V <- $s
        ),+
      }
    }
  };

  ( $E:ty { $( $V:ident -> $s:literal ),+ } ) => {
    impl std::fmt::Display for $E {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
          f,
          "{}",
          match self {
            $(
              Self::$V => $s,
            )+
          }
        )
      }
    }
  };

  ( $E:ty { $( $V:ident <- $s:literal ),+ } ) => {
    impl std::str::FromStr for $E {
      type Err = String;

      fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
          $(
            $s => Ok(Self::$V),
          )+
          _ => Err(format!("'{s}' is not supported, yet")),
        }
      }
    }
  };
}

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
/// let example = Example { a: 42, b: 23.0 };
///
/// assert_eq!(example.a(), 42);
/// assert_eq!(example.b(), 23.0);
/// ```
///
/// In case of further methods for the same struct, such a break in the
/// documentation might be rather unaesthetic.  For this use case, there is the
/// second mode of this macro:  `@fn @cp`.  Users can decide to have their
/// getter methods rendered into an already existing `impl` block, as shown by
/// the following example.
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
/// let example = Example { a: 42, b: 23.0 };
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
/// let example = Example { a: "string".to_string(), b: vec![1, 2, 3] };
///
/// assert_eq!(example.a(), "string");
/// assert_eq!(example.b(), &[1, 2, 3]);
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
/// let example = Example { a: "string".to_string(), b: vec![1, 2, 3] };
///
/// assert_eq!(example.a(), "string");
/// assert_eq!(example.b(), &[1, 2, 3]);
/// assert_eq!(Example::function(), 42);
/// ```
///
/// # Header Generation
///
/// All previously presented modes named the resulting method according to the
/// field they queried and did nothing more returning the data of that field
/// somehow.  There might be use cases in which a the getter should not have the
/// same name as the field it queries or in which the query is more complex than
/// just returning a copy or a reference.  For these cases, there is one last
/// mode:  `@header`.
///
/// This macro renders the documentation as well as some useful compiler
/// attributes for each generated getter method.  These information are
/// considered a getter's "header".  When defining a specialised getter method,
/// one might would like to have exactly this header for the new method, as
/// well.  This mode provides the required functionality therefore.  In contrast
/// to the other modes, only *one* method per call can be tagged by such a
/// header.  Furthermore, an *already existing* `impl` block is mandatory.
///
/// ```rust
/// use aeruginous::getters;
///
/// struct Example {
///   a: i32,
///   b: f64,
///   c: bool,
/// }
///
/// impl Example {
///   getters!(@fn @cp
///     a: i32,
///     b: f64
///   );
///
///   getters!(@header c = (
///     pub fn field_c(&self) -> String {
///       self.c.to_string()
///     }
///   ));
/// }
///
/// let example = Example {
///   a: 42,
///   b: 23.0,
///   c: true,
/// };
///
/// assert_eq!(example.a(), 42);
/// assert_eq!(example.b(), 23.0);
/// assert_eq!(example.field_c(), "true".to_string());
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
