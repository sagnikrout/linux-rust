//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/xen/pci.h
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

extern "C" {
    pub fn pci_xen_init() -> int __init;
}
extern "C" {
    pub fn pci_xen_hvm_init() -> int __init;
}
pub const pci_xen: c_int = 1;

pub const pci_xen: c_int = 0;

extern "C" {
    pub fn pci_xen_initial_domain() -> int __init;
}

// The drivers/pci/xen-pcifront.c sets this structure to
// its own functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_pci_frontend_ops {
    pub vectors[]): *mut *mut *mut int (enable_msi)(struct pci_dev dev, int,
    pub dev): *mut *mut void (disable_msi)(struct pci_dev,
    pub nvec): *mut *mut *mut int (enable_msix)(struct pci_dev dev, int vectors[], int,
    pub dev): *mut *mut void (disable_msix)(struct pci_dev,
}

