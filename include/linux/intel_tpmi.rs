//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/intel_tpmi.h
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
// intel_tpmi.h: Intel TPMI core external interface
//

pub const TPMI_VERSION_INVALID: c_uint = 0xff;

//
// List of supported TMPI IDs.
// Some TMPI IDs are not used by Linux, so the numbers are not consecutive.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_tpmi_id {
    TPMI_ID_RAPL = 0,	/* Running Average Power Limit */
    TPMI_ID_PEM = 1,	/* Power and Perf excursion Monitor */
    TPMI_ID_UNCORE = 2,	/* Uncore Frequency Scaling */
    TPMI_ID_SST = 5,	/* Speed Select Technology */
    TPMI_ID_PLR = 0xc,	/* Performance Limit Reasons */
    TPMI_CONTROL_ID = 0x80,	/* Special ID for getting feature status */
    TPMI_INFO_ID = 0x81,	/* Special ID for PCI BDF and Package ID information */
}

pub const TPMI_CORE_INIT: c_int = 0;
pub const TPMI_CORE_EXIT: c_int = 1;
extern "C" {
    pub fn tpmi_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn tpmi_unregister_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn tpmi_get_resource_count(auxdev: *mut auxiliary_device) -> c_int;
}
