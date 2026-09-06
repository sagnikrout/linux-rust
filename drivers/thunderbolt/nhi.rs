//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thunderbolt/nhi.h
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
// Thunderbolt driver - NHI driver
//
// Copyright (c) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2018, Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nhi_fw_mode {
    NHI_FW_SAFE_MODE,
    NHI_FW_AUTH_MODE,
    NHI_FW_EP_MODE,
    NHI_FW_CM_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nhi_mailbox_cmd {
    NHI_MAILBOX_SAVE_DEVS = 0x05,
    NHI_MAILBOX_DISCONNECT_PCIE_PATHS = 0x06,
    NHI_MAILBOX_DRV_UNLOADS = 0x07,
    NHI_MAILBOX_DISCONNECT_PA = 0x10,
    NHI_MAILBOX_DISCONNECT_PB = 0x11,
    NHI_MAILBOX_ALLOW_ALL_DEVS = 0x23,
}

extern "C" {
    pub fn nhi_mailbox_cmd(nhi: *mut tb_nhi, cmd: nhi_mailbox_cmd, data: u32) -> c_int;
}
extern "C" {
    pub fn nhi_mailbox_mode(nhi: *mut tb_nhi) -> nhi_fw_mode;
}
extern "C" {
    pub fn nhi_enable_int_throttling(nhi: *mut tb_nhi);
}
extern "C" {
    pub fn nhi_disable_interrupts(nhi: *mut tb_nhi);
}
extern "C" {
    pub fn nhi_interrupt_work(work: *mut work_struct);
}
extern "C" {
    pub fn nhi_msi(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ring_msix(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn nhi_probe(nhi: *mut tb_nhi) -> c_int;
}
extern "C" {
    pub fn nhi_shutdown(nhi: *mut tb_nhi);
}
extern "C" {
    pub fn nhi_reset_interface(nhi: *mut tb_nhi);
}
//
// struct tb_nhi_ops - NHI specific optional operations
// @init: NHI specific initialization
// @suspend_noirq: NHI specific suspend_noirq hook
// @resume_noirq: NHI specific resume_noirq hook
// @runtime_suspend: NHI specific runtime_suspend hook
// @runtime_resume: NHI specific runtime_resume hook
// @shutdown: NHI specific shutdown
// @pre_nvm_auth: hook to run before Thunderbolt 3 NVM authentication
// @post_nvm_auth: hook to run after Thunderbolt 3 NVM authentication
// @request_ring_irq: NHI specific interrupt retrieval hook
// @release_ring_irq: NHI specific interrupt release hook
// @is_present: Whether the device is currently present on the parent bus
// @init_interrupts: NHI specific interrupt initialization hook
// @reset_interface: Resets the host interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_nhi_ops {
    pub nhi): *mut *mut int (init)(struct tb_nhi,
    pub wakeup): *mut *mut *mut int (suspend_noirq)(struct tb_nhi nhi, bool,
    pub nhi): *mut *mut int (resume_noirq)(struct tb_nhi,
    pub nhi): *mut *mut int (runtime_suspend)(struct tb_nhi,
    pub nhi): *mut *mut int (runtime_resume)(struct tb_nhi,
    pub nhi): *mut *mut void (shutdown)(struct tb_nhi,
    pub nhi): *mut *mut void (pre_nvm_auth)(struct tb_nhi,
    pub nhi): *mut *mut void (post_nvm_auth)(struct tb_nhi,
    pub no_suspend): *mut *mut *mut int (request_ring_irq)(struct tb_ring ring, bool,
    pub ring): *mut *mut void (release_ring_irq)(struct tb_ring,
    pub nhi): *mut *mut bool (is_present)(struct tb_nhi,
    pub nhi): *mut *mut int (init_interrupts)(struct tb_nhi,
    pub nhi): *mut *mut void (reset_interface)(struct tb_nhi,
}

//
// PCI IDs used in this driver from Win Ridge forward. There is no
// need for the PCI quirk anymore as we will use ICM also on Apple
// hardware.
//
pub const PCI_DEVICE_ID_INTEL_MAPLE_RIDGE_2C_NHI: c_uint = 0x1134;
pub const PCI_DEVICE_ID_INTEL_MAPLE_RIDGE_4C_NHI: c_uint = 0x1137;
pub const PCI_DEVICE_ID_INTEL_WIN_RIDGE_2C_NHI: c_uint = 0x157d;
pub const PCI_DEVICE_ID_INTEL_WIN_RIDGE_2C_BRIDGE: c_uint = 0x157e;
pub const PCI_DEVICE_ID_INTEL_ALPINE_RIDGE_LP_NHI: c_uint = 0x15bf;
pub const PCI_DEVICE_ID_INTEL_ALPINE_RIDGE_LP_BRIDGE: c_uint = 0x15c0;
pub const PCI_DEVICE_ID_INTEL_ALPINE_RIDGE_C_4C_NHI: c_uint = 0x15d2;
pub const PCI_DEVICE_ID_INTEL_ALPINE_RIDGE_C_4C_BRIDGE: c_uint = 0x15d3;
pub const PCI_DEVICE_ID_INTEL_ALPINE_RIDGE_C_2C_NHI: c_uint = 0x15d9;
pub const PCI_DEVICE_ID_INTEL_ALPINE_RIDGE_C_2C_BRIDGE: c_uint = 0x15da;
pub const PCI_DEVICE_ID_INTEL_ALPINE_RIDGE_LP_USBONLY_NHI: c_uint = 0x15dc;
pub const PCI_DEVICE_ID_INTEL_ALPINE_RIDGE_USBONLY_NHI: c_uint = 0x15dd;
pub const PCI_DEVICE_ID_INTEL_ALPINE_RIDGE_C_USBONLY_NHI: c_uint = 0x15de;
pub const PCI_DEVICE_ID_INTEL_TITAN_RIDGE_2C_BRIDGE: c_uint = 0x15e7;
pub const PCI_DEVICE_ID_INTEL_TITAN_RIDGE_2C_NHI: c_uint = 0x15e8;
pub const PCI_DEVICE_ID_INTEL_TITAN_RIDGE_4C_BRIDGE: c_uint = 0x15ea;
pub const PCI_DEVICE_ID_INTEL_TITAN_RIDGE_4C_NHI: c_uint = 0x15eb;
pub const PCI_DEVICE_ID_INTEL_TITAN_RIDGE_DD_BRIDGE: c_uint = 0x15ef;
pub const PCI_DEVICE_ID_INTEL_ADL_NHI0: c_uint = 0x463e;
pub const PCI_DEVICE_ID_INTEL_ADL_NHI1: c_uint = 0x466d;
pub const PCI_DEVICE_ID_INTEL_WCL_NHI0: c_uint = 0x4d33;
pub const PCI_DEVICE_ID_INTEL_BARLOW_RIDGE_HOST_80G_NHI: c_uint = 0x5781;
pub const PCI_DEVICE_ID_INTEL_BARLOW_RIDGE_HOST_40G_NHI: c_uint = 0x5784;
pub const PCI_DEVICE_ID_INTEL_BARLOW_RIDGE_HUB_80G_BRIDGE: c_uint = 0x5786;
pub const PCI_DEVICE_ID_INTEL_BARLOW_RIDGE_HUB_40G_BRIDGE: c_uint = 0x57a4;
pub const PCI_DEVICE_ID_INTEL_MTL_M_NHI0: c_uint = 0x7eb2;
pub const PCI_DEVICE_ID_INTEL_MTL_P_NHI0: c_uint = 0x7ec2;
pub const PCI_DEVICE_ID_INTEL_MTL_P_NHI1: c_uint = 0x7ec3;
pub const PCI_DEVICE_ID_INTEL_ICL_NHI1: c_uint = 0x8a0d;
pub const PCI_DEVICE_ID_INTEL_ICL_NHI0: c_uint = 0x8a17;
pub const PCI_DEVICE_ID_INTEL_TGL_NHI0: c_uint = 0x9a1b;
pub const PCI_DEVICE_ID_INTEL_TGL_NHI1: c_uint = 0x9a1d;
pub const PCI_DEVICE_ID_INTEL_TGL_H_NHI0: c_uint = 0x9a1f;
pub const PCI_DEVICE_ID_INTEL_TGL_H_NHI1: c_uint = 0x9a21;
pub const PCI_DEVICE_ID_INTEL_RPL_NHI0: c_uint = 0xa73e;
pub const PCI_DEVICE_ID_INTEL_RPL_NHI1: c_uint = 0xa76d;
pub const PCI_DEVICE_ID_INTEL_LNL_NHI0: c_uint = 0xa833;
pub const PCI_DEVICE_ID_INTEL_LNL_NHI1: c_uint = 0xa834;
pub const PCI_DEVICE_ID_INTEL_PTL_M_NHI0: c_uint = 0xe333;
pub const PCI_DEVICE_ID_INTEL_PTL_M_NHI1: c_uint = 0xe334;
pub const PCI_DEVICE_ID_INTEL_PTL_P_NHI0: c_uint = 0xe433;
pub const PCI_DEVICE_ID_INTEL_PTL_P_NHI1: c_uint = 0xe434;
pub const PCI_DEVICE_ID_AMD_1AH_M60H_NHI0: c_uint = 0x1120;
pub const PCI_DEVICE_ID_AMD_1AH_M60H_NHI1: c_uint = 0x1121;
pub const PCI_DEVICE_ID_AMD_1AH_M68H_NHI0: c_uint = 0x113b;
pub const PCI_DEVICE_ID_AMD_1AH_M68H_NHI1: c_uint = 0x113c;
pub const PCI_DEVICE_ID_AMD_1AH_M80H_NHI0: c_uint = 0x1155;
pub const PCI_DEVICE_ID_AMD_1AH_M80H_NHI1: c_uint = 0x1158;
pub const PCI_DEVICE_ID_AMD_1AH_M80H_NHI2: c_uint = 0x1159;
pub const PCI_DEVICE_ID_AMD_1AH_M24H_NHI0: c_uint = 0x151c;
pub const PCI_DEVICE_ID_AMD_1AH_M24H_NHI1: c_uint = 0x151d;
pub const PCI_DEVICE_ID_AMD_1AH_M70H_NHI0: c_uint = 0x158d;
pub const PCI_DEVICE_ID_AMD_1AH_M70H_NHI1: c_uint = 0x158e;
pub const PCI_CLASS_SERIAL_USB_USB4: c_uint = 0x0c0340;
// Host interface quirks

//
// Minimal number of vectors when we use MSI-X. Two for control channel
// Rx/Tx and the rest four are for cross domain DMA paths.
//
pub const MSIX_MIN_VECS: c_int = 6;
pub const MSIX_MAX_VECS: c_int = 16;
