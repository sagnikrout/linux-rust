//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/dt2817.c
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
// comedi/drivers/dt2817.c
// Hardware driver for Data Translation DT2817
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1998 David A. Schleef <ds@schleef.org>
//
// Driver: dt2817
// Description: Data Translation DT2817
// Author: ds
// Status: complete
// Devices: [Data Translation] DT2817 (dt2817)
//
// A very simple digital I/O card.  Four banks of 8 lines, each bank
// is configurable for input or output.  One wonders why it takes a
// 50 page manual to describe this thing.
//
// The driver (which, btw, is much less than 50 pages) has 1 subdevice
// with 32 channels, configurable in groups of 8.
//
// Configuration options:
// [0] - I/O port base base address
//

pub const DT2817_CR: c_int = 0;
pub const DT2817_DATA: c_int = 1;
    static int dt2817_dio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut oe: c_uint = 0;
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
    if (s.io_bits & 0x000000ff)
    oe |= 0x1;
    if (s.io_bits & 0x0000ff00)
    oe |= 0x2;
    if (s.io_bits & 0x00ff0000)
    oe |= 0x4;
    if (s.io_bits & 0xff000000)
    oe |= 0x8;
    outb(oe, dev.iobase + DT2817_CR);
    return insn.n;
    }
    static int dt2817_dio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut iobase: c_ulong = dev.iobase + DT2817_DATA;
    unsigned int mask;
    unsigned int val;
    mask = comedi_dio_update_state(s, data);
    if (mask) {
    if (mask & 0x000000ff)
    outb(s.state & 0xff, iobase + 0);
    if (mask & 0x0000ff00)
    outb((s.state >> 8) & 0xff, iobase + 1);
    if (mask & 0x00ff0000)
    outb((s.state >> 16) & 0xff, iobase + 2);
    if (mask & 0xff000000)
    outb((s.state >> 24) & 0xff, iobase + 3);
    }
    val = inb(iobase + 0);
    val |= (inb(iobase + 1) << 8);
    val |= (inb(iobase + 2) << 16);
    val |= (inb(iobase + 3) << 24);
    data[1] = val;
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn dt2817_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    static int dt2817_attach(struct comedi_device *dev, struct comedi_devconfig *it)
    {
    int ret;
    struct comedi_subdevice *s;
    ret = comedi_check_request_region(dev, it.options[0], 0x5,
    0x200, 0x3ff, 8);
    if (ret)
    return ret;
    ret = comedi_alloc_subdevices(dev, 1);
    if (ret)
    return ret;
    s = &dev.subdevices[0];
    s.n_chan = 32;
    s.type = COMEDI_SUBD_DIO;
    s.subdev_flags = SDF_READABLE | SDF_WRITABLE;
    s.range_table = &range_digital;
    s.maxdata = 1;
    s.insn_bits = dt2817_dio_insn_bits;
    s.insn_config = dt2817_dio_insn_config;
    s.state = 0;
    outb(0, dev.iobase + DT2817_CR);
    return 0;
    }
    static struct comedi_driver dt2817_driver = {
    .driver_name	= "dt2817",
    .module		= THIS_MODULE,
    .attach		= dt2817_attach,
    .detach		= comedi_legacy_detach,
    };
    module_comedi_driver(dt2817_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi low-level driver");
    MODULE_LICENSE("GPL");
