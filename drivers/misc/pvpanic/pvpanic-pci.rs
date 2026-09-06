//! Automatically rewritten from C to Rust
//! Source: drivers/misc/pvpanic/pvpanic-pci.c
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
// Pvpanic PCI Device Support
//
// Copyright (C) 2021 Oracle.
//

pub const PCI_DEVICE_ID_REDHAT_PVPANIC: c_uint = 0x0011;
    MODULE_AUTHOR("Mihai Carabas <mihai.carabas@oracle.com>");
    MODULE_DESCRIPTION("pvpanic device driver");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn pvpanic_pci_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int pvpanic_pci_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    void __iomem *base;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret < 0)
    return ret;
    base = pcim_iomap(pdev, 0, 0);
    if (!base)
    return -ENOMEM;
    return devm_pvpanic_probe(&pdev.dev, base);
    }
    static const struct pci_device_id pvpanic_pci_id_tbl[]  = {
    { PCI_DEVICE(PCI_VENDOR_ID_REDHAT, PCI_DEVICE_ID_REDHAT_PVPANIC) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, pvpanic_pci_id_tbl);
    static struct pci_driver pvpanic_pci_driver = {
    .name =         "pvpanic-pci",
    .id_table =     pvpanic_pci_id_tbl,
    .probe =        pvpanic_pci_probe,
    .dev_groups =   pvpanic_dev_groups,
    };
    module_pci_driver(pvpanic_pci_driver);
