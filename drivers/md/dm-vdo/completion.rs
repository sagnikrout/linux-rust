//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/completion.h
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
// Copyright 2023 Red Hat
//

//
// vdo_run_completion() - Run a completion's callback or error handler on the current thread.
//
// Context: This function must be called from the correct callback thread.
//
extern "C" {
    pub fn vdo_set_completion_result(completion: *mut vdo_completion, result: c_int);
}
//
// vdo_reset_completion() - Reset a completion to a clean state, while keeping the type, vdo and
// parent information.
//
// vdo_launch_completion() - Launch a completion with default priority.
//
// vdo_continue_completion() - Continue processing a completion.
// @result: The current result (will not mask older errors).
//
// Continue processing a completion by setting the current result and calling
// vdo_launch_completion().
//
extern "C" {
    pub fn vdo_finish_completion(completion: *mut vdo_completion);
}
//
// vdo_fail_completion() - Set the result of a completion if it does not already have an error,
// then finish it.
//
// vdo_assert_completion_type() - Assert that a completion is of the correct type.
//
// Return: VDO_SUCCESS or an error
//
// vdo_launch_completion_callback() - Set the callback for a completion and launch it immediately.
//
// vdo_prepare_completion() - Prepare a completion for launch.
//
// Resets the completion, and then sets its callback, error handler, callback thread, and parent.
//
// vdo_prepare_completion_for_requeue() - Prepare a completion for launch ensuring that it will
// always be requeued.
//
// Resets the completion, and then sets its callback, error handler, callback thread, and parent.
//
