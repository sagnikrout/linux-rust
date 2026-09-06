//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vfio_pci_core.h
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
// Copyright (C) 2012 Red Hat, Inc.  All rights reserved.
// Author: Alex Williamson <alex.williamson@redhat.com>
//
// Derived from original vfio:
// Copyright 2010 Cisco Systems, Inc.  All rights reserved.
// Author: Tom Lyon, pugs@cisco.com
//

pub const VFIO_PCI_OFFSET_SHIFT: c_int = 40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_pci_eventfd {
    pub ctx: *mut eventfd_ctx,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_pci_regops {
    pub iswrite): *mut *mut size_t count, loff_t ppos, bool,
    pub region): *mut vfio_pci_region,
    pub vma): *mut vm_area_struct,
    pub caps): *mut vfio_info_cap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_pci_region {
    pub type: u32,
    pub subtype: u32,
    pub ops: *const vfio_pci_regops,
    pub data: *mut c_void,
    pub size: usize,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_pci_device_ops {
    pub nr_ranges): usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_pci_core_device {
    pub vdev: vfio_device,
    pub pdev: *mut pci_dev,
    pub pci_ops: *const vfio_pci_device_ops,
    pub barmap: [*mut void __iomem; PCI_STD_NUM_BARS],
    pub bar_mmap_supported: [bool; PCI_STD_NUM_BARS],
// Flags modified at runtime - dedicated storage unit
    pub virq_disabled: bool,
    pub bardirty: bool,
    pub pci_config_map: *mut u8,
    pub vconfig: *mut u8,
    pub msi_perm: *mut perm_bits,
    pub irqlock: spinlock_t,
    pub igate: mutex,
    pub ctx: xarray,
    pub irq_type: c_int,
    pub num_regions: c_int,
    pub region: *mut vfio_pci_region,
    pub msi_qmax: u8,
    pub msix_bar: u8,
    pub msix_size: u16,
    pub msix_offset: u32,
    pub rbar: [u32; 7],
// Flags only modified on setup/release - bitfield ok
    pub has_dyn_msix:1: bool,
    pub pci_2_3:1: bool,
    pub reset_works:1: bool,
    pub extended_caps:1: bool,
    pub has_vga:1: bool,
    pub nointx:1: bool,
    pub needs_pm_restore:1: bool,
    pub disable_idle_d3:1: bool,
    pub nointxmask:1: bool,
    pub disable_vga:1: bool,
// Flags modified at runtime - dedicated storage unit
    pub needs_reset: bool,
    pub pm_intx_masked: bool,
    pub pm_runtime_engaged: bool,
    pub sriov_active: bool,
    pub pci_saved_state: *mut pci_saved_state,
    pub pm_save: *mut pci_saved_state,
    pub ioeventfds_nr: c_int,
    pub err_trigger: *mut vfio_pci_eventfd __rcu,
    pub req_trigger: *mut vfio_pci_eventfd __rcu,
    pub pm_wake_eventfd_ctx: *mut eventfd_ctx,
    pub dummy_resources_list: list_head,
    pub ioeventfds_lock: mutex,
    pub ioeventfds_list: list_head,
    pub vf_token: *mut vfio_pci_vf_token,
    pub sriov_pfs_item: list_head,
    pub sriov_pf_core_dev: *mut vfio_pci_core_device,
    pub nb: notifier_block,
    pub memory_lock: rw_semaphore,
    pub dmabufs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vfio_pci_io_width {
    VFIO_PCI_IO_WIDTH_1 = 1,
    VFIO_PCI_IO_WIDTH_2 = 2,
    VFIO_PCI_IO_WIDTH_4 = 4,
    VFIO_PCI_IO_WIDTH_8 = 8,
}

// Will be exported for vfio pci drivers usage
extern "C" {
    pub fn vfio_pci_core_close_device(core_vdev: *mut vfio_device);
}
extern "C" {
    pub fn vfio_pci_core_init_dev(core_vdev: *mut vfio_device) -> c_int;
}
extern "C" {
    pub fn vfio_pci_core_release_dev(core_vdev: *mut vfio_device);
}
extern "C" {
    pub fn vfio_pci_core_register_device(vdev: *mut vfio_pci_core_device) -> c_int;
}
extern "C" {
    pub fn vfio_pci_core_unregister_device(vdev: *mut vfio_pci_core_device);
}
extern "C" {
    pub fn vfio_pci_core_mmap(core_vdev: *mut vfio_device, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn vfio_pci_core_request(core_vdev: *mut vfio_device, count: c_uint);
}
extern "C" {
    pub fn vfio_pci_core_match(core_vdev: *mut vfio_device, buf: *mut c_char) -> c_int;
}
extern "C" {
    pub fn vfio_pci_core_enable(vdev: *mut vfio_pci_core_device) -> c_int;
}
extern "C" {
    pub fn vfio_pci_core_disable(vdev: *mut vfio_pci_core_device);
}
extern "C" {
    pub fn vfio_pci_core_finish_enable(vdev: *mut vfio_pci_core_device);
}
extern "C" {
    pub fn __vfio_pci_memory_enabled(vdev: *mut vfio_pci_core_device) -> bool;
}

//
// Returns a BAR's iomap base or an ERR_PTR() if, for example, the
// BAR isn't valid, its resource wasn't acquired, or its iomap
// failed.  This shall only be used after vfio_pci_core_enable()
// has set up the BAR maps and before vfio_pci_core_disable()
// tears them down.
//
extern "C" {
    pub fn IOMEM_ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn IOMEM_ERR_PTR(_arg: -ENODEV) -> return;
}
