//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/addi_apci_1516.c
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
// addi_apci_1516.c
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
// PCI bar 1 I/O Register map - Digital input/output
//
pub const APCI1516_DI_REG: c_uint = 0x00;
pub const APCI1516_DO_REG: c_uint = 0x04;
//
// PCI bar 2 I/O Register map - Watchdog (APCI-1516 and APCI-2016)
//
pub const APCI1516_WDOG_REG: c_uint = 0x00;
    enum apci1516_boardid {
    BOARD_APCI1016,
    BOARD_APCI1516,
    BOARD_APCI2016,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apci1516_boardinfo {
    pub name: *const c_char,
    pub di_nchan: c_int,
    pub do_nchan: c_int,
    pub has_wdog: c_int,
}

    static const struct apci1516_boardinfo apci1516_boardtypes[] = {
    [BOARD_APCI1016] = {
    .name		= "apci1016",
    .di_nchan	= 16,
    },
    [BOARD_APCI1516] = {
    .name		= "apci1516",
    .di_nchan	= 8,
    .do_nchan	= 8,
    .has_wdog	= 1,
    },
    [BOARD_APCI2016] = {
    .name		= "apci2016",
    .do_nchan	= 16,
    .has_wdog	= 1,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apci1516_private {
    pub wdog_iobase: c_ulong,
}

    static int apci1516_di_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    data[1] = inw(dev.iobase + APCI1516_DI_REG);
    return insn.n;
    }
    static int apci1516_do_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    s.state = inw(dev.iobase + APCI1516_DO_REG);
    if (comedi_dio_update_state(s, data))
    outw(s.state, dev.iobase + APCI1516_DO_REG);
    data[1] = s.state;
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn apci1516_reset(dev: *mut comedi_device) -> c_int {
    static int apci1516_reset(struct comedi_device *dev)
    {
    const struct apci1516_boardinfo *board = dev.board_ptr;
    struct apci1516_private *devpriv = dev.private;
    if (!board.has_wdog)
    return 0;
    outw(0x0, dev.iobase + APCI1516_DO_REG);
    addi_watchdog_reset(devpriv.wdog_iobase);
    return 0;
    }
    static int apci1516_auto_attach(struct comedi_device *dev,
    unsigned long context)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    const struct apci1516_boardinfo *board = core::ptr::null_mut();
    struct apci1516_private *devpriv;
    struct comedi_subdevice *s;
    int ret;
    if (context < ARRAY_SIZE(apci1516_boardtypes))
    board = &apci1516_boardtypes[context];
    if (!board)
    return -ENODEV;
    dev.board_ptr = board;
    dev.board_name = board.name;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    dev.iobase = pci_resource_start(pcidev, 1);
    devpriv.wdog_iobase = pci_resource_start(pcidev, 2);
    ret = comedi_alloc_subdevices(dev, 3);
    if (ret)
    return ret;
// Initialize the digital input subdevice
    s = &dev.subdevices[0];
    if (board.di_nchan) {
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= board.di_nchan;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= apci1516_di_insn_bits;
    } else {
    s.type		= COMEDI_SUBD_UNUSED;
    }
// Initialize the digital output subdevice
    s = &dev.subdevices[1];
    if (board.do_nchan) {
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= board.do_nchan;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= apci1516_do_insn_bits;
    } else {
    s.type		= COMEDI_SUBD_UNUSED;
    }
// Initialize the watchdog subdevice
    s = &dev.subdevices[2];
    if (board.has_wdog) {
    ret = addi_watchdog_init(s, devpriv.wdog_iobase);
    if (ret)
    return ret;
    } else {
    s.type		= COMEDI_SUBD_UNUSED;
    }
    apci1516_reset(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apci1516_detach(dev: *mut comedi_device) {
    static void apci1516_detach(struct comedi_device *dev)
    {
    if (dev.iobase)
    apci1516_reset(dev);
    comedi_pci_detach(dev);
    }
    static struct comedi_driver apci1516_driver = {
    .driver_name	= "addi_apci_1516",
    .module		= THIS_MODULE,
    .auto_attach	= apci1516_auto_attach,
    .detach		= apci1516_detach,
    };
    static int apci1516_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &apci1516_driver, id.driver_data);
    }
    static const struct pci_device_id apci1516_pci_table[] = {
    { PCI_VDEVICE(ADDIDATA, 0x1000), .driver_data = BOARD_APCI1016 },
    { PCI_VDEVICE(ADDIDATA, 0x1001), .driver_data = BOARD_APCI1516 },
    { PCI_VDEVICE(ADDIDATA, 0x1002), .driver_data = BOARD_APCI2016 },
    { }
    };
    MODULE_DEVICE_TABLE(pci, apci1516_pci_table);
    static struct pci_driver apci1516_pci_driver = {
    .name		= "addi_apci_1516",
    .id_table	= apci1516_pci_table,
    .probe		= apci1516_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(apci1516_driver, apci1516_pci_driver);
    MODULE_DESCRIPTION("ADDI-DATA APCI-1016/1516/2016, 16 channel DIO boards");
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_LICENSE("GPL");
