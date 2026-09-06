//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cisco/enic/vnic_intr.h
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
// Copyright 2008-2010 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

pub const VNIC_INTR_TIMER_TYPE_ABS: c_int = 0;
pub const VNIC_INTR_TIMER_TYPE_QUIET: c_int = 1;
// Interrupt control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_intr_ctrl {
    pub /: *mut *mut u32 coalescing_timer; / 0x00,
    pub pad0: u32,
    pub /: *mut *mut u32 coalescing_value; / 0x08,
    pub pad1: u32,
    pub /: *mut *mut u32 coalescing_type; / 0x10,
    pub pad2: u32,
    pub /: *mut *mut u32 mask_on_assertion; / 0x18,
    pub pad3: u32,
    pub /: *mut *mut u32 mask; / 0x20,
    pub pad4: u32,
    pub /: *mut *mut u32 int_credits; / 0x28,
    pub pad5: u32,
    pub /: *mut *mut u32 int_credit_return; / 0x30,
    pub pad6: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_intr {
    pub index: c_uint,
    pub vdev: *mut vnic_dev,
    pub /: *mut *mut *mut vnic_intr_ctrl __iomem ctrl; / memory-mapped,
}

extern "C" {
    pub fn ioread32(_arg: &intr->ctrl->mask) -> return;
}
pub const VNIC_INTR_UNMASK_SHIFT: c_int = 16;
pub const VNIC_INTR_RESET_TIMER_SHIFT: c_int = 17;
extern "C" {
    pub fn ioread32(_arg: &intr->ctrl->int_credits) -> return;
}
// read PBA without clearing
extern "C" {
    pub fn ioread32(_arg: legacy_pba) -> return;
}
extern "C" {
    pub fn vnic_intr_free(intr: *mut vnic_intr);
}
extern "C" {
    pub fn vnic_intr_clean(intr: *mut vnic_intr);
}
