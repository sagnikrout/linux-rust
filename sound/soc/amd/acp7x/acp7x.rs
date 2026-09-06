//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/amd/acp7x/acp7x.h
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
// AMD Common ACP header file for ACP7.X variants(ACP7.D/7.E/7.F)
//
// Copyright (C) 2026 Advanced Micro Devices, Inc. All rights reserved.
//

pub const ACP_DEVICE_ID: c_uint = 0x15E2;
pub const ACP7X_REG_START: c_uint = 0x1240000;
pub const ACP7X_REG_END: c_uint = 0x125C000;
pub const ACP7D_PCI_REV: c_uint = 0x7D;
pub const ACP7E_PCI_REV: c_uint = 0x7E;
pub const ACP7F_PCI_REV: c_uint = 0x7F;
// Common register helper bits used by acp7x-common.c
pub const ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK: c_uint = 0x00010001;
pub const DELAY_US: c_int = 5;
pub const ACP7X_TIMEOUT: c_int = 5000;
pub const ACP7X_PGFSM_CNTL_POWER_ON_MASK: c_int = 7;
pub const ACP7X_PGFSM_STATUS_MASK: c_uint = 0x3F;
// time in ms for runtime suspend delay
pub const ACP_SUSPEND_DELAY_MS: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp_hw_ops {
    pub dev): *mut *mut *mut int (acp_init)(void __iomem acp_base, struct device,
    pub dev): *mut *mut *mut int (acp_deinit)(void __iomem acp_base, struct device,
    pub dev): *mut *mut int (acp_suspend)(struct device,
    pub dev): *mut *mut int (acp_resume)(struct device,
    pub dev): *mut *mut int (acp_suspend_runtime)(struct device,
    pub dev): *mut *mut int (acp_resume_runtime)(struct device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acp7x_dev_data {
    pub acp7x_base: *mut void __iomem,
    pub hw_ops: *mut acp_hw_ops,
    pub addr: u32,
    pub reg_range: u32,
    pub acp_rev: u32,
}

extern "C" {
    pub fn acp7x_hw_init_ops(hw_ops: *mut acp_hw_ops);
}
extern "C" {
    pub fn snd_amd_acp_find_config(pci: *mut pci_dev) -> c_int;
}
