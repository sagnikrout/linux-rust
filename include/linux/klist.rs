//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/klist.h
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
//
// klist.h - Some generic list helpers, extending struct list_head a bit.
//
// Implementations are found in lib/klist.c
//
// Copyright (C) 2005 Patrick Mochel
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct klist {
    pub k_lock: spinlock_t,
    pub k_list: list_head,
    pub ): *mut *mut void (get)(struct klist_node,
    pub ): *mut *mut void (put)(struct klist_node,
}

    pub )): *mut *mut void (put)(struct klist_node,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klist_node {
    pub /: *mut *mut *mut void n_klist; / never access directly,
    pub n_node: list_head,
    pub n_ref: kref,
}

extern "C" {
    pub fn klist_add_tail(n: *mut klist_node, k: *mut klist);
}
extern "C" {
    pub fn klist_add_head(n: *mut klist_node, k: *mut klist);
}
extern "C" {
    pub fn klist_add_behind(n: *mut klist_node, pos: *mut klist_node);
}
extern "C" {
    pub fn klist_add_before(n: *mut klist_node, pos: *mut klist_node);
}
extern "C" {
    pub fn klist_del(n: *mut klist_node);
}
extern "C" {
    pub fn klist_remove(n: *mut klist_node);
}
extern "C" {
    pub fn klist_node_attached(n: *mut klist_node) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct klist_iter {
    pub i_klist: *mut klist,
    pub i_cur: *mut klist_node,
}

extern "C" {
    pub fn klist_iter_init(k: *mut klist, i: *mut klist_iter);
}
extern "C" {
    pub fn klist_iter_exit(i: *mut klist_iter);
}
