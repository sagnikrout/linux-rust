//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/bpf_skel/lock_data.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Data structures shared between BPF and tools.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct owner_tracing_data {
    pub lock.: u32 pid; // Who has the,
    pub lock.: u32 count; // How many waiters for this,
    pub on.: u64 timestamp; // The time while the owner acquires lock and contention is going,
    pub `owner_stacks`: s32 stack_id; // Identifier for `owner_stat`, which stores as value in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstamp_data {
    pub timestamp: u64,
    pub lock: u64,
    pub cgroup_id: u64,
    pub flags: u32,
    pub stack_id: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct contention_key {
    pub stack_id: i32,
    pub pid: u32,
    pub lock_addr_or_cgroup: u64,
}

pub const TASK_COMM_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct contention_task_data {
    pub comm: [c_char; TASK_COMM_LEN],
}

// default buffer size
pub const MAX_ENTRIES: c_int = 16384;
//
// Upper bits of the flags in the contention_data are used to identify
// some well-known locks which do not have symbols (non-global locks).
//

pub const LCB_F_SLAB_ID_SHIFT: c_int = 16;

pub const LCB_F_SLAB_ID_MASK: c_uint = 0x03FF0000U;

pub const LCB_F_TYPE_MASK: c_uint = 0x0000007FU;
pub const SLAB_NAME_MAX: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct contention_data {
    pub total_time: u64,
    pub min_time: u64,
    pub max_time: u64,
    pub count: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lock_aggr_mode {
    LOCK_AGGR_ADDR = 0,
    LOCK_AGGR_TASK,
    LOCK_AGGR_CALLER,
    LOCK_AGGR_CGROUP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lock_class_sym {
    LOCK_CLASS_NONE,
    LOCK_CLASS_RQLOCK,
    LOCK_CLASS_ZONE_LOCK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slab_cache_data {
    pub id: u32,
    pub name: [c_char; SLAB_NAME_MAX],
}
