//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/assert.h
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
// Assertion and expectation serialization API.
//
// Copyright (C) 2019, Google LLC.
// Author: Brendan Higgins <brendanhiggins@google.com>
//

//
// enum kunit_assert_type - Type of expectation/assertion.
// @KUNIT_ASSERTION: Used to denote that a kunit_assert represents an assertion.
// @KUNIT_EXPECTATION: Denotes that a kunit_assert represents an expectation.
//
// Used in conjunction with a &struct kunit_assert to denote whether it
// represents an expectation or an assertion.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kunit_assert_type {
    KUNIT_ASSERTION,
    KUNIT_EXPECTATION,
}

//
// struct kunit_loc - Identifies the source location of a line of code.
// @line: the line number in the file.
// @file: the file name.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_loc {
    pub line: c_int,
    pub file: *const c_char,
}

//
// struct kunit_assert - Data for printing a failed assertion or expectation.
//
// Represents a failed expectation/assertion. Contains all the data necessary to
// format a string to a user reporting the failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_assert {
    pub stream): *mut string_stream,
    pub stream): *mut string_stream,
//
// struct kunit_fail_assert - Represents a plain fail expectation/assertion.
// @assert: The parent of this type.
//
// Represents a simple KUNIT_FAIL/KUNIT_FAIL_AND_ABORT that always fails.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_fail_assert {
    pub assert: kunit_assert,
}

//
// struct kunit_unary_assert - Represents a KUNIT_{EXPECT|ASSERT}_{TRUE|FALSE}
// @assert: The parent of this type.
// @condition: A string representation of a conditional expression.
// @expected_true: True if of type KUNIT_{EXPECT|ASSERT}_TRUE, false otherwise.
//
// Represents a simple expectation or assertion that simply asserts something is
// true or false. In other words, represents the expectations:
// KUNIT_{EXPECT|ASSERT}_{TRUE|FALSE}
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_unary_assert {
    pub assert: kunit_assert,
    pub condition: *const c_char,
    pub expected_true: bool,
}

//
// struct kunit_ptr_not_err_assert - An expectation/assertion that a pointer is
// not NULL and not a -errno.
// @assert: The parent of this type.
// @text: A string representation of the expression passed to the expectation.
// @value: The actual evaluated pointer value of the expression.
//
// Represents an expectation/assertion that a pointer is not null and is does
// not contain a -errno. (See IS_ERR_OR_NULL().)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_ptr_not_err_assert {
    pub assert: kunit_assert,
    pub text: *const c_char,
    pub value: *const c_void,
}

//
// struct kunit_binary_assert_text - holds strings for &struct
// kunit_binary_assert and friends to try and make the structs smaller.
// @operation: A string representation of the comparison operator (e.g. "==").
// @left_text: A string representation of the left expression (e.g. "2+2").
// @right_text: A string representation of the right expression (e.g. "2+2").
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_binary_assert_text {
    pub operation: *const c_char,
    pub left_text: *const c_char,
    pub right_text: *const c_char,
}

//
// struct kunit_binary_assert - An expectation/assertion that compares two
// non-pointer values (for example, KUNIT_EXPECT_EQ(test, 1 + 1, 2)).
// @assert: The parent of this type.
// @text: Holds the textual representations of the operands and op (e.g.  "==").
// @left_value: The actual evaluated value of the expression in the left slot.
// @right_value: The actual evaluated value of the expression in the right slot.
//
// Represents an expectation/assertion that compares two non-pointer values. For
// example, to expect that 1 + 1 == 2, you can use the expectation
// KUNIT_EXPECT_EQ(test, 1 + 1, 2);
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_binary_assert {
    pub assert: kunit_assert,
    pub text: *const kunit_binary_assert_text,
    pub left_value: c_longlong,
    pub right_value: c_longlong,
}

//
// struct kunit_binary_ptr_assert - An expectation/assertion that compares two
// pointer values (for example, KUNIT_EXPECT_PTR_EQ(test, foo, bar)).
// @assert: The parent of this type.
// @text: Holds the textual representations of the operands and op (e.g.  "==").
// @left_value: The actual evaluated value of the expression in the left slot.
// @right_value: The actual evaluated value of the expression in the right slot.
//
// Represents an expectation/assertion that compares two pointer values. For
// example, to expect that foo and bar point to the same thing, you can use the
// expectation KUNIT_EXPECT_PTR_EQ(test, foo, bar);
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_binary_ptr_assert {
    pub assert: kunit_assert,
    pub text: *const kunit_binary_assert_text,
    pub left_value: *const c_void,
    pub right_value: *const c_void,
}

//
// struct kunit_binary_str_assert - An expectation/assertion that compares two
// string values (for example, KUNIT_EXPECT_STREQ(test, foo, "bar")).
// @assert: The parent of this type.
// @text: Holds the textual representations of the operands and comparator.
// @left_value: The actual evaluated value of the expression in the left slot.
// @right_value: The actual evaluated value of the expression in the right slot.
//
// Represents an expectation/assertion that compares two string values. For
// example, to expect that the string in foo is equal to "bar", you can use the
// expectation KUNIT_EXPECT_STREQ(test, foo, "bar");
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_binary_str_assert {
    pub assert: kunit_assert,
    pub text: *const kunit_binary_assert_text,
    pub left_value: *const c_char,
    pub right_value: *const c_char,
}

//
// struct kunit_mem_assert - An expectation/assertion that compares two
// memory blocks.
// @assert: The parent of this type.
// @text: Holds the textual representations of the operands and comparator.
// @left_value: The actual evaluated value of the expression in the left slot.
// @right_value: The actual evaluated value of the expression in the right slot.
// @size: Size of the memory block analysed in bytes.
//
// Represents an expectation/assertion that compares two memory blocks. For
// example, to expect that the first three bytes of foo is equal to the
// first three bytes of bar, you can use the expectation
// KUNIT_EXPECT_MEMEQ(test, foo, bar, 3);
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_mem_assert {
    pub assert: kunit_assert,
    pub text: *const kunit_binary_assert_text,
    pub left_value: *const c_void,
    pub right_value: *const c_void,
    pub size: usize,
}

extern "C" {
    pub fn is_literal(text: *const c_char, value: c_longlong) -> bool;
}
extern "C" {
    pub fn is_str_literal(text: *const c_char, value: *const c_char) -> bool;
}

