//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/bpf/rqspinlock.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Resilient Queued Spin Lock defines
//
// (C) Copyright 2024-2025 Meta Platforms, Inc. and affiliates.
//
// Authors: Kumar Kartikeya Dwivedi <memxor@gmail.com>
//

//
// try_cmpxchg_tail - Return result of cmpxchg of tail word with a new value
// @lock: Pointer to queued spinlock structure
// @tail: The tail to compare against
// @new_tail: The new queue tail code word
// Return: Bool to indicate whether the cmpxchg operation succeeded
//
// This is used by the head of the wait queue to clean up the queue.
// Provides relaxed ordering, since observers only rely on initialized
// state of the node which was made visible through the xchg_tail operation,
// i.e. through the smp_wmb preceding xchg_tail.
//
// We avoid using 16-bit cmpxchg, which is not available on all architectures.
//
// Is the tail part we compare to already stale? Fail.
//
// Encode latest locked/pending state for new tail.
//
