//! Automatically rewritten from C to Rust
//! Source: drivers/mcb/mcb-pci.c
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
// MEN Chameleon Bus.
//
// Copyright (C) 2014 MEN Mikroelektronik GmbH (www.men.de)
// Author: Johannes Thumshirn <johannes.thumshirn@men.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct priv {
    pub bus: *mut mcb_bus,
    pub mapbase: phys_addr_t,
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn mcb_pci_get_irq(mdev: *mut mcb_device) -> c_int {
    static int mcb_pci_get_irq(struct mcb_device *mdev)
    {
    struct mcb_bus *mbus = mdev.bus;
    struct device *dev = mbus.carrier;
    struct pci_dev *pdev = to_pci_dev(dev);
    return pdev.irq;
    }
#[no_mangle]
unsafe extern "C" fn mcb_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int mcb_pci_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct resource *res;
    struct priv *priv;
    int ret, table_size;
    unsigned long flags;
    priv = devm_kzalloc(&pdev.dev, sizeof(struct priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ret = pci_enable_device(pdev);
    if (ret) {
    dev_err(&pdev.dev, "Failed to enable PCI device\n");
    return -ENODEV;
    }
    pci_set_master(pdev);
    flags = pci_resource_flags(pdev, 0);
    if (flags & IORESOURCE_IO) {
    ret = -ENOTSUPP;
    dev_err(&pdev.dev,
    "IO mapped PCI devices are not supported\n");
    goto out_disable;
    }
    priv.mapbase = pci_resource_start(pdev, 0);
    if (!priv.mapbase) {
    dev_err(&pdev.dev, "No PCI resource\n");
    ret = -ENODEV;
    goto out_disable;
    }
    res = devm_request_mem_region(&pdev.dev, priv.mapbase,
    CHAM_HEADER_SIZE,
    KBUILD_MODNAME);
    if (!res) {
    dev_err(&pdev.dev, "Failed to request PCI memory\n");
    ret = -EBUSY;
    goto out_disable;
    }
    priv.base = devm_ioremap(&pdev.dev, priv.mapbase, CHAM_HEADER_SIZE);
    if (!priv.base) {
    dev_err(&pdev.dev, "Cannot ioremap\n");
    ret = -ENOMEM;
    goto out_disable;
    }
    pci_set_drvdata(pdev, priv);
    priv.bus = mcb_alloc_bus(&pdev.dev);
    if (IS_ERR(priv.bus)) {
    ret = PTR_ERR(priv.bus);
    goto out_disable;
    }
    priv.bus.get_irq = mcb_pci_get_irq;
    ret = chameleon_parse_cells(priv.bus, priv.mapbase, priv.base);
    if (ret < 0)
    goto out_mcb_bus;
    table_size = ret;
    if (table_size < CHAM_HEADER_SIZE) {
// Release the previous resources
    devm_iounmap(&pdev.dev, priv.base);
    devm_release_mem_region(&pdev.dev, priv.mapbase, CHAM_HEADER_SIZE);
// Then, allocate it again with the actual chameleon table size
    res = devm_request_mem_region(&pdev.dev, priv.mapbase,
    table_size,
    KBUILD_MODNAME);
    if (!res) {
    dev_err(&pdev.dev, "Failed to request PCI memory\n");
    ret = -EBUSY;
    goto out_mcb_bus;
    }
    priv.base = devm_ioremap(&pdev.dev, priv.mapbase, table_size);
    if (!priv.base) {
    dev_err(&pdev.dev, "Cannot ioremap\n");
    ret = -ENOMEM;
    goto out_mcb_bus;
    }
    }
    mcb_bus_add_devices(priv.bus);
    return 0;
    out_mcb_bus:
    mcb_release_bus(priv.bus);
    out_disable:
    pci_disable_device(pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mcb_pci_remove(pdev: *mut pci_dev) {
    static void mcb_pci_remove(struct pci_dev *pdev)
    {
    struct priv *priv = pci_get_drvdata(pdev);
    mcb_release_bus(priv.bus);
    pci_disable_device(pdev);
    }
    static const struct pci_device_id mcb_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_MEN, PCI_DEVICE_ID_MEN_CHAMELEON) },
    { PCI_DEVICE(PCI_VENDOR_ID_ALTERA, PCI_DEVICE_ID_MEN_CHAMELEON) },
    { 0 },
    };
    MODULE_DEVICE_TABLE(pci, mcb_pci_tbl);
    static struct pci_driver mcb_pci_driver = {
    .name = "mcb-pci",
    .id_table = mcb_pci_tbl,
    .probe = mcb_pci_probe,
    .remove = mcb_pci_remove,
    };
    module_pci_driver(mcb_pci_driver);
    MODULE_AUTHOR("Johannes Thumshirn <johannes.thumshirn@men.de>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("MCB over PCI support");
    MODULE_IMPORT_NS("MCB");
