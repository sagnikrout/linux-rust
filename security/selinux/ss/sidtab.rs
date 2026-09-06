//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/sidtab.h
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
// A security identifier table (sidtab) is a lookup table
// of security context structures indexed by SID value.
//
// Original author: Stephen Smalley, <stephen.smalley.work@gmail.com>
// Author: Ondrej Mosnacek, <omosnacek@gmail.com>
//
// Copyright (C) 2018 Red Hat, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sidtab_entry {
    pub sid: u32,
    pub hash: u32,
    pub context: context,

    pub cache: *mut sidtab_str_cache __rcu,

    pub list: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sidtab_entry_inner {
    pub ptr_inner: *mut sidtab_node_inner,
    pub ptr_leaf: *mut sidtab_node_leaf,
}

// align node size to page boundary

pub const SIDTAB_MAX_BITS: c_int = 32;

// ensure enough tree levels for SIDTAB_MAX entries

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sidtab_node_leaf {
    pub entries: [sidtab_entry; SIDTAB_LEAF_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sidtab_node_inner {
    pub entries: [sidtab_entry_inner; SIDTAB_INNER_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sidtab_isid_entry {
    pub set: c_int,
    pub entry: sidtab_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sidtab_convert_params {
    pub args: *mut convert_context_args,
    pub target: *mut sidtab,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sidtab {
//
// lock-free read access only for as many items as a prior read of
// 'count'
//
    pub 1]: sidtab_entry_inner roots[SIDTAB_MAX_LEVEL +,
//
// access atomically via {READ|WRITE}_ONCE(); only increment under
// spinlock
//
    pub count: u32,
// access only under spinlock
    pub convert: *mut sidtab_convert_params,
    pub frozen: bool,
    pub lock: spinlock_t,

// SID -> context string cache
    pub cache_free_slots: u32,
    pub cache_lru_list: list_head,
    pub cache_lock: spinlock_t,

// index == SID - 1 (no entry for SECSID_NULL)
    pub isids: [sidtab_isid_entry; SECINITSID_NUM],
// Hash table for fast reverse context-to-sid lookups.
    pub SIDTAB_HASH_BITS): DECLARE_HASHTABLE(context_to_sid,,
}

extern "C" {
    pub fn sidtab_init(s: *mut sidtab) -> c_int;
}
extern "C" {
    pub fn sidtab_set_initial(s: *mut sidtab, sid: u32, context: *mut context) -> c_int;
}
extern "C" {
    pub fn sidtab_convert(s: *mut sidtab, params: *mut sidtab_convert_params) -> c_int;
}
extern "C" {
    pub fn sidtab_cancel_convert(s: *mut sidtab);
}
extern "C" {
    pub fn sidtab_context_to_sid(s: *mut sidtab, context: *mut context, sid: *mut u32) -> c_int;
}
extern "C" {
    pub fn sidtab_destroy(s: *mut sidtab);
}
extern "C" {
    pub fn sidtab_hash_stats(sidtab: *mut sidtab, page: *mut c_char) -> c_int;
}

