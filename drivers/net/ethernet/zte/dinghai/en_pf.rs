//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/zte/dinghai/en_pf.h
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
// ZTE DingHai Ethernet driver - PF header
// Copyright (c) 2022-2026, ZTE Corporation.
//

pub const ZXDH_PF_VENDOR_ID: c_uint = 0x1cf2;
pub const ZXDH_PF_DEVICE_ID: c_uint = 0x8040;
pub const ZXDH_VF_DEVICE_ID: c_uint = 0x8041;
// Common configuration
pub const ZXDH_PCI_CAP_COMMON_CFG: c_int = 1;
// Notifications
pub const ZXDH_PCI_CAP_NOTIFY_CFG: c_int = 2;
// ISR access
pub const ZXDH_PCI_CAP_ISR_CFG: c_int = 3;
// Device specific configuration
pub const ZXDH_PCI_CAP_DEVICE_CFG: c_int = 4;
// PCI configuration access
pub const ZXDH_PCI_CAP_PCI_CFG: c_int = 5;
pub const ZXDH_PF_MAX_BAR_VAL: c_uint = 0x5;
pub const ZXDH_PF_ALIGN4: c_int = 4;
pub const ZXDH_PF_ALIGN2: c_int = 2;
pub const ZXDH_PF_MAP_MINLEN2: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zxdh_core_dev {
    pub device: *mut device,
    pub pdev: *mut pci_dev,
    pub devlink: *mut devlink,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zxdh_pf_dev {
    pub common: *mut zxdh_pf_pci_common_cfg __iomem,
// Device-specific data (non-legacy mode)
// Base of vq notifications (non-legacy mode).
    pub device: *mut void __iomem,
    pub notify_base: *mut void __iomem,
// Physical base of vq notifications
    pub notify_pa: resource_size_t,
// So we can sanity-check accesses.
    pub notify_len: usize,
    pub device_len: usize,
// Capability for when we need to map notifications per-vq.
    pub notify_map_cap: i32,
    pub notify_offset_multiplier: u32,
// Multiply queue_notify_off by this value. (non-legacy mode).
    pub modern_bars: i32,
    pub pci_ioremap_addr: [*mut void __iomem; 6],
    pub dev_cfg_bar_off: u32,
}

extern "C" {
    pub fn zxdh_core_free_priv(zxdh_dev: *mut zxdh_core_dev);
}
extern "C" {
    pub fn zxdh_pf_pci_close(zxdh_dev: *mut zxdh_core_dev);
}
extern "C" {
    pub fn zxdh_pf_common_cfg_init(zxdh_dev: *mut zxdh_core_dev) -> c_int;
}
extern "C" {
    pub fn zxdh_pf_notify_cfg_init(zxdh_dev: *mut zxdh_core_dev) -> c_int;
}
extern "C" {
    pub fn zxdh_pf_device_cfg_init(zxdh_dev: *mut zxdh_core_dev) -> c_int;
}
extern "C" {
    pub fn zxdh_pf_modern_cfg_uninit(zxdh_dev: *mut zxdh_core_dev);
}
extern "C" {
    pub fn zxdh_pf_modern_cfg_init(zxdh_dev: *mut zxdh_core_dev) -> c_int;
}
