//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/futex/include/futex_thread.h
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

pub const WAIT_FOR_THREAD_SECS: c_int = 1;

pub const WAIT_THREAD_RETRIES: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_thread {
    pub thread: pthread_t,
    pub barrier: pthread_barrier_t,
    pub tid: pid_t,
    pub arg): *mut *mut int (threadfn)(void,
    pub arg: *mut c_void,
    pub retval: c_int,
}

//
// futex_wait_for_thread - Wait for the child thread to sleep in the futex context
// @t:          Thread handle.
// @_metadata:	Test metadata for TH_LOG() context
//
// If /proc/... is not available, sleep
//
// futex_thread_create - Create a new thread for testing.
// @t:        The handle of the newly created thread.
// @threadfn: The new thread starts execution by invoking threadfn
// @arg:      The parameters passed to threadfn.
//
// futex_thread_destroy - Wait for and reclaim the resources of the thread.
// @t:      Thread handle.
//
