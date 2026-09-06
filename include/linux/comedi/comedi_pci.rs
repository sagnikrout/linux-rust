//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/comedi/comedi_pci.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// comedi_pci.h
// header file for Comedi PCI drivers
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
//

//
// PCI Vendor IDs not in <linux/pci_ids.h>
//
pub const PCI_VENDOR_ID_KOLTER: c_uint = 0x1001;
pub const PCI_VENDOR_ID_ICP: c_uint = 0x104c;
pub const PCI_VENDOR_ID_DT: c_uint = 0x1116;
pub const PCI_VENDOR_ID_IOTECH: c_uint = 0x1616;
pub const PCI_VENDOR_ID_CONTEC: c_uint = 0x1221;
pub const PCI_VENDOR_ID_RTD: c_uint = 0x1435;
pub const PCI_VENDOR_ID_HUMUSOFT: c_uint = 0x186c;
extern "C" {
    pub fn comedi_pci_enable(dev: *mut comedi_device) -> c_int;
}
extern "C" {
    pub fn comedi_pci_disable(dev: *mut comedi_device);
}
extern "C" {
    pub fn comedi_pci_detach(dev: *mut comedi_device);
}
extern "C" {
    pub fn comedi_pci_auto_unconfig(pcidev: *mut pci_dev);
}
//
// module_comedi_pci_driver() - Helper macro for registering a comedi PCI driver
// @__comedi_driver: comedi_driver struct
// @__pci_driver: pci_driver struct
//
// Helper macro for comedi PCI drivers which do not do anything special
// in module init/exit. This eliminates a lot of boilerplate. Each
// module may only use this macro once, and calling it replaces
// module_init() and module_exit()
//

