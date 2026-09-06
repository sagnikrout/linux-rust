//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/cb_das16_cs.c
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
// cb_das16_cs.c
// Driver for Computer Boards PC-CARD DAS16/16.
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000, 2001, 2002 David A. Schleef <ds@schleef.org>
//
// PCMCIA support code for this driver is adapted from the dummy_cs.c
// driver of the Linux PCMCIA Card Services package.
//
// The initial developer of the original code is David A. Hinds
// <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
// are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
//
// Driver: cb_das16_cs
// Description: Computer Boards PC-CARD DAS16/16
// Devices: [ComputerBoards] PC-CARD DAS16/16 (cb_das16_cs),
// PC-CARD DAS16/16-AO
// Author: ds
// Updated: Mon, 04 Nov 2002 20:04:21 -0800
// Status: experimental
//

//
// Register I/O map
//
pub const DAS16CS_AI_DATA_REG: c_uint = 0x00;
pub const DAS16CS_AI_MUX_REG: c_uint = 0x02;

    DAS16CS_AI_MUX_LO_CHAN(x))
pub const DAS16CS_MISC1_REG: c_uint = 0x04;

pub const DAS16CS_MISC2_REG: c_uint = 0x06;

pub const DAS16CS_TIMER_BASE: c_uint = 0x08;
pub const DAS16CS_DIO_REG: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct das16cs_board {
    pub name: *const c_char,
    pub device_id: c_int,
    pub has_ao:1: c_uint,
    pub has_4dio:1: c_uint,
}

    static const struct das16cs_board das16cs_boards[] = {
    {
    .name		= "PC-CARD DAS16/16-AO",
    .device_id	= 0x0039,
    .has_ao		= 1,
    .has_4dio	= 1,
    }, {
    .name		= "PCM-DAS16s/16",
    .device_id	= 0x4009,
    }, {
    .name		= "PC-CARD DAS16/16",
    .device_id	= 0x0000,	/* unknown */
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct das16cs_private {
    pub misc1: c_ushort,
    pub misc2: c_ushort,
}

    static const struct comedi_lrange das16cs_ai_range = {
    4, {
    BIP_RANGE(10),
    BIP_RANGE(5),
    BIP_RANGE(2.5),
    BIP_RANGE(1.25),
    }
    };
    static int das16cs_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned int status;
    status = inw(dev.iobase + DAS16CS_MISC1_REG);
    if (status & DAS16CS_MISC1_EOC)
    return 0;
    return -EBUSY;
    }
    static int das16cs_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct das16cs_private *devpriv = dev.private;
    let mut chan: c_int = CR_CHAN(insn.chanspec);
    let mut range: c_int = CR_RANGE(insn.chanspec);
    let mut aref: c_int = CR_AREF(insn.chanspec);
    int ret;
    int i;
    outw(DAS16CS_AI_MUX_SINGLE_CHAN(chan),
    dev.iobase + DAS16CS_AI_MUX_REG);
// disable interrupts, software convert
    devpriv.misc1 &= ~(DAS16CS_MISC1_INTE | DAS16CS_MISC1_INT_SRC_MASK |
    DAS16CS_MISC1_AI_CONV_MASK);
    if (aref == AREF_DIFF)
    devpriv.misc1 &= ~DAS16CS_MISC1_SEDIFF;
    else
    devpriv.misc1 |= DAS16CS_MISC1_SEDIFF;
    outw(devpriv.misc1, dev.iobase + DAS16CS_MISC1_REG);
    devpriv.misc2 &= ~(DAS16CS_MISC2_BME | DAS16CS_MISC2_AI_GAIN_MASK);
    switch (range) {
    case 0:
    devpriv.misc2 |= DAS16CS_MISC2_AI_GAIN_1;
    break;
    case 1:
    devpriv.misc2 |= DAS16CS_MISC2_AI_GAIN_2;
    break;
    case 2:
    devpriv.misc2 |= DAS16CS_MISC2_AI_GAIN_4;
    break;
    case 3:
    devpriv.misc2 |= DAS16CS_MISC2_AI_GAIN_8;
    break;
    }
    outw(devpriv.misc2, dev.iobase + DAS16CS_MISC2_REG);
    for (i = 0; i < insn.n; i++) {
    outw(0, dev.iobase + DAS16CS_AI_DATA_REG);
    ret = comedi_timeout(dev, s, insn, das16cs_ai_eoc, 0);
    if (ret)
    return ret;
    data[i] = inw(dev.iobase + DAS16CS_AI_DATA_REG);
    }
    return i;
    }
    static int das16cs_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct das16cs_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut val: c_uint = s.readback[chan];
    unsigned short misc1;
    int bit;
    int i;
    for (i = 0; i < insn.n; i++) {
    val = data[i];
    outw(devpriv.misc1, dev.iobase + DAS16CS_MISC1_REG);
    udelay(1);
// raise the DACxCS line for the non-selected channel
    misc1 = devpriv.misc1 & ~DAS16CS_MISC1_DAC_MASK;
    if (chan)
    misc1 |= DAS16CS_MISC1_DAC0CS;
    else
    misc1 |= DAS16CS_MISC1_DAC1CS;
    outw(misc1, dev.iobase + DAS16CS_MISC1_REG);
    udelay(1);
    for (bit = 15; bit >= 0; bit--) {
    if ((val >> bit) & 0x1)
    misc1 |= DAS16CS_MISC1_DACSD;
    else
    misc1 &= ~DAS16CS_MISC1_DACSD;
    outw(misc1, dev.iobase + DAS16CS_MISC1_REG);
    udelay(1);
    outw(misc1 | DAS16CS_MISC1_DACCLK,
    dev.iobase + DAS16CS_MISC1_REG);
    udelay(1);
    }
//
// Make both DAC0CS and DAC1CS high to load
// the new data and update analog the output
//
    outw(misc1 | DAS16CS_MISC1_DAC0CS | DAS16CS_MISC1_DAC1CS,
    dev.iobase + DAS16CS_MISC1_REG);
    }
    s.readback[chan] = val;
    return insn.n;
    }
    static int das16cs_dio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    if (comedi_dio_update_state(s, data))
    outw(s.state, dev.iobase + DAS16CS_DIO_REG);
    data[1] = inw(dev.iobase + DAS16CS_DIO_REG);
    return insn.n;
    }
    static int das16cs_dio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct das16cs_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    unsigned int mask;
    int ret;
    if (chan < 4)
    mask = 0x0f;
    else
    mask = 0xf0;
    ret = comedi_dio_insn_config(dev, s, insn, data, mask);
    if (ret)
    return ret;
    if (s.io_bits & 0xf0)
    devpriv.misc2 |= DAS16CS_MISC2_UDIR;
    else
    devpriv.misc2 &= ~DAS16CS_MISC2_UDIR;
    if (s.io_bits & 0x0f)
    devpriv.misc2 |= DAS16CS_MISC2_LDIR;
    else
    devpriv.misc2 &= ~DAS16CS_MISC2_LDIR;
    outw(devpriv.misc2, dev.iobase + DAS16CS_MISC2_REG);
    return insn.n;
    }
    static int das16cs_counter_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct das16cs_private *devpriv = dev.private;
    switch (data[0]) {
    case INSN_CONFIG_SET_CLOCK_SRC:
    switch (data[1]) {
    case 0:	/* internal 100 kHz */
    devpriv.misc2 |= DAS16CS_MISC2_CTR1;
    break;
    case 1:	/* external */
    devpriv.misc2 &= ~DAS16CS_MISC2_CTR1;
    break;
    default:
    return -EINVAL;
    }
    outw(devpriv.misc2, dev.iobase + DAS16CS_MISC2_REG);
    break;
    case INSN_CONFIG_GET_CLOCK_SRC:
    if (devpriv.misc2 & DAS16CS_MISC2_CTR1) {
    data[1] = 0;
    data[2] = I8254_OSC_BASE_100KHZ;
    } else {
    data[1] = 1;
    data[2] = 0;	/* unknown */
    }
    break;
    default:
    return -EINVAL;
    }
    return insn.n;
    }
    static const void *das16cs_find_boardinfo(struct comedi_device *dev,
    struct pcmcia_device *link)
    {
    const struct das16cs_board *board;
    int i;
    for (i = 0; i < ARRAY_SIZE(das16cs_boards); i++) {
    board = &das16cs_boards[i];
    if (board.device_id == link.card_id)
    return board;
    }
    return core::ptr::null_mut();
    }
    static int das16cs_auto_attach(struct comedi_device *dev,
    unsigned long context)
    {
    struct pcmcia_device *link = comedi_to_pcmcia_dev(dev);
    const struct das16cs_board *board;
    struct das16cs_private *devpriv;
    struct comedi_subdevice *s;
    int ret;
    board = das16cs_find_boardinfo(dev, link);
    if (!board)
    return -ENODEV;
    dev.board_ptr = board;
    dev.board_name = board.name;
    link.config_flags |= CONF_AUTO_SET_IO | CONF_ENABLE_IRQ;
    ret = comedi_pcmcia_enable(dev, core::ptr::null_mut());
    if (ret)
    return ret;
    dev.iobase = link.resource[0].start;
    link.priv = dev;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    dev.pacer = comedi_8254_io_alloc(dev.iobase + DAS16CS_TIMER_BASE,
    I8254_OSC_BASE_10MHZ, I8254_IO16, 0);
    if (IS_ERR(dev.pacer))
    return PTR_ERR(dev.pacer);
    ret = comedi_alloc_subdevices(dev, 4);
    if (ret)
    return ret;
// Analog Input subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_GROUND | SDF_DIFF;
    s.n_chan	= 16;
    s.maxdata	= 0xffff;
    s.range_table	= &das16cs_ai_range;
    s.insn_read	= das16cs_ai_insn_read;
// Analog Output subdevice
    s = &dev.subdevices[1];
    if (board.has_ao) {
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 2;
    s.maxdata	= 0xffff;
    s.range_table	= &range_bipolar10;
    s.insn_write	= &das16cs_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
    } else {
    s.type		= COMEDI_SUBD_UNUSED;
    }
// Digital I/O subdevice
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_DIO;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= board.has_4dio ? 4 : 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= das16cs_dio_insn_bits;
    s.insn_config	= das16cs_dio_insn_config;
// Counter subdevice (8254)
    s = &dev.subdevices[3];
    comedi_8254_subdevice_init(s, dev.pacer);
    dev.pacer.insn_config = das16cs_counter_insn_config;
// counters 1 and 2 are used internally for the pacer
    comedi_8254_set_busy(dev.pacer, 1, true);
    comedi_8254_set_busy(dev.pacer, 2, true);
    return 0;
    }
    static struct comedi_driver driver_das16cs = {
    .driver_name	= "cb_das16_cs",
    .module		= THIS_MODULE,
    .auto_attach	= das16cs_auto_attach,
    .detach		= comedi_pcmcia_disable,
    };
#[no_mangle]
unsafe extern "C" fn das16cs_pcmcia_attach(link: *mut pcmcia_device) -> c_int {
    static int das16cs_pcmcia_attach(struct pcmcia_device *link)
    {
    return comedi_pcmcia_auto_config(link, &driver_das16cs);
    }
    static const struct pcmcia_device_id das16cs_id_table[] = {
    PCMCIA_DEVICE_MANF_CARD(0x01c5, 0x0039),
    PCMCIA_DEVICE_MANF_CARD(0x01c5, 0x4009),
    PCMCIA_DEVICE_NULL
    };
    MODULE_DEVICE_TABLE(pcmcia, das16cs_id_table);
    static struct pcmcia_driver das16cs_driver = {
    .name		= "cb_das16_cs",
    .owner		= THIS_MODULE,
    .id_table	= das16cs_id_table,
    .probe		= das16cs_pcmcia_attach,
    .remove		= comedi_pcmcia_auto_unconfig,
    };
    module_comedi_pcmcia_driver(driver_das16cs, das16cs_driver);
    MODULE_AUTHOR("David A. Schleef <ds@schleef.org>");
    MODULE_DESCRIPTION("Comedi driver for Computer Boards PC-CARD DAS16/16");
    MODULE_LICENSE("GPL");
