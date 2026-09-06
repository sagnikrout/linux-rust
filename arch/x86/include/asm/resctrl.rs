//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/resctrl.h
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
// This value can never be a valid CLOSID, and is used when mapping a
// (closid, rmid) pair to an index and back. On x86 only the RMID is
// needed. The index is a software defined value.
//

//
// struct resctrl_pqr_state - State cache for the PQR MSR
// @cur_rmid:		The cached Resource Monitoring ID
// @cur_closid:	The cached Class Of Service ID
// @default_rmid:	The user assigned Resource Monitoring ID
// @default_closid:	The user assigned cached Class Of Service ID
//
// The upper 32 bits of MSR_IA32_PQR_ASSOC contain closid and the
// lower 10 bits rmid. The update to MSR_IA32_PQR_ASSOC always
// contains both parts, so we need to cache them. This also
// stores the user configured per cpu CLOSID and RMID.
//
// The cache also helps to avoid pointless updates if the value does
// not change.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_pqr_state {
    pub cur_rmid: u32,
    pub cur_closid: u32,
    pub default_rmid: u32,
    pub default_closid: u32,
}

//
// __resctrl_sched_in() - Writes the task's CLOSid/RMID to IA32_PQR_MSR
//
// Following considerations are made so that this has minimal impact
// on scheduler hot path:
// - This will stay as no-op unless we are running on an Intel SKU
// which supports resource control or monitoring and we enable by
// mounting the resctrl file system.
// - Caches the per cpu CLOSid/RMID values and does the MSR write only
// when a task with a different CLOSid/RMID is scheduled in.
// - We allocate RMIDs/CLOSids globally in order to keep this as
// simple as possible.
// Must be called with preemption disabled.
//
// If this task has a closid/rmid assigned, use it.
// Else use the closid/rmid assigned to this cpu.
//
// h/w works in units of "boot_cpu_data.x86_cache_occ_scale"
// rmid = idx;
// closid = X86_RESCTRL_EMPTY_CLOSID;
// x86 can always read an rmid, nothing needs allocating
extern "C" {
    pub fn resctrl_cpu_detect(c: *mut cpuinfo_x86);
}

