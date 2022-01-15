#![allow(dead_code)]
use std::collections::HashMap;

// 効率的でなくても良いということなので、HashMapでやってみる
struct List {
    map: HashMap<i32, i32>,
    size: i32
}

enum ListErr {
    IndexErr
}

impl List {
    fn new() -> Self {
        let map = HashMap::new();
        let size = 0;
        List { map, size }
    }

    fn get(&self, i: i32) -> Option<&i32> {
        return if i >= self.size {
            None
        } else {
            self.map.get(&i)
        }
    }

    fn set(&self, i: i32, x: i32) -> Result<Self, ListErr> {
        if i >= self.size {
            return Err(ListErr::IndexErr)
        }
        let mut map = self.map.clone();
        map.insert(i, x);
        return Ok(List { map, size: self.size })
    }

    fn add(&self, i: i32, x: i32) -> Result<Self, ListErr> {
        if i > self.size {
            return Err(ListErr::IndexErr)
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(i, x);
        for (&k, &v) in self.map.iter() {
            if k < i {
                map.insert(k, v);
            } else {
                map.insert(k+1, v);
            }
        }
        return Ok(List { map, size: self.size+1 })
    }

    fn remove(&self, i: i32) -> Result<Self, ListErr> {
        if i > self.size {
            return Err(ListErr::IndexErr)
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (&k, &v) in self.map.iter() {
            if k < i {
                map.insert(k, v);
            } else if k > i {
                map.insert(k-1, v);
            }
        }
        return Ok(List { map, size: self.size-1 })
    }
}

// vecでやってみる
struct USet {
    v: Vec<i32>,
    size: i32
}

impl USet {
    fn new() -> Self {
        USet { v: vec![], size: 0 }
    }

    fn find(&self, x: i32) -> Option<i32> {
        for &val in self.v.iter() {
            if val == x {
                return Some(x)
            }
        }
        return None
    }

    fn add(&mut self, x: i32) -> bool {
        let mut isin = false;
        for &val in self.v.iter() {
            if val == x {
                isin = true;
            }
        }
        if !isin {
            self.v.push(x);
            self.size += 1;
        }
        return !isin
    }

    fn remove(&mut self, x: i32) -> Option<i32> {
        let mut isin = false;
        let mut v = vec![];
        for &val in self.v.iter() {
            if val == x {
                isin = true;
            } else {
                v.push(val);
            }
        }
        self.v = v;
        if isin {
            self.size -= 1;
            return Some(x)
        } else {
            return None
        }

    }
}

struct SSet {
    v: Vec<i32>,
    size: i32
}

// addとremoveは割愛
impl SSet {
    fn find(&self, x: i32) -> Option<i32> {
        let mut supmin = -1;
        for &val in self.v.iter() {
            if val >= x && val < supmin {
                supmin = val;
            }
        }
        if supmin < 0 {
            return None
        } else {
            return Some(supmin as i32)
        }
    }
}

fn main() {
}
