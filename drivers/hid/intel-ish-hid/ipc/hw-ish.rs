//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-ish-hid/ipc/hw-ish.h
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
// H/W layer of ISHTP provider device (ISH)
//
// Copyright (c) 2014-2016, Intel Corporation.
//

pub const PCI_DEVICE_ID_INTEL_ISH_CHV: c_uint = 0x22D8;
pub const PCI_DEVICE_ID_INTEL_ISH_BXT_Ax: c_uint = 0x0AA2;
pub const PCI_DEVICE_ID_INTEL_ISH_BXT_Bx: c_uint = 0x1AA2;
pub const PCI_DEVICE_ID_INTEL_ISH_APL_Ax: c_uint = 0x5AA2;
pub const PCI_DEVICE_ID_INTEL_ISH_SPT_Ax: c_uint = 0x9D35;
pub const PCI_DEVICE_ID_INTEL_ISH_CNL_Ax: c_uint = 0x9DFC;
pub const PCI_DEVICE_ID_INTEL_ISH_GLK_Ax: c_uint = 0x31A2;
pub const PCI_DEVICE_ID_INTEL_ISH_CNL_H: c_uint = 0xA37C;
pub const PCI_DEVICE_ID_INTEL_ISH_ICL_MOBILE: c_uint = 0x34FC;
pub const PCI_DEVICE_ID_INTEL_ISH_SPT_H: c_uint = 0xA135;
pub const PCI_DEVICE_ID_INTEL_ISH_CML_LP: c_uint = 0x02FC;
pub const PCI_DEVICE_ID_INTEL_ISH_CMP_H: c_uint = 0x06FC;
pub const PCI_DEVICE_ID_INTEL_ISH_EHL_Ax: c_uint = 0x4BB3;
pub const PCI_DEVICE_ID_INTEL_ISH_TGL_LP: c_uint = 0xA0FC;
pub const PCI_DEVICE_ID_INTEL_ISH_TGL_H: c_uint = 0x43FC;
pub const PCI_DEVICE_ID_INTEL_ISH_ADL_S: c_uint = 0x7AF8;
pub const PCI_DEVICE_ID_INTEL_ISH_ADL_P: c_uint = 0x51FC;
pub const PCI_DEVICE_ID_INTEL_ISH_ADL_N: c_uint = 0x54FC;
pub const PCI_DEVICE_ID_INTEL_ISH_RPL_S: c_uint = 0x7A78;
pub const PCI_DEVICE_ID_INTEL_ISH_MTL_P: c_uint = 0x7E45;
pub const PCI_DEVICE_ID_INTEL_ISH_ARL_H: c_uint = 0x7745;
pub const PCI_DEVICE_ID_INTEL_ISH_ARL_S: c_uint = 0x7F78;
pub const PCI_DEVICE_ID_INTEL_ISH_LNL_M: c_uint = 0xA845;
pub const PCI_DEVICE_ID_INTEL_ISH_PTL_H: c_uint = 0xE345;
pub const PCI_DEVICE_ID_INTEL_ISH_PTL_P: c_uint = 0xE445;
pub const PCI_DEVICE_ID_INTEL_ISH_WCL: c_uint = 0x4D45;
pub const PCI_DEVICE_ID_INTEL_ISH_NVL_H: c_uint = 0xD354;
pub const PCI_DEVICE_ID_INTEL_ISH_NVL_S: c_uint = 0x6E78;
pub const REVISION_ID_CHT_A0: c_uint = 0x6;
pub const REVISION_ID_CHT_Ax_SI: c_uint = 0x0;
pub const REVISION_ID_CHT_Bx_SI: c_uint = 0x10;
pub const REVISION_ID_CHT_Kx_SI: c_uint = 0x20;
pub const REVISION_ID_CHT_Dx_SI: c_uint = 0x30;
pub const REVISION_ID_CHT_B0: c_uint = 0xB0;
pub const REVISION_ID_SI_MASK: c_uint = 0x70;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_rst_payload_type {
    pub reset_id: u16,
    pub reserved: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_sync_format {
    pub ts1_source: u8,
    pub ts2_source: u8,
    pub reserved: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_time_update_msg {
    pub primary_host_time: u64,
    pub sync_info: time_sync_format,
    pub secondary_host_time: u64,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ish_hw {
    pub mem_addr: *mut void __iomem,
}

//
// ISH FW status type
//

extern "C" {
    pub fn ish_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ish_hw_start(dev: *mut ishtp_device) -> c_int;
}
extern "C" {
    pub fn ish_device_disable(dev: *mut ishtp_device);
}
extern "C" {
    pub fn ish_disable_dma(dev: *mut ishtp_device) -> c_int;
}
extern "C" {
    pub fn ish_set_host_ready(dev: *mut ishtp_device);
}
