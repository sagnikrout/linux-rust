//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/iidc_rdma.h
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
// Copyright (C) 2021-2025, Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iidc_rdma_event_type {
    IIDC_RDMA_EVENT_BEFORE_MTU_CHANGE,
    IIDC_RDMA_EVENT_AFTER_MTU_CHANGE,
    IIDC_RDMA_EVENT_BEFORE_TC_CHANGE,
    IIDC_RDMA_EVENT_AFTER_TC_CHANGE,
    IIDC_RDMA_EVENT_WARN_RESET,
    IIDC_RDMA_EVENT_CRIT_ERR,
    IIDC_RDMA_EVENT_NBITS		/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_event {
    pub IIDC_RDMA_EVENT_NBITS): DECLARE_BITMAP(type,,
    pub reg: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iidc_rdma_reset_type {
    IIDC_FUNC_RESET,
    IIDC_DEV_RESET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iidc_rdma_protocol {
    IIDC_RDMA_PROTOCOL_IWARP = BIT(0),
    IIDC_RDMA_PROTOCOL_ROCEV2 = BIT(1),
}

// Structure to be populated by core LAN PCI driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_core_dev_info {
    pub /: *mut *mut *mut pci_dev pdev; / PCI device of corresponding to main function,
    pub adev: *mut auxiliary_device,
// Current active RDMA protocol
    pub rdma_protocol: iidc_rdma_protocol,
    pub /: *mut *mut *mut void iidc_priv; / elements unique to each driver,
}

// Structure representing auxiliary driver tailored information about the core
// PCI dev, each auxiliary driver using the IIDC interface will have an
// instance of this struct dedicated to it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_core_auxiliary_dev {
    pub adev: auxiliary_device,
    pub cdev_info: *mut iidc_rdma_core_dev_info,
}

// structure representing the auxiliary driver. This struct is to be
// allocated and populated by the auxiliary driver's owner. The core PCI
// driver will access these ops by performing a container_of on the
// auxiliary_device->dev.driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidc_rdma_core_auxiliary_drv {
    pub adrv: auxiliary_driver,
    pub event): *mut iidc_rdma_event,
}
