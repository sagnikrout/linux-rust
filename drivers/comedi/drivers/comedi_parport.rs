//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/comedi_parport.c
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
// comedi_parport.c
// Comedi driver for standard parallel port
//
// For more information see:
// http://retired.beyondlogic.org/spp/parallel.htm
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1998,2001 David A. Schleef <ds@schleef.org>
//
// Driver: comedi_parport
// Description: Standard PC parallel port
// Author: ds
// Status: works in immediate mode
// Devices: [standard] parallel port (comedi_parport)
// Updated: Tue, 30 Apr 2002 21:11:45 -0700
//
// A cheap and easy way to get a few more digital I/O lines. Steal
// additional parallel ports from old computers or your neighbors'
// computers.
//
// Option list:
// 0: I/O port base for the parallel port.
// 1: IRQ (optional)
//
// Parallel Port Lines:
//
// pin   subdev  chan  type  name
// -----  ------  ----  ----  --------------
// 1      2       0    DO   strobe
// 2      0       0    DIO  data 0
// 3      0       1    DIO  data 1
// 4      0       2    DIO  data 2
// 5      0       3    DIO  data 3
// 6      0       4    DIO  data 4
// 7      0       5    DIO  data 5
// 8      0       6    DIO  data 6
// 9      0       7    DIO  data 7
// 10      1       3    DI   ack
// 11      1       4    DI   busy
// 12      1       2    DI   paper out
// 13      1       1    DI   select in
// 14      2       1    DO   auto LF
// 15      1       0    DI   error
// 16      2       2    DO   init
// 17      2       3    DO   select printer
// 18-25                      ground
//
// When an IRQ is configured subdevice 3 pretends to be a digital
// input subdevice, but it always returns 0 when read. However, if
// you run a command with scan_begin_src=TRIG_EXT, it uses pin 10
// as a external trigger, which can be used to wake up tasks.
//

//
// Register map
//
pub const PARPORT_DATA_REG: c_uint = 0x00;
pub const PARPORT_STATUS_REG: c_uint = 0x01;
pub const PARPORT_CTRL_REG: c_uint = 0x02;

    static int parport_data_reg_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    if (comedi_dio_update_state(s, data))
    outb(s.state, dev.iobase + PARPORT_DATA_REG);
    data[1] = inb(dev.iobase + PARPORT_DATA_REG);
    return insn.n;
    }
    static int parport_data_reg_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    unsigned int ctrl;
    int ret;
    ret = comedi_dio_insn_config(dev, s, insn, data, 0xff);
    if (ret)
    return ret;
    ctrl = inb(dev.iobase + PARPORT_CTRL_REG);
    if (s.io_bits)
    ctrl &= ~PARPORT_CTRL_BIDIR_ENA;
    else
    ctrl |= PARPORT_CTRL_BIDIR_ENA;
    outb(ctrl, dev.iobase + PARPORT_CTRL_REG);
    return insn.n;
    }
    static int parport_status_reg_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    data[1] = inb(dev.iobase + PARPORT_STATUS_REG) >> 3;
    return insn.n;
    }
    static int parport_ctrl_reg_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    unsigned int ctrl;
    if (comedi_dio_update_state(s, data)) {
    ctrl = inb(dev.iobase + PARPORT_CTRL_REG);
    ctrl &= (PARPORT_CTRL_IRQ_ENA | PARPORT_CTRL_BIDIR_ENA);
    ctrl |= s.state;
    outb(ctrl, dev.iobase + PARPORT_CTRL_REG);
    }
    data[1] = s.state;
    return insn.n;
    }
    static int parport_intr_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    data[1] = 0;
    return insn.n;
    }
    static int parport_intr_cmdtest(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    let mut err: c_int = 0;
// Step 1 : check if triggers are trivially valid
    err |= comedi_check_trigger_src(&cmd.start_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&cmd.scan_begin_src, TRIG_EXT);
    err |= comedi_check_trigger_src(&cmd.convert_src, TRIG_FOLLOW);
    err |= comedi_check_trigger_src(&cmd.scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&cmd.stop_src, TRIG_NONE);
    if (err)
    return 1;
// Step 2a : make sure trigger sources are unique
// Step 2b : and mutually compatible
// Step 3: check if arguments are trivially valid
    err |= comedi_check_trigger_arg_is(&cmd.start_arg, 0);
    err |= comedi_check_trigger_arg_is(&cmd.scan_begin_arg, 0);
    err |= comedi_check_trigger_arg_is(&cmd.convert_arg, 0);
    err |= comedi_check_trigger_arg_is(&cmd.scan_end_arg,
    cmd.chanlist_len);
    err |= comedi_check_trigger_arg_is(&cmd.stop_arg, 0);
    if (err)
    return 3;
// Step 4: fix up any arguments
// Step 5: check channel list if it exists
    return 0;
    }
    static int parport_intr_cmd(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    unsigned int ctrl;
    ctrl = inb(dev.iobase + PARPORT_CTRL_REG);
    ctrl |= PARPORT_CTRL_IRQ_ENA;
    outb(ctrl, dev.iobase + PARPORT_CTRL_REG);
    return 0;
    }
    static int parport_intr_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    unsigned int ctrl;
    ctrl = inb(dev.iobase + PARPORT_CTRL_REG);
    ctrl &= ~PARPORT_CTRL_IRQ_ENA;
    outb(ctrl, dev.iobase + PARPORT_CTRL_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parport_interrupt(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t parport_interrupt(int irq, void *d)
    {
    struct comedi_device *dev = d;
    struct comedi_subdevice *s = dev.read_subdev;
    unsigned int ctrl;
    let mut val: c_ushort = 0;
//
// Check device is fully attached.  Device interrupts should have
// been disabled, but do this in case of bad hardware.
//
    if (!dev.attached)
    return IRQ_NONE;
    ctrl = inb(dev.iobase + PARPORT_CTRL_REG);
    if (!(ctrl & PARPORT_CTRL_IRQ_ENA))
    return IRQ_NONE;
    comedi_buf_write_samples(s, &val, 1);
    comedi_handle_events(dev, s);
    return IRQ_HANDLED;
    }
    static int parport_attach(struct comedi_device *dev,
    struct comedi_devconfig *it)
    {
    struct comedi_subdevice *s;
    let mut iobase: c_uint = it.options[0];
    int ret;
    ret = comedi_check_request_region(dev, iobase, 0x03,
    0, UINT_MAX, 4);
    if (ret)
    return ret;
    outb(0, dev.iobase + PARPORT_DATA_REG);
    outb(0, dev.iobase + PARPORT_CTRL_REG);
    if (it.options[1]) {
    ret = request_irq(it.options[1], parport_interrupt, 0,
    dev.board_name, dev);
    if (ret == 0)
    dev.irq = it.options[1];
    }
    ret = comedi_alloc_subdevices(dev, dev.irq ? 4 : 3);
    if (ret)
    return ret;
// Digial I/O subdevice - Parallel port DATA register
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_DIO;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= parport_data_reg_insn_bits;
    s.insn_config	= parport_data_reg_insn_config;
// Digial Input subdevice - Parallel port STATUS register
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= 5;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= parport_status_reg_insn_bits;
// Digial Output subdevice - Parallel port CONTROL register
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 4;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= parport_ctrl_reg_insn_bits;
    if (dev.irq) {
// Digial Input subdevice - Interrupt support
    s = &dev.subdevices[3];
    dev.read_subdev = s;
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE | SDF_CMD_READ;
    s.n_chan	= 1;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= parport_intr_insn_bits;
    s.len_chanlist	= 1;
    s.do_cmdtest	= parport_intr_cmdtest;
    s.do_cmd	= parport_intr_cmd;
    s.cancel	= parport_intr_cancel;
    }
    return 0;
    }
    static struct comedi_driver parport_driver = {
    .driver_name	= "comedi_parport",
    .module		= THIS_MODULE,
    .attach		= parport_attach,
    .detach		= comedi_legacy_detach,
    };
    module_comedi_driver(parport_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi: Standard parallel port driver");
    MODULE_LICENSE("GPL");
