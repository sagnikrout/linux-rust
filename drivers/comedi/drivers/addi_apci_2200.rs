//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/addi_apci_2200.c
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
// addi_apci_2200.c
// Copyright (C) 2004,2005  ADDI-DATA GmbH for the source code of this module.
// Project manager: Eric Stolz
//
// ADDI-DATA GmbH
// Dieselstrasse 3
// D-77833 Ottersweier
// Tel: +19(0)7223/9493-0
// Fax: +49(0)7223/9493-92
// http://www.addi-data.com
// info@addi-data.com
//

//
// I/O Register Map
//
pub const APCI2200_DI_REG: c_uint = 0x00;
pub const APCI2200_DO_REG: c_uint = 0x04;
pub const APCI2200_WDOG_REG: c_uint = 0x08;
    static int apci2200_di_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    data[1] = inw(dev.iobase + APCI2200_DI_REG);
    return insn.n;
    }
    static int apci2200_do_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    s.state = inw(dev.iobase + APCI2200_DO_REG);
    if (comedi_dio_update_state(s, data))
    outw(s.state, dev.iobase + APCI2200_DO_REG);
    data[1] = s.state;
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn apci2200_reset(dev: *mut comedi_device) -> c_int {
    static int apci2200_reset(struct comedi_device *dev)
    {
    outw(0x0, dev.iobase + APCI2200_DO_REG);
    addi_watchdog_reset(dev.iobase + APCI2200_WDOG_REG);
    return 0;
    }
    static int apci2200_auto_attach(struct comedi_device *dev,
    unsigned long context_unused)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    struct comedi_subdevice *s;
    int ret;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    dev.iobase = pci_resource_start(pcidev, 1);
    ret = comedi_alloc_subdevices(dev, 3);
    if (ret)
    return ret;
// Initialize the digital input subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= apci2200_di_insn_bits;
// Initialize the digital output subdevice
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 16;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= apci2200_do_insn_bits;
// Initialize the watchdog subdevice
    s = &dev.subdevices[2];
    ret = addi_watchdog_init(s, dev.iobase + APCI2200_WDOG_REG);
    if (ret)
    return ret;
    apci2200_reset(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apci2200_detach(dev: *mut comedi_device) {
    static void apci2200_detach(struct comedi_device *dev)
    {
    if (dev.iobase)
    apci2200_reset(dev);
    comedi_pci_detach(dev);
    }
    static struct comedi_driver apci2200_driver = {
    .driver_name	= "addi_apci_2200",
    .module		= THIS_MODULE,
    .auto_attach	= apci2200_auto_attach,
    .detach		= apci2200_detach,
    };
    static int apci2200_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &apci2200_driver, id.driver_data);
    }
    static const struct pci_device_id apci2200_pci_table[] = {
    { PCI_VDEVICE(ADDIDATA, 0x1005) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, apci2200_pci_table);
    static struct pci_driver apci2200_pci_driver = {
    .name		= "addi_apci_2200",
    .id_table	= apci2200_pci_table,
    .probe		= apci2200_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(apci2200_driver, apci2200_pci_driver);
    MODULE_DESCRIPTION("ADDI-DATA APCI-2200 Relay board, optically isolated");
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_LICENSE("GPL");
