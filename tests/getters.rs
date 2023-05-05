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

use aeruginous::getters;

#[test]
fn cp_one_field() {
  struct Example {
    a: i32,
  }

  getters!(@cp Example {
    a: i32
  });

  let example = Example { a: 42 };

  assert_eq!(example.a(), 42);
}

#[test]
fn cp_two_fields() {
  struct Example {
    a: i32,
    b: f64,
  }

  getters!(@cp Example {
    a: i32,
    b: f64
  });

  let example = Example { a: 42, b: 23.0 };

  assert_eq!(example.a(), 42);
  assert_eq!(example.b(), 23.0);
}

#[test]
fn fn_cp_one_field() {
  struct Example {
    a: i32,
  }

  impl Example {
    getters!(@fn @cp
      a: i32
    );

    pub fn function() -> i32 {
      42
    }
  }

  let example = Example { a: 42 };

  assert_eq!(example.a(), 42);
  assert_eq!(Example::function(), 42);
}

#[test]
fn fn_cp_two_fields() {
  struct Example {
    a: i32,
    b: f64,
  }

  impl Example {
    getters!(@fn @cp
      a: i32,
      b: f64
    );

    pub fn function() -> i32 {
      42
    }
  }

  let example = Example { a: 42, b: 23.0 };

  assert_eq!(example.a(), 42);
  assert_eq!(example.b(), 23.0);
  assert_eq!(Example::function(), 42);
}

#[test]
fn fn_ref_one_field() {
  struct Example {
    a: String,
  }

  impl Example {
    getters!(@fn @ref
      a: String
    );

    pub fn function() -> i32 {
      42
    }
  }

  let example = Example {
    a: "string".to_string(),
  };

  assert_eq!(example.a(), "string");
  assert_eq!(Example::function(), 42);
}

#[test]
fn fn_ref_two_fields() {
  struct Example {
    a: String,
    b: Vec<i32>,
  }

  impl Example {
    getters!(@fn @ref
      a: String,
      b: Vec<i32>
    );

    pub fn function() -> i32 {
      42
    }
  }

  let example = Example {
    a: "string".to_string(),
    b: vec![1, 2, 3],
  };

  assert_eq!(example.a(), "string");
  assert_eq!(example.b(), &vec![1, 2, 3]);
  assert_eq!(Example::function(), 42);
}

#[test]
fn header() {
  struct Example {
    a: i32,
    b: f64,
    c: bool,
  }

  impl Example {
    getters!(@fn @cp
      a: i32,
      b: f64
    );

    getters!(@header c = (
      pub fn field_c(&self) -> String {
        self.c.to_string()
      }
    ));
  }

  let example = Example {
    a: 42,
    b: 23.0,
    c: true,
  };

  assert_eq!(example.a(), 42);
  assert_eq!(example.b(), 23.0);
  assert_eq!(example.field_c(), "true".to_string());
}

#[test]
fn ref_one_field() {
  struct Example {
    a: String,
  }

  getters!(@ref Example {
    a: String
  });

  let example = Example {
    a: "string".to_string(),
  };

  assert_eq!(example.a(), "string");
}

#[test]
fn ref_two_fields() {
  struct Example {
    a: String,
    b: Vec<i32>,
  }

  getters!(@ref Example {
    a: String,
    b: Vec<i32>
  });

  let example = Example {
    a: "string".to_string(),
    b: vec![1, 2, 3],
  };

  assert_eq!(example.a(), "string");
  assert_eq!(example.b(), &vec![1, 2, 3]);
}

/******************************************************************************/
