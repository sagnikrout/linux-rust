//! Automatically rewritten from C to Rust
//! Source: drivers/uio/uio_cif.c
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
// UIO Hilscher CIF card driver
//
// (C) 2007 Hans J. Koch <hjk@hansjkoch.de>
// Original code (C) 2005 Benedikt Spranger <b.spranger@linutronix.de>
//

pub const PLX9030_INTCSR: c_uint = 0x4C;
pub const INTSCR_INT1_ENABLE: c_uint = 0x01;
pub const INTSCR_INT1_STATUS: c_uint = 0x04;

pub const PCI_SUBVENDOR_ID_PEP: c_uint = 0x1518;
pub const CIF_SUBDEVICE_PROFIBUS: c_uint = 0x430;
pub const CIF_SUBDEVICE_DEVICENET: c_uint = 0x432;
#[no_mangle]
unsafe extern "C" fn hilscher_handler(irq: c_int, dev_info: *mut uio_info) -> irqreturn_t {
    static irqreturn_t hilscher_handler(int irq, struct uio_info *dev_info)
    {
    void __iomem *plx_intscr = dev_info.mem[0].internal_addr
    + PLX9030_INTCSR;
    if ((ioread8(plx_intscr) & INT1_ENABLED_AND_ACTIVE)
    != INT1_ENABLED_AND_ACTIVE)
    return IRQ_NONE;
// Disable interrupt
    iowrite8(ioread8(plx_intscr) & ~INTSCR_INT1_ENABLE, plx_intscr);
    return IRQ_HANDLED;
    }
    static int hilscher_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    struct uio_info *info;
    info = devm_kzalloc(&dev.dev, sizeof(struct uio_info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    if (pci_enable_device(dev))
    return -ENODEV;
    if (pci_request_regions(dev, "hilscher"))
    goto out_disable;
    info.mem[0].addr = pci_resource_start(dev, 0);
    if (!info.mem[0].addr)
    goto out_release;
    info.mem[0].internal_addr = pci_ioremap_bar(dev, 0);
    if (!info.mem[0].internal_addr)
    goto out_release;
    info.mem[0].size = pci_resource_len(dev, 0);
    info.mem[0].memtype = UIO_MEM_PHYS;
    info.mem[1].addr = pci_resource_start(dev, 2);
    info.mem[1].size = pci_resource_len(dev, 2);
    info.mem[1].memtype = UIO_MEM_PHYS;
    switch (id.subdevice) {
    case CIF_SUBDEVICE_PROFIBUS:
    info.name = "CIF_Profibus";
    break;
    case CIF_SUBDEVICE_DEVICENET:
    info.name = "CIF_Devicenet";
    break;
    default:
    info.name = "CIF_???";
    }
    info.version = "0.0.1";
    info.irq = dev.irq;
    info.irq_flags = IRQF_SHARED;
    info.handler = hilscher_handler;
    if (uio_register_device(&dev.dev, info))
    goto out_unmap;
    pci_set_drvdata(dev, info);
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
unsafe extern "C" fn hilscher_pci_remove(dev: *mut pci_dev) {
    static void hilscher_pci_remove(struct pci_dev *dev)
    {
    struct uio_info *info = pci_get_drvdata(dev);
    uio_unregister_device(info);
    pci_release_regions(dev);
    pci_disable_device(dev);
    iounmap(info.mem[0].internal_addr);
    }
    static const struct pci_device_id hilscher_pci_ids[] = {
    {
    .vendor =	PCI_VENDOR_ID_PLX,
    .device =	PCI_DEVICE_ID_PLX_9030,
    .subvendor =	PCI_SUBVENDOR_ID_PEP,
    .subdevice =	CIF_SUBDEVICE_PROFIBUS,
    },
    {
    .vendor =	PCI_VENDOR_ID_PLX,
    .device =	PCI_DEVICE_ID_PLX_9030,
    .subvendor =	PCI_SUBVENDOR_ID_PEP,
    .subdevice =	CIF_SUBDEVICE_DEVICENET,
    },
    { 0, }
    };
    static struct pci_driver hilscher_pci_driver = {
    .name = "hilscher",
    .id_table = hilscher_pci_ids,
    .probe = hilscher_pci_probe,
    .remove = hilscher_pci_remove,
    };
    module_pci_driver(hilscher_pci_driver);
    MODULE_DEVICE_TABLE(pci, hilscher_pci_ids);
    MODULE_DESCRIPTION("UIO Hilscher CIF card driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Hans J. Koch, Benedikt Spranger");
