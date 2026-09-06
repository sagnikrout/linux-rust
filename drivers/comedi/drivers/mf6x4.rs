//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/mf6x4.c
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
// comedi/drivers/mf6x4.c
// Driver for Humusoft MF634 and MF624 Data acquisition cards
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//
// Driver: mf6x4
// Description: Humusoft MF634 and MF624 Data acquisition card driver
// Devices: [Humusoft] MF634 (mf634), MF624 (mf624)
// Author: Rostislav Lisovy <lisovy@gmail.com>
// Status: works
// Updated:
// Configuration Options: none
//

// Registers present in BAR0 memory region
pub const MF624_GPIOC_REG: c_uint = 0x54;

// BAR1 registers
pub const MF6X4_ADDATA_REG: c_uint = 0x00;
pub const MF6X4_ADCTRL_REG: c_uint = 0x00;

pub const MF6X4_DIN_REG: c_uint = 0x10;
pub const MF6X4_DIN_MASK: c_uint = 0xff;
pub const MF6X4_DOUT_REG: c_uint = 0x10;
pub const MF6X4_ADSTART_REG: c_uint = 0x20;

// BAR2 registers
pub const MF634_GPIOC_REG: c_uint = 0x68;
    enum mf6x4_boardid {
    BOARD_MF634,
    BOARD_MF624,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mf6x4_board {
    pub name: *const c_char,
// We need to keep track of the order of BARs used by the cards
    pub bar_nums: [c_uint; 3],
}

    static const struct mf6x4_board mf6x4_boards[] = {
    [BOARD_MF634] = {
    .name           = "mf634",
    .bar_nums	= {0, 2, 3},
    },
    [BOARD_MF624] = {
    .name           = "mf624",
    .bar_nums	= {0, 2, 4},
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mf6x4_private {
//
// Documentation for both MF634 and MF624 describes registers
// present in BAR0, 1 and 2 regions.
// The real (i.e. in HW) BAR numbers are different for MF624
// and MF634 yet we will call them 0, 1, 2
//
    pub bar0_mem: *mut void __iomem,
    pub bar2_mem: *mut void __iomem,
//
// This configuration register has the same function and fields
// for both cards however it lies in different BARs on different
// offsets -- this variable makes the access easier
//
    pub gpioc_reg: *mut void __iomem,
}

    static int mf6x4_di_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    data[1] = ioread16(dev.mmio + MF6X4_DIN_REG) & MF6X4_DIN_MASK;
    return insn.n;
    }
    static int mf6x4_do_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    if (comedi_dio_update_state(s, data))
    iowrite16(s.state, dev.mmio + MF6X4_DOUT_REG);
    data[1] = s.state;
    return insn.n;
    }
    static int mf6x4_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    struct mf6x4_private *devpriv = dev.private;
    unsigned int status;
// EOLC goes low at end of conversion.
    status = ioread32(devpriv.gpioc_reg);
    if ((status & MF6X4_GPIOC_EOLC) == 0)
    return 0;
    return -EBUSY;
    }
    static int mf6x4_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    unsigned int d;
    int ret;
    int i;
// Set the ADC channel number in the scan list
    iowrite16(MF6X4_ADCTRL_CHAN(chan), dev.mmio + MF6X4_ADCTRL_REG);
    for (i = 0; i < insn.n; i++) {
// Trigger ADC conversion by reading ADSTART
    ioread16(dev.mmio + MF6X4_ADSTART_REG);
    ret = comedi_timeout(dev, s, insn, mf6x4_ai_eoc, 0);
    if (ret)
    return ret;
// Read the actual value
    d = ioread16(dev.mmio + MF6X4_ADDATA_REG);
    d &= s.maxdata;
// munge the 2's complement data to offset binary
    data[i] = comedi_offset_munge(s, d);
    }
    iowrite16(0x0, dev.mmio + MF6X4_ADCTRL_REG);
    return insn.n;
    }
    static int mf6x4_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct mf6x4_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut val: c_uint = s.readback[chan];
    unsigned int gpioc;
    int i;
// Enable instantaneous update of converters outputs + Enable DACs
    gpioc = ioread32(devpriv.gpioc_reg);
    iowrite32((gpioc & ~MF6X4_GPIOC_LDAC) | MF6X4_GPIOC_DACEN,
    devpriv.gpioc_reg);
    for (i = 0; i < insn.n; i++) {
    val = data[i];
    iowrite16(val, dev.mmio + MF6X4_DAC_REG(chan));
    }
    s.readback[chan] = val;
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn mf6x4_auto_attach(dev: *mut comedi_device, context: c_ulong) -> c_int {
    static int mf6x4_auto_attach(struct comedi_device *dev, unsigned long context)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    const struct mf6x4_board *board = core::ptr::null_mut();
    struct mf6x4_private *devpriv;
    struct comedi_subdevice *s;
    int ret;
    if (context < ARRAY_SIZE(mf6x4_boards))
    board = &mf6x4_boards[context];
    else
    return -ENODEV;
    dev.board_ptr = board;
    dev.board_name = board.name;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    devpriv.bar0_mem = pci_ioremap_bar(pcidev, board.bar_nums[0]);
    if (!devpriv.bar0_mem)
    return -ENODEV;
    dev.mmio = pci_ioremap_bar(pcidev, board.bar_nums[1]);
    if (!dev.mmio)
    return -ENODEV;
    devpriv.bar2_mem = pci_ioremap_bar(pcidev, board.bar_nums[2]);
    if (!devpriv.bar2_mem)
    return -ENODEV;
    if (board == &mf6x4_boards[BOARD_MF634])
    devpriv.gpioc_reg = devpriv.bar2_mem + MF634_GPIOC_REG;
    else
    devpriv.gpioc_reg = devpriv.bar0_mem + MF624_GPIOC_REG;
    ret = comedi_alloc_subdevices(dev, 4);
    if (ret)
    return ret;
// Analog Input subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_GROUND;
    s.n_chan	= 8;
    s.maxdata	= 0x3fff;
    s.range_table	= &range_bipolar10;
    s.insn_read	= mf6x4_ai_insn_read;
// Analog Output subdevice
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 8;
    s.maxdata	= 0x3fff;
    s.range_table	= &range_bipolar10;
    s.insn_write	= mf6x4_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
// Digital Input subdevice
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= mf6x4_di_insn_bits;
// Digital Output subdevice
    s = &dev.subdevices[3];
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= mf6x4_do_insn_bits;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mf6x4_detach(dev: *mut comedi_device) {
    static void mf6x4_detach(struct comedi_device *dev)
    {
    struct mf6x4_private *devpriv = dev.private;
    if (devpriv) {
    if (devpriv.bar0_mem)
    iounmap(devpriv.bar0_mem);
    if (devpriv.bar2_mem)
    iounmap(devpriv.bar2_mem);
    }
    comedi_pci_detach(dev);
    }
    static struct comedi_driver mf6x4_driver = {
    .driver_name    = "mf6x4",
    .module         = THIS_MODULE,
    .auto_attach    = mf6x4_auto_attach,
    .detach         = mf6x4_detach,
    };
#[no_mangle]
unsafe extern "C" fn mf6x4_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int mf6x4_pci_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &mf6x4_driver, id.driver_data);
    }
    static const struct pci_device_id mf6x4_pci_table[] = {
    {
    PCI_VDEVICE(HUMUSOFT, 0x0634),
    .driver_data = BOARD_MF634,
    }, {
    PCI_VDEVICE(HUMUSOFT, 0x0624),
    .driver_data = BOARD_MF624,
    },
    { }
    };
    MODULE_DEVICE_TABLE(pci, mf6x4_pci_table);
    static struct pci_driver mf6x4_pci_driver = {
    .name           = "mf6x4",
    .id_table       = mf6x4_pci_table,
    .probe          = mf6x4_pci_probe,
    .remove         = comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(mf6x4_driver, mf6x4_pci_driver);
    MODULE_AUTHOR("Rostislav Lisovy <lisovy@gmail.com>");
    MODULE_DESCRIPTION("Comedi MF634 and MF624 DAQ cards driver");
    MODULE_LICENSE("GPL");
