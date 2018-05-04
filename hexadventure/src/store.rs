//! Store values in a central owner that behaves like a HashMap with autogenerated keys.
//!
//! This is intended to avoid cycles in structs so that they can be serialized.

use std::marker::PhantomData;

const INVALID_VERSION: u32 = 0;
const FIRST_VALID_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct Store<T> {
    values: Vec<Versioned<T>>,
    reusable_ids: Vec<Id<T>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Id<T> {
    index: usize,
    version: u32,
    _marker: PhantomData<T>,
}

#[derive(Serialize, Deserialize)]
struct Versioned<T> {
    value: T,
    version: u32,
}

impl<T> Id<T> {
    fn new(index: usize, version: u32) -> Self {
        Id {
            index,
            version,
            _marker: PhantomData,
        }
    }
}

impl<T> Store<T> {
    /// Creates an empty store.
    pub fn new() -> Self {
        Store {
            values: Vec::new(),
            reusable_ids: Vec::new(),
        }
    }

    /// Adds a value to the store.
    ///
    /// Returns a handle to the inserted value.
    pub fn insert(&mut self, value: T) -> Id<T> {
        if let Some(id) = self.reusable_ids.pop() {
            self.values[id.index] = Versioned {
                value,
                version: id.version,
            };
            id
        } else {
            let id = Id::new(self.values.len(), FIRST_VALID_VERSION);
            self.values.push(Versioned {
                value,
                version: id.version,
            });
            id
        }
    }

    /// Returns a reference to the value corresponding to the id.
    pub fn get(&self, id: Id<T>) -> Option<&T> {
        match self.values.get(id.index) {
            Some(&Versioned { ref value, version }) if version == id.version => Some(value),
            _ => None,
        }
    }

    /// Returns a mutable reference to the value corresponding to the id.
    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        match self.values.get_mut(id.index) {
            Some(&mut Versioned {
                ref mut value,
                version,
            }) if version == id.version =>
            {
                Some(value)
            }
            _ => None,
        }
    }

    /// Removes a value from the store. Returns `true` if the value was present in the set.
    ///
    /// The values do not actually go out of scope right away.
    pub fn remove(&mut self, id: Id<T>) -> bool {
        match self.values.get_mut(id.index) {
            Some(&mut Versioned {
                ref mut version, ..
            }) if *version == id.version =>
            {
                *version = INVALID_VERSION;
                self.reusable_ids.push(id.reuse());
                true
            }
            _ => false,
        }
    }

    /// Returns an iterator that iterates through every value.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            values: &self.values,
            index: 0,
        }
    }
}

pub struct Iter<'a, T: 'a> {
    values: &'a Vec<Versioned<T>>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.values.len() {
                break None;
            }
            let versioned = &self.values[self.index];
            if versioned.version > 0 {
                break Some(&versioned.value);
            }
            self.index += 1;
        }
    }
}

impl<T> Id<T> {
    fn reuse(self) -> Self {
        Id::new(self.index, self.version + 1)
    }
}

// copy and clone are implemented manually to avoid
// restricting T to be copy or clone as well
impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}
