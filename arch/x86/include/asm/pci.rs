//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pci.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_sysdata {
    pub /: *mut *mut int domain; / PCI domain,
    pub /: *mut *mut int node; / NUMA node,

    pub /: *mut *mut *mut acpi_device companion; / ACPI companion device,

    pub /: *mut *mut *mut void iommu; / IOMMU private data,

    pub /: *mut *mut *mut void fwnode; / IRQ domain for MSI assignment,

    pub /: *mut *mut *mut pci_dev vmd_dev; / VMD Device if in Intel VMD domain,

}

extern "C" {
    pub fn pci_domain_nr(_arg: bus) -> return;
}

// Can be used to override the logic in pci_scan_bus for skipping
extern "C" {
    pub fn pcibios_assign_all_busses() -> c_uint;
}
extern "C" {
    pub fn pci_legacy_init() -> c_int;
}

pub const PCIBIOS_MIN_IO: c_uint = 0x1000;

pub const PCIBIOS_MIN_CARDBUS_IO: c_uint = 0x4000;
extern "C" {
    pub fn pcibios_scan_root(bus: c_int);
}
extern "C" {
    pub fn pcibios_set_irq_routing(dev: *mut pci_dev, pin: c_int, irq: c_int) -> c_int;
}
extern "C" {
    pub fn pci_dev_has_default_msi_parent_domain(dev: *mut pci_dev) -> bool;
}
// Macro flag: #define HAVE_PCI_MMAP

// Macro flag: #define ARCH_GENERIC_PCI_MMAP_RESOURCE

extern "C" {
    pub fn early_quirks();
}

extern "C" {
    pub fn pci_iommu_alloc();
}

// Returns the node based on pci bus

