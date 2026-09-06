//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pci-host-generic.c
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
// Simple, generic PCI host controller driver targeting firmware-initialised
// systems and virtual machines (e.g. the PCI emulation provided by kvmtool).
//
// Copyright (C) 2014 ARM Limited
//
// Author: Will Deacon <will.deacon@arm.com>
//

#[no_mangle]
unsafe extern "C" fn pci_dw_valid_device(bus: *mut pci_bus, devfn: c_uint) -> bool {
    static bool pci_dw_valid_device(struct pci_bus *bus, unsigned int devfn)
    {
    struct pci_config_window *cfg = bus.sysdata;
//
// The Synopsys DesignWare PCIe controller in ECAM mode will not filter
// type 0 config TLPs sent to devices 1 and up on its downstream port,
// resulting in devices appearing multiple times on bus 0 unless we
// filter out those accesses here.
//
    if (bus.number == cfg.busr.start && PCI_SLOT(devfn) > 0)
    return false;
    return true;
    }
    static void __iomem *pci_dw_ecam_map_bus(struct pci_bus *bus,
    unsigned int devfn, int where)
    {
    if (!pci_dw_valid_device(bus, devfn))
    return core::ptr::null_mut();
    return pci_ecam_map_bus(bus, devfn, where);
    }
    static const struct pci_ecam_ops pci_dw_ecam_bus_ops = {
    .pci_ops	= {
    .map_bus	= pci_dw_ecam_map_bus,
    .read		= pci_generic_config_read,
    .write		= pci_generic_config_write,
    }
    };
    static const struct of_device_id gen_pci_of_match[] = {
    { .compatible = "pci-host-cam-generic",
    .data = &pci_generic_cam_ops },
    { .compatible = "pci-host-ecam-generic",
    .data = &pci_generic_ecam_ops },
    { .compatible = "marvell,armada8k-pcie-ecam",
    .data = &pci_dw_ecam_bus_ops },
    { .compatible = "socionext,synquacer-pcie-ecam",
    .data = &pci_dw_ecam_bus_ops },
    { .compatible = "snps,dw-pcie-ecam",
    .data = &pci_dw_ecam_bus_ops },
    { },
    };
    MODULE_DEVICE_TABLE(of, gen_pci_of_match);
    static struct platform_driver gen_pci_driver = {
    .driver = {
    .name = "pci-host-generic",
    .of_match_table = gen_pci_of_match,
    },
    .probe = pci_host_common_probe,
    .remove = pci_host_common_remove,
    };
    module_platform_driver(gen_pci_driver);
    MODULE_DESCRIPTION("Generic PCI host controller driver");
    MODULE_LICENSE("GPL v2");
