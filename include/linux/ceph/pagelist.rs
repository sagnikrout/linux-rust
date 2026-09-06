//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/pagelist.h
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
pub struct ceph_pagelist {
    pub head: list_head,
    pub mapped_tail: *mut c_void,
    pub length: usize,
    pub room: usize,
    pub free_list: list_head,
    pub num_pages_free: usize,
    pub refcnt: refcount_t,
}

extern "C" {
    pub fn ceph_pagelist_release(pl: *mut ceph_pagelist);
}
extern "C" {
    pub fn ceph_pagelist_append(pl: *mut ceph_pagelist, d: *const c_void, l: usize) -> c_int;
}
extern "C" {
    pub fn ceph_pagelist_reserve(pl: *mut ceph_pagelist, space: usize) -> c_int;
}
extern "C" {
    pub fn ceph_pagelist_free_reserve(pl: *mut ceph_pagelist) -> c_int;
}
extern "C" {
    pub fn ceph_pagelist_append(_arg: pl, _arg: &ev, _arg: sizeof(ev)) -> return;
}
extern "C" {
    pub fn ceph_pagelist_append(_arg: pl, _arg: &ev, _arg: sizeof(ev)) -> return;
}
extern "C" {
    pub fn ceph_pagelist_append(_arg: pl, _arg: &ev, _arg: sizeof(ev)) -> return;
}
extern "C" {
    pub fn ceph_pagelist_append(_arg: pl, _arg: &v, _arg: 1) -> return;
}
extern "C" {
    pub fn ceph_pagelist_append(_arg: pl, _arg: s, _arg: len) -> return;
}
