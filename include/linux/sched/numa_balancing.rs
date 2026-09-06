//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/numa_balancing.h
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
// This is the interface between the scheduler and the MM that
// implements memory access pattern based NUMA-balancing:
//

pub const TNF_MIGRATED: c_uint = 0x01;
pub const TNF_NO_GROUP: c_uint = 0x02;
pub const TNF_SHARED: c_uint = 0x04;
pub const TNF_FAULT_LOCAL: c_uint = 0x08;
pub const TNF_MIGRATE_FAIL: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum numa_vmaskip_reason {
    NUMAB_SKIP_UNSUITABLE,
    NUMAB_SKIP_SHARED_RO,
    NUMAB_SKIP_INACCESSIBLE,
    NUMAB_SKIP_SCAN_DELAY,
    NUMAB_SKIP_PID_INACTIVE,
    NUMAB_SKIP_IGNORE_PID,
    NUMAB_SKIP_SEQ_COMPLETED,
}

extern "C" {
    pub fn task_numa_fault(last_node: c_int, node: c_int, pages: c_int, flags: c_int);
}
extern "C" {
    pub fn task_numa_group_id(p: *mut task_struct) -> pid_t;
}
extern "C" {
    pub fn set_numabalancing_state(enabled: bool);
}
extern "C" {
    pub fn task_numa_free(p: *mut task_struct, final: bool);
}

