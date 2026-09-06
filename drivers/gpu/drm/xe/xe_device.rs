//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_device.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2021 Intel Corporation
//

extern "C" {
    pub fn container_of(_arg: dev, xe_device: struct, _arg: drm) -> return;
}
extern "C" {
    pub fn container_of(_arg: ttm, xe_device: struct, _arg: ttm) -> return;
}
extern "C" {
    pub fn xe_device_init_early(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_device_probe_early(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_device_probe(xe: *mut xe_device) -> c_int;
}
extern "C" {
    pub fn xe_device_remove(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_device_shutdown(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_device_wmb(xe: *mut xe_device);
}
//
// Provide a GT structure suitable for performing non-GT MMIO operations against
// the primary tile.  Primarily intended for early tile initialization, display
// handling, top-most interrupt enable/disable, etc.  Since anything using the
// MMIO handle returned by this function doesn't need GSI offset translation,
// we'll return the primary GT from the root tile.
//
// FIXME: Fix the driver design so that 'gt' isn't the target of all MMIO
// operations.
//
// Returns the primary gt of the root tile.
//

extern "C" {
    pub fn xe_device_assert_mem_access(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_device_has_memirq(xe_device_has_msix(xe): xe) && (IS_SRIOV_VF(xe) ||) -> return;
}
extern "C" {
    pub fn IS_DGFX(_arg: xe) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &xe->in_reset) -> return;
}
extern "C" {
    pub fn xe_device_ccs_bytes(xe: *mut xe_device, size: u64) -> u32;
}
extern "C" {
    pub fn xe_device_snapshot_print(xe: *mut xe_device, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_device_canonicalize_addr(xe: *mut xe_device, address: u64) -> u64;
}
extern "C" {
    pub fn xe_device_uncanonicalize_addr(xe: *mut xe_device, address: u64) -> u64;
}
extern "C" {
    pub fn xe_device_is_l2_flush_optimized(xe: *mut xe_device) -> bool;
}
extern "C" {
    pub fn xe_device_td_flush(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_device_l2_flush(xe: *mut xe_device);
}
extern "C" {
    pub fn atomic_read(_arg: &xe->wedged.flag) -> return;
}

extern "C" {
    pub fn IS_DGFX(_arg: xe) -> return;
}

extern "C" {
    pub fn xe_device_set_wedged_method(xe: *mut xe_device, method: c_ulong);
}
extern "C" {
    pub fn xe_device_declare_wedged(xe: *mut xe_device);
}
extern "C" {
    pub fn xe_device_validate_wedged_mode(xe: *mut xe_device, mode: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_file_put(xef: *mut xe_file);
}
extern "C" {
    pub fn xe_is_injection_active() -> c_int;
}
extern "C" {
    pub fn xe_is_xe_file(file: *const file) -> bool;
}

extern "C" {
    pub fn xe_device_is_admin_only(xe: *const xe_device) -> bool;
}

//
// Occasionally it is seen that the G2H worker starts running after a delay of more than
// a second even after being queued and activated by the Linux workqueue subsystem. This
// leads to G2H timeout error. The root cause of issue lies with scheduling latency of
// Lunarlake Hybrid CPU. Issue disappears if we disable Lunarlake atom cores from BIOS
// and this is beyond xe kmd.
//
// TODO: Drop this change once workqueue scheduling delay issue is fixed on LNL Hybrid CPU.
//

