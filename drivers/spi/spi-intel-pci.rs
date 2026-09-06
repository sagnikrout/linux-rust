//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-intel-pci.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Intel PCH/PCU SPI flash PCI driver.
//
// Copyright (C) 2016 - 2022, Intel Corporation
// Author: Mika Westerberg <mika.westerberg@linux.intel.com>
//

pub const BCR: c_uint = 0xdc;

#[no_mangle]
unsafe extern "C" fn intel_spi_pci_set_writeable(base: *mut void __iomem, data: *mut c_void) -> bool {
    static bool intel_spi_pci_set_writeable(void __iomem *base, void *data)
    {
    struct pci_dev *pdev = data;
    u32 bcr;
// Try to make the chip read/write
    pci_read_config_dword(pdev, BCR, &bcr);
    if (!(bcr & BCR_WPD)) {
    bcr |= BCR_WPD;
    pci_write_config_dword(pdev, BCR, bcr);
    pci_read_config_dword(pdev, BCR, &bcr);
    }
    return bcr & BCR_WPD;
    }
    static const struct intel_spi_boardinfo bxt_info = {
    .type = INTEL_SPI_BXT,
    .set_writeable = intel_spi_pci_set_writeable,
    };
    static const struct intel_spi_boardinfo cnl_info = {
    .type = INTEL_SPI_CNL,
    .set_writeable = intel_spi_pci_set_writeable,
    };
    static int intel_spi_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct intel_spi_boardinfo *info;
    void __iomem *base;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    info = devm_kmemdup(&pdev.dev, (void *)id.driver_data, sizeof(*info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.data = pdev;
    base = pcim_iomap_region(pdev, 0, KBUILD_MODNAME);
    if (IS_ERR(base))
    return PTR_ERR(base);
    return intel_spi_probe(&pdev.dev, base, info);
    }
    static const struct pci_device_id intel_spi_pci_ids[] = {
    { PCI_VDEVICE(INTEL, 0x02a4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x06a4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x18e0), .driver_data = (unsigned long)&bxt_info },
    { PCI_VDEVICE(INTEL, 0x19e0), .driver_data = (unsigned long)&bxt_info },
    { PCI_VDEVICE(INTEL, 0x1bca), .driver_data = (unsigned long)&bxt_info },
    { PCI_VDEVICE(INTEL, 0x34a4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x38a4), .driver_data = (unsigned long)&bxt_info },
    { PCI_VDEVICE(INTEL, 0x43a4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x4b24), .driver_data = (unsigned long)&bxt_info },
    { PCI_VDEVICE(INTEL, 0x4d23), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x4da4), .driver_data = (unsigned long)&bxt_info },
    { PCI_VDEVICE(INTEL, 0x51a4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x54a4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x5794), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x5825), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x6e24), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x7723), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x7a24), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x7aa4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x7e23), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x7f24), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x9d24), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0x9da4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0xa0a4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0xa1a4), .driver_data = (unsigned long)&bxt_info },
    { PCI_VDEVICE(INTEL, 0xa224), .driver_data = (unsigned long)&bxt_info },
    { PCI_VDEVICE(INTEL, 0xa2a4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0xa324), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0xa3a4), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0xa823), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0xd323), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0xe323), .driver_data = (unsigned long)&cnl_info },
    { PCI_VDEVICE(INTEL, 0xe423), .driver_data = (unsigned long)&cnl_info },
    { },
    };
    MODULE_DEVICE_TABLE(pci, intel_spi_pci_ids);
    static struct pci_driver intel_spi_pci_driver = {
    .name = "intel-spi",
    .id_table = intel_spi_pci_ids,
    .probe = intel_spi_pci_probe,
    .dev_groups = intel_spi_groups,
    };
    module_pci_driver(intel_spi_pci_driver);
    MODULE_DESCRIPTION("Intel PCH/PCU SPI flash PCI driver");
    MODULE_AUTHOR("Mika Westerberg <mika.westerberg@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
