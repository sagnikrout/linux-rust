//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/pcmad.c
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
// pcmad.c
// Hardware driver for Winsystems PCM-A/D12 and PCM-A/D16
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000,2001 David A. Schleef <ds@schleef.org>
//
// Driver: pcmad
// Description: Winsystems PCM-A/D12, PCM-A/D16
// Devices: [Winsystems] PCM-A/D12 (pcmad12), PCM-A/D16 (pcmad16)
// Author: ds
// Status: untested
//
// This driver was written on a bet that I couldn't write a driver
// in less than 2 hours.  I won the bet, but never got paid.  =(
//
// Configuration options:
// [0] - I/O port base
// [1] - IRQ (unused)
// [2] - Analog input reference (must match jumpers)
// 0 = single-ended (16 channels)
// 1 = differential (8 channels)
// [3] - Analog input encoding (must match jumpers)
// 0 = straight binary (0-5V input range)
// 1 = two's complement (+-10V input range)
//

pub const PCMAD_STATUS: c_int = 0;
pub const PCMAD_LSB: c_int = 1;
pub const PCMAD_MSB: c_int = 2;
pub const PCMAD_CONVERT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmad_board_struct {
    pub name: *const c_char,
    pub ai_maxdata: c_uint,
}

    static const struct pcmad_board_struct pcmad_boards[] = {
    {
    .name		= "pcmad12",
    .ai_maxdata	= 0x0fff,
    }, {
    .name		= "pcmad16",
    .ai_maxdata	= 0xffff,
    },
    };
    static int pcmad_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned int status;
    status = inb(dev.iobase + PCMAD_STATUS);
    if ((status & 0x3) == 0x3)
    return 0;
    return -EBUSY;
    }
    static int pcmad_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut range: c_uint = CR_RANGE(insn.chanspec);
    unsigned int val;
    int ret;
    int i;
    for (i = 0; i < insn.n; i++) {
    outb(chan, dev.iobase + PCMAD_CONVERT);
    ret = comedi_timeout(dev, s, insn, pcmad_ai_eoc, 0);
    if (ret)
    return ret;
    val = inb(dev.iobase + PCMAD_LSB) |
    (inb(dev.iobase + PCMAD_MSB) << 8);
// data is shifted on the pcmad12, fix it
    if (s.maxdata == 0x0fff)
    val >>= 4;
    if (comedi_range_is_bipolar(s, range)) {
// munge the two's complement value
    val ^= ((s.maxdata + 1) >> 1);
    }
    data[i] = val;
    }
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn pcmad_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    static int pcmad_attach(struct comedi_device *dev, struct comedi_devconfig *it)
    {
    const struct pcmad_board_struct *board = dev.board_ptr;
    struct comedi_subdevice *s;
    int ret;
    ret = comedi_check_request_region(dev, it.options[0], 0x04,
    0, 0x3ff, 4);
    if (ret)
    return ret;
    ret = comedi_alloc_subdevices(dev, 1);
    if (ret)
    return ret;
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AI;
    if (it.options[1]) {
// 8 differential channels
    s.subdev_flags	= SDF_READABLE | AREF_DIFF;
    s.n_chan	= 8;
    } else {
// 16 single-ended channels
    s.subdev_flags	= SDF_READABLE | AREF_GROUND;
    s.n_chan	= 16;
    }
    s.len_chanlist	= 1;
    s.maxdata	= board.ai_maxdata;
    s.range_table	= it.options[2] ? &range_bipolar10 : &range_unipolar5;
    s.insn_read	= pcmad_ai_insn_read;
    return 0;
    }
    static struct comedi_driver pcmad_driver = {
    .driver_name	= "pcmad",
    .module		= THIS_MODULE,
    .attach		= pcmad_attach,
    .detach		= comedi_legacy_detach,
    .board_name	= &pcmad_boards[0].name,
    .num_names	= ARRAY_SIZE(pcmad_boards),
    .offset		= sizeof(pcmad_boards[0]),
    };
    module_comedi_driver(pcmad_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi low-level driver");
    MODULE_LICENSE("GPL");
