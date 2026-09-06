//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/iommufd/double_span.h
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
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES.
//

//
// This is a variation of the general interval_tree_span_iter that computes the
// spans over the union of two different interval trees. Used ranges are broken
// up and reported based on the tree that provides the interval. The first span
// always takes priority. Like interval_tree_span_iter it is greedy and the same
// value of is_used will not repeat on two iteration cycles.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct interval_tree_double_span_iter {
    pub itrees: [*mut rb_root_cached; 2],
    pub spans: [interval_tree_span_iter; 2],
    pub start_hole: c_ulong,
    pub start_used: c_ulong,
}

// 0 = hole, 1 = used span[0], 2 = used span[1], -1 done iteration

