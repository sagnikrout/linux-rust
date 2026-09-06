//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/interval_tree.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct interval_tree_node {
    pub rb: rb_node,
    pub /: *mut *mut unsigned long start; / Start of interval,
    pub /: *mut *mut unsigned long last; / Last location _in_ interval,
    pub __subtree_last: c_ulong,
}

//
// struct interval_tree_span_iter - Find used and unused spans.
// @start_hole: Start of an interval for a hole when is_hole == 1
// @last_hole: Inclusive end of an interval for a hole when is_hole == 1
// @start_used: Start of a used interval when is_hole == 0
// @last_used: Inclusive end of a used interval when is_hole == 0
// @is_hole: 0 == used, 1 == is_hole, -1 == done iteration
//
// This iterator travels over spans in an interval tree. It does not return
// nodes but classifies each span as either a hole, where no nodes intersect, or
// a used, which is fully covered by nodes. Each iteration step toggles between
// hole and used until the entire range is covered. The returned spans always
// fully cover the requested range.
//
// The iterator is greedy, it always returns the largest hole or used possible,
// consolidating all consecutive nodes.
//
// Use interval_tree_span_iter_done() to detect end of iteration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct interval_tree_span_iter {
// private: not for use by the caller
    pub nodes: [*mut interval_tree_node; 2],
    pub first_index: c_ulong,
    pub last_index: c_ulong,
// public:
    pub start_hole: c_ulong,
    pub start_used: c_ulong,
}

extern "C" {
    pub fn interval_tree_span_iter_next(state: *mut interval_tree_span_iter);
}

