//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/strlist.h
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
pub struct str_node {
    pub rb_node: rb_node,
    pub s: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct strlist {
    pub rblist: rblist,
    pub file_only: bool,
}

//
// @file_only: When dirname is present, only consider entries as filenames,
// that should not be added to the list if dirname/entry is not
// found
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct strlist_config {
    pub file_only: bool,
    pub dirname: *const c_char,
}

extern "C" {
    pub fn strlist__delete(slist: *mut strlist);
}
extern "C" {
    pub fn strlist__remove(slist: *mut strlist, sn: *mut str_node);
}
extern "C" {
    pub fn strlist__load(slist: *mut strlist, filename: *const c_char) -> c_int;
}
extern "C" {
    pub fn strlist__add(slist: *mut strlist, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn rblist__empty(_arg: &slist->rblist) -> return;
}
extern "C" {
    pub fn rblist__nr_entries(_arg: &slist->rblist) -> return;
}
// For strlist iteration
//
// strlist_for_each      - iterate over a strlist
// @pos:	the &struct str_node to use as a loop cursor.
// @slist:	the &struct strlist for loop.
//

//
// strlist_for_each_safe - iterate over a strlist safe against removal of
// str_node
// @pos:	the &struct str_node to use as a loop cursor.
// @n:		another &struct str_node to use as temporary storage.
// @slist:	the &struct strlist for loop.
//

