//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/topology.h
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
pub struct cpu_topology_s390 {
    pub thread_id: c_ushort,
    pub core_id: c_ushort,
    pub socket_id: c_ushort,
    pub book_id: c_ushort,
    pub drawer_id: c_ushort,
    pub 1: unsigned short dedicated :,
    pub booted_cores: c_int,
    pub thread_mask: cpumask_t,
    pub core_mask: cpumask_t,
    pub book_mask: cpumask_t,
    pub drawer_mask: cpumask_t,
}

pub const mc_capable(): c_int = 1;
extern "C" {
    pub fn topology_init_early();
}
extern "C" {
    pub fn topology_cpu_init(: *mut cpu) -> c_int;
}
extern "C" {
    pub fn topology_set_cpu_management(fc: c_int) -> c_int;
}
extern "C" {
    pub fn topology_schedule_update();
}
extern "C" {
    pub fn store_topology(info: *mut sysinfo_15_1_x);
}
extern "C" {
    pub fn update_cpu_masks();
}
extern "C" {
    pub fn topology_expect_change();
}

// Returns a pointer to the cpumask of CPUs on node 'node'.

