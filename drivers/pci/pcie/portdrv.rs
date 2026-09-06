//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/pcie/portdrv.h
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
// Purpose:	PCI Express Port Bus Driver's Internal Data Structures
//
// Copyright (C) 2004 Intel
// Copyright (C) Tom Long Nguyen (tom.l.nguyen@intel.com)
//

// Service Type

pub const PCIE_PORT_DEVICE_MAXSERVICES: c_int = 5;

extern "C" {
    pub fn pcie_aer_init() -> c_int;
}

extern "C" {
    pub fn pcie_hp_init() -> c_int;
}

extern "C" {
    pub fn pcie_pme_init() -> c_int;
}

extern "C" {
    pub fn pcie_dpc_init() -> c_int;
}

extern "C" {
    pub fn pcie_bwctrl_init() -> c_int;
}
// Port Type

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_device {
    pub /: *mut *mut int irq; / Service IRQ/MSI/MSI-X Vector,
    pub /: *mut *mut *mut pci_dev port; / Root/Upstream/Downstream Port,
    pub /: *mut *mut u32 service; / Port service this device represents,
    pub /: *mut *mut *mut void priv_data; / Service Private Data,
    pub /: *mut *mut device device; / Generic Device Interface,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_port_service_driver {
    pub name: *const c_char,
    pub dev): *mut *mut int (probe)(struct pcie_device,
    pub dev): *mut *mut void (remove)(struct pcie_device,
    pub dev): *mut *mut int (suspend)(struct pcie_device,
    pub dev): *mut *mut int (resume_noirq)(struct pcie_device,
    pub dev): *mut *mut int (resume)(struct pcie_device,
    pub dev): *mut *mut int (runtime_suspend)(struct pcie_device,
    pub dev): *mut *mut int (runtime_resume)(struct pcie_device,
    pub dev): *mut *mut int (slot_reset)(struct pcie_device,
    pub /: *mut *mut int port_type; / Type of the port this driver can handle,
    pub /: *mut *mut u32 service; / Port service this device represents,
    pub driver: device_driver,
}

extern "C" {
    pub fn pcie_port_service_register(new: *mut pcie_port_service_driver) -> c_int;
}
extern "C" {
    pub fn pcie_port_service_unregister(new: *mut pcie_port_service_driver);
}

extern "C" {
    pub fn pcie_pme_interrupt_enable(dev: *mut pci_dev, enable: bool);
}

extern "C" {
    pub fn is_aer_internal_error(info: *mut aer_err_info) -> bool;
}
extern "C" {
    pub fn cxl_rch_handle_error(dev: *mut pci_dev, info: *mut aer_err_info);
}
extern "C" {
    pub fn cxl_rch_enable_rcec(rcec: *mut pci_dev);
}

