//! Automatically rewritten from C to Rust
//! Source: drivers/misc/cardreader/alcor_pci.c
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
// Copyright (C) 2018 Oleksij Rempel <linux@rempel-privat.de>
//
// Driver for Alcor Micro AU6601 and AU6621 controllers
//

    static DEFINE_IDA(alcor_pci_idr);
    static struct mfd_cell alcor_pci_cells[] = {
    [ALCOR_SD_CARD] = {
    .name = DRV_NAME_ALCOR_PCI_SDMMC,
    },
    [ALCOR_MS_CARD] = {
    .name = DRV_NAME_ALCOR_PCI_MS,
    },
    };
    static const struct alcor_dev_cfg alcor_cfg = {
    .dma = 0,
    };
    static const struct alcor_dev_cfg au6621_cfg = {
    .dma = 1,
    };
    static const struct alcor_dev_cfg au6625_cfg = {
    .dma = 0,
    };
    static const struct pci_device_id pci_ids[] = {
    { PCI_DEVICE(PCI_ID_ALCOR_MICRO, PCI_ID_AU6601),
    .driver_data = (kernel_ulong_t)&alcor_cfg },
    { PCI_DEVICE(PCI_ID_ALCOR_MICRO, PCI_ID_AU6621),
    .driver_data = (kernel_ulong_t)&au6621_cfg },
    { PCI_DEVICE(PCI_ID_ALCOR_MICRO, PCI_ID_AU6625),
    .driver_data = (kernel_ulong_t)&au6625_cfg },
    { }
    };
    MODULE_DEVICE_TABLE(pci, pci_ids);
#[no_mangle]
pub unsafe extern "C" fn alcor_write8(priv: *mut alcor_pci_priv, val: u8, addr: c_uint) {
    void alcor_write8(struct alcor_pci_priv *priv, u8 val, unsigned int addr)
    {
    writeb(val, priv.iobase + addr);
    }
    EXPORT_SYMBOL_GPL(alcor_write8);
#[no_mangle]
pub unsafe extern "C" fn alcor_write16(priv: *mut alcor_pci_priv, val: u16, addr: c_uint) {
    void alcor_write16(struct alcor_pci_priv *priv, u16 val, unsigned int addr)
    {
    writew(val, priv.iobase + addr);
    }
    EXPORT_SYMBOL_GPL(alcor_write16);
#[no_mangle]
pub unsafe extern "C" fn alcor_write32(priv: *mut alcor_pci_priv, val: u32, addr: c_uint) {
    void alcor_write32(struct alcor_pci_priv *priv, u32 val, unsigned int addr)
    {
    writel(val, priv.iobase + addr);
    }
    EXPORT_SYMBOL_GPL(alcor_write32);
#[no_mangle]
pub unsafe extern "C" fn alcor_write32be(priv: *mut alcor_pci_priv, val: u32, addr: c_uint) {
    void alcor_write32be(struct alcor_pci_priv *priv, u32 val, unsigned int addr)
    {
    iowrite32be(val, priv.iobase + addr);
    }
    EXPORT_SYMBOL_GPL(alcor_write32be);
#[no_mangle]
pub unsafe extern "C" fn alcor_read8(priv: *mut alcor_pci_priv, addr: c_uint) -> u8 {
    u8 alcor_read8(struct alcor_pci_priv *priv, unsigned int addr)
    {
    return readb(priv.iobase + addr);
    }
    EXPORT_SYMBOL_GPL(alcor_read8);
#[no_mangle]
pub unsafe extern "C" fn alcor_read32(priv: *mut alcor_pci_priv, addr: c_uint) -> u32 {
    u32 alcor_read32(struct alcor_pci_priv *priv, unsigned int addr)
    {
    return readl(priv.iobase + addr);
    }
    EXPORT_SYMBOL_GPL(alcor_read32);
#[no_mangle]
pub unsafe extern "C" fn alcor_read32be(priv: *mut alcor_pci_priv, addr: c_uint) -> u32 {
    u32 alcor_read32be(struct alcor_pci_priv *priv, unsigned int addr)
    {
    return ioread32be(priv.iobase + addr);
    }
    EXPORT_SYMBOL_GPL(alcor_read32be);
    static int alcor_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct alcor_dev_cfg *cfg;
    struct alcor_pci_priv *priv;
    int ret, i, bar = 0;
    cfg = (void *)ent.driver_data;
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ret = ida_alloc(&alcor_pci_idr, GFP_KERNEL);
    if (ret < 0)
    return ret;
    priv.id = ret;
    priv.pdev = pdev;
    priv.parent_pdev = pdev.bus.self;
    priv.dev = &pdev.dev;
    priv.cfg = cfg;
    priv.irq = pdev.irq;
    ret = pcim_request_all_regions(pdev, DRV_NAME_ALCOR_PCI);
    if (ret) {
    dev_err(&pdev.dev, "Cannot request region\n");
    ret = -EBUSY;
    goto error_free_ida;
    }
    if (!(pci_resource_flags(pdev, bar) & IORESOURCE_MEM)) {
    dev_err(&pdev.dev, "BAR %d is not iomem. Aborting.\n", bar);
    ret = -ENODEV;
    goto error_free_ida;
    }
    priv.iobase = pcim_iomap(pdev, bar, 0);
    if (!priv.iobase) {
    ret = -ENOMEM;
    goto error_free_ida;
    }
// make sure irqs are disabled
    alcor_write32(priv, 0, AU6601_REG_INT_ENABLE);
    alcor_write32(priv, 0, AU6601_MS_INT_ENABLE);
    ret = dma_set_mask_and_coherent(priv.dev, AU6601_SDMA_MASK);
    if (ret) {
    dev_err(priv.dev, "Failed to set DMA mask\n");
    goto error_free_ida;
    }
    pci_set_master(pdev);
    pci_set_drvdata(pdev, priv);
    for (i = 0; i < ARRAY_SIZE(alcor_pci_cells); i++) {
    alcor_pci_cells[i].platform_data = priv;
    alcor_pci_cells[i].pdata_size = sizeof(*priv);
    }
    ret = mfd_add_devices(&pdev.dev, priv.id, alcor_pci_cells,
    ARRAY_SIZE(alcor_pci_cells), core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret < 0)
    goto error_clear_drvdata;
    pci_disable_link_state(pdev, PCIE_LINK_STATE_L0S | PCIE_LINK_STATE_L1);
    return 0;
    error_clear_drvdata:
    pci_clear_master(pdev);
    pci_set_drvdata(pdev, core::ptr::null_mut());
    error_free_ida:
    ida_free(&alcor_pci_idr, priv.id);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn alcor_pci_remove(pdev: *mut pci_dev) {
    static void alcor_pci_remove(struct pci_dev *pdev)
    {
    struct alcor_pci_priv *priv;
    priv = pci_get_drvdata(pdev);
    mfd_remove_devices(&pdev.dev);
    ida_free(&alcor_pci_idr, priv.id);
    pci_clear_master(pdev);
    pci_set_drvdata(pdev, core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn alcor_suspend(dev: *mut device) -> c_int {
    static int alcor_suspend(struct device *dev)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn alcor_resume(dev: *mut device) -> c_int {
    static int alcor_resume(struct device *dev)
    {
    struct alcor_pci_priv *priv = dev_get_drvdata(dev);
    pci_disable_link_state(priv.pdev,
    PCIE_LINK_STATE_L0S | PCIE_LINK_STATE_L1);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(alcor_pci_pm_ops, alcor_suspend, alcor_resume);
    static struct pci_driver alcor_driver = {
    .name	=	DRV_NAME_ALCOR_PCI,
    .id_table =	pci_ids,
    .probe	=	alcor_pci_probe,
    .remove =	alcor_pci_remove,
    .driver	=	{
    .pm	= &alcor_pci_pm_ops
    },
    };
    module_pci_driver(alcor_driver);
    MODULE_AUTHOR("Oleksij Rempel <linux@rempel-privat.de>");
    MODULE_DESCRIPTION("PCI driver for Alcor Micro AU6601 Secure Digital Host Controller Interface");
    MODULE_LICENSE("GPL");
