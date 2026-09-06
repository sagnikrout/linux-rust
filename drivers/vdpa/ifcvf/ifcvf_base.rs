//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vdpa/ifcvf/ifcvf_base.h
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
// Intel IFC VF NIC driver for virtio dataplane offloading
//
// Copyright (C) 2020 Intel Corporation.
//
// Author: Zhu Lingshan <lingshan.zhu@intel.com>
//

pub const N3000_DEVICE_ID: c_uint = 0x1041;
pub const N3000_SUBSYS_DEVICE_ID: c_uint = 0x001A;

pub const IFCVF_PCI_MAX_RESOURCE: c_int = 6;
pub const IFCVF_LM_BAR: c_int = 4;
pub const IFCVF_MIN_VQ_SIZE: c_int = 64;

// all vqs and config interrupt has its own vector
pub const MSIX_VECTOR_PER_VQ_AND_CONFIG: c_int = 1;
// all vqs share a vector, and config interrupt has a separate vector
pub const MSIX_VECTOR_SHARED_VQ_AND_CONFIG: c_int = 2;
// all vqs and config interrupt share a vector
pub const MSIX_VECTOR_DEV_SHARED: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_info {
    pub last_avail_idx: u16,
    pub notify_addr: *mut void __iomem,
    pub notify_pa: phys_addr_t,
    pub irq: u32,
    pub cb: vdpa_callback,
    pub msix_name: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifcvf_lm_cfg {
    pub control: __le64,
    pub status: __le64,
    pub lm_mem_log_start_addr: __le64,
    pub lm_mem_log_end_addr: __le64,
    pub vq_state_region: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifcvf_hw {
    pub isr: *mut u8 __iomem,
// Live migration
    pub lm_cfg: *mut ifcvf_lm_cfg __iomem,
// Notification bar number
    pub notify_bar: u8,
    pub msix_vector_status: u8,
// virtio-net or virtio-blk device config size
    pub config_size: u32,
// Notificaiton bar address
    pub notify_base: *mut void __iomem,
    pub notify_base_pa: phys_addr_t,
    pub notify_off_multiplier: u32,
    pub dev_type: u32,
    pub hw_features: u64,
// provisioned device features
    pub dev_features: u64,
    pub common_cfg: *mut virtio_pci_common_cfg __iomem,
    pub dev_cfg: *mut void __iomem,
    pub vring: *mut vring_info,
    pub base: *const *const void __iomem,
    pub config_msix_name: [c_char; 256],
    pub config_cb: vdpa_callback,
    pub config_irq: c_int,
    pub vqs_reused_irq: c_int,
    pub nr_vring: u16,
// VIRTIO_PCI_CAP_DEVICE_CFG size
    pub num_msix_vectors: u32,
    pub cap_dev_config_size: u32,
    pub pdev: *mut pci_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifcvf_adapter {
    pub vdpa: vdpa_device,
    pub pdev: *mut pci_dev,
    pub vf: *mut ifcvf_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifcvf_vdpa_mgmt_dev {
    pub mdev: vdpa_mgmt_dev,
    pub vf: ifcvf_hw,
    pub adapter: *mut ifcvf_adapter,
    pub pdev: *mut pci_dev,
}

extern "C" {
    pub fn ifcvf_init_hw(hw: *mut ifcvf_hw, dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn ifcvf_stop(hw: *mut ifcvf_hw);
}
extern "C" {
    pub fn ifcvf_notify_queue(hw: *mut ifcvf_hw, qid: u16);
}
extern "C" {
    pub fn ifcvf_get_status(hw: *mut ifcvf_hw) -> u8;
}
extern "C" {
    pub fn ifcvf_set_status(hw: *mut ifcvf_hw, status: u8);
}
extern "C" {
    pub fn ifcvf_reset(hw: *mut ifcvf_hw);
}
extern "C" {
    pub fn ifcvf_get_dev_features(hw: *mut ifcvf_hw) -> u64;
}
extern "C" {
    pub fn ifcvf_get_hw_features(hw: *mut ifcvf_hw) -> u64;
}
extern "C" {
    pub fn ifcvf_verify_min_features(hw: *mut ifcvf_hw, features: u64) -> c_int;
}
extern "C" {
    pub fn ifcvf_get_vq_state(hw: *mut ifcvf_hw, qid: u16) -> u16;
}
extern "C" {
    pub fn ifcvf_set_vq_state(hw: *mut ifcvf_hw, qid: u16, num: u16) -> c_int;
}
extern "C" {
    pub fn ifcvf_get_config_size(hw: *mut ifcvf_hw) -> u32;
}
extern "C" {
    pub fn ifcvf_set_vq_vector(hw: *mut ifcvf_hw, qid: u16, vector: c_int) -> u16;
}
extern "C" {
    pub fn ifcvf_set_config_vector(hw: *mut ifcvf_hw, vector: c_int) -> u16;
}
extern "C" {
    pub fn ifcvf_set_vq_num(hw: *mut ifcvf_hw, qid: u16, num: u32);
}
extern "C" {
    pub fn ifcvf_get_vq_ready(hw: *mut ifcvf_hw, qid: u16) -> bool;
}
extern "C" {
    pub fn ifcvf_set_vq_ready(hw: *mut ifcvf_hw, qid: u16, ready: bool);
}
extern "C" {
    pub fn ifcvf_set_driver_features(hw: *mut ifcvf_hw, features: u64);
}
extern "C" {
    pub fn ifcvf_get_driver_features(hw: *mut ifcvf_hw) -> u64;
}
extern "C" {
    pub fn ifcvf_get_max_vq_size(hw: *mut ifcvf_hw) -> u16;
}
extern "C" {
    pub fn ifcvf_get_vq_size(hw: *mut ifcvf_hw, qid: u16) -> u16;
}
