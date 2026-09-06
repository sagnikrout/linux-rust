//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/icp_multi.c
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
// icp_multi.c
// Comedi driver for Inova ICP_MULTI board
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1997-2002 David A. Schleef <ds@schleef.org>
//
// Driver: icp_multi
// Description: Inova ICP_MULTI
// Devices: [Inova] ICP_MULTI (icp_multi)
// Author: Anne Smorthit <anne.smorthit@sfwte.ch>
// Status: works
//
// Configuration options: not applicable, uses PCI auto config
//
// The driver works for analog input and output and digital input and
// output. It does not work with interrupts or with the counters. Currently
// no support for DMA.
//
// It has 16 single-ended or 8 differential Analogue Input channels with
// 12-bit resolution.  Ranges : 5V, 10V, +/-5V, +/-10V, 0..20mA and 4..20mA.
// Input ranges can be individually programmed for each channel.  Voltage or
// current measurement is selected by jumper.
//
// There are 4 x 12-bit Analogue Outputs.  Ranges : 5V, 10V, +/-5V, +/-10V
//
// 16 x Digital Inputs, 24V
//
// 8 x Digital Outputs, 24V, 1A
//
// 4 x 16-bit counters - not implemented
//

pub const ICP_MULTI_ADC_CSR: c_uint = 0x00	/* R/W: ADC command/status register */;

pub const ICP_MULTI_DAC_CSR: c_uint = 0x04	/* R/W: DAC command/status register */;

pub const ICP_MULTI_DO: c_uint = 0x0A	/* R/W: Digital outputs */;
pub const ICP_MULTI_INT_EN: c_uint = 0x0c	/* R/W: Interrupt enable register */;
pub const ICP_MULTI_INT_STAT: c_uint = 0x0e	/* R/W: Interrupt status register */;

pub const ICP_MULTI_INT_MASK: c_uint = 0xff	/* All interrupts */;
pub const ICP_MULTI_CNTR0: c_uint = 0x10	/* R/W: Counter 0 */;
pub const ICP_MULTI_CNTR1: c_uint = 0x12	/* R/W: counter 1 */;
pub const ICP_MULTI_CNTR2: c_uint = 0x14	/* R/W: Counter 2 */;
pub const ICP_MULTI_CNTR3: c_uint = 0x16	/* R/W: Counter 3 */;
// analog input and output have the same range options
    static const struct comedi_lrange icp_multi_ranges = {
    4, {
    UNI_RANGE(5),
    UNI_RANGE(10),
    BIP_RANGE(5),
    BIP_RANGE(10)
    }
    };
    static const char range_codes_analog[] = { 0x00, 0x20, 0x10, 0x30 };
    static int icp_multi_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned int status;
    status = readw(dev.mmio + ICP_MULTI_ADC_CSR);
    if ((status & ICP_MULTI_ADC_CSR_BSY) == 0)
    return 0;
    return -EBUSY;
    }
    static int icp_multi_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut range: c_uint = CR_RANGE(insn.chanspec);
    let mut aref: c_uint = CR_AREF(insn.chanspec);
    unsigned int adc_csr;
    let mut ret: c_int = 0;
    int n;
// Set mode and range data for specified channel
    if (aref == AREF_DIFF) {
    adc_csr = ICP_MULTI_ADC_CSR_DI_CHAN(chan) |
    ICP_MULTI_ADC_CSR_DI;
    } else {
    adc_csr = ICP_MULTI_ADC_CSR_SE_CHAN(chan);
    }
    adc_csr |= range_codes_analog[range];
    writew(adc_csr, dev.mmio + ICP_MULTI_ADC_CSR);
    for (n = 0; n < insn.n; n++) {
// Set start ADC bit
    writew(adc_csr | ICP_MULTI_ADC_CSR_ST,
    dev.mmio + ICP_MULTI_ADC_CSR);
    udelay(1);
// Wait for conversion to complete, or get fed up waiting
    ret = comedi_timeout(dev, s, insn, icp_multi_ai_eoc, 0);
    if (ret)
    break;
    data[n] = (readw(dev.mmio + ICP_MULTI_AI) >> 4) & 0x0fff;
    }
    return ret ? ret : n;
    }
    static int icp_multi_ao_ready(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    unsigned int status;
    status = readw(dev.mmio + ICP_MULTI_DAC_CSR);
    if ((status & ICP_MULTI_DAC_CSR_BSY) == 0)
    return 0;
    return -EBUSY;
    }
    static int icp_multi_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut range: c_uint = CR_RANGE(insn.chanspec);
    unsigned int dac_csr;
    int i;
// Select channel and range
    dac_csr = ICP_MULTI_DAC_CSR_CHAN(chan);
    dac_csr |= range_codes_analog[range];
    writew(dac_csr, dev.mmio + ICP_MULTI_DAC_CSR);
    for (i = 0; i < insn.n; i++) {
    let mut val: c_uint = data[i];
    int ret;
// Wait for analog output to be ready for new data
    ret = comedi_timeout(dev, s, insn, icp_multi_ao_ready, 0);
    if (ret)
    return ret;
    writew(val, dev.mmio + ICP_MULTI_AO);
// Set start conversion bit to write data to channel
    writew(dac_csr | ICP_MULTI_DAC_CSR_ST,
    dev.mmio + ICP_MULTI_DAC_CSR);
    s.readback[chan] = val;
    }
    return insn.n;
    }
    static int icp_multi_di_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    data[1] = readw(dev.mmio + ICP_MULTI_DI);
    return insn.n;
    }
    static int icp_multi_do_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    if (comedi_dio_update_state(s, data))
    writew(s.state, dev.mmio + ICP_MULTI_DO);
    data[1] = s.state;
    return insn.n;
    }
#[no_mangle]
unsafe extern "C" fn icp_multi_reset(dev: *mut comedi_device) -> c_int {
    static int icp_multi_reset(struct comedi_device *dev)
    {
    int i;
// Disable all interrupts and clear any requests
    writew(0, dev.mmio + ICP_MULTI_INT_EN);
    writew(ICP_MULTI_INT_MASK, dev.mmio + ICP_MULTI_INT_STAT);
// Reset the analog output channels to 0V
    for (i = 0; i < 4; i++) {
    let mut dac_csr: c_uint = ICP_MULTI_DAC_CSR_CHAN(i);
// Select channel and 0..5V range
    writew(dac_csr, dev.mmio + ICP_MULTI_DAC_CSR);
// Output 0V
    writew(0, dev.mmio + ICP_MULTI_AO);
// Set start conversion bit to write data to channel
    writew(dac_csr | ICP_MULTI_DAC_CSR_ST,
    dev.mmio + ICP_MULTI_DAC_CSR);
    udelay(1);
    }
// Digital outputs to 0
    writew(0, dev.mmio + ICP_MULTI_DO);
    return 0;
    }
    static int icp_multi_auto_attach(struct comedi_device *dev,
    unsigned long context_unused)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    struct comedi_subdevice *s;
    int ret;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    dev.mmio = pci_ioremap_bar(pcidev, 2);
    if (!dev.mmio)
    return -ENOMEM;
    ret = comedi_alloc_subdevices(dev, 4);
    if (ret)
    return ret;
    icp_multi_reset(dev);
// Analog Input subdevice
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_AI;
    s.subdev_flags	= SDF_READABLE | SDF_COMMON | SDF_GROUND | SDF_DIFF;
    s.n_chan	= 16;
    s.maxdata	= 0x0fff;
    s.range_table	= &icp_multi_ranges;
    s.insn_read	= icp_multi_ai_insn_read;
// Analog Output subdevice
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_AO;
    s.subdev_flags	= SDF_WRITABLE | SDF_GROUND | SDF_COMMON;
    s.n_chan	= 4;
    s.maxdata	= 0x0fff;
    s.range_table	= &icp_multi_ranges;
    s.insn_write	= icp_multi_ao_insn_write;
    ret = comedi_alloc_subdev_readback(s);
    if (ret)
    return ret;
// Digital Input subdevice
    s = &dev.subdevices[2];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= 16;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= icp_multi_di_insn_bits;
// Digital Output subdevice
    s = &dev.subdevices[3];
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= 8;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= icp_multi_do_insn_bits;
    return 0;
    }
    static struct comedi_driver icp_multi_driver = {
    .driver_name	= "icp_multi",
    .module		= THIS_MODULE,
    .auto_attach	= icp_multi_auto_attach,
    .detach		= comedi_pci_detach,
    };
    static int icp_multi_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &icp_multi_driver, id.driver_data);
    }
    static const struct pci_device_id icp_multi_pci_table[] = {
    { PCI_VDEVICE(ICP, 0x8000) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, icp_multi_pci_table);
    static struct pci_driver icp_multi_pci_driver = {
    .name		= "icp_multi",
    .id_table	= icp_multi_pci_table,
    .probe		= icp_multi_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(icp_multi_driver, icp_multi_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for Inova ICP_MULTI board");
    MODULE_LICENSE("GPL");
