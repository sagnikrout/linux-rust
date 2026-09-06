//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/bpf/range_tree.h
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
pub const _RANGE_TREE_H: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct range_tree {
// root of interval tree
    pub it_root: rb_root_cached,
// root of rbtree of interval sizes
    pub range_size_root: rb_root_cached,
}

extern "C" {
    pub fn range_tree_init(rt: *mut range_tree);
}
extern "C" {
    pub fn range_tree_destroy(rt: *mut range_tree);
}
extern "C" {
    pub fn range_tree_clear(rt: *mut range_tree, start: u32, len: u32) -> c_int;
}
extern "C" {
    pub fn range_tree_set(rt: *mut range_tree, start: u32, len: u32) -> c_int;
}
extern "C" {
    pub fn is_range_tree_set(rt: *mut range_tree, start: u32, len: u32) -> c_int;
}
extern "C" {
    pub fn range_tree_find(rt: *mut range_tree, len: u32) -> i64;
}
