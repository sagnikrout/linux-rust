//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma-buf/sync_debug.h
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


//
// Sync File validation framework and debug infomation
//
// Copyright (C) 2012 Google, Inc.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

//
// struct sync_timeline - sync object
// @kref:		reference count on fence.
// @name:		name of the sync_timeline. Useful for debugging
// @lock:		lock protecting @pt_list and @value
// @pt_tree:		rbtree of active (unsignaled/errored) sync_pts
// @pt_list:		list of active (unsignaled/errored) sync_pts
// @sync_timeline_list:	membership in global sync_timeline_list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_timeline {
    pub kref: kref,
    pub name: [c_char; 32],
// protected by lock
    pub context: u64,
    pub value: c_int,
    pub pt_tree: rb_root,
    pub pt_list: list_head,
    pub lock: spinlock_t,
    pub sync_timeline_list: list_head,
}

extern "C" {
    pub fn container_of(_arg: fence->extern_lock, sync_timeline: struct, _arg: lock) -> return;
}
//
// struct sync_pt - sync_pt object
// @base: base fence object
// @link: link on the sync timeline's list
// @node: node in the sync timeline's tree
// @deadline: the earliest fence deadline hint
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sync_pt {
    pub base: dma_fence,
    pub link: list_head,
    pub node: rb_node,
    pub deadline: ktime_t,
}

extern "C" {
    pub fn sync_timeline_debug_add(obj: *mut sync_timeline);
}
extern "C" {
    pub fn sync_timeline_debug_remove(obj: *mut sync_timeline);
}
