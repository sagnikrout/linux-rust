//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/zte/dinghai/dh_queue.h
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
// ZTE DingHai Ethernet driver - PCI capability definitions
// Copyright (c) 2022-2026, ZTE Corporation.
//

// This is the PCI capability header:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zxdh_pf_pci_cap {
    pub /: *mut *mut __u8 cap_vndr; / Generic PCI field: PCI_CAP_ID_VNDR,
    pub /: *mut *mut __u8 cap_next; / Generic PCI field: next ptr.,
    pub /: *mut *mut __u8 cap_len; / Generic PCI field: capability length,
    pub /: *mut *mut __u8 cfg_type; / Identifies the structure.,
    pub /: *mut *mut __u8 bar; / Where to find it.,
    pub /: *mut *mut __u8 id; / Multiple capabilities of the same type,
    pub /: *mut *mut __u8 padding[2]; / Pad to full dword.,
    pub /: *mut *mut __le32 offset; / Offset within bar.,
    pub /: *mut *mut __le32 length; / Length of the structure, in bytes.,
}

// Fields in ZXDH_PF_PCI_CAP_COMMON_CFG:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zxdh_pf_pci_common_cfg {
// About the whole device.
    pub /: *mut *mut __le32 device_feature_select; / read-write,
    pub /: *mut *mut __le32 device_feature; / read-only,
    pub /: *mut *mut __le32 guest_feature_select; / read-write,
    pub /: *mut *mut __le32 guest_feature; / read-write,
    pub /: *mut *mut __le16 msix_config; / read-write,
    pub /: *mut *mut __le16 num_queues; / read-only,
    pub /: *mut *mut __u8 device_status; / read-write,
    pub /: *mut *mut __u8 config_generation; / read-only,
// About a specific virtqueue.
    pub /: *mut *mut __le16 queue_select; / read-write,
    pub /: *mut *mut __le16 queue_size; / read-write, power of 2.,
    pub /: *mut *mut __le16 queue_msix_vector; / read-write,
    pub /: *mut *mut __le16 queue_enable; / read-write,
    pub /: *mut *mut __le16 queue_notify_off; / read-only,
    pub /: *mut *mut __le32 queue_desc_lo; / read-write,
    pub /: *mut *mut __le32 queue_desc_hi; / read-write,
    pub /: *mut *mut __le32 queue_avail_lo; / read-write,
    pub /: *mut *mut __le32 queue_avail_hi; / read-write,
    pub /: *mut *mut __le32 queue_used_lo; / read-write,
    pub /: *mut *mut __le32 queue_used_hi; / read-write,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zxdh_pf_pci_notify_cap {
    pub cap: zxdh_pf_pci_cap,
    pub /: *mut *mut __le32 notify_off_multiplier; / Multiplier for queue_notify_off.,
}
