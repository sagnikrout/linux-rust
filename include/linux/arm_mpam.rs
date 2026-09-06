//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/arm_mpam.h
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
// Copyright (C) 2025 Arm Ltd.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpam_msc_iface {
    MPAM_IFACE_MMIO,	/* a real MPAM MSC */
    MPAM_IFACE_PCC,		/* a fake MPAM MSC */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpam_class_types {
    MPAM_CLASS_CACHE,	/* Caches, e.g. L2, L3 */
    MPAM_CLASS_MEMORY,	/* Main memory */
    MPAM_CLASS_UNKNOWN,	/* Everything else, e.g. SMMU */
}

pub const MPAM_CLASS_ID_DEFAULT: c_int = 255;

extern "C" {
    pub fn acpi_mpam_count_msc() -> c_int;
}

extern "C" {
    pub fn resctrl_arch_alloc_capable() -> bool;
}
extern "C" {
    pub fn resctrl_arch_mon_capable() -> bool;
}
extern "C" {
    pub fn resctrl_arch_set_cpu_default_closid(cpu: c_int, closid: u32);
}
extern "C" {
    pub fn resctrl_arch_set_closid_rmid(tsk: *mut task_struct, closid: u32, rmid: u32);
}
extern "C" {
    pub fn resctrl_arch_set_cpu_default_closid_rmid(cpu: c_int, closid: u32, rmid: u32);
}
extern "C" {
    pub fn resctrl_arch_sched_in(tsk: *mut task_struct);
}
extern "C" {
    pub fn resctrl_arch_match_closid(tsk: *mut task_struct, closid: u32) -> bool;
}
extern "C" {
    pub fn resctrl_arch_match_rmid(tsk: *mut task_struct, closid: u32, rmid: u32) -> bool;
}
extern "C" {
    pub fn resctrl_arch_rmid_idx_encode(closid: u32, rmid: u32) -> u32;
}
extern "C" {
    pub fn resctrl_arch_rmid_idx_decode(idx: u32, closid: *mut u32, rmid: *mut u32);
}
extern "C" {
    pub fn resctrl_arch_system_num_rmid_idx() -> u32;
}
extern "C" {
    pub fn resctrl_arch_mon_ctx_free(r: *mut rdt_resource, evtid: resctrl_event_id, ctx: *mut c_void);
}
//
// The CPU configuration for MPAM is cheap to write, and is only written if it
// has changed. No need for fine grained enables.
//
// mpam_register_requestor() - Register a requestor with the MPAM driver
// @partid_max:		The maximum PARTID value the requestor can generate.
// @pmg_max:		The maximum PMG value the requestor can generate.
//
// Registers a requestor with the MPAM driver to ensure the chosen system-wide
// minimum PARTID and PMG values will allow the requestors features to be used.
//
// Returns an error if the registration is too late, and a larger PARTID/PMG
// value has been advertised to user-space. In this case the requestor should
// not use its MPAM features. Returns 0 on success.
//
extern "C" {
    pub fn mpam_register_requestor(partid_max: u16, pmg_max: u8) -> c_int;
}
