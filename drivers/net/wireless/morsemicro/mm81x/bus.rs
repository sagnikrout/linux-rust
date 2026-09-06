//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/morsemicro/mm81x/bus.h
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
// Copyright (c) 2017-2026 Morse Micro
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mm81x_bus_type {
    MM81X_BUS_TYPE_USB,
    MM81X_BUS_TYPE_SDIO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm81x_bus_ops {
    pub len): *mut *mut *mut *mut int (dm_read)(struct mm81x mors, u32 addr, u8 data, int,
    pub len): *const *const *const *const int (dm_write)(struct mm81x mors, u32 addr, u8 data, int,
    pub data): *mut *mut *mut int (reg32_read)(struct mm81x mors, u32 addr, u32,
    pub data): *mut *mut *mut int (reg32_write)(struct mm81x mors, u32 addr, u32,
    pub mors): *mut *mut int (digital_reset)(struct mm81x,
    pub enable): *mut *mut *mut void (set_bus_enable)(struct mm81x mors, bool,
    pub enable_burst): *mut *mut *mut void (config_burst_mode)(struct mm81x mors, bool,
    pub mors): *mut *mut void (claim)(struct mm81x,
    pub enable): *mut *mut *mut void (set_irq)(struct mm81x mors, bool,
    pub mors): *mut *mut void (release)(struct mm81x,
    pub bulk_alignment: c_uint,
}

//
// Default TX alignment for buses which don't care. mac80211 will give us
// SKBs aligned to the 2 byte boundary, so 2 is effectively a noop.
//

// mm81x_dm_read - len must be rounded up to the nearest 4-byte boundary
