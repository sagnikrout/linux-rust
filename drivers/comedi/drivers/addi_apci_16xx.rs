//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/addi_apci_16xx.c
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
// addi_apci_16xx.c
// Copyright (C) 2004,2005  ADDI-DATA GmbH for the source code of this module.
// Project manager: S. Weber
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
// Register I/O map
//

    enum apci16xx_boardid {
    BOARD_APCI1648,
    BOARD_APCI1696,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apci16xx_boardinfo {
    pub name: *const c_char,
    pub n_chan: c_int,
}

    static const struct apci16xx_boardinfo apci16xx_boardtypes[] = {
    [BOARD_APCI1648] = {
    .name		= "apci1648",
    .n_chan		= 48,		/* 2 subdevices */
    },
    [BOARD_APCI1696] = {
    .name		= "apci1696",
    .n_chan		= 96,		/* 3 subdevices */
    },
    };
    static int apci16xx_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    unsigned int mask;
    int ret;
    if (chan < 8)
    mask = 0x000000ff;
#[no_mangle]
pub unsafe extern "C" fn if(16: chan <) -> else {
    else if (chan < 16)
    mask = 0x0000ff00;
#[no_mangle]
pub unsafe extern "C" fn if(24: chan <) -> else {
    else if (chan < 24)
    mask = 0x00ff0000;
    else
    mask = 0xff000000;
    ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if (ret)
    return ret;
    outl(s.io_bits, dev.iobase + APCI16XX_DIR_REG(s.index));
    return insn.n;
    }
    static int apci16xx_dio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    if (comedi_dio_update_state(s, data))
    outl(s.state, dev.iobase + APCI16XX_OUT_REG(s.index));
    data[1] = inl(dev.iobase + APCI16XX_IN_REG(s.index));
    return insn.n;
    }
    static int apci16xx_auto_attach(struct comedi_device *dev,
    unsigned long context)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    const struct apci16xx_boardinfo *board = core::ptr::null_mut();
    struct comedi_subdevice *s;
    unsigned int n_subdevs;
    unsigned int last;
    int i;
    int ret;
    if (context < ARRAY_SIZE(apci16xx_boardtypes))
    board = &apci16xx_boardtypes[context];
    if (!board)
    return -ENODEV;
    dev.board_ptr = board;
    dev.board_name = board.name;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    dev.iobase = pci_resource_start(pcidev, 0);
//
// Work out the number of subdevices needed to support all the
// digital i/o channels on the board. Each subdevice supports
// up to 32 channels.
//
    n_subdevs = board.n_chan / 32;
    if ((n_subdevs * 32) < board.n_chan) {
    last = board.n_chan - (n_subdevs * 32);
    n_subdevs++;
    } else {
    last = 0;
    }
    ret = comedi_alloc_subdevices(dev, n_subdevs);
    if (ret)
    return ret;
// Initialize the TTL digital i/o subdevices
    for (i = 0; i < n_subdevs; i++) {
    s = &dev.subdevices[i];
    s.type		= COMEDI_SUBD_DIO;
    s.subdev_flags	= SDF_WRITABLE | SDF_READABLE;
    s.n_chan	= ((i * 32) < board.n_chan) ? 32 : last;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_config	= apci16xx_insn_config;
    s.insn_bits	= apci16xx_dio_insn_bits;
// Default all channels to inputs
    s.io_bits	= 0;
    outl(s.io_bits, dev.iobase + APCI16XX_DIR_REG(i));
    }
    return 0;
    }
    static struct comedi_driver apci16xx_driver = {
    .driver_name	= "addi_apci_16xx",
    .module		= THIS_MODULE,
    .auto_attach	= apci16xx_auto_attach,
    .detach		= comedi_pci_detach,
    };
    static int apci16xx_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &apci16xx_driver, id.driver_data);
    }
    static const struct pci_device_id apci16xx_pci_table[] = {
    { PCI_VDEVICE(ADDIDATA, 0x1009), .driver_data = BOARD_APCI1648 },
    { PCI_VDEVICE(ADDIDATA, 0x100a), .driver_data = BOARD_APCI1696 },
    { }
    };
    MODULE_DEVICE_TABLE(pci, apci16xx_pci_table);
    static struct pci_driver apci16xx_pci_driver = {
    .name		= "addi_apci_16xx",
    .id_table	= apci16xx_pci_table,
    .probe		= apci16xx_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(apci16xx_driver, apci16xx_pci_driver);
    MODULE_DESCRIPTION("ADDI-DATA APCI-1648/1696, TTL I/O boards");
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_LICENSE("GPL");
