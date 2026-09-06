//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/aio_aio12_8.c
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
// aio_aio12_8.c
// Driver for ACCES I/O Products PC-104 AIO12-8 Analog I/O Board
// Copyright (C) 2006 C&C Technologies, Inc.
//
// Driver: aio_aio12_8
// Description: ACCES I/O Products PC-104 AIO12-8 Analog I/O Board
// Author: Pablo Mejia <pablo.mejia@cctechnol.com>
// Devices: [ACCES I/O] PC-104 AIO12-8 (aio_aio12_8),
// [ACCES I/O] PC-104 AI12-8 (aio_ai12_8),
// [ACCES I/O] PC-104 AO12-4 (aio_ao12_4)
// Status: experimental
//
// Configuration Options:
// [0] - I/O port base address
//
// Notes:
// Only synchronous operations are supported.
//

//
// Register map
//
pub const AIO12_8_STATUS_REG: c_uint = 0x00;

pub const AIO12_8_INTERRUPT_REG: c_uint = 0x01;

pub const AIO12_8_ADC_REG: c_uint = 0x02;

pub const AIO12_8_8254_BASE_REG: c_uint = 0x0c;
pub const AIO12_8_8255_BASE_REG: c_uint = 0x10;
pub const AIO12_8_DIO_CONTROL_REG: c_uint = 0x14;

pub const AIO12_8_ADC_TRIGGER_REG: c_uint = 0x15;

pub const AIO12_8_TRIGGER_REG: c_uint = 0x16;

pub const AIO12_8_COS_REG: c_uint = 0x17;
pub const AIO12_8_DAC_ENABLE_REG: c_uint = 0x18;

    static const struct comedi_lrange aio_aio12_8_range = {
    4, {
    UNI_RANGE(5),
    BIP_RANGE(5),
    UNI_RANGE(10),
    BIP_RANGE(10)
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aio12_8_boardtype {
    pub name: *const c_char,
    pub has_ai:1: c_uint,
    pub has_ao:1: c_uint,
}

    static const struct aio12_8_boardtype board_types[] = {
    {
    .name		= "aio_aio12_8",
    .has_ai		= 1,
    .has_ao		= 1,
    }, {
    .name		= "aio_ai12_8",
    .has_ai		= 1,
    }, {
    .name		= "aio_ao12_4",
    .has_ao		= 1,
    },
    };
    static int aio_aio12_8_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned int status;
    status = inb(dev.iobase + AIO12_8_STATUS_REG);
    if (status & AIO12_8_STATUS_ADC_EOC)
    return 0;
    return -EBUSY;
    }
    static int aio_aio12_8_ai_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut range: c_uint = CR_RANGE(insn.chanspec);
    unsigned int val;
    unsigned char control;
    int ret;
    int i;
//
// Setup the control byte for internal 2MHz clock, 3uS conversion,
// at the desired range of the requested channel.
//
    control = AIO12_8_ADC_MODE_NORMAL | AIO12_8_ADC_ACQ_3USEC |
    AIO12_8_ADC_RANGE(range) | AIO12_8_ADC_CHAN(chan);
// Read status to clear EOC latch
    inb(dev.iobase + AIO12_8_STATUS_REG);
    for (i = 0; i < insn.n; i++) {
// Setup and start conversion
    outb(control, dev.iobase + AIO12_8_ADC_REG);
// Wait for conversion to complete
    ret = comedi_timeout(dev, s, insn, aio_aio12_8_ai_eoc, 0);
    if (ret)
    return ret;
    val = inw(dev.iobase + AIO12_8_ADC_REG) & s.maxdata;
// munge bipolar 2's complement data to offset binary
    if (comedi_range_is_bipolar(s, range))
    val = comedi_offset_munge(s, val);
    data[i] = val;
    }
    return insn.n;
    }
    static int aio_aio12_8_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut val: c_uint = s.readback[chan];
    int i;
// enable DACs
    outb(AIO12_8_DAC_ENABLE_REF_ENA, dev.iobase + AIO12_8_DAC_ENABLE_REG);
    for (i = 0; i < insn.n; i++) {
    val = data[i];
    outw(val, dev.iobase + AIO12_8_DAC_REG(chan));
    }
    s.readback[chan] = val;
    return insn.n;
    }
    static int aio_aio12_8_counter_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    switch (data[0]) {
    case INSN_CONFIG_GET_CLOCK_SRC:
//
// Channels 0 and 2 have external clock sources.
// Channel 1 has a fixed 1 MHz clock source.
//
    data[0] = 0;
    data[1] = (chan == 1) ? I8254_OSC_BASE_1MHZ : 0;
    break;
    default:
    return -EINVAL;
    }
    return insn.n;
    }
    static int aio_aio12_8_attach(struct comedi_device *dev,
    struct comedi_devconfig *it)
    {
    const struct aio12_8_boardtype *board = dev.board_ptr;
    struct comedi_subdevice *s;
    int ret;
    ret = comedi_check_request_region(dev, it.options[0], 32,
    0x100, 0x3ff, 32);
    if (ret)
    return ret;
    dev.pacer = comedi_8254_io_alloc(dev.iobase + AIO12_8_8254_BASE_REG,
    0, I8254_IO8, 0);
    if (IS_ERR(dev.pacer))
    return PTR_ERR(dev.pacer);
    ret = comedi_alloc_subdevices(dev, 4);
    if (ret)
    return ret;
// Analog Input subdevice
    s = &dev.subdevices[0];
    if (board.has_ai) {
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_GROUND | SDF_DIFF;
    s.n_chan	= 8;
    s.maxdata	= 0x0fff;
    s.range_table	= &aio_aio12_8_range;
    s.insn_read	= aio_aio12_8_ai_read;
    } else {
    s.type = COMEDI_SUBD_UNUSED;
    }
// Analog Output subdevice
    s = &dev.subdevices[1];
    if (board.has_ao) {
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE | SDF_GROUND;
    s.n_chan	= 4;
    s.maxdata	= 0x0fff;
    s.range_table	= &aio_aio12_8_range;
    s.insn_write	= aio_aio12_8_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
    } else {
    s.type = COMEDI_SUBD_UNUSED;
    }
// Digital I/O subdevice (8255)
    s = &dev.subdevices[2];
    ret = subdev_8255_io_init(dev, s, AIO12_8_8255_BASE_REG);
    if (ret)
    return ret;
// Counter subdevice (8254)
    s = &dev.subdevices[3];
    comedi_8254_subdevice_init(s, dev.pacer);
    dev.pacer.insn_config = aio_aio12_8_counter_insn_config;
    return 0;
    }
    static struct comedi_driver aio_aio12_8_driver = {
    .driver_name	= "aio_aio12_8",
    .module		= THIS_MODULE,
    .attach		= aio_aio12_8_attach,
    .detach		= comedi_legacy_detach,
    .board_name	= &board_types[0].name,
    .num_names	= ARRAY_SIZE(board_types),
    .offset		= sizeof(struct aio12_8_boardtype),
    };
    module_comedi_driver(aio_aio12_8_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for ACCES I/O AIO12-8 Analog I/O Board");
    MODULE_LICENSE("GPL");
