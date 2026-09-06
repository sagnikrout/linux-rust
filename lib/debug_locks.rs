//! Automatically rewritten from C to Rust
//! Source: lib/debug_locks.c
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
// lib/debug_locks.c
//
// Generic place for common debugging facilities for various locks:
// spinlocks, rwlocks, mutexes and rwsems.
//
// Started by Ingo Molnar:
//
// Copyright (C) 2006 Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
//

//
// We want to turn all lock-debugging facilities on/off at once,
// via a global flag. The reason is that once a single bug has been
// detected and reported, there might be cascade of followup bugs
// that would just muddy the log. So we report the first one and
// shut up after that.
//
    let mut __read_mostly: int debug_locks = 1;
    EXPORT_SYMBOL_GPL(debug_locks);
//
// The locking-testsuite uses <debug_locks_silent> to get a
// 'silent failure': nothing is printed to the console when
// a locking bug is detected.
//
    int debug_locks_silent __read_mostly;
    EXPORT_SYMBOL_GPL(debug_locks_silent);
//
// Generic 'turn off all lock debugging' function:
//
#[no_mangle]
pub unsafe extern "C" fn debug_locks_off() -> c_int {
    int debug_locks_off(void)
    {
    if (debug_locks && __debug_locks_off()) {
    if (!debug_locks_silent) {
    console_verbose();
    return 1;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(debug_locks_off);
