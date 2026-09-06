//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vdpa/solidrun/snet_vdpa.h
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
// SolidRun DPU driver for control plane
//
// Copyright (C) 2022-2023 SolidRun
//
// Author: Alvaro Karsz <alvaro.karsz@solid-run.com>
//

pub const SNET_NAME_SIZE: c_int = 256;

// Check if negotiated config version is at least @ver

// VQ struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snet_vq {
// VQ callback
    pub cb: vdpa_callback,
// VQ state received from bus
    pub vq_state: vdpa_vq_state,
// desc base address
    pub desc_area: u64,
// device base address
    pub device_area: u64,
// driver base address
    pub driver_area: u64,
// Queue size
    pub num: u32,
// Serial ID for VQ
    pub sid: u32,
// is ready flag
    pub ready: bool,
// IRQ number
    pub irq: u32,
// IRQ index, DPU uses this to parse data from MSI-X table
    pub irq_idx: u32,
// IRQ name
    pub irq_name: [c_char; SNET_NAME_SIZE],
// pointer to mapped PCI BAR register used by this VQ to kick
    pub kick_ptr: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snet {
// vdpa device
    pub vdpa: vdpa_device,
// Config callback
    pub cb: vdpa_callback,
// To lock the control mechanism
    pub ctrl_lock: mutex,
// Spinlock to protect critical parts in the control mechanism
    pub ctrl_spinlock: spinlock_t,
// array of virqueues
    pub vqs: *mut snet_vq,
// Used features
    pub negotiated_features: u64,
// Device serial ID
    pub sid: u32,
// device status
    pub status: u8,
// boolean indicating if snet config was passed to the device
    pub dpu_ready: bool,
// IRQ number
    pub cfg_irq: u32,
// IRQ index, DPU uses this to parse data from MSI-X table
    pub cfg_irq_idx: u32,
// IRQ name
    pub cfg_irq_name: [c_char; SNET_NAME_SIZE],
// BAR to access the VF
    pub bar: *mut void __iomem,
// PCI device
    pub pdev: *mut pci_dev,
// Pointer to snet pdev parent device
    pub psnet: *mut psnet,
// Pointer to snet config device
    pub cfg: *mut snet_dev_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snet_dev_cfg {
// Device ID following VirtIO spec.
    pub virtio_id: u32,
// Number of VQs for this device
    pub vq_num: u32,
// Size of every VQ
    pub vq_size: u32,
// Virtual Function id
    pub vfid: u32,
// Device features, following VirtIO spec
    pub features: u64,
// Reserved for future usage
    pub rsvd: [u32; 6],
// VirtIO device specific config size
    pub cfg_size: u32,
// VirtIO device specific config address
    pub virtio_cfg: *mut void __iomem,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snet_cfg {
// Magic key
    pub key: u32,
// Size of total config in bytes
    pub cfg_size: u32,
// Config version
    pub cfg_ver: u32,
// Number of Virtual Functions to create
    pub vf_num: u32,
// BAR to use for the VFs
    pub vf_bar: u32,
// Where should we write the SNET's config
    pub host_cfg_off: u32,
// Max. allowed size for a SNET's config
    pub max_size_host_cfg: u32,
// VirtIO config offset in BAR
    pub virtio_cfg_off: u32,
// Offset in PCI BAR for VQ kicks
    pub kick_off: u32,
// Offset in PCI BAR for HW monitoring
    pub hwmon_off: u32,
// Offset in PCI BAR for Control mechanism
    pub ctrl_off: u32,
// Config general flags - enum snet_cfg_flags
    pub flags: u32,
// Reserved for future usage
    pub rsvd: [u32; 6],
// Number of snet devices
    pub devices_num: u32,
// The actual devices
    pub devs: *mut snet_dev_cfg,
    pub __packed: },
// SolidNET PCIe device, one device per PCIe physical function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psnet {
// PCI BARs
    pub bars: [*mut void __iomem; PCI_STD_NUM_BARS],
// Negotiated config version
    pub negotiated_cfg_ver: u32,
// Next IRQ index to use in case when the IRQs are allocated from this device
    pub next_irq: u32,
// BAR number used to communicate with the device
    pub barno: u8,
// spinlock to protect data that can be changed by SNET devices
    pub lock: spinlock_t,
// Pointer to the device's config read from BAR
    pub cfg: snet_cfg,
// Name of monitor device
    pub hwmon_name: [c_char; SNET_NAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snet_cfg_flags {
// Create a HWMON device
    SNET_CFG_FLAG_HWMON = BIT(0),
// USE IRQs from the physical function
    SNET_CFG_FLAG_IRQ_PF = BIT(1),
}

extern "C" {
    pub fn ioread32(off: psnet->bars[psnet->barno] +) -> return;
}
extern "C" {
    pub fn ioread32(off: snet->bar +) -> return;
}
// 64bits are written in 2 halves, low part first
// The DPU expects a 64bit integer in 2 halves, the low part first

extern "C" {
    pub fn psnet_create_hwmon(pdev: *mut pci_dev);
}

extern "C" {
    pub fn snet_ctrl_clear(snet: *mut snet);
}
extern "C" {
    pub fn snet_destroy_dev(snet: *mut snet) -> c_int;
}
extern "C" {
    pub fn snet_read_vq_state(snet: *mut snet, idx: u16, state: *mut vdpa_vq_state) -> c_int;
}
extern "C" {
    pub fn snet_suspend_dev(snet: *mut snet) -> c_int;
}
extern "C" {
    pub fn snet_resume_dev(snet: *mut snet) -> c_int;
}
