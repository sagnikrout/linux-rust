//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/aer.h
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
// Copyright (C) 2006 Intel Corp.
// Tom Long Nguyen (tom.l.nguyen@intel.com)
// Zhang Yanmin (yanmin.zhang@intel.com)
//

pub const AER_NONFATAL: c_int = 0;
pub const AER_FATAL: c_int = 1;
pub const AER_CORRECTABLE: c_int = 2;
pub const DPC_FATAL: c_int = 3;
//
// AER and DPC capabilities TLP Logging register sizes (PCIe r6.2, sec 7.8.4
// & 7.9.14).
//
pub const PCIE_STD_NUM_TLP_HEADERLOG: c_int = 4;
pub const PCIE_STD_MAX_TLP_PREFIXLOG: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_tlp_log {
    pub dw: [u32; PCIE_STD_MAX_TLP_HEADERLOG],
    pub _do_not_use: [u32; PCIE_STD_NUM_TLP_HEADERLOG],
    pub prefix: [u32; PCIE_STD_MAX_TLP_PREFIXLOG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aer_capability_regs {
    pub header: u32,
    pub uncor_status: u32,
    pub uncor_mask: u32,
    pub uncor_severity: u32,
    pub cor_status: u32,
    pub cor_mask: u32,
    pub cap_control: u32,
    pub header_log: pcie_tlp_log,
    pub root_command: u32,
    pub root_status: u32,
    pub cor_err_source: u16,
    pub uncor_err_source: u16,
}

extern "C" {
    pub fn pci_aer_clear_nonfatal_status(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pcie_aer_is_native(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_aer_unmask_internal_errors(dev: *mut pci_dev);
}

extern "C" {
    pub fn cper_severity_to_aer(cper_severity: c_int) -> c_int;
}
