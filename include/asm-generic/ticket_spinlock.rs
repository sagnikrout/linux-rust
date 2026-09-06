//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/ticket_spinlock.h
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
// 'Generic' ticket-lock implementation.
//
// It relies on atomic_fetch_add() having well defined forward progress
// guarantees under contention. If your architecture cannot provide this, stick
// to a test-and-set lock.
//
// It also relies on atomic_fetch_add() being safe vs smp_store_release() on a
// sub-word of the value. This is generally true for anything LL/SC although
// you'd be hard pressed to find anything useful in architecture specifications
// about this. If your architecture cannot do this you might be better off with
// a test-and-set.
//
// It further assumes atomic_*_release() + atomic_*_acquire() is RCpc and hence
// uses atomic_fetch_add() which is RCsc to create an RCsc hot path, along with
// a full fence after the spin to upgrade the otherwise-RCpc
// atomic_cond_read_acquire().
//
// The implementation uses smp_cond_load_acquire() to spin, so if the
// architecture has WFE like instructions to sleep instead of poll for word
// modifications be sure to implement that (see ARM64 for example).
//

//
// atomic_cond_read_acquire() is RCpc, but rather than defining a
// custom cond_read_rcsc() here we just emit a full fence.  We only
// need the prior reads before subsequent writes ordering from
// smb_mb(), but as atomic_cond_read_acquire() just emits reads and we
// have no outstanding writes due to the atomic_fetch_add() the extra
// orderings are free.
//

//
// Remapping spinlock architecture specific functions to the corresponding
// ticket spinlock functions.
//

