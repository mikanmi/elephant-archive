// Copyright (c) 2022 Patineboot.
// All rights reserved.
// Elephant Archive is licensed under BSD 2-Clause License.

#[derive(Debug)]
pub struct Snapshot {
   name: String 
}

impl Snapshot {
    pub fn new(name: &str) -> Snapshot {
        Snapshot {
            name: name.to_string(),
        }
    }
}
