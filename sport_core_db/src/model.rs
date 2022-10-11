use std::collections::HashMap;

use crate::entity::{ID, Exercise, Person};

#[derive(Debug)]
pub enum TableType {
    Exercises,
}

pub struct Db {
    exercises: Exercises,
    persons: Persons,
}

impl Default for Db {
    fn default() -> Self {
        Self { 
            exercises: Exercises::default(),
            persons: Persons::default(),
        }
    }
}

impl Db {
    pub fn exercises(&self) -> &Exercises {
        &self.exercises
    }

    pub fn exercises_mut(&mut self) -> &mut Exercises {
        &mut self.exercises
    }

    pub fn persons(&self) -> &Persons {
        &self.persons
    }

    pub fn persons_mut(&mut self) -> &mut Persons {
        &mut self.persons
    }
}

pub trait Table {
    type Item;

    fn get_one(&self, id: ID) -> Option<&Self::Item>;

    fn get_all<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::Item> + 'a>;

    fn insert(&mut self, e: Self::Item) -> Option<&Self::Item>;

    fn remove(&mut self, id: ID);

    fn clear(&mut self);

    fn update(&mut self, e: Self::Item); 

    fn contains(&self, id: ID) -> bool;

    fn len(&self) -> usize;
}

pub struct Exercises {
    exercises: HashMap<ID, Exercise>,
}

impl Default for Exercises {
    fn default() -> Self {
        Self { exercises: HashMap::default() }
    }
}

impl Table for Exercises {
    type Item = Exercise;

    fn get_one(&self, id: ID) -> Option<&Exercise> {
        self.exercises.get(&id)
    }

    fn get_all<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Exercise> + 'a> {
        Box::new(self.exercises.values())
    }

    fn insert(&mut self, e: Exercise) -> Option<&Exercise> {
        let id = e.id;
        self.exercises.insert(id, e);
        self.exercises.get(&id)
    }

    fn remove(&mut self, id: ID) {
        self.exercises.remove(&id);
    }

    fn clear(&mut self) {
        self.exercises.clear();
    }

    fn update(&mut self, e: Exercise) {
        if let Some(data) = self.exercises.get_mut(&e.id) {
            *data = e;
        }
    }

    fn len(&self) -> usize {
        self.exercises.len()
    }

    fn contains(&self, id: ID) -> bool {
        self.exercises.contains_key(&id)
    }
}

pub struct Persons {
    persons: HashMap<ID, Person>,
}

impl Default for Persons {
    fn default() -> Self {
        Self { persons: HashMap::default() }
    }
}

impl Table for Persons {
    type Item = Person;

    fn get_one(&self, id: ID) -> Option<&Self::Item> {
        self.persons.get(&id)
    }

    fn get_all<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self::Item> + 'a> {
        Box::new(self.persons.values())
    }

    fn insert(&mut self, e: Self::Item) -> Option<&Self::Item> {
        let id = e.id;
        self.persons.insert(id, e);
        self.persons.get(&id)
    }

    fn remove(&mut self, id: ID) {
        self.persons.remove(&id);
    }

    fn clear(&mut self) {
        self.persons.clear();
    }

    fn update(&mut self, e: Self::Item) {
        if let Some(data) = self.persons.get_mut(&e.id) {
            *data = e;
        }
    }

    fn len(&self) -> usize {
        self.persons.len()
    }

    fn contains(&self, id: ID) -> bool {
        self.persons.contains_key(&id)
    }
}
