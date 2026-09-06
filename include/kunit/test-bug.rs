//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/test-bug.h
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
// KUnit API providing hooks for non-test code to interact with tests.
//
// Copyright (C) 2020, Google LLC.
// Author: Uriel Guajardo <urielguajardo@google.com>
//

// Static key if KUnit is running any tests.
// Hooks table: a table of function pointers filled in when kunit loads
//
// kunit_get_current_test() - Return a pointer to the currently running
// KUnit test.
//
// If a KUnit test is running in the current task, returns a pointer to its
// associated struct kunit. This pointer can then be passed to any KUnit
// function or assertion. If no test is running (or a test is running in a
// different task), returns NULL.
//
// This function is safe to call even when KUnit is disabled. If CONFIG_KUNIT
// is not enabled, it will compile down to nothing and will return quickly no
// test is running.
//
// kunit_fail_current_test() - If a KUnit test is running, fail it.
//
// If a KUnit test is running in the current task, mark that test as failed.
//

// Guaranteed to be non-NULL when kunit_running true*/	\
//
// kunit_is_suppressed_warning() - Check if warnings are being suppressed
// by the current KUnit test.
// @count: if true, increment the suppression counter on match.
//
// Returns true if the current task has active warning suppression.
// Uses the kunit_running static branch for zero overhead when no tests run.
//
// A single WARN*() may traverse multiple call sites in the warning path
// (e.g., __warn_printk() and __report_bug()). Pass @count = true at the
// primary suppression point to count each warning exactly once, and
// @count = false at secondary points to suppress output without
// inflating the count.
//

