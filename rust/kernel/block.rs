
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

//! Types for working with the block layer.

pub mod mq;

/// Bit mask for masking out [`SECTOR_SIZE`].
pub const SECTOR_MASK: u32 = bindings::SECTOR_MASK;

/// Sectors are size `1 << SECTOR_SHIFT`.
pub const SECTOR_SHIFT: u32 = bindings::SECTOR_SHIFT;

/// Size of a sector.
pub const SECTOR_SIZE: u32 = bindings::SECTOR_SIZE;

/// The difference between the size of a page and the size of a sector,
/// expressed as a power of two.
pub const PAGE_SECTORS_SHIFT: u32 = bindings::PAGE_SECTORS_SHIFT;
