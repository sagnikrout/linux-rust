//! Automatically rewritten from C to Rust
//! Source: drivers/uio/uio_aec.c
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
// uio_aec.c -- simple driver for Adrienne Electronics Corp time code PCI device
//
// Copyright (C) 2008 Brandon Philips <brandon@ifup.org>
//

pub const PCI_VENDOR_ID_AEC: c_uint = 0xaecb;
pub const PCI_DEVICE_ID_AEC_VITCLTC: c_uint = 0x6250;
pub const INT_ENABLE_ADDR: c_uint = 0xFC;
pub const INT_ENABLE: c_uint = 0x10;
pub const INT_DISABLE: c_uint = 0x0;
pub const INT_MASK_ADDR: c_uint = 0x2E;
pub const INT_MASK_ALL: c_uint = 0x3F;
pub const INTA_DRVR_ADDR: c_uint = 0xFE;
pub const INTA_ENABLED_FLAG: c_uint = 0x08;
pub const INTA_FLAG: c_uint = 0x01;
pub const MAILBOX: c_uint = 0x0F;
    static const struct pci_device_id ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_AEC, PCI_DEVICE_ID_AEC_VITCLTC), },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, ids);
#[no_mangle]
unsafe extern "C" fn aectc_irq(irq: c_int, dev_info: *mut uio_info) -> irqreturn_t {
    static irqreturn_t aectc_irq(int irq, struct uio_info *dev_info)
    {
    void __iomem *int_flag = dev_info.priv + INTA_DRVR_ADDR;
    let mut status: c_uchar = ioread8(int_flag);
    if ((status & INTA_ENABLED_FLAG) && (status & INTA_FLAG)) {
// application writes 0x00 to 0x2F to get next interrupt
    status = ioread8(dev_info.priv + MAILBOX);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn print_board_data(pdev: *mut pci_dev, i: *mut uio_info) {
    static void print_board_data(struct pci_dev *pdev, struct uio_info *i)
    {
    dev_info(&pdev.dev, "PCI-TC board vendor: %x%x number: %x%x"
    " revision: %c%c\n",
    ioread8(i.priv + 0x01),
    ioread8(i.priv + 0x00),
    ioread8(i.priv + 0x03),
    ioread8(i.priv + 0x02),
    ioread8(i.priv + 0x06),
    ioread8(i.priv + 0x07));
    }
#[no_mangle]
unsafe extern "C" fn probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct uio_info *info;
    int ret;
    info = devm_kzalloc(&pdev.dev, sizeof(struct uio_info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    if (pci_enable_device(pdev))
    return -ENODEV;
    if (pci_request_regions(pdev, "aectc"))
    goto out_disable;
    info.name = "aectc";
    info.port[0].start = pci_resource_start(pdev, 0);
    if (!info.port[0].start)
    goto out_release;
    info.priv = pci_iomap(pdev, 0, 0);
    if (!info.priv)
    goto out_release;
    info.port[0].size = pci_resource_len(pdev, 0);
    info.port[0].porttype = UIO_PORT_GPIO;
    info.version = "0.0.1";
    info.irq = pdev.irq;
    info.irq_flags = IRQF_SHARED;
    info.handler = aectc_irq;
    print_board_data(pdev, info);
    ret = uio_register_device(&pdev.dev, info);
    if (ret)
    goto out_unmap;
    iowrite32(INT_ENABLE, info.priv + INT_ENABLE_ADDR);
    iowrite8(INT_MASK_ALL, info.priv + INT_MASK_ADDR);
    if (!(ioread8(info.priv + INTA_DRVR_ADDR)
    & INTA_ENABLED_FLAG))
    dev_err(&pdev.dev, "aectc: interrupts not enabled\n");
    pci_set_drvdata(pdev, info);
    return 0;
    out_unmap:
    pci_iounmap(pdev, info.priv);
    out_release:
    pci_release_regions(pdev);
    out_disable:
    pci_disable_device(pdev);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn remove(pdev: *mut pci_dev) {
    static void remove(struct pci_dev *pdev)
    {
    struct uio_info *info = pci_get_drvdata(pdev);
// disable interrupts
    iowrite8(INT_DISABLE, info.priv + INT_MASK_ADDR);
    iowrite32(INT_DISABLE, info.priv + INT_ENABLE_ADDR);
// read mailbox to ensure board drops irq
    ioread8(info.priv + MAILBOX);
    uio_unregister_device(info);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    pci_iounmap(pdev, info.priv);
    }
    static struct pci_driver pci_driver = {
    .name = "aectc",
    .id_table = ids,
    .probe = probe,
    .remove = remove,
    };
    module_pci_driver(pci_driver);
    MODULE_DESCRIPTION("Adrienne Electronics Corp time code PCI device");
    MODULE_LICENSE("GPL");
