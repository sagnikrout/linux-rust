//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/completion.h
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
// (C) Copyright 2001 Linus Torvalds
//
// Atomic wait-for-completion handler data structures.
// See kernel/sched/completion.c for details.
//

//
// struct completion - structure used to maintain state for a "completion"
//
// This is the opaque structure used to maintain the state for a "completion".
// Completions currently use a FIFO to queue threads that have to wait for
// the "completion" event.
//
// See also:  complete(), wait_for_completion() (and friends _timeout,
// _interruptible, _interruptible_timeout, and _killable), init_completion(),
// reinit_completion(), and macros DECLARE_COMPLETION(),
// DECLARE_COMPLETION_ONSTACK().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct completion {
    pub done: c_uint,
    pub wait: swait_queue_head,
}

//
// DECLARE_COMPLETION - declare and initialize a completion structure
// @work:  identifier for the completion structure
//
// This macro declares and initializes a completion structure. Generally used
// for static declarations. You should use the _ONSTACK variant for automatic
// variables.
//

//
// Lockdep needs to run a non-constant initializer for on-stack
// completions - so we use the _ONSTACK() variant for those that
// are on the kernel stack:
//
// DECLARE_COMPLETION_ONSTACK - declare and initialize a completion structure
// @work:  identifier for the completion structure
//
// This macro declares and initializes a completion structure on the kernel
// stack.
//

//
// init_completion - Initialize a dynamically allocated completion
// @x:  pointer to completion structure that is to be initialized
//
// This inline function will initialize a dynamically created completion
// structure.
//
// reinit_completion - reinitialize a completion structure
// @x:  pointer to completion structure that is to be reinitialized
//
// This inline function should be used to reinitialize a completion structure so it can
// be reused. This is especially important after complete_all() is used.
//
extern "C" {
    pub fn wait_for_completion(: *mut completion);
}
extern "C" {
    pub fn wait_for_completion_io(: *mut completion);
}
extern "C" {
    pub fn wait_for_completion_interruptible(x: *mut completion) -> c_int;
}
extern "C" {
    pub fn wait_for_completion_killable(x: *mut completion) -> c_int;
}
extern "C" {
    pub fn wait_for_completion_state(x: *mut completion, state: c_uint) -> c_int;
}
extern "C" {
    pub fn try_wait_for_completion(x: *mut completion) -> bool;
}
extern "C" {
    pub fn completion_done(x: *mut completion) -> bool;
}
extern "C" {
    pub fn complete(: *mut completion);
}
extern "C" {
    pub fn complete_on_current_cpu(x: *mut completion);
}
extern "C" {
    pub fn complete_all(: *mut completion);
}
