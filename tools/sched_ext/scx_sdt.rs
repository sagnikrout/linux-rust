//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/scx_sdt.h
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
// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2025 Tejun Heo <tj@kernel.org>
// Copyright (c) 2025 Emil Tsalapatis <etsal@meta.com>
//

// Macro flag: #define __arena

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_alloc_stats {
    pub chunk_allocs: __u64,
    pub data_allocs: __u64,
    pub alloc_ops: __u64,
    pub free_ops: __u64,
    pub active_allocs: __u64,
    pub arena_pages_used: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdt_pool {
    pub slab: *mut void __arena,
    pub elem_size: __u64,
    pub max_elems: __u64,
    pub idx: __u64,
}

pub type sdt_desc_t = sdt_desc __arena;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdt_consts {
    SDT_TASK_ENTS_PER_PAGE_SHIFT	= 9,
    SDT_TASK_LEVELS			= 3,
    SDT_TASK_ENTS_PER_CHUNK		= 1 << SDT_TASK_ENTS_PER_PAGE_SHIFT,
    SDT_TASK_CHUNK_BITMAP_U64S	= div_round_up(SDT_TASK_ENTS_PER_CHUNK, 64),
    SDT_TASK_MIN_ELEM_PER_ALLOC 	= 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sdt_id {
    pub val: __s64,
    pub /: *mut *mut __s32 idx; / index in the radix tree,
    pub /: *mut *mut __s32 genn; / ++'d on recycle so that it forms unique'ish 64bit ID,
}

//
// Each index page is described by the following descriptor which carries the
// bitmap. This way the actual index can host power-of-two numbers of entries
// which makes indexing cheaper.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdt_desc {
    pub allocated: [__u64; SDT_TASK_CHUNK_BITMAP_U64S],
    pub nr_free: __u64,
    pub chunk: *mut sdt_chunk __arena,
}

//
// Leaf node containing per-task data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdt_data {
    pub tid: sdt_id,
    pub payload: [__u64; ],
}

//
// Intermediate node pointing to another intermediate node or leaf node.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdt_chunk {
    pub descs: [*mut *mut sdt_desc_t; SDT_TASK_ENTS_PER_CHUNK],
    pub data: [*mut sdt_data __arena; SDT_TASK_ENTS_PER_CHUNK],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_allocator {
    pub pool: sdt_pool,
    pub root: *mut sdt_desc_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_stats {
    pub seq: c_int,
    pub pid: pid_t,
    pub enqueue: __u64,
    pub exit: __u64,
    pub init: __u64,
    pub select_busy_cpu: __u64,
    pub select_idle_cpu: __u64,
}

extern "C" {
    pub fn scx_task_init(data_size: __u64) -> c_int;
}
extern "C" {
    pub fn scx_task_free(p: *mut task_struct);
}
extern "C" {
    pub fn scx_arena_subprog_init();
}
extern "C" {
    pub fn scx_alloc_init(alloc: *mut scx_allocator, data_size: __u64) -> c_int;
}
extern "C" {
    pub fn scx_alloc_internal(alloc: *mut scx_allocator) -> u64;
}
extern "C" {
    pub fn scx_alloc_free_idx(alloc: *mut scx_allocator, idx: __u64) -> c_int;
}
