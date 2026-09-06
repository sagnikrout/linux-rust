//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/strfilter.h
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
// General purpose glob matching filter

// A node of string filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strfilter_node {
    pub /: *mut *mut *mut strfilter_node l; / Tree left branch (for &,|),
    pub /: *mut *mut *mut strfilter_node r; / Tree right branch (for !,&,|),
    pub /: *const *const *const char p; / Operator or rule,
}

// String filter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strfilter {
    pub root: *mut strfilter_node,
}

//
// strfilter__new - Create a new string filter
// @rules: Filter rule, which is a combination of glob expressions.
// @err: Pointer which points an error detected on @rules
//
// Parse @rules and return new strfilter. Return NULL if an error detected.
// In that case, *@err will indicate where it is detected, and *@err is NULL
// if a memory allocation is failed.
//
// strfilter__or - Append an additional rule by logical-or
// @filter: Original string filter
// @rules: Filter rule to be appended at left of the root of
// @filter by using logical-or.
// @err: Pointer which points an error detected on @rules
//
// Parse @rules and join it to the @filter by using logical-or.
// Return 0 if success, or return the error code.
//
// strfilter__add - Append an additional rule by logical-and
// @filter: Original string filter
// @rules: Filter rule to be appended at left of the root of
// @filter by using logical-and.
// @err: Pointer which points an error detected on @rules
//
// Parse @rules and join it to the @filter by using logical-and.
// Return 0 if success, or return the error code.
//
// strfilter__compare - compare given string and a string filter
// @filter: String filter
// @str: target string
//
// Compare @str and @filter. Return true if the str match the rule
//
extern "C" {
    pub fn strfilter__compare(filter: *mut strfilter, str: *const c_char) -> bool;
}
//
// strfilter__delete - delete a string filter
// @filter: String filter to delete
//
// Delete @filter.
//
extern "C" {
    pub fn strfilter__delete(filter: *mut strfilter);
}
//
// strfilter__string - Reconstruct a rule string from filter
// @filter: String filter to reconstruct
//
// Reconstruct a rule string from @filter. This will be good for
// debug messages. Note that returning string must be freed afterward.
//
