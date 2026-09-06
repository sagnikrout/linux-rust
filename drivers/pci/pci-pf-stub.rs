//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pci-pf-stub.c
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
// pci-pf-stub - simple stub driver for PCI SR-IOV PF device
//
// This driver is meant to act as a "whitelist" for devices that provide
// SR-IOV functionality while at the same time not actually needing a
// driver of their own.
//

//
// pci_pf_stub_whitelist - White list of devices to bind pci-pf-stub onto
//
// This table provides the list of IDs this driver is supposed to bind
// onto.  You could think of this as a list of "quirked" devices where we
// are adding support for SR-IOV here since there are no other drivers
// that they would be running under.
//
    static const struct pci_device_id pci_pf_stub_whitelist[] = {
    { PCI_VDEVICE(AMAZON, 0x0053) },
// required last entry
    { 0 }
    };
    MODULE_DEVICE_TABLE(pci, pci_pf_stub_whitelist);
    static int pci_pf_stub_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    pci_info(dev, "claimed by pci-pf-stub\n");
    return 0;
    }
    static struct pci_driver pf_stub_driver = {
    .name			= "pci-pf-stub",
    .id_table		= pci_pf_stub_whitelist,
    .probe			= pci_pf_stub_probe,
    .sriov_configure	= pci_sriov_configure_simple,
    };
    module_pci_driver(pf_stub_driver);
    MODULE_DESCRIPTION("SR-IOV PF stub driver with no functionality");
    MODULE_LICENSE("GPL");
