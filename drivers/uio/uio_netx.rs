//! Automatically rewritten from C to Rust
//! Source: drivers/uio/uio_netx.c
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
// UIO driver for Hilscher NetX based fieldbus cards (cifX, comX).
// See http://www.hilscher.com for details.
//
// (C) 2007 Hans J. Koch <hjk@hansjkoch.de>
// (C) 2008 Manuel Traut <manut@linutronix.de>
//

pub const PCI_VENDOR_ID_HILSCHER: c_uint = 0x15CF;
pub const PCI_DEVICE_ID_HILSCHER_NETX: c_uint = 0x0000;
pub const PCI_DEVICE_ID_HILSCHER_NETPLC: c_uint = 0x0010;
pub const PCI_SUBDEVICE_ID_NETPLC_RAM: c_uint = 0x0000;
pub const PCI_SUBDEVICE_ID_NETPLC_FLASH: c_uint = 0x0001;
pub const PCI_SUBDEVICE_ID_NXSB_PCA: c_uint = 0x3235;
pub const PCI_SUBDEVICE_ID_NXPCA: c_uint = 0x3335;
pub const DPM_HOST_INT_EN0: c_uint = 0xfff0;
pub const DPM_HOST_INT_STAT0: c_uint = 0xffe0;
pub const DPM_HOST_INT_MASK: c_uint = 0xe600ffff;
pub const DPM_HOST_INT_GLOBAL_EN: c_uint = 0x80000000;
#[no_mangle]
unsafe extern "C" fn netx_handler(irq: c_int, dev_info: *mut uio_info) -> irqreturn_t {
    static irqreturn_t netx_handler(int irq, struct uio_info *dev_info)
    {
    void __iomem *int_enable_reg = dev_info.mem[0].internal_addr
    + DPM_HOST_INT_EN0;
    void __iomem *int_status_reg = dev_info.mem[0].internal_addr
    + DPM_HOST_INT_STAT0;
// Is one of our interrupts enabled and active ?
    if (!(ioread32(int_enable_reg) & ioread32(int_status_reg)
    & DPM_HOST_INT_MASK))
    return IRQ_NONE;
// Disable interrupt
    iowrite32(ioread32(int_enable_reg) & ~DPM_HOST_INT_GLOBAL_EN,
    int_enable_reg);
    return IRQ_HANDLED;
    }
    static int netx_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    struct uio_info *info;
    int bar;
    info = devm_kzalloc(&dev.dev, sizeof(struct uio_info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    if (pci_enable_device(dev))
    return -ENODEV;
    if (pci_request_regions(dev, "netx"))
    goto out_disable;
    switch (id.device) {
    case PCI_DEVICE_ID_HILSCHER_NETX:
    bar = 0;
    info.name = "netx";
    break;
    case PCI_DEVICE_ID_HILSCHER_NETPLC:
    bar = 0;
    info.name = "netplc";
    break;
    default:
    bar = 2;
    info.name = "netx_plx";
    }
// BAR0 or 2 points to the card's dual port memory
    info.mem[0].addr = pci_resource_start(dev, bar);
    if (!info.mem[0].addr)
    goto out_release;
    info.mem[0].internal_addr = ioremap(pci_resource_start(dev, bar),
    pci_resource_len(dev, bar));
    if (!info.mem[0].internal_addr)
    goto out_release;
    info.mem[0].size = pci_resource_len(dev, bar);
    info.mem[0].memtype = UIO_MEM_PHYS;
    info.irq = dev.irq;
    info.irq_flags = IRQF_SHARED;
    info.handler = netx_handler;
    info.version = "0.0.1";
// Make sure all interrupts are disabled
    iowrite32(0, info.mem[0].internal_addr + DPM_HOST_INT_EN0);
    if (uio_register_device(&dev.dev, info))
    goto out_unmap;
    pci_set_drvdata(dev, info);
    dev_info(&dev.dev, "Found %s card, registered UIO device.\n",
    info.name);
    return 0;
    out_unmap:
    iounmap(info.mem[0].internal_addr);
    out_release:
    pci_release_regions(dev);
    out_disable:
    pci_disable_device(dev);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn netx_pci_remove(dev: *mut pci_dev) {
    static void netx_pci_remove(struct pci_dev *dev)
    {
    struct uio_info *info = pci_get_drvdata(dev);
// Disable all interrupts
    iowrite32(0, info.mem[0].internal_addr + DPM_HOST_INT_EN0);
    uio_unregister_device(info);
    pci_release_regions(dev);
    pci_disable_device(dev);
    iounmap(info.mem[0].internal_addr);
    }
    static const struct pci_device_id netx_pci_ids[] = {
    {
    .vendor =	PCI_VENDOR_ID_HILSCHER,
    .device =	PCI_DEVICE_ID_HILSCHER_NETX,
    .subvendor =	0,
    .subdevice =	0,
    },
    {
    .vendor =       PCI_VENDOR_ID_HILSCHER,
    .device =       PCI_DEVICE_ID_HILSCHER_NETPLC,
    .subvendor =    PCI_VENDOR_ID_HILSCHER,
    .subdevice =    PCI_SUBDEVICE_ID_NETPLC_RAM,
    },
    {
    .vendor =       PCI_VENDOR_ID_HILSCHER,
    .device =       PCI_DEVICE_ID_HILSCHER_NETPLC,
    .subvendor =    PCI_VENDOR_ID_HILSCHER,
    .subdevice =    PCI_SUBDEVICE_ID_NETPLC_FLASH,
    },
    {
    .vendor =	PCI_VENDOR_ID_PLX,
    .device =	PCI_DEVICE_ID_PLX_9030,
    .subvendor =	PCI_VENDOR_ID_PLX,
    .subdevice =	PCI_SUBDEVICE_ID_NXSB_PCA,
    },
    {
    .vendor =	PCI_VENDOR_ID_PLX,
    .device =	PCI_DEVICE_ID_PLX_9030,
    .subvendor =	PCI_VENDOR_ID_PLX,
    .subdevice =	PCI_SUBDEVICE_ID_NXPCA,
    },
    { 0, }
    };
    static struct pci_driver netx_pci_driver = {
    .name = "netx",
    .id_table = netx_pci_ids,
    .probe = netx_pci_probe,
    .remove = netx_pci_remove,
    };
    module_pci_driver(netx_pci_driver);
    MODULE_DEVICE_TABLE(pci, netx_pci_ids);
    MODULE_DESCRIPTION("UIO driver for Hilscher NetX based fieldbus cards");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Hans J. Koch, Manuel Traut");
