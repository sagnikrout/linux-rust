//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/srcline.h
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

extern "C" {
    pub fn zfree_srcline(srcline: *mut c_char);
}
// insert the srcline into the DSO, which will take ownership
extern "C" {
    pub fn srcline__tree_insert(tree: *mut rb_root_cached, addr: u64, srcline: *mut c_char);
}
// find previously inserted srcline
// delete all srclines within the tree
extern "C" {
    pub fn srcline__tree_delete(tree: *mut rb_root_cached);
}

pub const MAX_INLINE_NEST: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inline_list {
    pub symbol: *mut symbol,
    pub srcline: *mut c_char,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inline_node {
    pub addr: u64,
    pub val: list_head,
    pub rb_node: rb_node,
}

// parse inlined frames for the given address
// free resources associated to the inline node list
extern "C" {
    pub fn inline_node__delete(node: *mut inline_node);
}
extern "C" {
    pub fn inline_node__clear_frames(node: *mut inline_node);
}
// insert the inline node list into the DSO, which will take ownership
// find previously inserted inline node list
// delete all nodes within the tree of inline_node s
extern "C" {
    pub fn inlines__tree_delete(tree: *mut rb_root_cached);
}
extern "C" {
    pub fn inline_list__append(symbol: *mut symbol, srcline: *mut c_char, node: *mut inline_node) -> c_int;
}
extern "C" {
    pub fn inline_list__append_tail(symbol: *mut symbol, srcline: *mut c_char, node: *mut inline_node) -> c_int;
}
extern "C" {
    pub fn addr2line_configure(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int;
}
