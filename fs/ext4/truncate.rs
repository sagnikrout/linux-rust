//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ext4/truncate.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

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
//
// linux/fs/ext4/truncate.h
//
// Common inline functions needed for truncate support
//
// Truncate blocks that were not used by write. We have to truncate the
// pagecache as well so that corresponding buffers get properly unmapped.
//
// We don't need to call ext4_break_layouts() because the blocks we
// are truncating were never visible to userspace.
//
// Work out how many blocks we need to proceed with the next chunk of a
// truncate transaction.
//
// Give ourselves just enough room to cope with inodes in which
// i_blocks is corrupt: we've seen disk corruptions in the past
// which resulted in random data in an inode which looked enough
// like a regular file for ext4 to try to delete it.  Things
// will go a bit crazy if that happens, but at least we should
// try not to panic the whole kernel.
// But we need to bound the transaction so we don't overflow the
// journal.
