//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/ni_at_ao.c
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
// ni_at_ao.c
// Driver for NI AT-AO-6/10 boards
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000,2002 David A. Schleef <ds@schleef.org>
//
// Driver: ni_at_ao
// Description: National Instruments AT-AO-6/10
// Devices: [National Instruments] AT-AO-6 (at-ao-6), AT-AO-10 (at-ao-10)
// Status: should work
// Author: David A. Schleef <ds@schleef.org>
// Updated: Sun Dec 26 12:26:28 EST 2004
//
// Configuration options:
// [0] - I/O port base address
// [1] - IRQ (unused)
// [2] - DMA (unused)
// [3] - analog output range, set by jumpers on hardware
// 0 for -10 to 10V bipolar
// 1 for 0V to 10V unipolar
//

//
// Register map
//
// Register-level programming information can be found in NI
// document 320379.pdf.
//
pub const ATAO_DIO_REG: c_uint = 0x00;
pub const ATAO_CFG2_REG: c_uint = 0x02;

pub const ATAO_CFG3_REG: c_uint = 0x04;

pub const ATAO_82C53_BASE: c_uint = 0x06;
pub const ATAO_CFG1_REG: c_uint = 0x0a;

pub const ATAO_STATUS_REG: c_uint = 0x0a;

pub const ATAO_FIFO_WRITE_REG: c_uint = 0x0c;
pub const ATAO_FIFO_CLEAR_REG: c_uint = 0x0c;

// registers with _2_ are accessed when GRP2WR is set in CFG1
pub const ATAO_2_DMATCCLR_REG: c_uint = 0x00;
pub const ATAO_2_INT1CLR_REG: c_uint = 0x02;
pub const ATAO_2_INT2CLR_REG: c_uint = 0x04;
pub const ATAO_2_RTSISHFT_REG: c_uint = 0x06;

pub const ATAO_2_RTSISTRB_REG: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atao_board {
    pub name: *const c_char,
    pub n_ao_chans: c_int,
}

    static const struct atao_board atao_boards[] = {
    {
    .name		= "at-ao-6",
    .n_ao_chans	= 6,
    }, {
    .name		= "at-ao-10",
    .n_ao_chans	= 10,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atao_private {
    pub cfg1: c_ushort,
    pub cfg3: c_ushort,
// Used for caldac readback
    pub caldac: [c_uchar; 21],
}

#[no_mangle]
unsafe extern "C" fn atao_select_reg_group(dev: *mut comedi_device, group: c_int) {
    static void atao_select_reg_group(struct comedi_device *dev, int group)
    {
    struct atao_private *devpriv = dev.private;
    if (group)
    devpriv.cfg1 |= ATAO_CFG1_GRP2WR;
    else
    devpriv.cfg1 &= ~ATAO_CFG1_GRP2WR;
    outw(devpriv.cfg1, dev.iobase + ATAO_CFG1_REG);
    }
    static int atao_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut val: c_uint = s.readback[chan];
    int i;
    if (chan == 0)
    atao_select_reg_group(dev, 1);
    for (i = 0; i < insn.n; i++) {
    val = data[i];
// the hardware expects two's complement values
    outw(comedi_offset_munge(s, val),
    dev.iobase + ATAO_AO_REG(chan));
    }
    s.readback[chan] = val;
    if (chan == 0)
    atao_select_reg_group(dev, 0);
    return insn.n;
    }
    static int atao_dio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    if (comedi_dio_update_state(s, data))
    outw(s.state, dev.iobase + ATAO_DIO_REG);
    data[1] = inw(dev.iobase + ATAO_DIO_REG);
    return insn.n;
    }
    static int atao_dio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct atao_private *devpriv = dev.private;
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
    if (s.io_bits & 0x0f)
    devpriv.cfg3 |= ATAO_CFG3_DOUTEN1;
    else
    devpriv.cfg3 &= ~ATAO_CFG3_DOUTEN1;
    if (s.io_bits & 0xf0)
    devpriv.cfg3 |= ATAO_CFG3_DOUTEN2;
    else
    devpriv.cfg3 &= ~ATAO_CFG3_DOUTEN2;
    outw(devpriv.cfg3, dev.iobase + ATAO_CFG3_REG);
    return insn.n;
    }
//
// There are three DAC8800 TrimDACs on the board. These are 8-channel,
// 8-bit DACs that are used to calibrate the Analog Output channels.
// The factory default calibration values are stored in the EEPROM.
// The TrimDACs, and EEPROM addresses, are mapped as:
//
// Channel       EEPROM  Description
// -----------------  ------  -----------------------------------
// 0 - DAC0 Chan 0    0x30   AO Channel 0 Offset
// 1 - DAC0 Chan 1    0x31   AO Channel 0 Gain
// 2 - DAC0 Chan 2    0x32   AO Channel 1 Offset
// 3 - DAC0 Chan 3    0x33   AO Channel 1 Gain
// 4 - DAC0 Chan 4    0x34   AO Channel 2 Offset
// 5 - DAC0 Chan 5    0x35   AO Channel 2 Gain
// 6 - DAC0 Chan 6    0x36   AO Channel 3 Offset
// 7 - DAC0 Chan 7    0x37   AO Channel 3 Gain
// 8 - DAC1 Chan 0    0x38   AO Channel 4 Offset
// 9 - DAC1 Chan 1    0x39   AO Channel 4 Gain
// 10 - DAC1 Chan 2    0x3a   AO Channel 5 Offset
// 11 - DAC1 Chan 3    0x3b   AO Channel 5 Gain
// 12 - DAC1 Chan 4    0x3c   2.5V Offset
// 13 - DAC1 Chan 5    0x3d   AO Channel 6 Offset (at-ao-10 only)
// 14 - DAC1 Chan 6    0x3e   AO Channel 6 Gain   (at-ao-10 only)
// 15 - DAC1 Chan 7    0x3f   AO Channel 7 Offset (at-ao-10 only)
// 16 - DAC2 Chan 0    0x40   AO Channel 7 Gain   (at-ao-10 only)
// 17 - DAC2 Chan 1    0x41   AO Channel 8 Offset (at-ao-10 only)
// 18 - DAC2 Chan 2    0x42   AO Channel 8 Gain   (at-ao-10 only)
// 19 - DAC2 Chan 3    0x43   AO Channel 9 Offset (at-ao-10 only)
// 20 - DAC2 Chan 4    0x44   AO Channel 9 Gain   (at-ao-10 only)
// DAC2 Chan 5    0x45   Reserved
// DAC2 Chan 6    0x46   Reserved
// DAC2 Chan 7    0x47   Reserved
//
    static int atao_calib_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    if (insn.n) {
    let mut val: c_uint = data[insn.n - 1];
    let mut bitstring: c_uint = ((chan & 0x7) << 8) | val;
    unsigned int bits;
    int bit;
// write the channel and last data value to the caldac
// clock the bitstring to the caldac; MSB -> LSB
    for (bit = BIT(10); bit; bit >>= 1) {
    bits = (bit & bitstring) ? ATAO_CFG2_SDATA : 0;
    outw(bits, dev.iobase + ATAO_CFG2_REG);
    outw(bits | ATAO_CFG2_SCLK,
    dev.iobase + ATAO_CFG2_REG);
    }
// strobe the caldac to load the value
    outw(ATAO_CFG2_CALLD(chan), dev.iobase + ATAO_CFG2_REG);
    outw(ATAO_CFG2_CALLD_NOP, dev.iobase + ATAO_CFG2_REG);
    s.readback[chan] = val;
    }
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn atao_reset(dev: *mut comedi_device) {
    static void atao_reset(struct comedi_device *dev)
    {
    struct atao_private *devpriv = dev.private;
// This is the reset sequence described in the manual
    devpriv.cfg1 = 0;
    outw(devpriv.cfg1, dev.iobase + ATAO_CFG1_REG);
// Put outputs of counter 1 and counter 2 in a high state
    comedi_8254_set_mode(dev.pacer, 0, I8254_MODE4 | I8254_BINARY);
    comedi_8254_set_mode(dev.pacer, 1, I8254_MODE4 | I8254_BINARY);
    comedi_8254_write(dev.pacer, 0, 0x0003);
    outw(ATAO_CFG2_CALLD_NOP, dev.iobase + ATAO_CFG2_REG);
    devpriv.cfg3 = 0;
    outw(devpriv.cfg3, dev.iobase + ATAO_CFG3_REG);
    inw(dev.iobase + ATAO_FIFO_CLEAR_REG);
    atao_select_reg_group(dev, 1);
    outw(0, dev.iobase + ATAO_2_INT1CLR_REG);
    outw(0, dev.iobase + ATAO_2_INT2CLR_REG);
    outw(0, dev.iobase + ATAO_2_DMATCCLR_REG);
    atao_select_reg_group(dev, 0);
    }
#[no_mangle]
unsafe extern "C" fn atao_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    static int atao_attach(struct comedi_device *dev, struct comedi_devconfig *it)
    {
    const struct atao_board *board = dev.board_ptr;
    struct atao_private *devpriv;
    struct comedi_subdevice *s;
    int ret;
    ret = comedi_check_request_region(dev, it.options[0], 0x20,
    0, 0x3ff, 32);
    if (ret)
    return ret;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    dev.pacer = comedi_8254_io_alloc(dev.iobase + ATAO_82C53_BASE,
    0, I8254_IO8, 0);
    if (IS_ERR(dev.pacer))
    return PTR_ERR(dev.pacer);
    ret = comedi_alloc_subdevices(dev, 4);
    if (ret)
    return ret;
// Analog Output subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= board.n_ao_chans;
    s.maxdata	= 0x0fff;
    s.range_table	= it.options[3] ? &range_unipolar10 : &range_bipolar10;
    s.insn_write	= atao_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
// Digital I/O subdevice
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_DIO;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= atao_dio_insn_bits;
    s.insn_config	= atao_dio_insn_config;
// caldac subdevice
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_CALIB;
    s.subdev_flags	= SDF_WRITABLE | SDF_INTERNAL;
    s.n_chan	= (board.n_ao_chans * 2) + 1;
    s.maxdata	= 0xff;
    s.insn_write	= atao_calib_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
// EEPROM subdevice
    s = &dev.subdevices[3];
    s.type		= COMEDI_SUBD_UNUSED;
    atao_reset(dev);
    return 0;
    }
    static struct comedi_driver ni_at_ao_driver = {
    .driver_name	= "ni_at_ao",
    .module		= THIS_MODULE,
    .attach		= atao_attach,
    .detach		= comedi_legacy_detach,
    .board_name	= &atao_boards[0].name,
    .offset		= sizeof(struct atao_board),
    .num_names	= ARRAY_SIZE(atao_boards),
    };
    module_comedi_driver(ni_at_ao_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for NI AT-AO-6/10 boards");
    MODULE_LICENSE("GPL");
