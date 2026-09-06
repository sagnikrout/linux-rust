//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/folio_batch.h
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
// include/linux/folio_batch.h
//
// In many places it is efficient to batch an operation up against multiple
// folios.  A folio_batch is a container which is used for that.
//

// 31 pointers + header align the folio_batch structure to a power of two
pub const FOLIO_BATCH_SIZE: c_int = 31;
//
// struct folio_batch - A collection of folios.
//
// The folio_batch is used to amortise the cost of retrieving and
// operating on a set of folios.  The order of folios in the batch may be
// significant (eg delete_from_page_cache_batch()).  Some users of the
// folio_batch store "exceptional" entries in it which can be removed
// by calling folio_batch_remove_exceptionals().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct folio_batch {
    pub nr: c_uchar,
    pub i: c_uchar,
    pub percpu_pvec_drained: bool,
    pub folios: [*mut folio; FOLIO_BATCH_SIZE],
}

//
// folio_batch_init() - Initialise a batch of folios
// @fbatch: The folio batch.
//
// A freshly initialised folio_batch contains zero folios.
//
// folio_batch_add() - Add a folio to a batch.
// @fbatch: The folio batch.
// @folio: The folio to add.
//
// The folio is added to the end of the batch.
// The batch must have previously been initialised using folio_batch_init().
//
// Return: The number of slots still available.
//
extern "C" {
    pub fn folio_batch_space(_arg: fbatch) -> return;
}
//
// folio_batch_next - Return the next folio to process.
// @fbatch: The folio batch being processed.
//
// Use this function to implement a queue of folios.
//
// Return: The next folio in the queue, or NULL if the queue is empty.
//
extern "C" {
    pub fn __folio_batch_release(fbatch: *mut folio_batch);
}
extern "C" {
    pub fn folio_batch_remove_exceptionals(fbatch: *mut folio_batch);
}
