//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel_scu_pcidrv.c
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
// PCI driver for the Intel SCU.
//
// Copyright (C) 2008-2010, 2015, 2020 Intel Corporation
// Authors: Sreedhara DS (sreedhara.ds@intel.com)
// Mika Westerberg <mika.westerberg@linux.intel.com>
//

    static int intel_scu_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    let mut scu_data: intel_scu_ipc_data = {};
    struct intel_scu_ipc_dev *scu;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    scu_data.mem = pdev.resource[0];
    scu_data.irq = pdev.irq;
    scu = intel_scu_ipc_register(&pdev.dev, &scu_data);
    return PTR_ERR_OR_ZERO(scu);
    }
    static const struct pci_device_id pci_ids[] = {
    { PCI_VDEVICE(INTEL, 0x080e) },
    { PCI_VDEVICE(INTEL, 0x082a) },
    { PCI_VDEVICE(INTEL, 0x08ea) },
    { PCI_VDEVICE(INTEL, 0x0a94) },
    { PCI_VDEVICE(INTEL, 0x11a0) },
    { PCI_VDEVICE(INTEL, 0x1a94) },
    { PCI_VDEVICE(INTEL, 0x5a94) },
    {}
    };
    static struct pci_driver intel_scu_pci_driver = {
    .driver = {
    .suppress_bind_attrs = true,
    },
    .name = "intel_scu",
    .id_table = pci_ids,
    .probe = intel_scu_pci_probe,
    };
    builtin_pci_driver(intel_scu_pci_driver);
