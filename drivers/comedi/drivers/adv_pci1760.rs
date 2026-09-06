//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/adv_pci1760.c
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
// COMEDI driver for the Advantech PCI-1760
// Copyright (C) 2015 H Hartley Sweeten <hsweeten@visionengravers.com>
//
// Based on the pci1760 support in the adv_pci_dio driver written by:
// Michal Dobes <dobes@tesnet.cz>
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//
// Driver: adv_pci1760
// Description: Advantech PCI-1760 Relay & Isolated Digital Input Card
// Devices: [Advantech] PCI-1760 (adv_pci1760)
// Author: H Hartley Sweeten <hsweeten@visionengravers.com>
// Updated: Fri, 13 Nov 2015 12:34:00 -0700
// Status: untested
//
// Configuration Options: not applicable, uses PCI auto config
//

//
// PCI-1760 Register Map
//
// Outgoing Mailbox Bytes
// OMB3: Not used (must be 0)
// OMB2: The command code to the PCI-1760
// OMB1: The hi byte of the parameter for the command in OMB2
// OMB0: The lo byte of the parameter for the command in OMB2
//
// Incoming Mailbox Bytes
// IMB3: The Isolated Digital Input status (updated every 100us)
// IMB2: The current command (matches OMB2 when command is successful)
// IMB1: The hi byte of the feedback data for the command in OMB2
// IMB0: The lo byte of the feedback data for the command in OMB2
//
// Interrupt Control/Status
// INTCSR3: Not used (must be 0)
// INTCSR2: The interrupt status (read only)
// INTCSR1: Interrupt enable/disable
// INTCSR0: Not used (must be 0)
//

// PCI-1760 command codes
pub const PCI1760_CMD_CLR_IMB2: c_uint = 0x00	/* Clears IMB2 */;
pub const PCI1760_CMD_SET_DO: c_uint = 0x01	/* Set output state */;
pub const PCI1760_CMD_GET_DO: c_uint = 0x02	/* Read output status */;
pub const PCI1760_CMD_GET_STATUS: c_uint = 0x07	/* Read current status */;
pub const PCI1760_CMD_GET_FW_VER: c_uint = 0x0e	/* Read firmware version */;
pub const PCI1760_CMD_GET_HW_VER: c_uint = 0x0f	/* Read hardware version */;

pub const PCI1760_CMD_ENA_PWM: c_uint = 0x1f	/* Enable PWM outputs */;
pub const PCI1760_CMD_ENA_FILT: c_uint = 0x20	/* Enable input filter */;
pub const PCI1760_CMD_ENA_PAT_MATCH: c_uint = 0x21	/* Enable input pattern match */;
pub const PCI1760_CMD_SET_PAT_MATCH: c_uint = 0x22	/* Set input pattern match */;
pub const PCI1760_CMD_ENA_RISE_EDGE: c_uint = 0x23	/* Enable input rising edge */;
pub const PCI1760_CMD_ENA_FALL_EDGE: c_uint = 0x24	/* Enable input falling edge */;
pub const PCI1760_CMD_ENA_CNT: c_uint = 0x28	/* Enable counter */;
pub const PCI1760_CMD_RST_CNT: c_uint = 0x29	/* Reset counter */;
pub const PCI1760_CMD_ENA_CNT_OFLOW: c_uint = 0x2a	/* Enable counter overflow */;
pub const PCI1760_CMD_ENA_CNT_MATCH: c_uint = 0x2b	/* Enable counter match */;
pub const PCI1760_CMD_SET_CNT_EDGE: c_uint = 0x2c	/* Set counter edge */;
pub const PCI1760_CMD_GET_CNT: c_uint = 0x2f	/* Reads counter value */;

pub const PCI1760_CMD_GET_INT_FLAGS: c_uint = 0x60	/* Read interrupt flags */;

pub const PCI1760_CMD_GET_OS: c_uint = 0x61	/* Read edge change flags */;
pub const PCI1760_CMD_GET_CNT_STATUS: c_uint = 0x62	/* Read counter oflow/match */;

    static int pci1760_send_cmd(struct comedi_device *dev,
    unsigned char cmd, unsigned short val)
    {
    unsigned long timeout;
// send the command and parameter
    outb(val & 0xff, dev.iobase + PCI1760_OMB_REG(0));
    outb((val >> 8) & 0xff, dev.iobase + PCI1760_OMB_REG(1));
    outb(cmd, dev.iobase + PCI1760_OMB_REG(2));
    outb(0, dev.iobase + PCI1760_OMB_REG(3));
// datasheet says to allow up to 250 usec for the command to complete
    timeout = jiffies + usecs_to_jiffies(PCI1760_CMD_TIMEOUT);
    do {
    if (inb(dev.iobase + PCI1760_IMB_REG(2)) == cmd) {
// command success; return the feedback data
    return inb(dev.iobase + PCI1760_IMB_REG(0)) |
    (inb(dev.iobase + PCI1760_IMB_REG(1)) << 8);
    }
    cpu_relax();
    } while (time_before(jiffies, timeout));
    return -EBUSY;
    }
    static int pci1760_cmd(struct comedi_device *dev,
    unsigned char cmd, unsigned short val)
    {
    int repeats;
    int ret;
// send PCI1760_CMD_CLR_IMB2 between identical commands
    if (inb(dev.iobase + PCI1760_IMB_REG(2)) == cmd) {
    ret = pci1760_send_cmd(dev, PCI1760_CMD_CLR_IMB2, 0);
    if (ret < 0) {
// timeout? try it once more
    ret = pci1760_send_cmd(dev, PCI1760_CMD_CLR_IMB2, 0);
    if (ret < 0)
    return -ETIMEDOUT;
    }
    }
// datasheet says to keep retrying the command
    for (repeats = 0; repeats < PCI1760_CMD_RETRIES; repeats++) {
    ret = pci1760_send_cmd(dev, cmd, val);
    if (ret >= 0)
    return ret;
    }
// command failed!
    return -ETIMEDOUT;
    }
    static int pci1760_di_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    data[1] = inb(dev.iobase + PCI1760_IMB_REG(3));
    return insn.n;
    }
    static int pci1760_do_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    int ret;
    if (comedi_dio_update_state(s, data)) {
    ret = pci1760_cmd(dev, PCI1760_CMD_SET_DO, s.state);
    if (ret < 0)
    return ret;
    }
    data[1] = s.state;
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn pci1760_pwm_ns_to_div(flags: c_uint, ns: c_uint) -> c_int {
    static int pci1760_pwm_ns_to_div(unsigned int flags, unsigned int ns)
    {
    unsigned int divisor;
    switch (flags) {
    case CMDF_ROUND_NEAREST:
    divisor = DIV_ROUND_CLOSEST(ns, PCI1760_PWM_TIMEBASE);
    break;
    case CMDF_ROUND_UP:
    divisor = DIV_ROUND_UP(ns, PCI1760_PWM_TIMEBASE);
    break;
    case CMDF_ROUND_DOWN:
    divisor = ns / PCI1760_PWM_TIMEBASE;
    break;
    default:
    return -EINVAL;
    }
    if (divisor < 1)
    divisor = 1;
    if (divisor > 0xffff)
    divisor = 0xffff;
    return divisor;
    }
    static int pci1760_pwm_enable(struct comedi_device *dev,
    unsigned int chan, bool enable)
    {
    int ret;
    ret = pci1760_cmd(dev, PCI1760_CMD_GET_STATUS, PCI1760_CMD_ENA_PWM);
    if (ret < 0)
    return ret;
    if (enable)
    ret |= BIT(chan);
    else
    ret &= ~BIT(chan);
    return pci1760_cmd(dev, PCI1760_CMD_ENA_PWM, ret);
    }
    static int pci1760_pwm_insn_config(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    int hi_div;
    int lo_div;
    int ret;
    switch (data[0]) {
    case INSN_CONFIG_ARM:
    ret = pci1760_pwm_enable(dev, chan, false);
    if (ret < 0)
    return ret;
    if (data[1] > 0xffff)
    return -EINVAL;
    ret = pci1760_cmd(dev, PCI1760_CMD_SET_PWM_CNT(chan), data[1]);
    if (ret < 0)
    return ret;
    ret = pci1760_pwm_enable(dev, chan, true);
    if (ret < 0)
    return ret;
    break;
    case INSN_CONFIG_DISARM:
    ret = pci1760_pwm_enable(dev, chan, false);
    if (ret < 0)
    return ret;
    break;
    case INSN_CONFIG_PWM_OUTPUT:
    ret = pci1760_pwm_enable(dev, chan, false);
    if (ret < 0)
    return ret;
    hi_div = pci1760_pwm_ns_to_div(data[1], data[2]);
    lo_div = pci1760_pwm_ns_to_div(data[3], data[4]);
    if (hi_div < 0 || lo_div < 0)
    return -EINVAL;
    if ((hi_div * PCI1760_PWM_TIMEBASE) != data[2] ||
    (lo_div * PCI1760_PWM_TIMEBASE) != data[4]) {
    data[2] = hi_div * PCI1760_PWM_TIMEBASE;
    data[4] = lo_div * PCI1760_PWM_TIMEBASE;
    return -EAGAIN;
    }
    ret = pci1760_cmd(dev, PCI1760_CMD_SET_PWM_HI(chan), hi_div);
    if (ret < 0)
    return ret;
    ret = pci1760_cmd(dev, PCI1760_CMD_SET_PWM_LO(chan), lo_div);
    if (ret < 0)
    return ret;
    break;
    case INSN_CONFIG_GET_PWM_OUTPUT:
    hi_div = pci1760_cmd(dev, PCI1760_CMD_GET_STATUS,
    PCI1760_CMD_SET_PWM_HI(chan));
    lo_div = pci1760_cmd(dev, PCI1760_CMD_GET_STATUS,
    PCI1760_CMD_SET_PWM_LO(chan));
    if (hi_div < 0 || lo_div < 0)
    return -ETIMEDOUT;
    data[1] = hi_div * PCI1760_PWM_TIMEBASE;
    data[2] = lo_div * PCI1760_PWM_TIMEBASE;
    break;
    case INSN_CONFIG_GET_PWM_STATUS:
    ret = pci1760_cmd(dev, PCI1760_CMD_GET_STATUS,
    PCI1760_CMD_ENA_PWM);
    if (ret < 0)
    return ret;
    data[1] = (ret & BIT(chan)) ? 1 : 0;
    break;
    default:
    return -EINVAL;
    }
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn pci1760_reset(dev: *mut comedi_device) {
    static void pci1760_reset(struct comedi_device *dev)
    {
    int i;
// disable interrupts (intcsr2 is read-only)
    outb(0, dev.iobase + PCI1760_INTCSR_REG(0));
    outb(0, dev.iobase + PCI1760_INTCSR_REG(1));
    outb(0, dev.iobase + PCI1760_INTCSR_REG(3));
// disable counters
    pci1760_cmd(dev, PCI1760_CMD_ENA_CNT, 0);
// disable overflow interrupts
    pci1760_cmd(dev, PCI1760_CMD_ENA_CNT_OFLOW, 0);
// disable match
    pci1760_cmd(dev, PCI1760_CMD_ENA_CNT_MATCH, 0);
// set match and counter reset values
    for (i = 0; i < 8; i++) {
    pci1760_cmd(dev, PCI1760_CMD_SET_CNT_MATCH(i), 0x8000);
    pci1760_cmd(dev, PCI1760_CMD_SET_CNT(i), 0x0000);
    }
// reset counters to reset values
    pci1760_cmd(dev, PCI1760_CMD_RST_CNT, 0xff);
// set counter count edges
    pci1760_cmd(dev, PCI1760_CMD_SET_CNT_EDGE, 0);
// disable input filters
    pci1760_cmd(dev, PCI1760_CMD_ENA_FILT, 0);
// disable pattern matching
    pci1760_cmd(dev, PCI1760_CMD_ENA_PAT_MATCH, 0);
// set pattern match value
    pci1760_cmd(dev, PCI1760_CMD_SET_PAT_MATCH, 0);
    }
    static int pci1760_auto_attach(struct comedi_device *dev,
    unsigned long context)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    struct comedi_subdevice *s;
    int ret;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    dev.iobase = pci_resource_start(pcidev, 0);
    pci1760_reset(dev);
    ret = comedi_alloc_subdevices(dev, 4);
    if (ret)
    return ret;
// Digital Input subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= pci1760_di_insn_bits;
// Digital Output subdevice
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= pci1760_do_insn_bits;
// get the current state of the outputs
    ret = pci1760_cmd(dev, PCI1760_CMD_GET_DO, 0);
    if (ret < 0)
    return ret;
    s.state	= ret;
// PWM subdevice
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_PWM;
    s.subdev_flags	= SDF_PWM_COUNTER;
    s.n_chan	= 2;
    s.insn_config	= pci1760_pwm_insn_config;
// Counter subdevice
    s = &dev.subdevices[3];
    s.type		= COMEDI_SUBD_UNUSED;
    return 0;
    }
    static struct comedi_driver pci1760_driver = {
    .driver_name	= "adv_pci1760",
    .module		= THIS_MODULE,
    .auto_attach	= pci1760_auto_attach,
    .detach		= comedi_pci_detach,
    };
    static int pci1760_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &pci1760_driver, id.driver_data);
    }
    static const struct pci_device_id pci1760_pci_table[] = {
    { PCI_VDEVICE(ADVANTECH, 0x1760) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, pci1760_pci_table);
    static struct pci_driver pci1760_pci_driver = {
    .name		= "adv_pci1760",
    .id_table	= pci1760_pci_table,
    .probe		= pci1760_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(pci1760_driver, pci1760_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for Advantech PCI-1760");
    MODULE_LICENSE("GPL");
