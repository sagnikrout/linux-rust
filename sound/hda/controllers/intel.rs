//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/controllers/intel.h
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
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_intel {
    pub chip: azx,
// sync probing
    pub probe_wait: completion,
    pub probe_work: delayed_work,
// card list (for power_save trigger)
    pub list: list_head,
// extra flags
    pub irq_pending_warned:1: c_uint,
    pub probe_continued:1: c_uint,
    pub runtime_pm_disabled:1: c_uint,
// vga_switcheroo setup
    pub use_vga_switcheroo:1: c_uint,
    pub vga_switcheroo_registered:1: c_uint,
    pub /: *mut *mut unsigned int init_failed:1; / delayed init failed,
    pub /: *mut *mut unsigned int freed:1; / resources already released,
    pub /: *mut *mut bool need_i915_power:1; / the hda controller needs i915 power,
    pub /: *mut *mut int probe_retry; / being probe-retry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_intel_stream {
    pub azx_dev: azx_dev,
// for pending irqs
    pub hda: *mut hda_intel,
    pub irq_pending_work: work_struct,
    pub irq_pending: bool,
}

