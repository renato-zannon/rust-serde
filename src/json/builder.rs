// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::TreeMap;
use std::str::StrAllocating;

use super::{Json, JsonObject, List, Object, ToJson};

pub struct ListBuilder {
    list: Vec<Json>,
}

impl ListBuilder {
    pub fn new() -> ListBuilder {
        ListBuilder { list: Vec::new() }
    }

    pub fn unwrap(self) -> Json {
        List(self.list)
    }

    pub fn push<T: ToJson>(self, value: T) -> ListBuilder {
        let mut builder = self;
        builder.list.push(value.to_json());
        builder
    }

    pub fn push_list(self, f: |ListBuilder| -> ListBuilder) -> ListBuilder {
        let builder = ListBuilder::new();
        self.push(f(builder).unwrap())
    }

    pub fn push_object(self, f: |ObjectBuilder| -> ObjectBuilder) -> ListBuilder {
        let builder = ObjectBuilder::new();
        self.push(f(builder).unwrap())
    }
}

pub struct ObjectBuilder {
    object: JsonObject,
}

impl ObjectBuilder {
    pub fn new() -> ObjectBuilder {
        ObjectBuilder { object: TreeMap::new() }
    }

    pub fn unwrap(self) -> Json {
        Object(self.object)
    }

    pub fn insert<T: ToJson>(self, key: String, value: T) -> ObjectBuilder {
        let mut builder = self;
        builder.object.insert(key, value.to_json());
        builder
    }

    pub fn insert_list<S: StrAllocating>(self, key: S, f: |ListBuilder| -> ListBuilder) -> ObjectBuilder {
        let builder = ListBuilder::new();
        self.insert(key.into_string(), f(builder).unwrap())
    }

    pub fn insert_object<S: StrAllocating>(self, key: S, f: |ObjectBuilder| -> ObjectBuilder) -> ObjectBuilder {
        let builder = ObjectBuilder::new();
        self.insert(key.into_string(), f(builder).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::TreeMap;
    use json::{List, Integer, Object};
    use super::{ListBuilder, ObjectBuilder};

    #[test]
    fn test_list_builder() {
        let value = ListBuilder::new().unwrap();
        assert_eq!(value, List(Vec::new()));

        let value = ListBuilder::new()
            .push(1i)
            .push(2i)
            .push(3i)
            .unwrap();
        assert_eq!(value, List(vec!(Integer(1), Integer(2), Integer(3))));

        let value = ListBuilder::new()
            .push_list(|bld| bld.push(1i).push(2i).push(3i))
            .unwrap();
        assert_eq!(value, List(vec!(List(vec!(Integer(1), Integer(2), Integer(3))))));

        let value = ListBuilder::new()
            .push_object(|bld|
                bld
                    .insert("a".to_string(), 1i)
                    .insert("b".to_string(), 2i))
            .unwrap();

        let mut map = TreeMap::new();
        map.insert("a".to_string(), Integer(1));
        map.insert("b".to_string(), Integer(2));
        assert_eq!(value, List(vec!(Object(map))));
    }

    #[test]
    fn test_object_builder() {
        let value = ObjectBuilder::new().unwrap();
        assert_eq!(value, Object(TreeMap::new()));

        let value = ObjectBuilder::new()
            .insert("a".to_string(), 1i)
            .insert("b".to_string(), 2i)
            .unwrap();

        let mut map = TreeMap::new();
        map.insert("a".to_string(), Integer(1));
        map.insert("b".to_string(), Integer(2));
        assert_eq!(value, Object(map));
    }
}
