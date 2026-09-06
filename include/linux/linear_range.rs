//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/linear_range.h
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
// Copyright (C) 2020 ROHM Semiconductors

//
// struct linear_range - table of selector - value pairs
//
// Define a lookup-table for range of values. Intended to help when looking
// for a register value matching certaing physical measure (like voltage).
// Usable when increment of one in register always results a constant increment
// of the physical measure (like voltage).
//
// @min:  Lowest value in range
// @min_sel: Lowest selector for range
// @max_sel: Highest selector for range
// @step: Value step size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct linear_range {
    pub min: c_uint,
    pub min_sel: c_uint,
    pub max_sel: c_uint,
    pub step: c_uint,
}

extern "C" {
    pub fn linear_range_values_in_range(r: *const linear_range) -> c_uint;
}
extern "C" {
    pub fn linear_range_get_max_value(r: *const linear_range) -> c_uint;
}
