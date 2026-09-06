
// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------

// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::iter;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::option;
use std::slice;

#[repr(transparent)]
pub(crate) struct NoDrop<T: ?Sized>(ManuallyDrop<T>);

impl<T> NoDrop<T> {
    pub(crate) fn new(value: T) -> Self
    where
        T: TrivialDrop,
    {
        NoDrop(ManuallyDrop::new(value))
    }
}

impl<T: ?Sized> Deref for NoDrop<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for NoDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(crate) trait TrivialDrop {}

impl<T> TrivialDrop for iter::Empty<T> {}
impl<T> TrivialDrop for slice::Iter<'_, T> {}
impl<T> TrivialDrop for slice::IterMut<'_, T> {}
impl<T> TrivialDrop for option::IntoIter<&T> {}
impl<T> TrivialDrop for option::IntoIter<&mut T> {}

#[test]
fn test_needs_drop() {
    use std::mem::needs_drop;

    struct NeedsDrop;

    impl Drop for NeedsDrop {
        fn drop(&mut self) {}
    }

    assert!(needs_drop::<NeedsDrop>());

    // Test each of the types with a handwritten TrivialDrop impl above.
    assert!(!needs_drop::<iter::Empty<NeedsDrop>>());
    assert!(!needs_drop::<slice::Iter<NeedsDrop>>());
    assert!(!needs_drop::<slice::IterMut<NeedsDrop>>());
    assert!(!needs_drop::<option::IntoIter<&NeedsDrop>>());
    assert!(!needs_drop::<option::IntoIter<&mut NeedsDrop>>());
}
