//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/intlist.h
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
pub struct int_node {
    pub rb_node: rb_node,
    pub i: c_ulong,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intlist {
    pub rblist: rblist,
}

extern "C" {
    pub fn intlist__delete(ilist: *mut intlist);
}
extern "C" {
    pub fn intlist__remove(ilist: *mut intlist, in: *mut int_node);
}
extern "C" {
    pub fn intlist__add(ilist: *mut intlist, i: c_ulong) -> c_int;
}
extern "C" {
    pub fn rblist__empty(_arg: &ilist->rblist) -> return;
}
extern "C" {
    pub fn rblist__nr_entries(_arg: &ilist->rblist) -> return;
}
// For intlist iteration
//
// intlist__for_each_entry      - iterate over a intlist
// @pos:	the &struct int_node to use as a loop cursor.
// @ilist:	the &struct intlist for loop.
//

//
// intlist__for_each_entry_safe - iterate over a intlist safe against removal of
// int_node
// @pos:	the &struct int_node to use as a loop cursor.
// @n:		another &struct int_node to use as temporary storage.
// @ilist:	the &struct intlist for loop.
//

