//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/alibaba/eea/eea_pci.h
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
// Driver for Alibaba Elastic Ethernet Adapter.
//
// Copyright (C) 2025 Alibaba Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eea_pci_status {
    EEA_PCI_STATUS_NONE,
    EEA_PCI_STATUS_ERR,
    EEA_PCI_STATUS_READY,
    EEA_PCI_STATUS_DONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_pci_cap {
    pub cap_vndr: __u8,
    pub cap_next: __u8,
    pub cap_len: __u8,
    pub cfg_type: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_pci_reset_reg {
    pub cap: eea_pci_cap,
    pub driver: __le16,
    pub device: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_device {
    pub ep_dev: *mut eea_pci_device,
    pub dma_dev: *mut device,
    pub enet: *mut eea_net,
    pub features: u64,
    pub status: eea_pci_status,
    pub ha_reset_netdev_running: bool,
// ha lock for the race between ha work and pci remove
    pub ha_lock: mutex,
    pub rx_num: u32,
    pub tx_num: u32,
    pub db_blk_size: u32,
}

extern "C" {
    pub fn eea_pci_domain_nr(edev: *mut eea_device) -> c_int;
}
extern "C" {
    pub fn eea_pci_bdf(edev: *mut eea_device) -> u16;
}
extern "C" {
    pub fn eea_device_reset(dev: *mut eea_device) -> c_int;
}
extern "C" {
    pub fn eea_pci_set_aq_up(dev: *mut eea_device) -> c_int;
}
extern "C" {
    pub fn eea_pci_active_aq(ering: *mut eea_ring, msix_vec: c_int) -> c_int;
}
extern "C" {
    pub fn eea_pci_free_irq(blk: *mut eea_irq_blk);
}
extern "C" {
    pub fn eea_pci_device_ts(edev: *mut eea_device) -> u64;
}
