//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/helpers/pci.c
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
// pci_acc_init
//
// PCI access helper function depending on libpci
//
// **pacc : if a valid pci_dev is returned
// *pacc must be passed to pci_acc_cleanup to free it
//
// domain: domain
// bus:    bus
// slot:   slot
// func:   func
// vendor: vendor
// device: device
// Pass -1 for one of the six above to match any
//
// Returns :
// struct pci_dev which can be used with pci_{read,write}_* functions
// to access the PCI config space of matching pci devices
//
    struct pci_dev *pci_acc_init(struct pci_access **pacc, int domain, int bus,
    int slot, int func, int vendor, int dev)
    {
    struct pci_filter filter_nb_link;
    struct pci_dev *device;
// pacc = pci_alloc();
    if (*pacc == core::ptr::null_mut())
    return core::ptr::null_mut();
    pci_filter_init(*pacc, &filter_nb_link);
    filter_nb_link.domain	= domain;
    filter_nb_link.bus	= bus;
    filter_nb_link.slot	= slot;
    filter_nb_link.func	= func;
    filter_nb_link.vendor	= vendor;
    filter_nb_link.device	= dev;
    pci_init(*pacc);
    pci_scan_bus(*pacc);
    for (device = (*pacc).devices; device; device = device.next) {
    if (pci_filter_match(&filter_nb_link, device))
    return device;
    }
    pci_cleanup(*pacc);
    return core::ptr::null_mut();
    }
// Typically one wants to get a specific slot(device)/func of the root domain
    and bus */
    struct pci_dev *pci_slot_func_init(struct pci_access **pacc, int slot,
    int func)
    {
    return pci_acc_init(pacc, 0, 0, slot, func, -1, -1);
    }
