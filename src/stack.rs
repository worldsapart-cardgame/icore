//! Card stack related types.

use alloc::collections::VecDeque;
use rand::Rng;

/// A stack of items.
///
/// # Serde
///
/// This type can be serialized and deserialized using Serde as a sequence
/// of `T`.
#[derive(Debug)]
pub struct Stack<T> {
    pub(crate) inner: VecDeque<T>,
}

impl<T> Stack<T> {
    /// Gets the top item.
    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.inner.front()
    }

    /// Pops the top item.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    /// Pushes an item to the top.
    #[inline]
    pub fn push_top(&mut self, item: T) {
        self.inner.push_front(item);
    }

    /// Pushes an item to the bottom.
    #[inline]
    pub fn push_bottom(&mut self, item: T) {
        self.inner.push_back(item);
    }

    /// Picks a random item from the stack from the given rng, or returns
    /// `None` if the stack is empty.
    #[allow(clippy::missing_panics_doc)]
    pub fn pick_random<R>(&mut self, rng: &mut R) -> Option<T>
    where
        R: Rng,
    {
        if self.inner.is_empty() {
            None
        } else {
            let index = rng.gen_range(0..self.inner.len());
            Some(self.inner.remove(index).expect("unreachable"))
        }
    }

    /// Gets an item from the stack by index, or returns `None` if the index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    /// Removes an item from the stack by index, or returns `None` if the index
    /// is out of range.
    #[inline]
    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.inner.remove(index)
    }

    /// Gets the length of the stack.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Checks if the stack is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Gets the iterator of the stack.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.into_iter()
    }

    /// Clears the stack.
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

/// An iterator for the stack.
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    inner: alloc::collections::vec_deque::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.inner.iter(),
        }
    }
}

#[cfg(feature = "serde")]
mod _serde {
    use serde::{Deserialize, Serialize};

    use super::*;

    impl<T> Serialize for Stack<T>
    where
        T: Serialize,
    {
        #[inline]
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.inner.serialize(serializer)
        }
    }

    impl<'de, T> Deserialize<'de> for Stack<T>
    where
        T: Deserialize<'de>,
    {
        #[inline]
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            VecDeque::deserialize(deserializer).map(|items| Self { inner: items })
        }
    }
}
