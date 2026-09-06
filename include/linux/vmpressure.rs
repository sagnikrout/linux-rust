//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vmpressure.h
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
pub struct vmpressure {
    pub scanned: c_ulong,
    pub reclaimed: c_ulong,
// The lock is used to keep the scanned/reclaimed in sync.
    pub sr_lock: spinlock_t,

//
// tree=true accumulators feed the v1 userspace eventfd interface
// (memory.pressure_level). Drained by @work. v2 has no equivalent
// interface, so this state is omitted on CONFIG_MEMCG_V1=n builds.
//
    pub tree_scanned: c_ulong,
    pub tree_reclaimed: c_ulong,
// The list of vmpressure_event structs.
    pub events: list_head,
// Have to grab the lock on events traversal or modifications.
    pub events_lock: mutex,
    pub work: work_struct,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmpressure_levels {
    VMPRESSURE_LOW = 0,
    VMPRESSURE_MEDIUM,
    VMPRESSURE_CRITICAL,
    VMPRESSURE_NUM_LEVELS,
}

extern "C" {
    pub fn vmpressure_init(vmpr: *mut vmpressure);
}
extern "C" {
    pub fn vmpressure_cleanup(vmpr: *mut vmpressure);
}
// Shared with the v1 vmpressure block in mm/memcontrol-v1.c.

extern "C" {
    pub fn vmpressure_prio(gfp: gfp_t, memcg: *mut mem_cgroup, prio: c_int);
}
// v1 hooks called from mm/vmpressure.c; no-ops below when !MEMCG_V1.
extern "C" {
    pub fn vmpressure_v1_init(vmpr: *mut vmpressure);
}
extern "C" {
    pub fn vmpressure_v1_cleanup(vmpr: *mut vmpressure);
}

