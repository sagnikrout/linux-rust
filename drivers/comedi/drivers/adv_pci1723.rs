//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/adv_pci1723.c
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
// adv_pci1723.c
// Comedi driver for the Advantech PCI-1723 card.
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//
// Driver: adv_pci1723
// Description: Advantech PCI-1723
// Author: yonggang <rsmgnu@gmail.com>, Ian Abbott <abbotti@mev.co.uk>
// Devices: [Advantech] PCI-1723 (adv_pci1723)
// Updated: Mon, 14 Apr 2008 15:12:56 +0100
// Status: works
//
// Configuration Options: not applicable, uses comedi PCI auto config
//
// Subdevice 0 is 8-channel AO, 16-bit, range +/- 10 V.
//
// Subdevice 1 is 16-channel DIO.  The channels are configurable as
// input or output in 2 groups (0 to 7, 8 to 15). Configuring any
// channel implicitly configures all channels in the same group.
//
// TODO:
// 1. Add the two milliamp ranges to the AO subdevice (0 to 20 mA,
// 4 to 20 mA).
// 2. Read the initial ranges and values of the AO subdevice at
// start-up instead of reinitializing them.
// 3. Implement calibration.
//

//
// PCI Bar 2 I/O Register map (dev->iobase)
//

pub const PCI1723_BOARD_ID_REG: c_uint = 0x10;

pub const PCI1723_SYNC_CTRL_REG: c_uint = 0x12;

pub const PCI1723_CTRL_REG: c_uint = 0x14;

pub const PCI1723_CALIB_CTRL_REG: c_uint = 0x16;

pub const PCI1723_CALIB_STROBE_REG: c_uint = 0x18;
pub const PCI1723_DIO_CTRL_REG: c_uint = 0x1a;

pub const PCI1723_DIO_DATA_REG: c_uint = 0x1c;
pub const PCI1723_CALIB_DATA_REG: c_uint = 0x1e;
pub const PCI1723_SYNC_STROBE_REG: c_uint = 0x20;
pub const PCI1723_RESET_AO_STROBE_REG: c_uint = 0x22;
pub const PCI1723_RESET_CALIB_STROBE_REG: c_uint = 0x24;
pub const PCI1723_RANGE_STROBE_REG: c_uint = 0x26;
pub const PCI1723_VREF_REG: c_uint = 0x28;

    static int pci1723_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    int i;
    for (i = 0; i < insn.n; i++) {
    let mut val: c_uint = data[i];
    outw(val, dev.iobase + PCI1723_AO_REG(chan));
    s.readback[chan] = val;
    }
    return insn.n;
    }
    static int pci1723_dio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut mask: c_uint = (chan < 8) ? 0x00ff : 0xff00;
    unsigned short mode = 0x0000;		/* assume output */
    int ret;
    ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if (ret)
    return ret;
    if (!(s.io_bits & 0x00ff))
    mode |= PCI1723_DIO_CTRL_LDIO;	/* low byte input */
    if (!(s.io_bits & 0xff00))
    mode |= PCI1723_DIO_CTRL_HDIO;	/* high byte input */
    outw(mode, dev.iobase + PCI1723_DIO_CTRL_REG);
    return insn.n;
    }
    static int pci1723_dio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    if (comedi_dio_update_state(s, data))
    outw(s.state, dev.iobase + PCI1723_DIO_DATA_REG);
    data[1] = inw(dev.iobase + PCI1723_DIO_DATA_REG);
    return insn.n;
    }
    static int pci1723_auto_attach(struct comedi_device *dev,
    unsigned long context_unused)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    struct comedi_subdevice *s;
    unsigned int val;
    int ret;
    int i;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    dev.iobase = pci_resource_start(pcidev, 2);
    ret = comedi_alloc_subdevices(dev, 2);
    if (ret)
    return ret;
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE | SDF_GROUND | SDF_COMMON;
    s.n_chan	= 8;
    s.maxdata	= 0xffff;
    s.range_table	= &range_bipolar10;
    s.insn_write	= pci1723_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
// synchronously reset all analog outputs to 0V, +/-10V range
    outw(PCI1723_SYNC_CTRL_SYNC, dev.iobase + PCI1723_SYNC_CTRL_REG);
    for (i = 0; i < s.n_chan; i++) {
    outw(PCI1723_CTRL_RANGE(0) | PCI1723_CTRL_CHAN(i),
    PCI1723_CTRL_REG);
    outw(0, dev.iobase + PCI1723_RANGE_STROBE_REG);
    outw(0x8000, dev.iobase + PCI1723_AO_REG(i));
    s.readback[i] = 0x8000;
    }
    outw(0, dev.iobase + PCI1723_SYNC_STROBE_REG);
// disable syncronous control
    outw(PCI1723_SYNC_CTRL_ASYNC, dev.iobase + PCI1723_SYNC_CTRL_REG);
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_DIO;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 16;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_config	= pci1723_dio_insn_config;
    s.insn_bits	= pci1723_dio_insn_bits;
// get initial DIO direction and state
    val = inw(dev.iobase + PCI1723_DIO_CTRL_REG);
    if (!(val & PCI1723_DIO_CTRL_LDIO))
    s.io_bits |= 0x00ff;	/* low byte output */
    if (!(val & PCI1723_DIO_CTRL_HDIO))
    s.io_bits |= 0xff00;	/* high byte output */
    s.state = inw(dev.iobase + PCI1723_DIO_DATA_REG);
    return 0;
    }
    static struct comedi_driver adv_pci1723_driver = {
    .driver_name	= "adv_pci1723",
    .module		= THIS_MODULE,
    .auto_attach	= pci1723_auto_attach,
    .detach		= comedi_pci_detach,
    };
    static int adv_pci1723_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &adv_pci1723_driver,
    id.driver_data);
    }
    static const struct pci_device_id adv_pci1723_pci_table[] = {
    { PCI_VDEVICE(ADVANTECH, 0x1723) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, adv_pci1723_pci_table);
    static struct pci_driver adv_pci1723_pci_driver = {
    .name		= "adv_pci1723",
    .id_table	= adv_pci1723_pci_table,
    .probe		= adv_pci1723_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(adv_pci1723_driver, adv_pci1723_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Advantech PCI-1723 Comedi driver");
    MODULE_LICENSE("GPL");
