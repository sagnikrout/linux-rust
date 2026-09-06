//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/android/dbitmap.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2024 Google LLC
//
// dbitmap - dynamically sized bitmap library.
//
// Used by the binder driver to optimize the allocation of the smallest
// available descriptor ID. Each bit in the bitmap represents the state
// of an ID.
//
// A dbitmap can grow or shrink as needed. This part has been designed
// considering that users might need to briefly release their locks in
// order to allocate memory for the new bitmap. These operations then,
// are verified to determine if the grow or shrink is sill valid.
//
// This library does not provide protection against concurrent access
// by itself. Binder uses the proc->outer_lock for this purpose.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbitmap {
    pub nbits: c_uint,
    pub map: *mut c_ulong,
}

// Returns the nbits that a dbitmap can shrink to, 0 if not possible.
//
// Determine if the bitmap can shrink based on the position of
// its last set bit. If the bit is within the first quarter of
// the bitmap then shrinking is possible. In this case, the
// bitmap should shrink to half its current size.
//
// find_last_bit() returns dmap->nbits when no bits are set.
// Replace the internal bitmap with a new one of different size
//
// Verify that shrinking to @nbits is still possible. The @new
// bitmap might have been allocated without locks, so this call
// could now be outdated. In this case, free @new and move on.
//
// Returns the nbits that a dbitmap can grow to.
//
// Verify that growing to @nbits is still possible. The @new
// bitmap might have been allocated without locks, so this call
// could now be outdated. In this case, free @new and move on.
//
// Check for ENOMEM after confirming the grow operation is still
// required. This ensures we only disable the dbitmap when it's
// necessary. Once the dbitmap is disabled, binder will fallback
// to slow_desc_lookup_olocked().
//
// Finds and sets the next zero bit in the bitmap. Upon success @bit
// is populated with the index and 0 is returned. Otherwise, -ENOSPC
// is returned to indicate that a dbitmap_grow() is needed.
//
// bit = n;
