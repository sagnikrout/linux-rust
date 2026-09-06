
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

use super::Vmalloc;

use crate::page;

use core::{
    marker::PhantomData,
    ptr::NonNull, //
};

/// An [`Iterator`] of [`page::BorrowedPage`] items owned by a [`Vmalloc`] allocation.
///
/// # Guarantees
///
/// The pages iterated by the [`Iterator`] appear in the order as they are mapped in the CPU's
/// virtual address space ascendingly.
///
/// # Invariants
///
/// - `buf` is a valid and [`page::PAGE_SIZE`] aligned pointer into a [`Vmalloc`] allocation.
/// - `size` is the number of bytes from `buf` until the end of the [`Vmalloc`] allocation `buf`
///   points to.
pub struct VmallocPageIter<'a> {
    /// The base address of the [`Vmalloc`] buffer.
    buf: NonNull<u8>,
    /// The size of the buffer pointed to by `buf` in bytes.
    size: usize,
    /// The current page index of the [`Iterator`].
    index: usize,
    _p: PhantomData<page::BorrowedPage<'a>>,
}

impl<'a> Iterator for VmallocPageIter<'a> {
    type Item = page::BorrowedPage<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.index.checked_mul(page::PAGE_SIZE)?;

        // Even though `self.size()` may be smaller than `Self::page_count() * page::PAGE_SIZE`, it
        // is always a number between `(Self::page_count() - 1) * page::PAGE_SIZE` and
        // `Self::page_count() * page::PAGE_SIZE`, hence the check below is sufficient.
        if offset < self.size() {
            self.index += 1;
        } else {
            return None;
        }

        // SAFETY: `offset` is in the interval `[0, (self.page_count() - 1) * page::PAGE_SIZE]`,
        // hence the resulting pointer is guaranteed to be within the same allocation.
        let ptr = unsafe { self.buf.add(offset) };

        // SAFETY:
        // - `ptr` is a valid pointer to a `Vmalloc` allocation.
        // - `ptr` is valid for the duration of `'a`.
        Some(unsafe { Vmalloc::to_page(ptr) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.page_count().saturating_sub(self.index);

        (remaining, Some(remaining))
    }
}

impl<'a> VmallocPageIter<'a> {
    /// Creates a new [`VmallocPageIter`] instance.
    ///
    /// # Safety
    ///
    /// - `buf` must be a [`page::PAGE_SIZE`] aligned pointer into a [`Vmalloc`] allocation.
    /// - `buf` must be valid for at least the lifetime of `'a`.
    /// - `size` must be the number of bytes from `buf` until the end of the [`Vmalloc`] allocation
    ///   `buf` points to.
    pub unsafe fn new(buf: NonNull<u8>, size: usize) -> Self {
        // INVARIANT: By the safety requirements, `buf` is a valid and `page::PAGE_SIZE` aligned
        // pointer into a [`Vmalloc`] allocation.
        Self {
            buf,
            size,
            index: 0,
            _p: PhantomData,
        }
    }

    /// Returns the size of the backing [`Vmalloc`] allocation in bytes.
    ///
    /// Note that this is the size the [`Vmalloc`] allocation has been allocated with. Hence, this
    /// number may be smaller than `[`Self::page_count`] * [`page::PAGE_SIZE`]`.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the number of pages owned by the backing [`Vmalloc`] allocation.
    #[inline]
    pub fn page_count(&self) -> usize {
        self.size().div_ceil(page::PAGE_SIZE)
    }
}
