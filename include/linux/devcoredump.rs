//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/devcoredump.h
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
// Copyright(c) 2015 Intel Deutschland GmbH
//

// if data isn't read by userspace after 5 minutes then delete it

//
// _devcd_free_sgtable - free all the memory of the given scatterlist table
// (i.e. both pages and scatterlist instances)
// NOTE: if two tables allocated and chained using the sg_chain function then
// this function should be called only once on the first table
// @table: pointer to sg_table to free
//
// free pages
// then free all chained tables
// free the last table

extern "C" {
    pub fn dev_coredump_put(dev: *mut device);
}

//
// dev_coredumpm - create device coredump with read/free methods
// @dev: the struct device for the crashed device
// @owner: the module that contains the read/free functions, use %THIS_MODULE
// @data: data cookie for the @read/@free functions
// @datalen: length of the data
// @gfp: allocation flags
// @read: function to read from the given buffer
// @free: function to free the given buffer
//
// Creates a new device coredump for the given device. If a previous one hasn't
// been read yet, the new coredump is discarded. The data lifetime is determined
// by the device coredump framework and when it is no longer needed the @free
// function will be called to free the data.
//
