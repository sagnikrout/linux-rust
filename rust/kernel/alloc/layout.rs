
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

//! Memory layout.
//!
//! Custom layout types extending or improving [`Layout`].

use core::{
    alloc::Layout,
    marker::PhantomData, //
};

/// Error when constructing an [`ArrayLayout`].
pub struct LayoutError;

/// A layout for an array `[T; n]`.
///
/// # Invariants
///
/// - `len * size_of::<T>() <= isize::MAX`.
pub struct ArrayLayout<T> {
    len: usize,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Clone for ArrayLayout<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for ArrayLayout<T> {}

const ISIZE_MAX: usize = isize::MAX as usize;

impl<T> ArrayLayout<T> {
    /// Creates a new layout for `[T; 0]`.
    pub const fn empty() -> Self {
        // INVARIANT: `0 * size_of::<T>() <= isize::MAX`.
        Self {
            len: 0,
            _phantom: PhantomData,
        }
    }

    /// Creates a new layout for `[T; len]`.
    ///
    /// # Errors
    ///
    /// When `len * size_of::<T>()` overflows or when `len * size_of::<T>() > isize::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use kernel::alloc::layout::{
    /// #     ArrayLayout,
    /// #     LayoutError, //
    /// # };
    /// let layout = ArrayLayout::<i32>::new(15)?;
    /// assert_eq!(layout.len(), 15);
    ///
    /// // Errors because `len * size_of::<T>()` overflows.
    /// let layout = ArrayLayout::<i32>::new(isize::MAX as usize);
    /// assert!(layout.is_err());
    ///
    /// // Errors because `len * size_of::<i32>() > isize::MAX`,
    /// // even though `len < isize::MAX`.
    /// let layout = ArrayLayout::<i32>::new(isize::MAX as usize / 2);
    /// assert!(layout.is_err());
    ///
    /// # Ok::<(), Error>(())
    /// ```
    pub const fn new(len: usize) -> Result<Self, LayoutError> {
        match len.checked_mul(core::mem::size_of::<T>()) {
            Some(size) if size <= ISIZE_MAX => {
                // INVARIANT: We checked above that `len * size_of::<T>() <= isize::MAX`.
                Ok(Self {
                    len,
                    _phantom: PhantomData,
                })
            }
            _ => Err(LayoutError),
        }
    }

    /// Creates a new layout for `[T; len]`.
    ///
    /// # Safety
    ///
    /// `len` must be a value, for which `len * size_of::<T>() <= isize::MAX` is true.
    pub const unsafe fn new_unchecked(len: usize) -> Self {
        // INVARIANT: By the safety requirements of this function
        // `len * size_of::<T>() <= isize::MAX`.
        Self {
            len,
            _phantom: PhantomData,
        }
    }

    /// Returns the number of array elements represented by this layout.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` when no array elements are represented by this layout.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the size of the [`ArrayLayout`] in bytes.
    pub const fn size(&self) -> usize {
        self.len() * core::mem::size_of::<T>()
    }
}

impl<T> From<ArrayLayout<T>> for Layout {
    fn from(value: ArrayLayout<T>) -> Self {
        let res = Layout::array::<T>(value.len);
        // SAFETY: By the type invariant of `ArrayLayout` we have
        // `len * size_of::<T>() <= isize::MAX` and thus the result must be `Ok`.
        unsafe { res.unwrap_unchecked() }
    }
}
