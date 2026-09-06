//! Automatically rewritten from C Header to Rust Module
//! Source: tools/net/ynl/ynltool/json_writer.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Simple streaming JSON writer
//
// This takes care of the annoying bits of JSON syntax like the commas
// after elements
//
// Authors:	Stephen Hemminger <stephen@networkplumber.org>
//

// Opaque class structure
pub type json_writer_t = json_writer;
// Create a new JSON stream
// End output to JSON stream
extern "C" {
    pub fn jsonw_destroy(self_p: *mut json_writer_t);
}
// Cause output to have pretty whitespace
extern "C" {
    pub fn jsonw_pretty(self: *mut json_writer_t, on: bool);
}
// Reset separator to create new JSON
extern "C" {
    pub fn jsonw_reset(self: *mut json_writer_t);
}
// Add property name
extern "C" {
    pub fn jsonw_name(self: *mut json_writer_t, name: *const c_char);
}
// Add value
extern "C" {
    pub fn jsonw_string(self: *mut json_writer_t, value: *const c_char);
}
extern "C" {
    pub fn jsonw_bool(self: *mut json_writer_t, value: bool);
}
extern "C" {
    pub fn jsonw_float(self: *mut json_writer_t, number: double);
}
extern "C" {
    pub fn jsonw_float_fmt(self: *mut json_writer_t, fmt: *const c_char, num: double);
}
extern "C" {
    pub fn jsonw_uint(self: *mut json_writer_t, number: u64);
}
extern "C" {
    pub fn jsonw_hu(self: *mut json_writer_t, number: c_ushort);
}
extern "C" {
    pub fn jsonw_int(self: *mut json_writer_t, number: i64);
}
extern "C" {
    pub fn jsonw_null(self: *mut json_writer_t);
}
extern "C" {
    pub fn jsonw_lluint(self: *mut json_writer_t, num: unsigned long long int);
}
// Useful Combinations of name and value
extern "C" {
    pub fn jsonw_string_field(self: *mut json_writer_t, prop: *const c_char, val: *const c_char);
}
extern "C" {
    pub fn jsonw_bool_field(self: *mut json_writer_t, prop: *const c_char, value: bool);
}
extern "C" {
    pub fn jsonw_float_field(self: *mut json_writer_t, prop: *const c_char, num: double);
}
extern "C" {
    pub fn jsonw_uint_field(self: *mut json_writer_t, prop: *const c_char, num: u64);
}
extern "C" {
    pub fn jsonw_hu_field(self: *mut json_writer_t, prop: *const c_char, num: c_ushort);
}
extern "C" {
    pub fn jsonw_int_field(self: *mut json_writer_t, prop: *const c_char, num: i64);
}
extern "C" {
    pub fn jsonw_null_field(self: *mut json_writer_t, prop: *const c_char);
}
// Collections
extern "C" {
    pub fn jsonw_start_object(self: *mut json_writer_t);
}
extern "C" {
    pub fn jsonw_end_object(self: *mut json_writer_t);
}
extern "C" {
    pub fn jsonw_start_array(self: *mut json_writer_t);
}
extern "C" {
    pub fn jsonw_end_array(self: *mut json_writer_t);
}
// Override default exception handling
extern "C" {
    pub fn void(: *const jsonw_err_handler_fn)(char) -> typedef;
}
