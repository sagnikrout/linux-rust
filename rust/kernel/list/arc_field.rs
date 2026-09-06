
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

// SPDX-License-Identifier: GPL-2.0

// Copyright (C) 2024 Google LLC.

//! A field that is exclusively owned by a [`ListArc`].
//!
//! This can be used to have reference counted struct where one of the reference counted pointers
//! has exclusive access to a field of the struct.
//!
//! [`ListArc`]: crate::list::ListArc

use core::cell::UnsafeCell;

/// A field owned by a specific [`ListArc`].
///
/// [`ListArc`]: crate::list::ListArc
pub struct ListArcField<T, const ID: u64 = 0> {
    value: UnsafeCell<T>,
}

// SAFETY: If the inner type is thread-safe, then it's also okay for `ListArc` to be thread-safe.
unsafe impl<T: Send + Sync, const ID: u64> Send for ListArcField<T, ID> {}
// SAFETY: If the inner type is thread-safe, then it's also okay for `ListArc` to be thread-safe.
unsafe impl<T: Send + Sync, const ID: u64> Sync for ListArcField<T, ID> {}

impl<T, const ID: u64> ListArcField<T, ID> {
    /// Creates a new `ListArcField`.
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    /// Access the value when we have exclusive access to the `ListArcField`.
    ///
    /// This allows access to the field using an `UniqueArc` instead of a `ListArc`.
    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }

    /// Unsafely assert that you have shared access to the `ListArc` for this field.
    ///
    /// # Safety
    ///
    /// The caller must have shared access to the `ListArc<ID>` containing the struct with this
    /// field for the duration of the returned reference.
    pub unsafe fn assert_ref(&self) -> &T {
        // SAFETY: The caller has shared access to the `ListArc`, so they also have shared access
        // to this field.
        unsafe { &*self.value.get() }
    }

    /// Unsafely assert that you have mutable access to the `ListArc` for this field.
    ///
    /// # Safety
    ///
    /// The caller must have mutable access to the `ListArc<ID>` containing the struct with this
    /// field for the duration of the returned reference.
    #[expect(clippy::mut_from_ref)]
    pub unsafe fn assert_mut(&self) -> &mut T {
        // SAFETY: The caller has exclusive access to the `ListArc`, so they also have exclusive
        // access to this field.
        unsafe { &mut *self.value.get() }
    }
}

/// Defines getters for a [`ListArcField`].
#[macro_export]
#[doc(hidden)]
macro_rules! define_list_arc_field_getter {
    ($pub:vis fn $name:ident(&self $(<$id:tt>)?) -> &$typ:ty { $field:ident }
     $($rest:tt)*
    ) => {
        $pub fn $name<'a>(self: &'a $crate::list::ListArc<Self $(, $id)?>) -> &'a $typ {
            let field = &(&**self).$field;
            // SAFETY: We have a shared reference to the `ListArc`.
            unsafe { $crate::list::ListArcField::<$typ $(, $id)?>::assert_ref(field) }
        }

        $crate::list::define_list_arc_field_getter!($($rest)*);
    };

    ($pub:vis fn $name:ident(&mut self $(<$id:tt>)?) -> &mut $typ:ty { $field:ident }
     $($rest:tt)*
    ) => {
        $pub fn $name<'a>(self: &'a mut $crate::list::ListArc<Self $(, $id)?>) -> &'a mut $typ {
            let field = &(&**self).$field;
            // SAFETY: We have a mutable reference to the `ListArc`.
            unsafe { $crate::list::ListArcField::<$typ $(, $id)?>::assert_mut(field) }
        }

        $crate::list::define_list_arc_field_getter!($($rest)*);
    };

    () => {};
}
pub use define_list_arc_field_getter;
