//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci-ecam.h
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
// Copyright 2016 Broadcom
//

//
// Memory address shift values for the byte-level address that
// can be used when accessing the PCI Express Configuration Space.
//
// Enhanced Configuration Access Mechanism (ECAM)
//
// See PCI Express Base Specification, Revision 5.0, Version 1.0,
// Section 7.2.2, Table 7-1, p. 677.
//

pub const PCIE_ECAM_BUS_MASK: c_uint = 0xff;
pub const PCIE_ECAM_DEVFN_MASK: c_uint = 0xff;
pub const PCIE_ECAM_REG_MASK: c_uint = 0xfff /* Limit offset to a maximum of 4K */;

//
// struct to hold pci ops and bus shift of the config window
// for a PCI controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_ecam_ops {
    pub bus_shift: c_uint,
    pub pci_ops: pci_ops,
    pub ): *mut *mut int (init)(struct pci_config_window,
    pub ): *mut pci_dev,
    pub ): *mut pci_dev,
}

//
// struct to hold the mappings of a config space window. This
// is expected to be used as sysdata for PCI controllers that
// use ECAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_config_window {
    pub res: resource,
    pub busr: resource,
    pub bus_shift: c_uint,
    pub priv: *mut c_void,
    pub ops: *const pci_ecam_ops,
    pub /: *mut *mut *mut void __iomem win; / 64-bit single mapping,
    pub /: *mut *mut *mut *mut void __iomem winp; / 32-bit per-bus mapping,
}

// create and free pci_config_window
extern "C" {
    pub fn pci_ecam_free(cfg: *mut pci_config_window);
}
// map_bus when ->sysdata is an instance of pci_config_window
// default ECAM ops
// default CAM ops

