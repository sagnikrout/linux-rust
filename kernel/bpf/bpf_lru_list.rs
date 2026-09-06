//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/bpf/bpf_lru_list.h
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
// Copyright (c) 2016 Facebook
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bpf_lru_list_type {
    BPF_LRU_LIST_T_ACTIVE,
    BPF_LRU_LIST_T_INACTIVE,
    BPF_LRU_LIST_T_FREE,
    BPF_LRU_LOCAL_LIST_T_FREE,
    BPF_LRU_LOCAL_LIST_T_PENDING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_lru_node {
//
// A node is in at most one list at a time. The free path on the
// per-CPU locallist uses an llist, so share storage via a union.
//
    pub list: list_head,
    pub llist: llist_node,
}

//
// Marks nodes whose *_push_free() lock acquire failed; reclaimed
// by flush/shrink which honor the flag instead of del_from_htab().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_lru_list {
    pub lists: [list_head; NR_BPF_LRU_LIST_T],
    pub counts: [c_uint; NR_BPF_LRU_LIST_COUNT],
// The next inactive list rotation starts from here
    pub next_inactive_rotation: *mut list_head,
    pub ____cacheline_aligned_in_smp: rqspinlock_t lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_lru_locallist {
    pub pending_list: list_head,
    pub free_llist: llist_head,
    pub next_steal: u16,
    pub lock: rqspinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_common_lru {
    pub lru_list: bpf_lru_list,
    pub local_list: *mut bpf_lru_locallist __percpu,
}

extern "C" {
    pub fn bool(arg: *mut *mut del_from_htab_func)(void, node: *mut bpf_lru_node) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_lru {
    pub common_lru: bpf_common_lru,
    pub percpu_lru: *mut bpf_lru_list __percpu,
}

extern "C" {
    pub fn bpf_lru_destroy(lru: *mut bpf_lru);
}
extern "C" {
    pub fn bpf_lru_push_free(lru: *mut bpf_lru, node: *mut bpf_lru_node);
}
