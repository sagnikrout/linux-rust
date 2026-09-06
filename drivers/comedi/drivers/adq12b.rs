//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/adq12b.c
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
// adq12b.c
// Driver for MicroAxial ADQ12-B data acquisition and control card
// written by jeremy theler <thelerg@ib.cnea.gov.ar>
// instituto balseiro
// commission nacional de energia atomica
// universidad nacional de cuyo
// argentina
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//
// Driver: adq12b
// Description: Driver for MicroAxial ADQ12-B data acquisition and control card
// Devices: [MicroAxial] ADQ12-B (adq12b)
// Author: jeremy theler <thelerg@ib.cnea.gov.ar>
// Updated: Thu, 21 Feb 2008 02:56:27 -0300
// Status: works
//
// Configuration options:
// [0] - I/O base address (set with hardware jumpers)
// address		jumper JADR
// 0x300		1 (factory default)
// 0x320		2
// 0x340		3
// 0x360		4
// 0x380		5
// 0x3A0		6
// [1] - Analog Input unipolar/bipolar selection
// selection	option	JUB
// bipolar		0	2-3 (factory default)
// unipolar	1	1-2
// [2] - Analog Input single-ended/differential selection
// selection	option	JCHA	JCHB
// single-ended	0	1-2	1-2 (factory default)
// differential	1	2-3	2-3
//
// Driver for the acquisition card ADQ12-B (without any add-on).
//
// - Analog input is subdevice 0 (16 channels single-ended or 8 differential)
// - Digital input is subdevice 1 (5 channels)
// - Digital output is subdevice 1 (8 channels)
// - The PACER is not supported in this version
//

// address scheme (page 2.17 of the manual)
pub const ADQ12B_CTREG: c_uint = 0x00;

pub const ADQ12B_STINR: c_uint = 0x00;

pub const ADQ12B_OUTBR: c_uint = 0x04;
pub const ADQ12B_ADLOW: c_uint = 0x08;
pub const ADQ12B_ADHIG: c_uint = 0x09;
pub const ADQ12B_TIMER_BASE: c_uint = 0x0c;
// available ranges through the PGA gains
    static const struct comedi_lrange range_adq12b_ai_bipolar = {
    4, {
    BIP_RANGE(5),
    BIP_RANGE(2),
    BIP_RANGE(1),
    BIP_RANGE(0.5)
    }
    };
    static const struct comedi_lrange range_adq12b_ai_unipolar = {
    4, {
    UNI_RANGE(5),
    UNI_RANGE(2),
    UNI_RANGE(1),
    UNI_RANGE(0.5)
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adq12b_private {
    pub last_ctreg: c_uint,
}

    static int adq12b_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned char status;
    status = inb(dev.iobase + ADQ12B_STINR);
    if (status & ADQ12B_STINR_EOC)
    return 0;
    return -EBUSY;
    }
    static int adq12b_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct adq12b_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut range: c_uint = CR_RANGE(insn.chanspec);
    unsigned int val;
    int ret;
    int i;
// change channel and range only if it is different from the previous
    val = ADQ12B_CTREG_RANGE(range) | ADQ12B_CTREG_CHAN(chan);
    if (val != devpriv.last_ctreg) {
    outb(val, dev.iobase + ADQ12B_CTREG);
    devpriv.last_ctreg = val;
    usleep_range(50, 100);	/* wait for the mux to settle */
    }
    val = inb(dev.iobase + ADQ12B_ADLOW);	/* trigger A/D */
    for (i = 0; i < insn.n; i++) {
    ret = comedi_timeout(dev, s, insn, adq12b_ai_eoc, 0);
    if (ret)
    return ret;
    val = inb(dev.iobase + ADQ12B_ADHIG) << 8;
    val |= inb(dev.iobase + ADQ12B_ADLOW);	/* retriggers A/D */
    data[i] = val;
    }
    return insn.n;
    }
    static int adq12b_di_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn, unsigned int *data)
    {
// only bits 0-4 have information about digital inputs
    data[1] = (inb(dev.iobase + ADQ12B_STINR) & ADQ12B_STINR_IN_MASK);
    return insn.n;
    }
    static int adq12b_do_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    unsigned int mask;
    unsigned int chan;
    unsigned int val;
    mask = comedi_dio_update_state(s, data);
    if (mask) {
    for (chan = 0; chan < 8; chan++) {
    if ((mask >> chan) & 0x01) {
    val = (s.state >> chan) & 0x01;
    outb((val << 3) | chan,
    dev.iobase + ADQ12B_OUTBR);
    }
    }
    }
    data[1] = s.state;
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn adq12b_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    static int adq12b_attach(struct comedi_device *dev, struct comedi_devconfig *it)
    {
    struct adq12b_private *devpriv;
    struct comedi_subdevice *s;
    int ret;
    ret = comedi_check_request_region(dev, it.options[0], 0x10,
    0x300, 0x3af, 0x20);
    if (ret)
    return ret;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    devpriv.last_ctreg = -1;	/* force ctreg update */
    ret = comedi_alloc_subdevices(dev, 3);
    if (ret)
    return ret;
// Analog Input subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AI;
    if (it.options[2]) {
    s.subdev_flags	= SDF_READABLE | SDF_DIFF;
    s.n_chan	= 8;
    } else {
    s.subdev_flags	= SDF_READABLE | SDF_GROUND;
    s.n_chan	= 16;
    }
    s.maxdata	= 0xfff;
    s.range_table	= it.options[1] ? &range_adq12b_ai_unipolar
    : &range_adq12b_ai_bipolar;
    s.insn_read	= adq12b_ai_insn_read;
// Digital Input subdevice
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= 5;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= adq12b_di_insn_bits;
// Digital Output subdevice
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= adq12b_do_insn_bits;
    return 0;
    }
    static struct comedi_driver adq12b_driver = {
    .driver_name	= "adq12b",
    .module		= THIS_MODULE,
    .attach		= adq12b_attach,
    .detach		= comedi_legacy_detach,
    };
    module_comedi_driver(adq12b_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi low-level driver");
    MODULE_LICENSE("GPL");
