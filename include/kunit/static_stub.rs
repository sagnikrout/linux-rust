//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/static_stub.h
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
// KUnit function redirection (static stubbing) API.
//
// Copyright (C) 2022, Google LLC.
// Author: David Gow <davidgow@google.com>
//

// If CONFIG_KUNIT is not enabled, these stubs quietly disappear.

//
// KUNIT_STATIC_STUB_REDIRECT() - call a replacement 'static stub' if one exists
// @real_fn_name: The name of this function (as an identifier, not a string)
// @args: All of the arguments passed to this function
//
// This is a function prologue which is used to allow calls to the current
// function to be redirected by a KUnit test. KUnit tests can call
// kunit_activate_static_stub() to pass a replacement function in. The
// replacement function will be called by KUNIT_STATIC_STUB_REDIRECT(), which
// will then return from the function. If the caller is not in a KUnit context,
// the function will continue execution as normal.
//
// Example:
//
// .. code-block:: c
//
// int real_func(int n)
// {
// KUNIT_STATIC_STUB_REDIRECT(real_func, n);
// return 0;
// }
//
// int replacement_func(int n)
// {
// return 42;
// }
//
// void example_test(struct kunit *test)
// {
// kunit_activate_static_stub(test, real_func, replacement_func);
// KUNIT_EXPECT_EQ(test, real_func(1), 42);
// }
//

// Helper function for kunit_activate_static_stub(). The macro does
// typechecking, so use it instead.
//
// kunit_activate_static_stub() - replace a function using static stubs.
// @test: A pointer to the 'struct kunit' test context for the current test.
// @real_fn_addr: The address of the function to replace.
// @replacement_addr: The address of the function to replace it with.
//
// When activated, calls to real_fn_addr from within this test (even if called
// indirectly) will instead call replacement_addr. The function pointed to by
// real_fn_addr must begin with the static stub prologue in
// KUNIT_STATIC_STUB_REDIRECT() for this to work. real_fn_addr and
// replacement_addr must have the same type.
//
// The redirection can be disabled again with kunit_deactivate_static_stub().
//

//
// kunit_deactivate_static_stub() - disable a function redirection
// @test: A pointer to the 'struct kunit' test context for the current test.
// @real_fn_addr: The address of the function to no-longer redirect
//
// Deactivates a redirection configured with kunit_activate_static_stub(). After
// this function returns, calls to real_fn_addr() will execute the original
// real_fn, not any previously-configured replacement.
//
extern "C" {
    pub fn kunit_deactivate_static_stub(test: *mut kunit, real_fn_addr: *mut c_void);
}

