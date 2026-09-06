//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/adl_pci8164.c
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
// comedi/drivers/adl_pci8164.c
//
// Hardware comedi driver for PCI-8164 Adlink card
// Copyright (C) 2004 Michel Lachine <mike@mikelachaine.ca>
//
// Driver: adl_pci8164
// Description: Driver for the Adlink PCI-8164 4 Axes Motion Control board
// Devices: [ADLink] PCI-8164 (adl_pci8164)
// Author: Michel Lachaine <mike@mikelachaine.ca>
// Status: experimental
// Updated: Mon, 14 Apr 2008 15:10:32 +0100
//
// Configuration Options: not applicable, uses PCI auto config
//

pub const PCI8164_CMD_MSTS_REG: c_uint = 0x00;
pub const PCI8164_OTP_SSTS_REG: c_uint = 0x02;
pub const PCI8164_BUF0_REG: c_uint = 0x04;
pub const PCI8164_BUF1_REG: c_uint = 0x06;
    static int adl_pci8164_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut offset: c_ulong = (unsigned long)s.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    int i;
    for (i = 0; i < insn.n; i++)
    data[i] = inw(dev.iobase + PCI8164_AXIS(chan) + offset);
    return insn.n;
    }
    static int adl_pci8164_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut offset: c_ulong = (unsigned long)s.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    int i;
    for (i = 0; i < insn.n; i++)
    outw(data[i], dev.iobase + PCI8164_AXIS(chan) + offset);
    return insn.n;
    }
    static int adl_pci8164_auto_attach(struct comedi_device *dev,
    unsigned long context_unused)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    struct comedi_subdevice *s;
    int ret;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    dev.iobase = pci_resource_start(pcidev, 2);
    ret = comedi_alloc_subdevices(dev, 4);
    if (ret)
    return ret;
// read MSTS register / write CMD register for each axis (channel)
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_PROC;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 4;
    s.maxdata	= 0xffff;
    s.len_chanlist	= 4;
    s.insn_read	= adl_pci8164_insn_read;
    s.insn_write	= adl_pci8164_insn_write;
    s.private	= (void *)PCI8164_CMD_MSTS_REG;
// read SSTS register / write OTP register for each axis (channel)
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_PROC;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 4;
    s.maxdata	= 0xffff;
    s.len_chanlist	= 4;
    s.insn_read	= adl_pci8164_insn_read;
    s.insn_write	= adl_pci8164_insn_write;
    s.private	= (void *)PCI8164_OTP_SSTS_REG;
// read/write BUF0 register for each axis (channel)
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_PROC;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 4;
    s.maxdata	= 0xffff;
    s.len_chanlist	= 4;
    s.insn_read	= adl_pci8164_insn_read;
    s.insn_write	= adl_pci8164_insn_write;
    s.private	= (void *)PCI8164_BUF0_REG;
// read/write BUF1 register for each axis (channel)
    s = &dev.subdevices[3];
    s.type		= COMEDI_SUBD_PROC;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 4;
    s.maxdata	= 0xffff;
    s.len_chanlist	= 4;
    s.insn_read	= adl_pci8164_insn_read;
    s.insn_write	= adl_pci8164_insn_write;
    s.private	= (void *)PCI8164_BUF1_REG;
    return 0;
    }
    static struct comedi_driver adl_pci8164_driver = {
    .driver_name	= "adl_pci8164",
    .module		= THIS_MODULE,
    .auto_attach	= adl_pci8164_auto_attach,
    .detach		= comedi_pci_detach,
    };
    static int adl_pci8164_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &adl_pci8164_driver,
    id.driver_data);
    }
    static const struct pci_device_id adl_pci8164_pci_table[] = {
    { PCI_VDEVICE(ADLINK, 0x8164) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, adl_pci8164_pci_table);
    static struct pci_driver adl_pci8164_pci_driver = {
    .name		= "adl_pci8164",
    .id_table	= adl_pci8164_pci_table,
    .probe		= adl_pci8164_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(adl_pci8164_driver, adl_pci8164_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi low-level driver");
    MODULE_LICENSE("GPL");
