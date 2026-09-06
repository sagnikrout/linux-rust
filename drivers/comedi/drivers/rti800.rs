//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/rti800.c
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
// comedi/drivers/rti800.c
// Hardware driver for Analog Devices RTI-800/815 board
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1998 David A. Schleef <ds@schleef.org>
//
// Driver: rti800
// Description: Analog Devices RTI-800/815
// Devices: [Analog Devices] RTI-800 (rti800), RTI-815 (rti815)
// Author: David A. Schleef <ds@schleef.org>
// Status: unknown
// Updated: Fri, 05 Sep 2008 14:50:44 +0100
//
// Configuration options:
// [0] - I/O port base address
// [1] - IRQ (not supported / unused)
// [2] - A/D mux/reference (number of channels)
// 0 = differential
// 1 = pseudodifferential (common)
// 2 = single-ended
// [3] - A/D range
// 0 = [-10,10]
// 1 = [-5,5]
// 2 = [0,10]
// [4] - A/D encoding
// 0 = two's complement
// 1 = straight binary
// [5] - DAC 0 range
// 0 = [-10,10]
// 1 = [0,10]
// [6] - DAC 0 encoding
// 0 = two's complement
// 1 = straight binary
// [7] - DAC 1 range (same as DAC 0)
// [8] - DAC 1 encoding (same as DAC 0)
//

//
// Register map
//
pub const RTI800_CSR: c_uint = 0x00;

pub const RTI800_MUXGAIN: c_uint = 0x01;
pub const RTI800_CONVERT: c_uint = 0x02;
pub const RTI800_ADCLO: c_uint = 0x03;
pub const RTI800_ADCHI: c_uint = 0x04;
pub const RTI800_DAC0LO: c_uint = 0x05;
pub const RTI800_DAC0HI: c_uint = 0x06;
pub const RTI800_DAC1LO: c_uint = 0x07;
pub const RTI800_DAC1HI: c_uint = 0x08;
pub const RTI800_CLRFLAGS: c_uint = 0x09;
pub const RTI800_DI: c_uint = 0x0a;
pub const RTI800_DO: c_uint = 0x0b;
pub const RTI800_9513A_DATA: c_uint = 0x0c;
pub const RTI800_9513A_CNTRL: c_uint = 0x0d;
pub const RTI800_9513A_STATUS: c_uint = 0x0d;
    static const struct comedi_lrange range_rti800_ai_10_bipolar = {
    4, {
    BIP_RANGE(10),
    BIP_RANGE(1),
    BIP_RANGE(0.1),
    BIP_RANGE(0.02)
    }
    };
    static const struct comedi_lrange range_rti800_ai_5_bipolar = {
    4, {
    BIP_RANGE(5),
    BIP_RANGE(0.5),
    BIP_RANGE(0.05),
    BIP_RANGE(0.01)
    }
    };
    static const struct comedi_lrange range_rti800_ai_unipolar = {
    4, {
    UNI_RANGE(10),
    UNI_RANGE(1),
    UNI_RANGE(0.1),
    UNI_RANGE(0.02)
    }
    };
    static const struct comedi_lrange *const rti800_ai_ranges[] = {
    &range_rti800_ai_10_bipolar,
    &range_rti800_ai_5_bipolar,
    &range_rti800_ai_unipolar,
    };
    static const struct comedi_lrange *const rti800_ao_ranges[] = {
    &range_bipolar10,
    &range_unipolar10,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rti800_board {
    pub name: *const c_char,
    pub has_ao: c_int,
}

    static const struct rti800_board rti800_boardtypes[] = {
    {
    .name		= "rti800",
    }, {
    .name		= "rti815",
    .has_ao		= 1,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rti800_private {
    pub adc_2comp: bool,
    pub dac_2comp: [bool; 2],
    pub ao_range_type_list: [*const comedi_lrange; 2],
    pub muxgain_bits: c_uchar,
}

    static int rti800_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned char status;
    status = inb(dev.iobase + RTI800_CSR);
    if (status & RTI800_CSR_OVERRUN) {
    outb(0, dev.iobase + RTI800_CLRFLAGS);
    return -EOVERFLOW;
    }
    if (status & RTI800_CSR_DONE)
    return 0;
    return -EBUSY;
    }
    static int rti800_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct rti800_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut gain: c_uint = CR_RANGE(insn.chanspec);
    unsigned char muxgain_bits;
    int ret;
    int i;
    inb(dev.iobase + RTI800_ADCHI);
    outb(0, dev.iobase + RTI800_CLRFLAGS);
    muxgain_bits = chan | (gain << 5);
    if (muxgain_bits != devpriv.muxgain_bits) {
    devpriv.muxgain_bits = muxgain_bits;
    outb(devpriv.muxgain_bits, dev.iobase + RTI800_MUXGAIN);
//
// Without a delay here, the RTI_CSR_OVERRUN bit
// gets set, and you will have an error.
//
    if (insn.n > 0) {
    int delay = (gain == 0) ? 10 :
    (gain == 1) ? 20 :
    (gain == 2) ? 40 : 80;
    udelay(delay);
    }
    }
    for (i = 0; i < insn.n; i++) {
    unsigned int val;
    outb(0, dev.iobase + RTI800_CONVERT);
    ret = comedi_timeout(dev, s, insn, rti800_ai_eoc, 0);
    if (ret)
    return ret;
    val = inb(dev.iobase + RTI800_ADCLO);
    val |= (inb(dev.iobase + RTI800_ADCHI) & 0xf) << 8;
    if (devpriv.adc_2comp)
    val = comedi_offset_munge(s, val);
    data[i] = val;
    }
    return insn.n;
    }
    static int rti800_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct rti800_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut reg_lo: c_int = chan ? RTI800_DAC1LO : RTI800_DAC0LO;
    let mut reg_hi: c_int = chan ? RTI800_DAC1HI : RTI800_DAC0HI;
    int i;
    for (i = 0; i < insn.n; i++) {
    let mut val: c_uint = data[i];
    s.readback[chan] = val;
    if (devpriv.dac_2comp[chan])
    val = comedi_offset_munge(s, val);
    outb(val & 0xff, dev.iobase + reg_lo);
    outb((val >> 8) & 0xff, dev.iobase + reg_hi);
    }
    return insn.n;
    }
    static int rti800_di_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    data[1] = inb(dev.iobase + RTI800_DI);
    return insn.n;
    }
    static int rti800_do_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    if (comedi_dio_update_state(s, data)) {
// Outputs are inverted...
    outb(s.state ^ 0xff, dev.iobase + RTI800_DO);
    }
    data[1] = s.state;
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn rti800_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    static int rti800_attach(struct comedi_device *dev, struct comedi_devconfig *it)
    {
    const struct rti800_board *board = dev.board_ptr;
    struct rti800_private *devpriv;
    struct comedi_subdevice *s;
    int ret;
    ret = comedi_check_request_region(dev, it.options[0], 0x10,
    0, 0x3ff, 16);
    if (ret)
    return ret;
    outb(0, dev.iobase + RTI800_CSR);
    inb(dev.iobase + RTI800_ADCHI);
    outb(0, dev.iobase + RTI800_CLRFLAGS);
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    devpriv.adc_2comp = (it.options[4] == 0);
    devpriv.dac_2comp[0] = (it.options[6] == 0);
    devpriv.dac_2comp[1] = (it.options[8] == 0);
// invalid, forces the MUXGAIN register to be set when first used
    devpriv.muxgain_bits = 0xff;
    ret = comedi_alloc_subdevices(dev, 4);
    if (ret)
    return ret;
    s = &dev.subdevices[0];
// ai subdevice
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_GROUND;
    s.n_chan	= (it.options[2] ? 16 : 8);
    s.insn_read	= rti800_ai_insn_read;
    s.maxdata	= 0x0fff;
    s.range_table	= (it.options[3] < ARRAY_SIZE(rti800_ai_ranges))
    ? rti800_ai_ranges[it.options[3]]
    : &range_unknown;
    s = &dev.subdevices[1];
    if (board.has_ao) {
// ao subdevice (only on rti815)
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 2;
    s.maxdata	= 0x0fff;
    s.range_table_list = devpriv.ao_range_type_list;
    devpriv.ao_range_type_list[0] =
    (it.options[5] < ARRAY_SIZE(rti800_ao_ranges))
    ? rti800_ao_ranges[it.options[5]]
    : &range_unknown;
    devpriv.ao_range_type_list[1] =
    (it.options[7] < ARRAY_SIZE(rti800_ao_ranges))
    ? rti800_ao_ranges[it.options[7]]
    : &range_unknown;
    s.insn_write	= rti800_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
    } else {
    s.type		= COMEDI_SUBD_UNUSED;
    }
    s = &dev.subdevices[2];
// di
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= 8;
    s.insn_bits	= rti800_di_insn_bits;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s = &dev.subdevices[3];
// do
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 8;
    s.insn_bits	= rti800_do_insn_bits;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
//
// There is also an Am9513 timer on these boards. This subdevice
// is not currently supported.
//
    return 0;
    }
    static struct comedi_driver rti800_driver = {
    .driver_name	= "rti800",
    .module		= THIS_MODULE,
    .attach		= rti800_attach,
    .detach		= comedi_legacy_detach,
    .num_names	= ARRAY_SIZE(rti800_boardtypes),
    .board_name	= &rti800_boardtypes[0].name,
    .offset		= sizeof(struct rti800_board),
    };
    module_comedi_driver(rti800_driver);
    MODULE_DESCRIPTION("Comedi: RTI-800 Multifunction Analog/Digital board");
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_LICENSE("GPL");
