//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/adl_pci7250.c
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
// adl_pci7250.c
//
// Comedi driver for ADLink PCI-7250 series cards.
//
// Copyright (C) 2015, 2025 Ian Abbott <abbotti@mev.co.uk>
//
// Driver: adl_pci7250
// Description: Driver for the ADLINK PCI-7250 relay output & digital input card
// Devices: [ADLINK] PCI-7250 (adl_pci7250) LPCI-7250 LPCIe-7250
// Author: Ian Abbott <abbotti@mev.co.uk>
// Status: works
// Updated: Mon, 02 Jun 2025 13:54:11 +0100
//
// The driver assumes that 3 PCI-7251 modules are fitted to the PCI-7250,
// giving 32 channels of relay outputs and 32 channels of isolated digital
// inputs.  That is also the case for the LPCI-7250 and older LPCIe-7250
// cards although they do not physically support the PCI-7251 modules.
// Newer LPCIe-7250 cards have a different PCI subsystem device ID, so
// set the number of channels to 8 for these cards.
//
// Not fitting the PCI-7251 modules shouldn't do any harm, but the extra
// inputs and relay outputs won't work!
//
// Configuration Options: not applicable, uses PCI auto config
//

    static unsigned char adl_pci7250_read8(struct comedi_device *dev,
    unsigned int offset)
    {

    if (!dev.mmio)
    return inb(dev.iobase + offset);

    return readb(dev.mmio + offset);
    }
    static void adl_pci7250_write8(struct comedi_device *dev, unsigned int offset,
    unsigned char val)
    {

    if (!dev.mmio) {
    outb(val, dev.iobase + offset);
    return;
    }

    writeb(val, dev.mmio + offset);
    }
    static int adl_pci7250_do_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut mask: c_uint = comedi_dio_update_state(s, data);
    if (mask) {
    let mut state: c_uint = s.state;
    unsigned int i;
    for (i = 0; i * 8 < s.n_chan; i++) {
    if ((mask & 0xffu) != 0) {
// write relay data to even offset registers
    adl_pci7250_write8(dev, i * 2, state & 0xffu);
    }
    state >>= 8;
    mask >>= 8;
    }
    }
    data[1] = s.state;
    return 2;
    }
    static int adl_pci7250_di_insn_bits(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    let mut value: c_uint = 0;
    unsigned int i;
    for (i = 0; i * 8 < s.n_chan; i++) {
// read DI value from odd offset registers
    value |= (unsigned int)adl_pci7250_read8(dev, i * 2 + 1) <<
    (i * 8);
    }
    data[1] = value;
    return 2;
    }
    static int pci7250_auto_attach(struct comedi_device *dev,
    unsigned long context_unused)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    struct comedi_subdevice *s;
    unsigned int max_chans;
    unsigned int i;
    int ret;
    ret = comedi_pci_enable(dev);
    if (ret)
    return ret;
    if (pci_resource_len(pcidev, 2) < 8)
    return -ENXIO;
//
// Newer LPCIe-7250 boards use MMIO.  Older LPCIe-7250, LPCI-7250, and
// PCI-7250 boards use Port I/O.
//
    if (pci_resource_flags(pcidev, 2) & IORESOURCE_MEM) {
    dev.mmio = pci_ioremap_bar(pcidev, 2);
    if (!dev.mmio)
    return -ENOMEM;
    } else if (IS_ENABLED(CONFIG_HAS_IOPORT)) {
    dev.iobase = pci_resource_start(pcidev, 2);
    } else {
    dev_err(dev.class_dev,
    "error! need I/O port support\n");
    return -ENXIO;
    }
    if (pcidev.subsystem_device == 0x7000) {
//
// This is a newer LPCIe-7250 variant and cannot possibly
// have PCI-7251 modules fitted, so limit the number of
// channels to 8.
//
    max_chans = 8;
    } else {
//
// It is unknown whether the board is a PCI-7250, an LPCI-7250,
// or an older LPCIe-7250 variant, so treat it as a PCI-7250
// and assume it can have PCI-7251 modules fitted to increase
// the number of channels to a maximum of 32.
//
    max_chans = 32;
    }
    ret = comedi_alloc_subdevices(dev, 2);
    if (ret)
    return ret;
// Relay digital output.
    s = &dev.subdevices[0];
    s.type		= COMEDI_SUBD_DO;
    s.subdev_flags	= SDF_WRITABLE;
    s.n_chan	= max_chans;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits	= adl_pci7250_do_insn_bits;
// Read initial state of relays from the even offset registers.
    s.state = 0;
    for (i = 0; i * 8 < max_chans; i++) {
    s.state |= (unsigned int)adl_pci7250_read8(dev, i * 2) <<
    (i * 8);
    }
// Isolated digital input.
    s = &dev.subdevices[1];
    s.type		= COMEDI_SUBD_DI;
    s.subdev_flags	= SDF_READABLE;
    s.n_chan	= max_chans;
    s.maxdata	= 1;
    s.range_table	= &range_digital;
    s.insn_bits = adl_pci7250_di_insn_bits;
    return 0;
    }
    static struct comedi_driver adl_pci7250_driver = {
    .driver_name	= "adl_pci7250",
    .module		= THIS_MODULE,
    .auto_attach	= pci7250_auto_attach,
    .detach		= comedi_pci_detach,
    };
    static int adl_pci7250_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &adl_pci7250_driver,
    id.driver_data);
    }
    static const struct pci_device_id adl_pci7250_pci_table[] = {

    { PCI_VDEVICE_SUB(PLX, PCI_DEVICE_ID_PLX_9050,
    0x9999, 0x7250) },
    { PCI_VDEVICE_SUB(ADLINK, 0x7250,
    0x9999, 0x7250) },
    { PCI_VDEVICE_SUB(ADLINK, 0x7250,
    PCI_VENDOR_ID_ADLINK, 0x7250) },

    { PCI_VDEVICE_SUB(ADLINK, 0x7250,
    PCI_VENDOR_ID_ADLINK, 0x7000) }, /* newer LPCIe-7250 */
    { }
    };
    MODULE_DEVICE_TABLE(pci, adl_pci7250_pci_table);
    static struct pci_driver adl_pci7250_pci_driver = {
    .name		= "adl_pci7250",
    .id_table	= adl_pci7250_pci_table,
    .probe		= adl_pci7250_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(adl_pci7250_driver, adl_pci7250_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for ADLink PCI-7250 series boards");
    MODULE_LICENSE("GPL");
