//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_range_fence.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

// struct xe_range_fence_ops - Xe range fence ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_range_fence_ops {
// @free: free range fence op
    pub rfence): *mut *mut void (free)(struct xe_range_fence,
}

// struct xe_range_fence - Xe range fence (address conflict tracking)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_range_fence {
// @rb: RB tree node inserted into interval tree
    pub rb: rb_node,
// @start: start address of range fence is interval tree
    pub start: u64,
// @last: last address (inclusive) of range fence is interval tree
    pub last: u64,
// @__subtree_last: interval tree internal usage
    pub __subtree_last: u64,
//
// @fence: fence signals address in range fence no longer has conflict
//
    pub fence: *mut dma_fence,
// @tree: interval tree which range fence belongs to
    pub tree: *mut xe_range_fence_tree,
//
// @cb: callback when fence signals to remove range fence free from interval tree
//
    pub cb: dma_fence_cb,
// @link: used to defer free of range fence to non-irq context
    pub link: llist_node,
// @ops: range fence ops
    pub ops: *const xe_range_fence_ops,
}

// struct xe_range_fence_tree - interval tree to store range fences
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_range_fence_tree {
// @root: interval tree root
    pub root: rb_root_cached,
// @list: list of pending range fences to be freed
    pub list: llist_head,
}

extern "C" {
    pub fn xe_range_fence_tree_init(tree: *mut xe_range_fence_tree);
}
extern "C" {
    pub fn xe_range_fence_tree_fini(tree: *mut xe_range_fence_tree);
}
