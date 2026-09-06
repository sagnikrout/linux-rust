//! Automatically rewritten from C to Rust
//! Source: lib/bsearch.c
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
// A generic implementation of binary search for the Linux kernel
//
// Copyright (C) 2008-2009 Ksplice, Inc.
// Author: Tim Abbott <tabbott@ksplice.com>
//

//
// bsearch - binary search an array of elements
// @key: pointer to item being searched for
// @base: pointer to first element to search
// @num: number of elements
// @size: size of each element
// @cmp: pointer to comparison function
//
// This function does a binary search on the given array.  The
// contents of the array should already be in ascending sorted order
// under the provided comparison function.
//
// Note that the key need not have the same type as the elements in
// the array, e.g. key could be a string and the comparison function
// could compare the string with the struct's name field.  However, if
// the key and elements in the array are of the same type, you can use
// the same comparison function for both sort() and bsearch().
//
    void *bsearch(const void *key, const void *base, size_t num, size_t size, cmp_func_t cmp)
    {
    return __inline_bsearch(key, base, num, size, cmp);
    }
    EXPORT_SYMBOL(bsearch);
    NOKPROBE_SYMBOL(bsearch);
