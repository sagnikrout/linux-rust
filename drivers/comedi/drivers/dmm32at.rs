//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/dmm32at.c
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
// dmm32at.c
// Diamond Systems Diamond-MM-32-AT Comedi driver
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//
// Driver: dmm32at
// Description: Diamond Systems Diamond-MM-32-AT
// Devices: [Diamond Systems] Diamond-MM-32-AT (dmm32at)
// Author: Perry J. Piplani <perry.j.piplani@nasa.gov>
// Updated: Fri Jun  4 09:13:24 CDT 2004
// Status: experimental
//
// Configuration Options:
// comedi_config /dev/comedi0 dmm32at baseaddr,irq
//
// This driver is for the Diamond Systems MM-32-AT board
// http://www.diamondsystems.com/products/diamondmm32at
//
// It is being used on several projects inside NASA, without
// problems so far. For analog input commands, TRIG_EXT is not
// yet supported.
//

// Board register addresses
pub const DMM32AT_AI_START_CONV_REG: c_uint = 0x00;
pub const DMM32AT_AI_LSB_REG: c_uint = 0x00;
pub const DMM32AT_AUX_DOUT_REG: c_uint = 0x01;

pub const DMM32AT_AI_MSB_REG: c_uint = 0x01;
pub const DMM32AT_AI_LO_CHAN_REG: c_uint = 0x02;
pub const DMM32AT_AI_HI_CHAN_REG: c_uint = 0x03;
pub const DMM32AT_AUX_DI_REG: c_uint = 0x04;

pub const DMM32AT_AO_LSB_REG: c_uint = 0x04;
pub const DMM32AT_AO_MSB_REG: c_uint = 0x05;

pub const DMM32AT_FIFO_DEPTH_REG: c_uint = 0x06;
pub const DMM32AT_FIFO_CTRL_REG: c_uint = 0x07;

pub const DMM32AT_FIFO_STATUS_REG: c_uint = 0x07;

pub const DMM32AT_CTRL_REG: c_uint = 0x08;

pub const DMM32AT_AI_STATUS_REG: c_uint = 0x08;

pub const DMM32AT_INTCLK_REG: c_uint = 0x09;

pub const DMM32AT_CTRDIO_CFG_REG: c_uint = 0x0a;

pub const DMM32AT_AI_CFG_REG: c_uint = 0x0b;

pub const DMM32AT_AI_READBACK_REG: c_uint = 0x0b;

pub const DMM32AT_CLK1: c_uint = 0x0d;
pub const DMM32AT_CLK2: c_uint = 0x0e;
pub const DMM32AT_CLKCT: c_uint = 0x0f;
pub const DMM32AT_8255_IOBASE: c_uint = 0x0c  /* Page 1 registers */;
// Board register values.
// DMM32AT_AI_CFG_REG 0x0b
pub const DMM32AT_RANGE_U10: c_uint = 0x0c;
pub const DMM32AT_RANGE_U5: c_uint = 0x0d;
pub const DMM32AT_RANGE_B10: c_uint = 0x08;
pub const DMM32AT_RANGE_B5: c_uint = 0x00;
// DMM32AT_CLKCT 0x0f
pub const DMM32AT_CLKCT1: c_uint = 0x56	/* mode3 counter 1 - write low byte only */;
pub const DMM32AT_CLKCT2: c_uint = 0xb6	/*  mode3 counter 2 - write high and low byte */;
// board AI ranges in comedi structure
    static const struct comedi_lrange dmm32at_airanges = {
    4, {
    UNI_RANGE(10),
    UNI_RANGE(5),
    BIP_RANGE(10),
    BIP_RANGE(5)
    }
    };
// register values for above ranges
    static const unsigned char dmm32at_rangebits[] = {
    DMM32AT_RANGE_U10,
    DMM32AT_RANGE_U5,
    DMM32AT_RANGE_B10,
    DMM32AT_RANGE_B5,
    };
// only one of these ranges is valid, as set by a jumper on the
// board. The application should only use the range set by the jumper
//
    static const struct comedi_lrange dmm32at_aoranges = {
    4, {
    UNI_RANGE(10),
    UNI_RANGE(5),
    BIP_RANGE(10),
    BIP_RANGE(5)
    }
    };
    static void dmm32at_ai_set_chanspec(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int chanspec, int nchan)
    {
    let mut chan: c_uint = CR_CHAN(chanspec);
    let mut range: c_uint = CR_RANGE(chanspec);
    let mut last_chan: c_uint = (chan + nchan - 1) % s.n_chan;
    outb(DMM32AT_FIFO_CTRL_FIFORST, dev.iobase + DMM32AT_FIFO_CTRL_REG);
    if (nchan > 1)
    outb(DMM32AT_FIFO_CTRL_SCANEN,
    dev.iobase + DMM32AT_FIFO_CTRL_REG);
    outb(chan, dev.iobase + DMM32AT_AI_LO_CHAN_REG);
    outb(last_chan, dev.iobase + DMM32AT_AI_HI_CHAN_REG);
    outb(dmm32at_rangebits[range], dev.iobase + DMM32AT_AI_CFG_REG);
    }
    static unsigned int dmm32at_ai_get_sample(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    unsigned int val;
    val = inb(dev.iobase + DMM32AT_AI_LSB_REG);
    val |= (inb(dev.iobase + DMM32AT_AI_MSB_REG) << 8);
// munge two's complement value to offset binary
    return comedi_offset_munge(s, val);
    }
    static int dmm32at_ai_status(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned char status;
    status = inb(dev.iobase + context);
    if ((status & DMM32AT_AI_STATUS_STS) == 0)
    return 0;
    return -EBUSY;
    }
    static int dmm32at_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    int ret;
    int i;
    dmm32at_ai_set_chanspec(dev, s, insn.chanspec, 1);
// wait for circuit to settle
    ret = comedi_timeout(dev, s, insn, dmm32at_ai_status,
    DMM32AT_AI_READBACK_REG);
    if (ret)
    return ret;
    for (i = 0; i < insn.n; i++) {
    outb(0xff, dev.iobase + DMM32AT_AI_START_CONV_REG);
    ret = comedi_timeout(dev, s, insn, dmm32at_ai_status,
    DMM32AT_AI_STATUS_REG);
    if (ret)
    return ret;
    data[i] = dmm32at_ai_get_sample(dev, s);
    }
    return insn.n;
    }
    static int dmm32at_ai_check_chanlist(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    let mut chan0: c_uint = CR_CHAN(cmd.chanlist[0]);
    let mut range0: c_uint = CR_RANGE(cmd.chanlist[0]);
    int i;
    for (i = 1; i < cmd.chanlist_len; i++) {
    let mut chan: c_uint = CR_CHAN(cmd.chanlist[i]);
    let mut range: c_uint = CR_RANGE(cmd.chanlist[i]);
    if (chan != (chan0 + i) % s.n_chan) {
    dev_dbg(dev.class_dev,
    "entries in chanlist must be consecutive channels, counting upwards\n");
    return -EINVAL;
    }
    if (range != range0) {
    dev_dbg(dev.class_dev,
    "entries in chanlist must all have the same gain\n");
    return -EINVAL;
    }
    }
    return 0;
    }
    static int dmm32at_ai_cmdtest(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    let mut err: c_int = 0;
    unsigned int arg;
// Step 1 : check if triggers are trivially valid
    err |= comedi_check_trigger_src(&cmd.start_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&cmd.scan_begin_src, TRIG_TIMER);
    err |= comedi_check_trigger_src(&cmd.convert_src, TRIG_TIMER);
    err |= comedi_check_trigger_src(&cmd.scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&cmd.stop_src, TRIG_COUNT | TRIG_NONE);
    if (err)
    return 1;
// Step 2a : make sure trigger sources are unique
    err |= comedi_check_trigger_is_unique(cmd.stop_src);
// Step 2b : and mutually compatible
    if (err)
    return 2;
// Step 3: check if arguments are trivially valid
    err |= comedi_check_trigger_arg_is(&cmd.start_arg, 0);
    err |= comedi_check_trigger_arg_min(&cmd.scan_begin_arg, 1000000);
    err |= comedi_check_trigger_arg_max(&cmd.scan_begin_arg, 1000000000);
    if (cmd.convert_arg >= 17500)
    cmd.convert_arg = 20000;
#[no_mangle]
pub unsafe extern "C" fn if(12500: cmd->convert_arg >=) -> else {
    else if (cmd.convert_arg >= 12500)
    cmd.convert_arg = 15000;
#[no_mangle]
pub unsafe extern "C" fn if(7500: cmd->convert_arg >=) -> else {
    else if (cmd.convert_arg >= 7500)
    cmd.convert_arg = 10000;
    else
    cmd.convert_arg = 5000;
    err |= comedi_check_trigger_arg_is(&cmd.scan_end_arg,
    cmd.chanlist_len);
    if (cmd.stop_src == TRIG_COUNT)
    err |= comedi_check_trigger_arg_min(&cmd.stop_arg, 1);
    else /* TRIG_NONE */
    err |= comedi_check_trigger_arg_is(&cmd.stop_arg, 0);
    if (err)
    return 3;
// Step 4: fix up any arguments
    arg = cmd.convert_arg * cmd.scan_end_arg;
    err |= comedi_check_trigger_arg_min(&cmd.scan_begin_arg, arg);
    if (err)
    return 4;
// Step 5: check channel list if it exists
    if (cmd.chanlist && cmd.chanlist_len > 0)
    err |= dmm32at_ai_check_chanlist(dev, s, cmd);
    if (err)
    return 5;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmm32at_setaitimer(dev: *mut comedi_device, nansec: c_uint) {
    static void dmm32at_setaitimer(struct comedi_device *dev, unsigned int nansec)
    {
    unsigned long irq_flags;
    unsigned char lo1, lo2, hi2;
    unsigned short both2;
// based on 10mhz clock
    lo1 = 200;
    both2 = nansec / 20000;
    hi2 = (both2 & 0xff00) >> 8;
    lo2 = both2 & 0x00ff;
// set counter clocks to 10MHz, disable all aux dio
    outb(0, dev.iobase + DMM32AT_CTRDIO_CFG_REG);
// serialize access to control register and paged registers
    spin_lock_irqsave(&dev.spinlock, irq_flags);
// get access to the clock regs
    outb(DMM32AT_CTRL_PAGE_8254, dev.iobase + DMM32AT_CTRL_REG);
// write the counter 1 control word and low byte to counter
    outb(DMM32AT_CLKCT1, dev.iobase + DMM32AT_CLKCT);
    outb(lo1, dev.iobase + DMM32AT_CLK1);
// write the counter 2 control word and low byte then to counter
    outb(DMM32AT_CLKCT2, dev.iobase + DMM32AT_CLKCT);
    outb(lo2, dev.iobase + DMM32AT_CLK2);
    outb(hi2, dev.iobase + DMM32AT_CLK2);
    spin_unlock_irqrestore(&dev.spinlock, irq_flags);
// enable the ai conversion interrupt and the clock to start scans
    outb(DMM32AT_INTCLK_ADINT |
    DMM32AT_INTCLK_CLKEN | DMM32AT_INTCLK_CLKSEL,
    dev.iobase + DMM32AT_INTCLK_REG);
    }
#[no_mangle]
unsafe extern "C" fn dmm32at_ai_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int {
    static int dmm32at_ai_cmd(struct comedi_device *dev, struct comedi_subdevice *s)
    {
    struct comedi_cmd *cmd = &s.async.cmd;
    unsigned long irq_flags;
    int ret;
    dmm32at_ai_set_chanspec(dev, s, cmd.chanlist[0], cmd.chanlist_len);
// serialize access to control register and paged registers
    spin_lock_irqsave(&dev.spinlock, irq_flags);
// reset the interrupt just in case
    outb(DMM32AT_CTRL_INTRST, dev.iobase + DMM32AT_CTRL_REG);
    spin_unlock_irqrestore(&dev.spinlock, irq_flags);
//
// wait for circuit to settle
// we don't have the 'insn' here but it's not needed
//
    ret = comedi_timeout(dev, s, core::ptr::null_mut(), dmm32at_ai_status,
    DMM32AT_AI_READBACK_REG);
    if (ret)
    return ret;
    if (cmd.stop_src == TRIG_NONE || cmd.stop_arg > 1) {
// start the clock and enable the interrupts
    dmm32at_setaitimer(dev, cmd.scan_begin_arg);
    } else {
// start the interrupts and initiate a single scan
    outb(DMM32AT_INTCLK_ADINT, dev.iobase + DMM32AT_INTCLK_REG);
    outb(0xff, dev.iobase + DMM32AT_AI_START_CONV_REG);
    }
    return 0;
    }
    static int dmm32at_ai_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
// disable further interrupts and clocks
    outb(0x0, dev.iobase + DMM32AT_INTCLK_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmm32at_isr(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t dmm32at_isr(int irq, void *d)
    {
    struct comedi_device *dev = d;
    unsigned char intstat;
    unsigned short val;
    int i;
    if (!dev.attached) {
    dev_err(dev.class_dev, "spurious interrupt\n");
    return IRQ_HANDLED;
    }
    intstat = inb(dev.iobase + DMM32AT_INTCLK_REG);
    if (intstat & DMM32AT_INTCLK_ADINT) {
    struct comedi_subdevice *s = dev.read_subdev;
    struct comedi_cmd *cmd = &s.async.cmd;
    for (i = 0; i < cmd.chanlist_len; i++) {
    val = dmm32at_ai_get_sample(dev, s);
    comedi_buf_write_samples(s, &val, 1);
    }
    if (cmd.stop_src == TRIG_COUNT &&
    s.async.scans_done >= cmd.stop_arg)
    s.async.events |= COMEDI_CB_EOA;
    comedi_handle_events(dev, s);
    }
// serialize access to control register and paged registers
    spin_lock(&dev.spinlock);
// reset the interrupt
    outb(DMM32AT_CTRL_INTRST, dev.iobase + DMM32AT_CTRL_REG);
    spin_unlock(&dev.spinlock);
    return IRQ_HANDLED;
    }
    static int dmm32at_ao_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned char status;
    status = inb(dev.iobase + DMM32AT_AUX_DI_REG);
    if ((status & DMM32AT_AUX_DI_DACBUSY) == 0)
    return 0;
    return -EBUSY;
    }
    static int dmm32at_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    int i;
    for (i = 0; i < insn.n; i++) {
    let mut val: c_uint = data[i];
    int ret;
// write LSB then MSB + chan to load DAC
    outb(val & 0xff, dev.iobase + DMM32AT_AO_LSB_REG);
    outb((val >> 8) | DMM32AT_AO_MSB_DACH(chan),
    dev.iobase + DMM32AT_AO_MSB_REG);
// wait for circuit to settle
    ret = comedi_timeout(dev, s, insn, dmm32at_ao_eoc, 0);
    if (ret)
    return ret;
// dummy read to update DAC
    inb(dev.iobase + DMM32AT_AO_MSB_REG);
    s.readback[chan] = val;
    }
    return insn.n;
    }
    static int dmm32at_8255_io(struct comedi_device *dev,
    int dir, int port, int data, unsigned long regbase)
    {
    unsigned long irq_flags;
    int ret;
// serialize access to control register and paged registers
    spin_lock_irqsave(&dev.spinlock, irq_flags);
// get access to the DIO regs
    outb(DMM32AT_CTRL_PAGE_8255, dev.iobase + DMM32AT_CTRL_REG);
    if (dir) {
    outb(data, dev.iobase + regbase + port);
    ret = 0;
    } else {
    ret = inb(dev.iobase + regbase + port);
    }
    spin_unlock_irqrestore(&dev.spinlock, irq_flags);
    return ret;
    }
// Make sure the board is there and put it to a known state
#[no_mangle]
unsafe extern "C" fn dmm32at_reset(dev: *mut comedi_device) -> c_int {
    static int dmm32at_reset(struct comedi_device *dev)
    {
    unsigned char aihi, ailo, fifostat, aistat, intstat, airback;
// reset the board
    outb(DMM32AT_CTRL_RESETA, dev.iobase + DMM32AT_CTRL_REG);
// allow a millisecond to reset
    usleep_range(1000, 3000);
// zero scan and fifo control
    outb(0x0, dev.iobase + DMM32AT_FIFO_CTRL_REG);
// zero interrupt and clock control
    outb(0x0, dev.iobase + DMM32AT_INTCLK_REG);
// write a test channel range, the high 3 bits should drop
    outb(0x80, dev.iobase + DMM32AT_AI_LO_CHAN_REG);
    outb(0xff, dev.iobase + DMM32AT_AI_HI_CHAN_REG);
// set the range at 10v unipolar
    outb(DMM32AT_RANGE_U10, dev.iobase + DMM32AT_AI_CFG_REG);
// should take 10 us to settle, here's a hundred
    usleep_range(100, 200);
// read back the values
    ailo = inb(dev.iobase + DMM32AT_AI_LO_CHAN_REG);
    aihi = inb(dev.iobase + DMM32AT_AI_HI_CHAN_REG);
    fifostat = inb(dev.iobase + DMM32AT_FIFO_STATUS_REG);
    aistat = inb(dev.iobase + DMM32AT_AI_STATUS_REG);
    intstat = inb(dev.iobase + DMM32AT_INTCLK_REG);
    airback = inb(dev.iobase + DMM32AT_AI_READBACK_REG);
//
// NOTE: The (DMM32AT_AI_STATUS_SD1 | DMM32AT_AI_STATUS_SD0)
// test makes this driver only work if the board is configured
// with all A/D channels set for single-ended operation.
//
    if (ailo != 0x00 || aihi != 0x1f ||
    fifostat != DMM32AT_FIFO_STATUS_EF ||
    aistat != (DMM32AT_AI_STATUS_SD1 | DMM32AT_AI_STATUS_SD0) ||
    intstat != 0x00 || airback != 0x0c)
    return -EIO;
    return 0;
    }
    static int dmm32at_attach(struct comedi_device *dev,
    struct comedi_devconfig *it)
    {
    struct comedi_subdevice *s;
    let mut iobase: c_uint = it.options[0];
    int ret;
    switch (iobase) {
    case 0x100:
    case 0x140:
    case 0x180:
    case 0x200:
    case 0x280:
    case 0x300:
    case 0x340:
    case 0x380:
    ret = comedi_request_region(dev, iobase, 0x10);
    if (ret)
    return ret;
    break;
    default:
    dev_err(dev.class_dev, "unsupported base address %#x\n",
    iobase);
    return -EINVAL;
    }
    ret = dmm32at_reset(dev);
    if (ret) {
    dev_err(dev.class_dev, "board detection failed\n");
    return ret;
    }
    if (it.options[1]) {
    ret = request_irq(it.options[1], dmm32at_isr, 0,
    dev.board_name, dev);
    if (ret == 0)
    dev.irq = it.options[1];
    }
    ret = comedi_alloc_subdevices(dev, 3);
    if (ret)
    return ret;
// Analog Input subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_GROUND | SDF_DIFF;
    s.n_chan	= 32;
    s.maxdata	= 0xffff;
    s.range_table	= &dmm32at_airanges;
    s.insn_read	= dmm32at_ai_insn_read;
    if (dev.irq) {
    dev.read_subdev = s;
    s.subdev_flags	|= SDF_CMD_READ;
    s.len_chanlist	= s.n_chan;
    s.do_cmd	= dmm32at_ai_cmd;
    s.do_cmdtest	= dmm32at_ai_cmdtest;
    s.cancel	= dmm32at_ai_cancel;
    }
// Analog Output subdevice
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 4;
    s.maxdata	= 0x0fff;
    s.range_table	= &dmm32at_aoranges;
    s.insn_write	= dmm32at_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
// Digital I/O subdevice
    s = &dev.subdevices[2];
    return subdev_8255_cb_init(dev, s, dmm32at_8255_io,
    DMM32AT_8255_IOBASE);
    }
    static struct comedi_driver dmm32at_driver = {
    .driver_name	= "dmm32at",
    .module		= THIS_MODULE,
    .attach		= dmm32at_attach,
    .detach		= comedi_legacy_detach,
    };
    module_comedi_driver(dmm32at_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi: Diamond Systems Diamond-MM-32-AT");
    MODULE_LICENSE("GPL");
