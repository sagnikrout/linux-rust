//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/try-catch.h
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
// An API to allow a function, that may fail, to be executed, and recover in a
// controlled manner.
//
// Copyright (C) 2019, Google LLC.
// Author: Brendan Higgins <brendanhiggins@google.com>
//

extern "C" {
    pub fn void(: *mut *mut kunit_try_catch_func_t)(void) -> typedef;
}
//
// struct kunit_try_catch - provides a generic way to run code which might fail.
// @test: The test case that is currently being executed.
// @try_result: Contains any errno obtained while running test case.
// @try: The function, the test case, to attempt to run.
// @catch: The function called if @try bails out.
// @context: used to pass user data to the try and catch functions.
//
// kunit_try_catch provides a generic, architecture independent way to execute
// an arbitrary function of type kunit_try_catch_func_t which may bail out by
// calling kunit_try_catch_throw(). If kunit_try_catch_throw() is called, @try
// is stopped at the site of invocation and @catch is called.
//
// struct kunit_try_catch provides a generic interface for the functionality
// needed to implement kunit->abort() which in turn is needed for implementing
// assertions. Assertions allow stating a precondition for a test simplifying
// how test cases are written and presented.
//
// Assertions are like expectations, except they abort (call
// kunit_try_catch_throw()) when the specified condition is not met. This is
// useful when you look at a test case as a logical statement about some piece
// of code, where assertions are the premises for the test case, and the
// conclusion is a set of predicates, rather expectations, that must all be
// true. If your premises are violated, it does not makes sense to continue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_try_catch {
// private: internal use only.
    pub test: *mut kunit,
    pub try_result: c_int,
    pub try: kunit_try_catch_func_t,
    pub catch: kunit_try_catch_func_t,
    pub timeout: c_ulong,
    pub context: *mut c_void,
}

extern "C" {
    pub fn kunit_try_catch_run(try_catch: *mut kunit_try_catch, context: *mut c_void);
}
extern "C" {
    pub fn kunit_try_catch_throw(try_catch: *mut kunit_try_catch) -> void __noreturn;
}
