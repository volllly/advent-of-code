use std::{collections::BTreeSet, marker::PhantomData};

#[derive(Debug)]
pub struct Id<T> {
    id: usize,
    generation: u32,
    __p: PhantomData<T>,
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.generation == other.generation
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> Id<T> {
    fn new(generation: u32, index: usize) -> Id<T> {
        Self {
            id: index,
            generation,
            __p: PhantomData,
        }
    }
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn get(self, arena: &impl AsRef<Arena<T>>) -> Option<&T> {
        arena.as_ref().get(self)
    }

    pub fn get_mut(self, arena: &mut impl AsMut<Arena<T>>) -> Option<&mut T> {
        arena.as_mut().get_mut(self)
    }
}

#[derive(Debug)]
struct Slot<T> {
    generation: u32,
    item: Option<T>,
}

impl<T> Default for Slot<T> {
    fn default() -> Self {
        Self {
            generation: Default::default(),
            item: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct Arena<T> {
    data: Vec<Slot<T>>,
    items: BTreeSet<Id<T>>,
    free: BTreeSet<usize>,
}

impl<T> AsRef<Arena<T>> for Arena<T> {
    fn as_ref(&self) -> &Arena<T> {
        self
    }
}

impl<T> AsMut<Arena<T>> for Arena<T> {
    fn as_mut(&mut self) -> &mut Arena<T> {
        self
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            free: Default::default(),
            items: Default::default(),
        }
    }
}

impl<T> From<Vec<T>> for Arena<T> {
    fn from(value: Vec<T>) -> Self {
        let data: Vec<Slot<T>> = value
            .into_iter()
            .map(|item| Slot {
                generation: 0,
                item: Some(item),
            })
            .collect();
        Arena {
            items: (0..data.len()).map(|index| Id::new(0, index)).collect(),
            data,
            free: Default::default(),
        }
    }
}

impl<T> Arena<T> {
    pub fn insert(&mut self, item: T) -> Id<T> {
        let (index, slot) = if let Some(free) = self.free.pop_first() {
            (free, &mut self.data[free])
        } else {
            self.data.push(Slot::default());
            (self.data.len() - 1, self.data.last_mut().unwrap())
        };

        slot.generation += 1;
        slot.item = Some(item);

        let id = Id::new(slot.generation, index);

        self.items.insert(id);

        id
    }

    pub fn get(&self, id: Id<T>) -> Option<&T> {
        let slot = &self.data[id.id];
        if slot.generation == id.generation {
            slot.item.as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        let slot = &mut self.data[id.id];
        if slot.generation == id.generation {
            slot.item.as_mut()
        } else {
            None
        }
    }

    pub fn ids(&self) -> impl DoubleEndedIterator<Item = &Id<T>> {
        self.items.iter()
    }
}

#[cfg(test)]
mod test {
    use chumsky::chain::Chain;

    use super::Arena;

    #[test]
    fn insert_and_get() {
        let mut arena = Arena::<i32>::default();

        let index = arena.insert(1);

        assert_eq!(arena.get(index).copied(), Some(1i32))
    }

    #[test]
    fn create_from_vec() {
        let arena = Arena::from(vec![1, 2, 3, 4]);

        assert_eq!(arena.ids().count(), 4);
        for (index, id) in arena.ids().enumerate() {
            assert_eq!(id.generation, 0);
            assert_eq!(id.id, index);
            assert_eq!(arena.get(*id).copied(), Some(index + 1))
        }
    }
}
