//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-loongson-pci.c
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
// PCI interface driver for Loongson SPI Support
// Copyright (C) 2023 Loongson Technology Corporation Limited

    static int loongson_spi_pci_register(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    int ret;
    void __iomem *reg_base;
    struct device *dev = &pdev.dev;
    let mut pci_bar: c_int = 0;
    ret = pcim_enable_device(pdev);
    if (ret < 0)
    return dev_err_probe(dev, ret, "cannot enable pci device\n");
    reg_base = pcim_iomap_region(pdev, pci_bar, pci_name(pdev));
    ret = PTR_ERR_OR_ZERO(reg_base);
    if (ret)
    return dev_err_probe(dev, ret, "failed to request and remap memory\n");
    ret = loongson_spi_init_controller(dev, reg_base);
    if (ret)
    return dev_err_probe(dev, ret, "failed to initialize controller\n");
    return 0;
    }
    static struct pci_device_id loongson_spi_devices[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_LOONGSON, 0x7a0b) },
    { PCI_DEVICE(PCI_VENDOR_ID_LOONGSON, 0x7a1b) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, loongson_spi_devices);
    static struct pci_driver loongson_spi_pci_driver = {
    .name       = "loongson-spi-pci",
    .id_table   = loongson_spi_devices,
    .probe      = loongson_spi_pci_register,
    .driver	= {
    .bus = &pci_bus_type,
    .pm = pm_sleep_ptr(&loongson_spi_dev_pm_ops),
    },
    };
    module_pci_driver(loongson_spi_pci_driver);
    MODULE_DESCRIPTION("Loongson spi pci driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("SPI_LOONGSON_CORE");
