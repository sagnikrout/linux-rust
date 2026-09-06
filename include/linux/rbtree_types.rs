//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rbtree_types.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_node {
    pub __rb_parent_color: c_ulong,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
    pub __attribute__((aligned(sizeof(long)))): },
// The alignment might seem pointless, but allegedly CRIS needs it
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_node_linked {
    pub node: rb_node,
    pub prev: *mut rb_node_linked,
    pub next: *mut rb_node_linked,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

//
// Leftmost-cached rbtrees.
//
// We do not cache the rightmost node based on footprint
// size vs number of potential users that could benefit
// from O(1) rb_last(). Just not worth it, users that want
// this feature can always implement the logic explicitly.
// Furthermore, users that want to cache both pointers may
// find it a bit asymmetric, but that's ok.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_root_cached {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node,
}

//
// Leftmost tree with links. This would allow a trivial rb_rightmost update,
// but that has been omitted due to the lack of users.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_root_linked {
    pub rb_root: rb_root,
    pub rb_leftmost: *mut rb_node_linked,
}

