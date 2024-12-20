use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

#[derive(PartialEq, Eq)]
pub struct Id<T> {
    id: usize,
    __p: PhantomData<T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> Id<T> {
    fn from(id: usize) -> Id<T> {
        Id {
            id,
            __p: PhantomData,
        }
    }

    pub fn get(self, arena: &impl AsRef<Arena<T>>) -> &T {
        &arena.as_ref()[self]
    }

    pub fn get_mut(self, arena: &mut impl AsMut<Arena<T>>) -> &mut T {
        &mut arena.as_mut()[self]
    }
}

#[derive(Default)]
pub struct Arena<T> {
    data: Vec<T>,
}

impl<T> Index<Id<T>> for Arena<T> {
    type Output = T;

    fn index(&self, index: Id<T>) -> &Self::Output {
        &self.data[index.id]
    }
}

impl<T> IndexMut<Id<T>> for Arena<T> {
    fn index_mut(&mut self, index: Id<T>) -> &mut Self::Output {
        &mut self.data[index.id]
    }
}

impl<T> From<Vec<T>> for Arena<T> {
    fn from(value: Vec<T>) -> Self {
        Arena { data: value }
    }
}

impl<T> Arena<T> {
    pub fn insert(&mut self, item: T) -> Id<T> {
        self.data.push(item);

        Id::from(self.data.len() - 1)
    }

    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.data.get(id.id)
    }

    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        self.data.get_mut(id.id)
    }

    pub fn ids(&self) -> impl Iterator<Item = Id<T>> {
        (0..self.data.len()).map(Id::from)
    }
}
