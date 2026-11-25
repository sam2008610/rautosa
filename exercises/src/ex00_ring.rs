//! Exercise 00 — a fixed-capacity double-ended queue of bytes.
//!
//! Task description and hints: `docs/exercise-00.md`.
//!
//! Replace every `todo!()` until `cargo test-host` is green. Do not change the
//! tests in part A.

// The skeleton does not use its fields and parameters yet.
// Delete this line when part A is done; the compiler should then stay quiet.
#![allow(unused_variables, dead_code)]

/// Number of elements a [`Ring`] can hold.
pub const CAPACITY: usize = 4;

/// Error returned when pushing into a full [`Ring`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Full;

/// A double-ended queue with a fixed capacity, stored in a ring buffer.
///
/// `head` is the index of the front element in `buf`; `len` is the number of
/// elements stored. The elements occupy `buf[head]`, `buf[head + 1]`, ...,
/// wrapping around at the end of the array.
#[derive(Debug)]
pub struct Ring {
    buf: [u8; CAPACITY],
    head: usize,
    len: usize,
}

impl Ring {
    /// Creates an empty ring.
    pub const fn new() -> Self {
        // Return a `Ring` value with every field initialised.
        todo!()
    }

    /// Number of elements currently stored.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// `true` if the ring holds no element.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// `true` if no further element fits.
    pub fn is_full(&self) -> bool {
        todo!()
    }

    /// Returns the front element without removing it, or `None` if empty.
    pub fn front(&self) -> Option<u8> {
        todo!()
    }

    /// Appends `value` at the back. Fails with [`Full`] if there is no room.
    pub fn push_back(&mut self, value: u8) -> Result<(), Full> {
        todo!()
    }

    /// Inserts `value` at the front. Fails with [`Full`] if there is no room.
    pub fn push_front(&mut self, value: u8) -> Result<(), Full> {
        todo!()
    }

    /// Removes and returns the front element, or `None` if empty.
    pub fn pop_front(&mut self) -> Option<u8> {
        todo!()
    }
}

impl Default for Ring {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a01_new_ring_is_empty() {
        let ring = Ring::new();
        assert_eq!(ring.len(), 0);
        assert!(ring.is_empty());
        assert!(!ring.is_full());
    }

    #[test]
    fn a02_front_and_pop_on_empty_ring_return_none() {
        let mut ring = Ring::new();
        assert_eq!(ring.front(), None);
        assert_eq!(ring.pop_front(), None);
        assert_eq!(ring.len(), 0);
    }

    #[test]
    fn a03_push_back_then_pop_front_is_fifo() {
        let mut ring = Ring::new();
        assert_eq!(ring.push_back(10), Ok(()));
        assert_eq!(ring.push_back(20), Ok(()));
        assert_eq!(ring.push_back(30), Ok(()));
        assert_eq!(ring.len(), 3);
        assert_eq!(ring.pop_front(), Some(10));
        assert_eq!(ring.pop_front(), Some(20));
        assert_eq!(ring.pop_front(), Some(30));
        assert_eq!(ring.pop_front(), None);
    }

    #[test]
    fn a04_front_does_not_remove() {
        let mut ring = Ring::new();
        ring.push_back(7).unwrap();
        assert_eq!(ring.front(), Some(7));
        assert_eq!(ring.front(), Some(7));
        assert_eq!(ring.len(), 1);
    }

    #[test]
    fn a05_push_back_fails_when_full_and_changes_nothing() {
        let mut ring = Ring::new();
        for value in 0..CAPACITY as u8 {
            assert_eq!(ring.push_back(value), Ok(()));
        }
        assert!(ring.is_full());
        assert_eq!(ring.push_back(99), Err(Full));
        assert_eq!(ring.len(), CAPACITY);
        assert_eq!(ring.front(), Some(0));
    }

    #[test]
    fn a06_push_front_puts_the_element_ahead_of_all_others() {
        let mut ring = Ring::new();
        ring.push_back(1).unwrap();
        ring.push_back(2).unwrap();
        assert_eq!(ring.push_front(9), Ok(()));
        assert_eq!(ring.pop_front(), Some(9));
        assert_eq!(ring.pop_front(), Some(1));
        assert_eq!(ring.pop_front(), Some(2));
    }

    #[test]
    fn a07_push_front_works_on_an_empty_ring() {
        let mut ring = Ring::new();
        assert_eq!(ring.push_front(5), Ok(()));
        assert_eq!(ring.len(), 1);
        assert_eq!(ring.pop_front(), Some(5));
        assert!(ring.is_empty());
    }

    #[test]
    fn a08_push_front_fails_when_full() {
        let mut ring = Ring::new();
        for value in 0..CAPACITY as u8 {
            ring.push_back(value).unwrap();
        }
        assert_eq!(ring.push_front(99), Err(Full));
        assert_eq!(ring.front(), Some(0));
    }

    #[test]
    fn a09_indices_wrap_around_the_end_of_the_buffer() {
        let mut ring = Ring::new();
        // Move `head` through the whole array several times.
        for round in 0..3 * CAPACITY as u8 {
            ring.push_back(round).unwrap();
            ring.push_back(round + 100).unwrap();
            assert_eq!(ring.pop_front(), Some(round));
            assert_eq!(ring.pop_front(), Some(round + 100));
        }
        assert!(ring.is_empty());
    }

    #[test]
    fn a10_push_front_wraps_below_index_zero() {
        let mut ring = Ring::new();
        for value in 1..=CAPACITY as u8 {
            ring.push_front(value).unwrap();
        }
        assert!(ring.is_full());
        for value in (1..=CAPACITY as u8).rev() {
            assert_eq!(ring.pop_front(), Some(value));
        }
    }

    #[test]
    fn a11_scheduler_scenario_preempted_entry_runs_before_queued_ones() {
        // Two activations are queued, the running task (42) is preempted and
        // must be the first one to run again.
        let mut ring = Ring::new();
        ring.push_back(1).unwrap();
        ring.push_back(2).unwrap();
        ring.push_front(42).unwrap();
        ring.push_back(3).unwrap();
        assert_eq!(ring.pop_front(), Some(42));
        assert_eq!(ring.pop_front(), Some(1));
        assert_eq!(ring.pop_front(), Some(2));
        assert_eq!(ring.pop_front(), Some(3));
    }
}
