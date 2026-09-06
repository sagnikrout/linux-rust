//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/xhci-debugfs.h
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
// xhci-debugfs.h - xHCI debugfs interface
//
// Copyright (C) 2017 Intel Corporation
//
// Author: Lu Baolu <baolu.lu@linux.intel.com>
//

pub const DEBUGFS_NAMELEN: c_int = 32;
pub const REG_CAPLENGTH: c_uint = 0x00;
pub const REG_HCSPARAMS1: c_uint = 0x04;
pub const REG_HCSPARAMS2: c_uint = 0x08;
pub const REG_HCSPARAMS3: c_uint = 0x0c;
pub const REG_HCCPARAMS1: c_uint = 0x10;
pub const REG_DOORBELLOFF: c_uint = 0x14;
pub const REG_RUNTIMEOFF: c_uint = 0x18;
pub const REG_HCCPARAMS2: c_uint = 0x1c;
pub const REG_USBCMD: c_uint = 0x00;
pub const REG_USBSTS: c_uint = 0x04;
pub const REG_PAGESIZE: c_uint = 0x08;
pub const REG_DNCTRL: c_uint = 0x14;
pub const REG_CRCR: c_uint = 0x18;
pub const REG_DCBAAP_LOW: c_uint = 0x30;
pub const REG_DCBAAP_HIGH: c_uint = 0x34;
pub const REG_CONFIG: c_uint = 0x38;
pub const REG_MFINDEX: c_uint = 0x00;
pub const REG_IR0_IMAN: c_uint = 0x20;
pub const REG_IR0_IMOD: c_uint = 0x24;
pub const REG_IR0_ERSTSZ: c_uint = 0x28;
pub const REG_IR0_ERSTBA_LOW: c_uint = 0x30;
pub const REG_IR0_ERSTBA_HIGH: c_uint = 0x34;
pub const REG_IR0_ERDP_LOW: c_uint = 0x38;
pub const REG_IR0_ERDP_HIGH: c_uint = 0x3c;
pub const REG_EXTCAP_USBLEGSUP: c_uint = 0x00;
pub const REG_EXTCAP_USBLEGCTLSTS: c_uint = 0x04;
pub const REG_EXTCAP_REVISION: c_uint = 0x00;
pub const REG_EXTCAP_NAME: c_uint = 0x04;
pub const REG_EXTCAP_PORTINFO: c_uint = 0x08;
pub const REG_EXTCAP_PORTTYPE: c_uint = 0x0c;
pub const REG_EXTCAP_MANTISSA1: c_uint = 0x10;
pub const REG_EXTCAP_MANTISSA2: c_uint = 0x14;
pub const REG_EXTCAP_MANTISSA3: c_uint = 0x18;
pub const REG_EXTCAP_MANTISSA4: c_uint = 0x1c;
pub const REG_EXTCAP_MANTISSA5: c_uint = 0x20;
pub const REG_EXTCAP_MANTISSA6: c_uint = 0x24;
pub const REG_EXTCAP_DBC_CAPABILITY: c_uint = 0x00;
pub const REG_EXTCAP_DBC_DOORBELL: c_uint = 0x04;
pub const REG_EXTCAP_DBC_ERSTSIZE: c_uint = 0x08;
pub const REG_EXTCAP_DBC_ERST_LOW: c_uint = 0x10;
pub const REG_EXTCAP_DBC_ERST_HIGH: c_uint = 0x14;
pub const REG_EXTCAP_DBC_ERDP_LOW: c_uint = 0x18;
pub const REG_EXTCAP_DBC_ERDP_HIGH: c_uint = 0x1c;
pub const REG_EXTCAP_DBC_CONTROL: c_uint = 0x20;
pub const REG_EXTCAP_DBC_STATUS: c_uint = 0x24;
pub const REG_EXTCAP_DBC_PORTSC: c_uint = 0x28;
pub const REG_EXTCAP_DBC_CONT_LOW: c_uint = 0x30;
pub const REG_EXTCAP_DBC_CONT_HIGH: c_uint = 0x34;
pub const REG_EXTCAP_DBC_DEVINFO1: c_uint = 0x38;
pub const REG_EXTCAP_DBC_DEVINFO2: c_uint = 0x3c;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_regset {
    pub name: [c_char; DEBUGFS_NAMELEN],
    pub regset: debugfs_regset32,
    pub nregs: usize,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_file_map {
    pub name: *const c_char,
    pub unused): *mut *mut *mut int (show)(struct seq_file s, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_ep_priv {
    pub name: [c_char; DEBUGFS_NAMELEN],
    pub root: *mut dentry,
    pub stream_info: *mut xhci_stream_info,
    pub show_ring: *mut xhci_ring,
    pub stream_id: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xhci_slot_priv {
    pub name: [c_char; DEBUGFS_NAMELEN],
    pub root: *mut dentry,
    pub eps: [*mut xhci_ep_priv; 31],
    pub dev: *mut xhci_virt_device,
}

extern "C" {
    pub fn xhci_debugfs_init(xhci: *mut xhci_hcd);
}
extern "C" {
    pub fn xhci_debugfs_exit(xhci: *mut xhci_hcd);
}
extern "C" {
    pub fn xhci_debugfs_create_root() -> void __init;
}
extern "C" {
    pub fn xhci_debugfs_remove_root() -> void __exit;
}
extern "C" {
    pub fn xhci_debugfs_create_slot(xhci: *mut xhci_hcd, slot_id: c_int);
}
extern "C" {
    pub fn xhci_debugfs_remove_slot(xhci: *mut xhci_hcd, slot_id: c_int);
}

