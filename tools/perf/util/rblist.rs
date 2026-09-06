//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/rblist.h
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
// create node structs of the form:
// struct my_node {
// struct rb_node rb_node;
// ... my data ...
// };
//
// create list structs of the form:
// struct mylist {
// struct rblist rblist;
// ... my data ...
// };
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rblist {
    pub entries: rb_root_cached,
    pub nr_entries: c_uint,
    pub entry): *const *const *const int (node_cmp)(struct rb_node rbn, void,
    pub new_entry): *const *const *const *const rb_node (node_new)(rblist rlist, void,
    pub rb_node): *mut *mut *mut void (node_delete)(struct rblist rblist, struct rb_node,
}

extern "C" {
    pub fn rblist__init(rblist: *mut rblist);
}
extern "C" {
    pub fn rblist__exit(rblist: *mut rblist);
}
extern "C" {
    pub fn rblist__delete(rblist: *mut rblist);
}
extern "C" {
    pub fn rblist__add_node(rblist: *mut rblist, new_entry: *const c_void) -> c_int;
}
extern "C" {
    pub fn rblist__remove_node(rblist: *mut rblist, rb_node: *mut rb_node);
}
