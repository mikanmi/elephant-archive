// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

use std::fmt;

#[derive(Debug)]
pub struct Snapshot {
   name: String 
}

impl Snapshot {
    pub fn new(name: String) -> Snapshot {
        Snapshot {
            name
        }
    }
}

impl fmt::Display for Snapshot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}", self.name)
    }
}
