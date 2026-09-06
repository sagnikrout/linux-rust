//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/pciif.h
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


// SPDX-License-Identifier: MIT
//
// PCI Backend/Frontend Common Data Structures & Macros
//
// Author: Ryan Wilson <hap9@epoch.ncsc.mil>
//
// Be sure to bump this number if you change this file

// xen_pci_sharedinfo flags

// xen_pci_op commands

// xen_pci_op error numbers

// XEN_PCI_ERR_op_failed - backend failed to complete the operation

//
// it should be PAGE_SIZE-sizeof(struct xen_pci_op))/sizeof(struct msix_entry))
// Should not exceed 128
//
pub const SH_INFO_MAX_VEC: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_msix_entry {
    pub vector: u16,
    pub entry: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pci_op {
// IN: what action to perform: XEN_PCI_OP_*
    pub cmd: u32,
// OUT: will contain an error number (if any) from errno.h
    pub err: i32,
// IN: which device to touch
    pub /: *mut *mut uint32_t domain; / PCI Domain/Segment,
    pub bus: u32,
    pub devfn: u32,
// IN: which configuration registers to touch
    pub offset: i32,
    pub size: i32,
// IN/OUT: Contains the result after a READ or the value to WRITE
    pub value: u32,
// IN: Contains extra infor for this operation
    pub info: u32,
// IN:  param for msi-x
    pub msix_entries: [xen_msix_entry; SH_INFO_MAX_VEC],
}

// used for pcie aer handling
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pcie_aer_op {
// IN: what action to perform: XEN_PCI_OP_*
    pub cmd: u32,
// IN/OUT: return aer_op result or carry error_detected state as input
    pub err: i32,
// IN: which device to touch
    pub Domain/Segment*/: *mut *mut uint32_t domain; / PCI,
    pub bus: u32,
    pub devfn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pci_sharedinfo {
// flags - XEN_PCIF_*
    pub flags: u32,
    pub op: xen_pci_op,
    pub aer_op: xen_pcie_aer_op,
}
