//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-vdo/indexer/radix-sort.h
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
// Radix sort is implemented using an American Flag sort, an unstable, in-place 8-bit radix
// exchange sort. This is adapted from the algorithm in the paper by Peter M. McIlroy, Keith
// Bostic, and M. Douglas McIlroy, "Engineering Radix Sort".
//
// http://www.usenix.org/publications/compsystems/1993/win_mcilroy.pdf
//
extern "C" {
    pub fn uds_make_radix_sorter(count: c_uint, sorter: *mut radix_sorter) -> int __must_check;
}
extern "C" {
    pub fn uds_free_radix_sorter(sorter: *mut radix_sorter);
}
