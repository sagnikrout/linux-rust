//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ppc-pci.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// c 2001 PPC 64 Team, IBM Corp
//

// Bus Unit ID macros; get low and hi 32-bits of the 64-bit BUID

// PCI device_node operations
extern "C" {
    pub fn pci_devs_phb_init_dynamic(phb: *mut pci_controller);
}

extern "C" {
    pub fn ppc_iommu_register_device(phb: *mut pci_controller);
}
extern "C" {
    pub fn ppc_iommu_unregister_device(phb: *mut pci_controller);
}

// From rtas_pci.h
extern "C" {
    pub fn init_pci_config_tokens();
}
extern "C" {
    pub fn get_phb_buid(: *mut device_node) -> c_ulong;
}
extern "C" {
    pub fn rtas_setup_phb(phb: *mut pci_controller) -> c_int;
}
extern "C" {
    pub fn rtas_pci_dn_read_config(pdn: *mut pci_dn, where: c_int, size: c_int, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn rtas_pci_dn_write_config(pdn: *mut pci_dn, where: c_int, size: c_int, val: u32) -> c_int;
}

extern "C" {
    pub fn eeh_addr_cache_insert_dev(dev: *mut pci_dev);
}
extern "C" {
    pub fn eeh_addr_cache_rmv_dev(dev: *mut pci_dev);
}
extern "C" {
    pub fn eeh_slot_error_detail(pe: *mut eeh_pe, severity: c_int);
}
extern "C" {
    pub fn eeh_pci_enable(pe: *mut eeh_pe, function: c_int) -> c_int;
}
extern "C" {
    pub fn eeh_pe_reset_full(pe: *mut eeh_pe, include_passed: bool) -> c_int;
}
extern "C" {
    pub fn eeh_save_bars(edev: *mut eeh_dev);
}
extern "C" {
    pub fn eeh_pe_state_mark(pe: *mut eeh_pe, state: c_int);
}
extern "C" {
    pub fn eeh_pe_mark_isolated(pe: *mut eeh_pe);
}
extern "C" {
    pub fn eeh_pe_state_clear(pe: *mut eeh_pe, state: c_int, include_passed: bool);
}
extern "C" {
    pub fn eeh_pe_state_mark_with_cfg(pe: *mut eeh_pe, state: c_int);
}
extern "C" {
    pub fn eeh_pe_dev_mode_mark(pe: *mut eeh_pe, mode: c_int);
}
extern "C" {
    pub fn eeh_sysfs_add_device(pdev: *mut pci_dev);
}
extern "C" {
    pub fn eeh_sysfs_remove_device(pdev: *mut pci_dev);
}

extern "C" {
    pub fn uli_init() -> void __init;
}

