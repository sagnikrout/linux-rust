//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/oom.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum oom_constraint {
    CONSTRAINT_NONE,
    CONSTRAINT_CPUSET,
    CONSTRAINT_MEMORY_POLICY,
    CONSTRAINT_MEMCG,
}

//
// Details of the page allocation that triggered the oom killer that are used to
// determine what should be killed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oom_control {
// Used to determine cpuset
    pub zonelist: *mut zonelist,
// Used to determine mempolicy
    pub nodemask: *const nodemask_t,
// Memory cgroup in which oom is invoked, or NULL for global oom
    pub memcg: *mut mem_cgroup,
// Used to determine cpuset and node locality requirement
    pub gfp_mask: gfp_t,
//
// order == -1 means the oom kill is required by sysrq, otherwise only
// for display purposes.
//
    pub order: c_int,
// Used by oom implementation, do not set
    pub totalpages: c_ulong,
    pub chosen: *mut task_struct,
    pub chosen_points: c_long,
// Used to print the constraint info.
    pub constraint: oom_constraint,
}

//
// Checks whether a page fault on the given mm is still reliable.
// This is no longer true if the oom reaper started to reap the
// address space which is reflected by MMF_UNSTABLE flag set in
// the mm. At that moment any !shared mapping would lose the content
// and could cause a memory corruption (zero pages instead of the
// original content).
//
// User should call this before establishing a page table entry for
// a !shared mapping and under the proper page table lock.
//
// Return 0 when the PF is safe VM_FAULT_SIGBUS otherwise.
//
extern "C" {
    pub fn out_of_memory(oc: *mut oom_control) -> bool;
}
extern "C" {
    pub fn exit_oom_victim();
}
extern "C" {
    pub fn register_oom_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_oom_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn oom_killer_disable(timeout: signed long) -> bool;
}
extern "C" {
    pub fn oom_killer_enable();
}
