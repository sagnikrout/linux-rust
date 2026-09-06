//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci-tph.h
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
// TPH (TLP Processing Hints)
//
// Copyright (C) 2024 Advanced Micro Devices, Inc.
// Eric Van Tassell <Eric.VanTassell@amd.com>
// Wei Huang <wei.huang2@amd.com>
//
// According to the ECN for PCI Firmware Spec, Steering Tag can be different
// depending on the memory type: Volatile Memory or Persistent Memory. When a
// caller query about a target's Steering Tag, it must provide the target's
// tph_mem_type. ECN link: https://members.pcisig.com/wg/PCI-SIG/document/15470.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tph_mem_type {
    TPH_MEM_TYPE_VM,	/* volatile memory */
    TPH_MEM_TYPE_PM		/* persistent memory */
}

extern "C" {
    pub fn pcie_disable_tph(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pcie_enable_tph(pdev: *mut pci_dev, mode: c_int) -> c_int;
}
extern "C" {
    pub fn pcie_tph_get_st_table_size(pdev: *mut pci_dev) -> u16;
}
extern "C" {
    pub fn pcie_tph_get_st_table_loc(pdev: *mut pci_dev) -> u32;
}

