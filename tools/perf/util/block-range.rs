//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/block-range.h
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
// struct block_range - non-overlapping parts of basic blocks
// @node:	treenode
// @start:	inclusive start of range
// @end:	inclusive end of range
// @is_target:	@start is a jump target
// @is_branch:	@end is a branch instruction
// @coverage:	number of blocks that cover this range
// @taken:	number of times the branch is taken (requires @is_branch)
// @pred:	number of times the taken branch was predicted
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_range {
    pub node: rb_node,
    pub sym: *mut symbol,
    pub start: u64,
    pub end: u64,
    pub is_branch: int is_target,,
    pub coverage: u64,
    pub entry: u64,
    pub taken: u64,
    pub pred: u64,
}

extern "C" {
    pub fn rb_entry(_arg: n, block_range: struct, _arg: node) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_range_iter {
    pub start: *mut block_range,
    pub end: *mut block_range,
}

extern "C" {
    pub fn block_range__create(start: u64, end: u64) -> block_range_iter;
}
extern "C" {
    pub fn block_range__coverage(br: *mut block_range) -> double;
}
