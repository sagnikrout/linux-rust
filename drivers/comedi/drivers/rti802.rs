//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/rti802.c
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
// rti802.c
// Comedi driver for Analog Devices RTI-802 board
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1999 Anders Blomdell <anders.blomdell@control.lth.se>
//
// Driver: rti802
// Description: Analog Devices RTI-802
// Author: Anders Blomdell <anders.blomdell@control.lth.se>
// Devices: [Analog Devices] RTI-802 (rti802)
// Status: works
//
// Configuration Options:
// [0] - i/o base
// [1] - unused
// [2,4,6,8,10,12,14,16] - dac#[0-7]  0=two's comp, 1=straight
// [3,5,7,9,11,13,15,17] - dac#[0-7]  0=bipolar, 1=unipolar
//

//
// Register I/O map
//
pub const RTI802_SELECT: c_uint = 0x00;
pub const RTI802_DATALOW: c_uint = 0x01;
pub const RTI802_DATAHIGH: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rti802_private {
    enum {
    dac_2comp, dac_straight
    pub dac_coding: [}; 8],
    pub range_type_list: [*const comedi_lrange; 8],
}

    static int rti802_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct rti802_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    int i;
    outb(chan, dev.iobase + RTI802_SELECT);
    for (i = 0; i < insn.n; i++) {
    let mut val: c_uint = data[i];
    s.readback[chan] = val;
// munge offset binary to two's complement if needed
    if (devpriv.dac_coding[chan] == dac_2comp)
    val = comedi_offset_munge(s, val);
    outb(val & 0xff, dev.iobase + RTI802_DATALOW);
    outb((val >> 8) & 0xff, dev.iobase + RTI802_DATAHIGH);
    }
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn rti802_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    static int rti802_attach(struct comedi_device *dev, struct comedi_devconfig *it)
    {
    struct rti802_private *devpriv;
    struct comedi_subdevice *s;
    int i;
    int ret;
    ret = comedi_check_request_region(dev, it.options[0], 0x04,
    0, 0x3ff, 4);
    if (ret)
    return ret;
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    ret = comedi_alloc_subdevices(dev, 1);
    if (ret)
    return ret;
// Analog Output subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE;
    s.maxdata	= 0xfff;
    s.n_chan	= 8;
    s.insn_write	= rti802_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
    s.range_table_list = devpriv.range_type_list;
    for (i = 0; i < 8; i++) {
    devpriv.dac_coding[i] = (it.options[3 + 2 * i])
    ? (dac_straight) : (dac_2comp);
    devpriv.range_type_list[i] = (it.options[2 + 2 * i])
    ? &range_unipolar10 : &range_bipolar10;
    }
    return 0;
    }
    static struct comedi_driver rti802_driver = {
    .driver_name	= "rti802",
    .module		= THIS_MODULE,
    .attach		= rti802_attach,
    .detach		= comedi_legacy_detach,
    };
    module_comedi_driver(rti802_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for Analog Devices RTI-802 board");
    MODULE_LICENSE("GPL");
