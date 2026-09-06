//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/ssv_dnp.c
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
// ssv_dnp.c
// generic comedi driver for SSV Embedded Systems' DIL/Net-PCs
// Copyright (C) 2001 Robert Schwebel <robert@schwebel.de>
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//
// Driver: ssv_dnp
// Description: SSV Embedded Systems DIL/Net-PC
// Author: Robert Schwebel <robert@schwebel.de>
// Devices: [SSV Embedded Systems] DIL/Net-PC 1486 (dnp-1486)
// Status: unknown
//
// include files -----------------------------------------------------------

// Some global definitions: the registers of the DNP -----------------------
//
// For port A and B the mode register has bits corresponding to the output
// pins, where Bit-N = 0 -> input, Bit-N = 1 -> output. Note that bits
// 4 to 7 correspond to pin 0..3 for port C data register. Ensure that bits
// 0..3 remain unchanged! For details about Port C Mode Register see
// the remarks in dnp_insn_config() below.
pub const CSCIR: c_uint = 0x22		/* Chip Setup and Control Index Register     */;
pub const CSCDR: c_uint = 0x23		/* Chip Setup and Control Data Register      */;
pub const PAMR: c_uint = 0xa5		/* Port A Mode Register                      */;
pub const PADR: c_uint = 0xa9		/* Port A Data Register                      */;
pub const PBMR: c_uint = 0xa4		/* Port B Mode Register                      */;
pub const PBDR: c_uint = 0xa8		/* Port B Data Register                      */;
pub const PCMR: c_uint = 0xa3		/* Port C Mode Register                      */;
pub const PCDR: c_uint = 0xa7		/* Port C Data Register                      */;
    static int dnp_dio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    unsigned int mask;
    unsigned int val;
//
// Ports A and B are straight forward: each bit corresponds to an
// output pin with the same order. Port C is different: bits 0...3
// correspond to bits 4...7 of the output register (PCDR).
//
    mask = comedi_dio_update_state(s, data);
    if (mask) {
    outb(PADR, CSCIR);
    outb(s.state & 0xff, CSCDR);
    outb(PBDR, CSCIR);
    outb((s.state >> 8) & 0xff, CSCDR);
    outb(PCDR, CSCIR);
    val = inb(CSCDR) & 0x0f;
    outb(((s.state >> 12) & 0xf0) | val, CSCDR);
    }
    outb(PADR, CSCIR);
    val = inb(CSCDR);
    outb(PBDR, CSCIR);
    val |= (inb(CSCDR) << 8);
    outb(PCDR, CSCIR);
    val |= ((inb(CSCDR) & 0xf0) << 12);
    data[1] = val;
    return insn.n;
    }
    static int dnp_dio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    unsigned int mask;
    unsigned int val;
    int ret;
    ret = comedi_dio_insn_config(dev, s, insn, data, 0);
    if (ret)
    return ret;
    if (chan < 8) {			/* Port A */
    mask = 1 << chan;
    outb(PAMR, CSCIR);
    } else if (chan < 16) {		/* Port B */
    mask = 1 << (chan - 8);
    outb(PBMR, CSCIR);
    } else {			/* Port C */
//
// We have to pay attention with port C.
// This is the meaning of PCMR:
// Bit in PCMR:              7 6 5 4 3 2 1 0
// Corresponding port C pin: d 3 d 2 d 1 d 0   d= don't touch
//
// Multiplication by 2 brings bits into correct position
// for PCMR!
//
    mask = 1 << ((chan - 16) * 2);
    outb(PCMR, CSCIR);
    }
    val = inb(CSCDR);
    if (data[0] == COMEDI_OUTPUT)
    val |= mask;
    else
    val &= ~mask;
    outb(val, CSCDR);
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn dnp_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> c_int {
    static int dnp_attach(struct comedi_device *dev, struct comedi_devconfig *it)
    {
    struct comedi_subdevice *s;
    int ret;
//
// We use I/O ports 0x22, 0x23 and 0xa3-0xa9, which are always
// allocated for the primary 8259, so we don't need to allocate
// them ourselves.
//
    ret = comedi_alloc_subdevices(dev, 1);
    if (ret)
    return ret;
    s = &dev.subdevices[0];
// digital i/o subdevice
    s.type = COMEDI_SUBD_DIO;
    s.subdev_flags = SDF_READABLE | SDF_WRITABLE;
    s.n_chan = 20;
    s.maxdata = 1;
    s.range_table = &range_digital;
    s.insn_bits = dnp_dio_insn_bits;
    s.insn_config = dnp_dio_insn_config;
// configure all ports as input (default)
    outb(PAMR, CSCIR);
    outb(0x00, CSCDR);
    outb(PBMR, CSCIR);
    outb(0x00, CSCDR);
    outb(PCMR, CSCIR);
    outb((inb(CSCDR) & 0xAA), CSCDR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dnp_detach(dev: *mut comedi_device) {
    static void dnp_detach(struct comedi_device *dev)
    {
    outb(PAMR, CSCIR);
    outb(0x00, CSCDR);
    outb(PBMR, CSCIR);
    outb(0x00, CSCDR);
    outb(PCMR, CSCIR);
    outb((inb(CSCDR) & 0xAA), CSCDR);
    }
    static struct comedi_driver dnp_driver = {
    .driver_name	= "dnp-1486",
    .module		= THIS_MODULE,
    .attach		= dnp_attach,
    .detach		= dnp_detach,
    };
    module_comedi_driver(dnp_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi low-level driver");
    MODULE_LICENSE("GPL");
