//! Automatically rewritten from C Header to Rust Module
//! Source: mm/memcontrol-v1.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

// Cgroup v1 and v2 common declarations
//
// Iteration constructs for visiting all cgroups (under a tree).  If
// loops are exited prematurely (break), mem_cgroup_iter_break() must
// be used for reference counting.
//

extern "C" {
    pub fn drain_all_stock(root_memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn memory_stat_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}
// Cgroup v1-specific declarations

// Whether legacy memory+swap accounting is active
extern "C" {
    pub fn memcg_events_local(memcg: *mut mem_cgroup, event: c_int) -> c_ulong;
}
extern "C" {
    pub fn memcg_page_state_local(memcg: *mut mem_cgroup, idx: c_int) -> c_ulong;
}
extern "C" {
    pub fn memcg_page_state_local_output(memcg: *mut mem_cgroup, item: c_int) -> c_ulong;
}
extern "C" {
    pub fn memcg1_alloc_events(memcg: *mut mem_cgroup) -> bool;
}
extern "C" {
    pub fn memcg1_free_events(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn memcg1_memcg_init(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn memcg1_remove_from_trees(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn memcg1_css_offline(memcg: *mut mem_cgroup);
}
// for encoding cft->private value on file
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum res_type {
    _MEM,
    _MEMSWAP,
    _KMEM,
    _TCP,
}

extern "C" {
    pub fn memcg1_oom_prepare(memcg: *mut mem_cgroup, locked: *mut bool) -> bool;
}
extern "C" {
    pub fn memcg1_oom_finish(memcg: *mut mem_cgroup, locked: bool);
}
extern "C" {
    pub fn memcg1_oom_recover(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn memcg1_commit_charge(folio: *mut folio, memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn memcg1_stat_format(memcg: *mut mem_cgroup, s: *mut seq_buf);
}
extern "C" {
    pub fn reparent_memcg1_state_local(memcg: *mut mem_cgroup, parent: *mut mem_cgroup);
}
extern "C" {
    pub fn reparent_memcg1_lruvec_state_local(memcg: *mut mem_cgroup, parent: *mut mem_cgroup);
}
extern "C" {
    pub fn memcg1_account_kmem(memcg: *mut mem_cgroup, nr_pages: c_int);
}

// locked = false;

