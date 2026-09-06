//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/ii_pci20kc.c
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


// SPDX-License-Identifier: GPL-2.0
//
// ii_pci20kc.c
// Driver for Intelligent Instruments PCI-20001C carrier board and modules.
//
// Copyright (C) 2000 Markus Kempf <kempf@matsci.uni-sb.de>
// with suggestions from David Schleef		16.06.2000
//
// Driver: ii_pci20kc
// Description: Intelligent Instruments PCI-20001C carrier board
// Devices: [Intelligent Instrumentation] PCI-20001C (ii_pci20kc)
// Author: Markus Kempf <kempf@matsci.uni-sb.de>
// Status: works
//
// Supports the PCI-20001C-1a and PCI-20001C-2a carrier boards. The
// -2a version has 32 on-board DIO channels. Three add-on modules
// can be added to the carrier board for additional functionality.
//
// Supported add-on modules:
// PCI-20006M-1   1 channel, 16-bit analog output module
// PCI-20006M-2   2 channel, 16-bit analog output module
// PCI-20341M-1A  4 channel, 16-bit analog input module
//
// Options:
// 0   Board base address
// 1   IRQ (not-used)
//

//
// Register I/O map
//
pub const II20K_SIZE: c_uint = 0x400;
pub const II20K_MOD_OFFSET: c_uint = 0x100;
pub const II20K_ID_REG: c_uint = 0x00;

pub const II20K_ID_MASK: c_uint = 0x1f;
pub const II20K_ID_PCI20001C_1A: c_uint = 0x1b	/* no on-board DIO */;
pub const II20K_ID_PCI20001C_2A: c_uint = 0x1d	/* on-board DIO */;
pub const II20K_MOD_STATUS_REG: c_uint = 0x40;

pub const II20K_DIO0_REG: c_uint = 0x80;
pub const II20K_DIO1_REG: c_uint = 0x81;
pub const II20K_DIR_ENA_REG: c_uint = 0x82;

pub const II20K_CTRL01_REG: c_uint = 0x83;

pub const II20K_DIO2_REG: c_uint = 0xc0;
pub const II20K_DIO3_REG: c_uint = 0xc1;
pub const II20K_CTRL23_REG: c_uint = 0xc3;

pub const II20K_ID_PCI20006M_1: c_uint = 0xe2	/* 1 AO channels */;
pub const II20K_ID_PCI20006M_2: c_uint = 0xe3	/* 2 AO channels */;

pub const II20K_AO_STRB_BOTH_REG: c_uint = 0x1b;
pub const II20K_ID_PCI20341M_1: c_uint = 0x77	/* 4 AI channels */;
pub const II20K_AI_STATUS_CMD_REG: c_uint = 0x01;

pub const II20K_AI_LSB_REG: c_uint = 0x02;
pub const II20K_AI_MSB_REG: c_uint = 0x03;
pub const II20K_AI_PACER_RESET_REG: c_uint = 0x04;
pub const II20K_AI_16BIT_DATA_REG: c_uint = 0x06;
pub const II20K_AI_CONF_REG: c_uint = 0x10;

pub const II20K_AI_OPT_REG: c_uint = 0x11;

pub const II20K_AI_STATUS_REG: c_uint = 0x12;

pub const II20K_AI_LAST_CHAN_ADDR_REG: c_uint = 0x13;
pub const II20K_AI_CUR_ADDR_REG: c_uint = 0x14;
pub const II20K_AI_SET_TIME_REG: c_uint = 0x15;
pub const II20K_AI_DELAY_LSB_REG: c_uint = 0x16;
pub const II20K_AI_DELAY_MSB_REG: c_uint = 0x17;
pub const II20K_AI_CHAN_ADV_REG: c_uint = 0x18;
pub const II20K_AI_CHAN_RESET_REG: c_uint = 0x19;
pub const II20K_AI_START_TRIG_REG: c_uint = 0x1a;
pub const II20K_AI_COUNT_RESET_REG: c_uint = 0x1b;
pub const II20K_AI_CHANLIST_REG: c_uint = 0x80;

pub const II20K_AI_CHANLIST_LEN: c_uint = 0x80;
// the AO range is set by jumpers on the 20006M module
    static const struct comedi_lrange ii20k_ao_ranges = {
    3, {
    BIP_RANGE(5),	/* Chan 0 - W1/W3 in   Chan 1 - W2/W4 in  */
    UNI_RANGE(10),	/* Chan 0 - W1/W3 out  Chan 1 - W2/W4 in  */
    BIP_RANGE(10)	/* Chan 0 - W1/W3 in   Chan 1 - W2/W4 out */
    }
    };
    static const struct comedi_lrange ii20k_ai_ranges = {
    4, {
    BIP_RANGE(5),		/* gain 1 */
    BIP_RANGE(0.5),		/* gain 10 */
    BIP_RANGE(0.05),	/* gain 100 */
    BIP_RANGE(0.025)	/* gain 200 */
    },
    };
    static void __iomem *ii20k_module_iobase(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    return dev.mmio + (s.index + 1) * II20K_MOD_OFFSET;
    }
    static int ii20k_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    void __iomem *iobase = ii20k_module_iobase(dev, s);
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    int i;
    for (i = 0; i < insn.n; i++) {
    let mut val: c_uint = data[i];
    s.readback[chan] = val;
// munge the offset binary data to 2's complement
    val = comedi_offset_munge(s, val);
    writeb(val & 0xff, iobase + II20K_AO_LSB_REG(chan));
    writeb((val >> 8) & 0xff, iobase + II20K_AO_MSB_REG(chan));
    writeb(0x00, iobase + II20K_AO_STRB_REG(chan));
    }
    return insn.n;
    }
    static int ii20k_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    void __iomem *iobase = ii20k_module_iobase(dev, s);
    unsigned char status;
    status = readb(iobase + II20K_AI_STATUS_REG);
    if ((status & II20K_AI_STATUS_INT) == 0)
    return 0;
    return -EBUSY;
    }
    static void ii20k_ai_setup(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int chanspec)
    {
    void __iomem *iobase = ii20k_module_iobase(dev, s);
    let mut chan: c_uint = CR_CHAN(chanspec);
    let mut range: c_uint = CR_RANGE(chanspec);
    unsigned char val;
// initialize module
    writeb(II20K_AI_CONF_ENA, iobase + II20K_AI_CONF_REG);
// software conversion
    writeb(0, iobase + II20K_AI_STATUS_CMD_REG);
// set the time base for the settling time counter based on the gain
    val = (range < 3) ? II20K_AI_OPT_TIMEBASE(0) : II20K_AI_OPT_TIMEBASE(2);
    writeb(val, iobase + II20K_AI_OPT_REG);
// set the settling time counter based on the gain
    val = (range < 2) ? 0x58 : (range < 3) ? 0x93 : 0x99;
    writeb(val, iobase + II20K_AI_SET_TIME_REG);
// set number of input channels
    writeb(1, iobase + II20K_AI_LAST_CHAN_ADDR_REG);
// set the channel list byte
    val = II20K_AI_CHANLIST_ONBOARD_ONLY |
    II20K_AI_CHANLIST_MUX_ENA |
    II20K_AI_CHANLIST_GAIN(range) |
    II20K_AI_CHANLIST_CHAN(chan);
    writeb(val, iobase + II20K_AI_CHANLIST_REG);
// reset settling time counter and trigger delay counter
    writeb(0, iobase + II20K_AI_COUNT_RESET_REG);
// reset channel scanner
    writeb(0, iobase + II20K_AI_CHAN_RESET_REG);
    }
    static int ii20k_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    void __iomem *iobase = ii20k_module_iobase(dev, s);
    int ret;
    int i;
    ii20k_ai_setup(dev, s, insn.chanspec);
    for (i = 0; i < insn.n; i++) {
    unsigned int val;
// generate a software start convert signal
    readb(iobase + II20K_AI_PACER_RESET_REG);
    ret = comedi_timeout(dev, s, insn, ii20k_ai_eoc, 0);
    if (ret)
    return ret;
    val = readb(iobase + II20K_AI_LSB_REG);
    val |= (readb(iobase + II20K_AI_MSB_REG) << 8);
// munge the 2's complement data to offset binary
    data[i] = comedi_offset_munge(s, val);
    }
    return insn.n;
    }
    static void ii20k_dio_config(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    let mut ctrl01: c_uchar = 0;
    let mut ctrl23: c_uchar = 0;
    let mut dir_ena: c_uchar = 0;
// port 0 - channels 0-7
    if (s.io_bits & 0x000000ff) {
// output port
    ctrl01 &= ~II20K_CTRL01_DIO0_IN;
    dir_ena &= ~II20K_BUF_DISAB_DIO0;
    dir_ena |= II20K_DIR_DIO0_OUT;
    } else {
// input port
    ctrl01 |= II20K_CTRL01_DIO0_IN;
    dir_ena &= ~II20K_DIR_DIO0_OUT;
    }
// port 1 - channels 8-15
    if (s.io_bits & 0x0000ff00) {
// output port
    ctrl01 &= ~II20K_CTRL01_DIO1_IN;
    dir_ena &= ~II20K_BUF_DISAB_DIO1;
    dir_ena |= II20K_DIR_DIO1_OUT;
    } else {
// input port
    ctrl01 |= II20K_CTRL01_DIO1_IN;
    dir_ena &= ~II20K_DIR_DIO1_OUT;
    }
// port 2 - channels 16-23
    if (s.io_bits & 0x00ff0000) {
// output port
    ctrl23 &= ~II20K_CTRL23_DIO2_IN;
    dir_ena &= ~II20K_BUF_DISAB_DIO2;
    dir_ena |= II20K_DIR_DIO2_OUT;
    } else {
// input port
    ctrl23 |= II20K_CTRL23_DIO2_IN;
    dir_ena &= ~II20K_DIR_DIO2_OUT;
    }
// port 3 - channels 24-31
    if (s.io_bits & 0xff000000) {
// output port
    ctrl23 &= ~II20K_CTRL23_DIO3_IN;
    dir_ena &= ~II20K_BUF_DISAB_DIO3;
    dir_ena |= II20K_DIR_DIO3_OUT;
    } else {
// input port
    ctrl23 |= II20K_CTRL23_DIO3_IN;
    dir_ena &= ~II20K_DIR_DIO3_OUT;
    }
    ctrl23 |= II20K_CTRL01_SET;
    ctrl23 |= II20K_CTRL23_SET;
// order is important
    writeb(ctrl01, dev.mmio + II20K_CTRL01_REG);
    writeb(ctrl23, dev.mmio + II20K_CTRL23_REG);
    writeb(dir_ena, dev.mmio + II20K_DIR_ENA_REG);
    }
    static int ii20k_dio_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
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
    ii20k_dio_config(dev, s);
    return insn.n;
    }
    static int ii20k_dio_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    unsigned int mask;
    mask = comedi_dio_update_state(s, data);
    if (mask) {
    if (mask & 0x000000ff)
    writeb((s.state >> 0) & 0xff,
    dev.mmio + II20K_DIO0_REG);
    if (mask & 0x0000ff00)
    writeb((s.state >> 8) & 0xff,
    dev.mmio + II20K_DIO1_REG);
    if (mask & 0x00ff0000)
    writeb((s.state >> 16) & 0xff,
    dev.mmio + II20K_DIO2_REG);
    if (mask & 0xff000000)
    writeb((s.state >> 24) & 0xff,
    dev.mmio + II20K_DIO3_REG);
    }
    data[1] = readb(dev.mmio + II20K_DIO0_REG);
    data[1] |= readb(dev.mmio + II20K_DIO1_REG) << 8;
    data[1] |= readb(dev.mmio + II20K_DIO2_REG) << 16;
    data[1] |= readb(dev.mmio + II20K_DIO3_REG) << 24;
    return insn.n;
    }
    static int ii20k_init_module(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    void __iomem *iobase = ii20k_module_iobase(dev, s);
    unsigned char id;
    int ret;
    id = readb(iobase + II20K_ID_REG);
    switch (id) {
    case II20K_ID_PCI20006M_1:
    case II20K_ID_PCI20006M_2:
// Analog Output subdevice
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= (id == II20K_ID_PCI20006M_2) ? 2 : 1;
    s.maxdata	= 0xffff;
    s.range_table	= &ii20k_ao_ranges;
    s.insn_write	= ii20k_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
    break;
    case II20K_ID_PCI20341M_1:
// Analog Input subdevice
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_DIFF;
    s.n_chan	= 4;
    s.maxdata	= 0xffff;
    s.range_table	= &ii20k_ai_ranges;
    s.insn_read	= ii20k_ai_insn_read;
    break;
    default:
    s.type = COMEDI_SUBD_UNUSED;
    break;
    }
    return 0;
    }
    static int ii20k_attach(struct comedi_device *dev,
    struct comedi_devconfig *it)
    {
    struct comedi_subdevice *s;
    unsigned int membase;
    unsigned char id;
    bool has_dio;
    int ret;
    membase = it.options[0];
    if (!membase || (membase & ~(0x100000 - II20K_SIZE))) {
    dev_warn(dev.class_dev,
    "%s: invalid memory address specified\n",
    dev.board_name);
    return -EINVAL;
    }
    if (!request_mem_region(membase, II20K_SIZE, dev.board_name)) {
    dev_warn(dev.class_dev, "%s: I/O mem conflict (%#x,%u)\n",
    dev.board_name, membase, II20K_SIZE);
    return -EIO;
    }
    dev.iobase = membase;	/* actually, a memory address */
    dev.mmio = ioremap(membase, II20K_SIZE);
    if (!dev.mmio)
    return -ENOMEM;
    id = readb(dev.mmio + II20K_ID_REG);
    switch (id & II20K_ID_MASK) {
    case II20K_ID_PCI20001C_1A:
    has_dio = false;
    break;
    case II20K_ID_PCI20001C_2A:
    has_dio = true;
    break;
    default:
    return -ENODEV;
    }
    ret = comedi_alloc_subdevices(dev, 4);
    if (ret)
    return ret;
    s = &dev.subdevices[0];
    if (id & II20K_ID_MOD1_EMPTY) {
    s.type = COMEDI_SUBD_UNUSED;
    } else {
    ret = ii20k_init_module(dev, s);
    if (ret)
    return ret;
    }
    s = &dev.subdevices[1];
    if (id & II20K_ID_MOD2_EMPTY) {
    s.type = COMEDI_SUBD_UNUSED;
    } else {
    ret = ii20k_init_module(dev, s);
    if (ret)
    return ret;
    }
    s = &dev.subdevices[2];
    if (id & II20K_ID_MOD3_EMPTY) {
    s.type = COMEDI_SUBD_UNUSED;
    } else {
    ret = ii20k_init_module(dev, s);
    if (ret)
    return ret;
    }
// Digital I/O subdevice
    s = &dev.subdevices[3];
    if (has_dio) {
    s.type		= COMEDI_SUBD_DIO;
    s.subdev_flags	= SDF_READABLE | SDF_WRITABLE;
    s.n_chan	= 32;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= ii20k_dio_insn_bits;
    s.insn_config	= ii20k_dio_insn_config;
// default all channels to input
    ii20k_dio_config(dev, s);
    } else {
    s.type = COMEDI_SUBD_UNUSED;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ii20k_detach(dev: *mut comedi_device) {
    static void ii20k_detach(struct comedi_device *dev)
    {
    if (dev.mmio)
    iounmap(dev.mmio);
    if (dev.iobase)	/* actually, a memory address */
    release_mem_region(dev.iobase, II20K_SIZE);
    }
    static struct comedi_driver ii20k_driver = {
    .driver_name	= "ii_pci20kc",
    .module		= THIS_MODULE,
    .attach		= ii20k_attach,
    .detach		= ii20k_detach,
    };
    module_comedi_driver(ii20k_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for Intelligent Instruments PCI-20001C");
    MODULE_LICENSE("GPL");
