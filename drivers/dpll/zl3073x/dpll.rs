//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/zl3073x/dpll.h
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


// SPDX-License-Identifier: GPL-2.0-only

//
// struct zl3073x_dpll - ZL3073x DPLL sub-device structure
// @list: this DPLL list entry
// @dev: pointer to multi-function parent device
// @id: DPLL index
// @check_count: periodic check counter
// @phase_monitor: is phase offset monitor enabled
// @ops: DPLL device operations for this instance
// @dpll_dev: pointer to registered DPLL device
// @tracker: tracking object for the acquired reference
// @lock: per-DPLL mutex serializing all operations
// @type: DPLL type (PPS or EEC)
// @lock_status: last saved DPLL lock status
// @pins: list of pins
// @ptp_info: PTP clock info
// @ptp_clock: registered PTP clock (or NULL)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_dpll {
    pub list: list_head,
    pub dev: *mut zl3073x_dev,
    pub id: u8,
    pub check_count: u8,
    pub phase_monitor: bool,
    pub ops: dpll_device_ops,
    pub dpll_dev: *mut dpll_device,
    pub tracker: dpll_tracker,
    pub lock: mutex,
    pub type: dpll_type,
    pub lock_status: dpll_lock_status,
    pub pins: list_head,
    pub ptp_info: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
}

extern "C" {
    pub fn zl3073x_dpll_free(zldpll: *mut zl3073x_dpll);
}
extern "C" {
    pub fn zl3073x_dpll_register(zldpll: *mut zl3073x_dpll) -> c_int;
}
extern "C" {
    pub fn zl3073x_dpll_unregister(zldpll: *mut zl3073x_dpll);
}
extern "C" {
    pub fn zl3073x_dpll_init_fine_phase_adjust(zldev: *mut zl3073x_dev) -> c_int;
}
extern "C" {
    pub fn zl3073x_dpll_changes_check(zldpll: *mut zl3073x_dpll);
}
