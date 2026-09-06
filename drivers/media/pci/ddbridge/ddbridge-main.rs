//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/ddbridge/ddbridge-main.c
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
// ddbridge.c: Digital Devices PCIe bridge driver
//
// Copyright (C) 2010-2017 Digital Devices GmbH
// Ralph Metzler <rjkm@metzlerbros.de>
// Marcus Metzler <mocm@metzlerbros.de>
//

//
// module parameters

    let mut msi: static int = 1;

    static int msi;

    module_param(msi, int, 0444);

    MODULE_PARM_DESC(msi, "Control MSI interrupts: 0-disable, 1-enable (default)");

    MODULE_PARM_DESC(msi, "Control MSI interrupts: 0-disable (default), 1-enable");

//
#[no_mangle]
unsafe extern "C" fn ddb_irq_disable(dev: *mut ddb) {
    static void ddb_irq_disable(struct ddb *dev)
    {
    ddbwritel(dev, 0, INTERRUPT_ENABLE);
    ddbwritel(dev, 0, MSI1_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn ddb_msi_exit(dev: *mut ddb) {
    static void ddb_msi_exit(struct ddb *dev)
    {

    if (dev.msi)
    pci_free_irq_vectors(dev.pdev);

    }
#[no_mangle]
unsafe extern "C" fn ddb_irq_exit(dev: *mut ddb) {
    static void ddb_irq_exit(struct ddb *dev)
    {
    ddb_irq_disable(dev);
    if (dev.msi == 2)
    free_irq(pci_irq_vector(dev.pdev, 1), dev);
    free_irq(pci_irq_vector(dev.pdev, 0), dev);
    }
#[no_mangle]
unsafe extern "C" fn ddb_remove(pdev: *mut pci_dev) {
    static void ddb_remove(struct pci_dev *pdev)
    {
    struct ddb *dev = (struct ddb *)pci_get_drvdata(pdev);
    ddb_device_destroy(dev);
    ddb_ports_detach(dev);
    ddb_i2c_release(dev);
    ddb_irq_exit(dev);
    ddb_msi_exit(dev);
    ddb_ports_release(dev);
    ddb_buffers_free(dev);
    ddb_unmap(dev);
    pci_set_drvdata(pdev, core::ptr::null_mut());
    pci_disable_device(pdev);
    }

#[no_mangle]
unsafe extern "C" fn ddb_irq_msi(dev: *mut ddb, nr: c_int) {
    static void ddb_irq_msi(struct ddb *dev, int nr)
    {
    int stat;
    if (msi && pci_msi_enabled()) {
    stat = pci_alloc_irq_vectors(dev.pdev, 1, nr,
    PCI_IRQ_MSI | PCI_IRQ_MSIX);
    if (stat >= 1) {
    dev.msi = stat;
    dev_info(dev.dev, "using %d MSI interrupt(s)\n",
    dev.msi);
    } else {
    dev_info(dev.dev, "MSI not available.\n");
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn ddb_irq_init(dev: *mut ddb) -> c_int {
    static int ddb_irq_init(struct ddb *dev)
    {
    int stat;
    let mut irq_flag: c_int = IRQF_SHARED;
    ddbwritel(dev, 0x00000000, INTERRUPT_ENABLE);
    ddbwritel(dev, 0x00000000, MSI1_ENABLE);
    ddbwritel(dev, 0x00000000, MSI2_ENABLE);
    ddbwritel(dev, 0x00000000, MSI3_ENABLE);
    ddbwritel(dev, 0x00000000, MSI4_ENABLE);
    ddbwritel(dev, 0x00000000, MSI5_ENABLE);
    ddbwritel(dev, 0x00000000, MSI6_ENABLE);
    ddbwritel(dev, 0x00000000, MSI7_ENABLE);

    ddb_irq_msi(dev, 2);
    if (dev.msi)
    irq_flag = 0;
    if (dev.msi == 2) {
    stat = request_irq(pci_irq_vector(dev.pdev, 0),
    ddb_irq_handler0, irq_flag, "ddbridge",
    (void *)dev);
    if (stat < 0)
    return stat;
    stat = request_irq(pci_irq_vector(dev.pdev, 1),
    ddb_irq_handler1, irq_flag, "ddbridge",
    (void *)dev);
    if (stat < 0) {
    free_irq(pci_irq_vector(dev.pdev, 0), dev);
    return stat;
    }
    } else

    {
    stat = request_irq(pci_irq_vector(dev.pdev, 0),
    ddb_irq_handler, irq_flag, "ddbridge",
    (void *)dev);
    if (stat < 0)
    return stat;
    }
    if (dev.msi == 2) {
    ddbwritel(dev, 0x0fffff00, INTERRUPT_ENABLE);
    ddbwritel(dev, 0x0000000f, MSI1_ENABLE);
    } else {
    ddbwritel(dev, 0x0fffff0f, INTERRUPT_ENABLE);
    ddbwritel(dev, 0x00000000, MSI1_ENABLE);
    }
    return stat;
    }
    static int ddb_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct ddb *dev;
    let mut stat: c_int = 0;
    if (pci_enable_device(pdev) < 0)
    return -ENODEV;
    pci_set_master(pdev);
    if (dma_set_mask(&pdev.dev, DMA_BIT_MASK(64)))
    if (dma_set_mask(&pdev.dev, DMA_BIT_MASK(32)))
    return -ENODEV;
    dev = vzalloc(sizeof(*dev));
    if (!dev)
    return -ENOMEM;
    mutex_init(&dev.mutex);
    dev.has_dma = 1;
    dev.pdev = pdev;
    dev.dev = &pdev.dev;
    pci_set_drvdata(pdev, dev);
    dev.link[0].ids.vendor = id.vendor;
    dev.link[0].ids.device = id.device;
    dev.link[0].ids.subvendor = id.subvendor;
    dev.link[0].ids.subdevice = pdev.subsystem_device;
    dev.link[0].ids.devid = (id.device << 16) | id.vendor;
    dev.link[0].dev = dev;
    dev.link[0].info = get_ddb_info(id.vendor, id.device,
    id.subvendor, pdev.subsystem_device);
    dev_info(&pdev.dev, "detected %s\n", dev.link[0].info.name);
    dev.regs_len = pci_resource_len(dev.pdev, 0);
    dev.regs = ioremap(pci_resource_start(dev.pdev, 0),
    pci_resource_len(dev.pdev, 0));
    if (!dev.regs) {
    dev_err(&pdev.dev, "not enough memory for register map\n");
    stat = -ENOMEM;
    goto fail;
    }
    if (ddbreadl(dev, 0) == 0xffffffff) {
    dev_err(&pdev.dev, "cannot read registers\n");
    stat = -ENODEV;
    goto fail;
    }
    dev.link[0].ids.hwid = ddbreadl(dev, 0);
    dev.link[0].ids.regmapid = ddbreadl(dev, 4);
    dev_info(&pdev.dev, "HW %08x REGMAP %08x\n",
    dev.link[0].ids.hwid, dev.link[0].ids.regmapid);
    ddbwritel(dev, 0, DMA_BASE_READ);
    ddbwritel(dev, 0, DMA_BASE_WRITE);
    stat = ddb_irq_init(dev);
    if (stat < 0)
    goto fail0;
    if (ddb_init(dev) == 0)
    return 0;
    ddb_irq_exit(dev);
    fail0:
    dev_err(&pdev.dev, "fail0\n");
    ddb_msi_exit(dev);
    fail:
    dev_err(&pdev.dev, "fail\n");
    ddb_unmap(dev);
    pci_set_drvdata(pdev, core::ptr::null_mut());
    pci_disable_device(pdev);
    return stat;
    }
//

    { PCI_DEVICE_SUB(DDVID, _device, DDVID, PCI_ANY_ID) }
    static const struct pci_device_id ddb_id_table[] = {
    DDB_DEVICE_ANY(0x0002),
    DDB_DEVICE_ANY(0x0003),
    DDB_DEVICE_ANY(0x0005),
    DDB_DEVICE_ANY(0x0006),
    DDB_DEVICE_ANY(0x0007),
    DDB_DEVICE_ANY(0x0008),
    DDB_DEVICE_ANY(0x0009),
    DDB_DEVICE_ANY(0x0011),
    DDB_DEVICE_ANY(0x0012),
    DDB_DEVICE_ANY(0x0013),
    DDB_DEVICE_ANY(0x0201),
    DDB_DEVICE_ANY(0x0203),
    DDB_DEVICE_ANY(0x0210),
    DDB_DEVICE_ANY(0x0220),
    DDB_DEVICE_ANY(0x0320),
    DDB_DEVICE_ANY(0x0321),
    DDB_DEVICE_ANY(0x0322),
    DDB_DEVICE_ANY(0x0323),
    DDB_DEVICE_ANY(0x0328),
    DDB_DEVICE_ANY(0x0329),
    { }
    };
    MODULE_DEVICE_TABLE(pci, ddb_id_table);
    static struct pci_driver ddb_pci_driver = {
    .name        = "ddbridge",
    .id_table    = ddb_id_table,
    .probe       = ddb_probe,
    .remove      = ddb_remove,
    };
#[no_mangle]
unsafe extern "C" fn module_init_ddbridge() -> __init int {
    static __init int module_init_ddbridge(void)
    {
    int stat;
    pr_info("Digital Devices PCIE bridge driver "
    DDBRIDGE_VERSION
    ", Copyright (C) 2010-17 Digital Devices GmbH\n");
    stat = ddb_init_ddbridge();
    if (stat < 0)
    return stat;
    stat = pci_register_driver(&ddb_pci_driver);
    if (stat < 0)
    ddb_exit_ddbridge(0, stat);
    return stat;
    }
#[no_mangle]
unsafe extern "C" fn module_exit_ddbridge() -> __exit void {
    static __exit void module_exit_ddbridge(void)
    {
    pci_unregister_driver(&ddb_pci_driver);
    ddb_exit_ddbridge(0, 0);
    }
    module_init(module_init_ddbridge);
    module_exit(module_exit_ddbridge);
    MODULE_DESCRIPTION("Digital Devices PCIe Bridge");
    MODULE_AUTHOR("Ralph and Marcus Metzler, Metzler Brothers Systementwicklung GbR");
    MODULE_LICENSE("GPL v2");
    MODULE_VERSION(DDBRIDGE_VERSION);
