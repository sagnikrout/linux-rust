//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/vx855.c
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
// Linux multi-function-device driver (MFD) for the integrated peripherals
// of the VIA VX855 chipset
//
// Copyright (C) 2009 VIA Technologies, Inc.
// Copyright (C) 2010 One Laptop per Child
// Author: Harald Welte <HaraldWelte@viatech.com>
// All rights reserved.
//

// offset into pci config space indicating the 16bit register containing
// the power management IO space base
pub const VX855_CFG_PMIO_OFFSET: c_uint = 0x88;
// ACPI I/O Space registers
pub const VX855_PMIO_ACPI: c_uint = 0x00;
pub const VX855_PMIO_ACPI_LEN: c_uint = 0x0b;
// Processor Power Management
pub const VX855_PMIO_PPM: c_uint = 0x10;
pub const VX855_PMIO_PPM_LEN: c_uint = 0x08;
// General Purpose Power Management
pub const VX855_PMIO_GPPM: c_uint = 0x20;
pub const VX855_PMIO_R_GPI: c_uint = 0x48;
pub const VX855_PMIO_R_GPO: c_uint = 0x4c;
pub const VX855_PMIO_GPPM_LEN: c_uint = 0x33;
pub const VSPIC_MMIO_SIZE: c_uint = 0x1000;
    static struct resource vx855_gpio_resources[] = {
    {
    .flags = IORESOURCE_IO,
    },
    {
    .flags = IORESOURCE_IO,
    },
    };
    static const struct mfd_cell vx855_cells[] = {
    {
    .name = "vx855_gpio",
    .num_resources = ARRAY_SIZE(vx855_gpio_resources),
    .resources = vx855_gpio_resources,
// we must ignore resource conflicts, for reasons outlined in
// the vx855_gpio driver
    .ignore_resource_conflicts = true,
    },
    };
    static int vx855_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    int ret;
    u16 gpio_io_offset;
    ret = pci_enable_device(pdev);
    if (ret)
    return -ENODEV;
    pci_read_config_word(pdev, VX855_CFG_PMIO_OFFSET, &gpio_io_offset);
    if (!gpio_io_offset) {
    dev_warn(&pdev.dev,
    "BIOS did not assign PMIO base offset?!?\n");
    ret = -ENODEV;
    goto out;
    }
// mask out the lowest seven bits, as they are always zero, but
// hardware returns them as 0x01
    gpio_io_offset &= 0xff80;
// As the region identified here includes many non-GPIO things, we
// only work with the specific registers that concern us.
    vx855_gpio_resources[0].start = gpio_io_offset + VX855_PMIO_R_GPI;
    vx855_gpio_resources[0].end = vx855_gpio_resources[0].start + 3;
    vx855_gpio_resources[1].start = gpio_io_offset + VX855_PMIO_R_GPO;
    vx855_gpio_resources[1].end = vx855_gpio_resources[1].start + 3;
    ret = mfd_add_devices(&pdev.dev, -1, vx855_cells, ARRAY_SIZE(vx855_cells),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
// we always return -ENODEV here in order to enable other
// drivers like old, not-yet-platform_device ported i2c-viapro
    return -ENODEV;
    out:
    pci_disable_device(pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vx855_remove(pdev: *mut pci_dev) {
    static void vx855_remove(struct pci_dev *pdev)
    {
    mfd_remove_devices(&pdev.dev);
    pci_disable_device(pdev);
    }
    static const struct pci_device_id vx855_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_VX855) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, vx855_pci_tbl);
    static struct pci_driver vx855_pci_driver = {
    .name		= "vx855",
    .id_table	= vx855_pci_tbl,
    .probe		= vx855_probe,
    .remove		= vx855_remove,
    };
    module_pci_driver(vx855_pci_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Harald Welte <HaraldWelte@viatech.com>");
    MODULE_DESCRIPTION("Driver for the VIA VX855 chipset");
