//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/attributes.h
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
// KUnit API to save and access test attributes
//
// Copyright (C) 2023, Google LLC.
// Author: Rae Moar <rmoar@google.com>
//
// struct kunit_attr_filter - representation of attributes filter with the
// attribute object and string input
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_attr_filter {
    pub attr: *mut kunit_attr,
    pub input: *mut c_char,
}

//
// Returns the name of the filter's attribute.
//
// Print all test attributes for a test case or suite.
// Output format for test cases: "# <test_name>.<attribute>: <value>"
// Output format for test suites: "# <attribute>: <value>"
//
extern "C" {
    pub fn kunit_print_attr(test_or_suite: *mut c_void, is_test: bool, test_level: c_uint);
}
//
// Returns the number of fitlers in input.
//
extern "C" {
    pub fn kunit_get_filter_count(input: *mut c_char) -> c_int;
}
//
// Parse attributes filter input and return an objects containing the
// attribute object and the string input of the next filter.
//
extern "C" {
    pub fn kunit_next_attr_filter(filters: *mut c_char, err: *mut c_int) -> kunit_attr_filter;
}
//
// Returns a copy of the suite containing only tests that pass the filter.
//
