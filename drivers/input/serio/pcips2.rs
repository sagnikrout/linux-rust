//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/pcips2.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/drivers/input/serio/pcips2.c
//
// Copyright (C) 2003 Russell King, All Rights Reserved.
//
// I'm not sure if this is a generic PS/2 PCI interface or specific to
// the Mobility Electronics docking station.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcips2_data {
    pub io: *mut serio,
    pub base: c_uint,
    pub dev: *mut pci_dev,
}

#[no_mangle]
unsafe extern "C" fn pcips2_write(io: *mut serio, val: c_uchar) -> c_int {
    static int pcips2_write(struct serio *io, unsigned char val)
    {
    struct pcips2_data *ps2if = io.port_data;
    unsigned int stat;
    do {
    stat = inb(ps2if.base + PS2_STATUS);
    cpu_relax();
    } while (!(stat & PS2_STAT_TXEMPTY));
    outb(val, ps2if.base + PS2_DATA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcips2_interrupt(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t pcips2_interrupt(int irq, void *devid)
    {
    struct pcips2_data *ps2if = devid;
    unsigned char status, scancode;
    let mut handled: c_int = 0;
    do {
    unsigned int flag;
    status = inb(ps2if.base + PS2_STATUS);
    if (!(status & PS2_STAT_RXFULL))
    break;
    handled = 1;
    scancode = inb(ps2if.base + PS2_DATA);
    if (status == 0xff && scancode == 0xff)
    break;
    flag = (status & PS2_STAT_PARITY) ? 0 : SERIO_PARITY;
    if (hweight8(scancode) & 1)
    flag ^= SERIO_PARITY;
    serio_interrupt(ps2if.io, scancode, flag);
    } while (1);
    return IRQ_RETVAL(handled);
    }
#[no_mangle]
unsafe extern "C" fn pcips2_flush_input(ps2if: *mut pcips2_data) {
    static void pcips2_flush_input(struct pcips2_data *ps2if)
    {
    unsigned char status, scancode;
    do {
    status = inb(ps2if.base + PS2_STATUS);
    if (!(status & PS2_STAT_RXFULL))
    break;
    scancode = inb(ps2if.base + PS2_DATA);
    if (status == 0xff && scancode == 0xff)
    break;
    } while (1);
    }
#[no_mangle]
unsafe extern "C" fn pcips2_open(io: *mut serio) -> c_int {
    static int pcips2_open(struct serio *io)
    {
    struct pcips2_data *ps2if = io.port_data;
    int ret, val = 0;
    outb(PS2_CTRL_ENABLE, ps2if.base);
    pcips2_flush_input(ps2if);
    ret = request_irq(ps2if.dev.irq, pcips2_interrupt, IRQF_SHARED,
    "pcips2", ps2if);
    if (ret == 0)
    val = PS2_CTRL_ENABLE | PS2_CTRL_RXIRQ;
    outb(val, ps2if.base);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pcips2_close(io: *mut serio) {
    static void pcips2_close(struct serio *io)
    {
    struct pcips2_data *ps2if = io.port_data;
    outb(0, ps2if.base);
    free_irq(ps2if.dev.irq, ps2if);
    }
#[no_mangle]
unsafe extern "C" fn pcips2_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int pcips2_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    struct pcips2_data *ps2if;
    struct serio *serio;
    int ret;
    ret = pci_enable_device(dev);
    if (ret)
    goto out;
    ret = pci_request_regions(dev, "pcips2");
    if (ret)
    goto disable;
    ps2if = kzalloc_obj(*ps2if);
    serio = kzalloc_obj(*serio);
    if (!ps2if || !serio) {
    ret = -ENOMEM;
    goto release;
    }
    serio.id.type		= SERIO_8042;
    serio.write		= pcips2_write;
    serio.open		= pcips2_open;
    serio.close		= pcips2_close;
    strscpy(serio.name, pci_name(dev), sizeof(serio.name));
    strscpy(serio.phys, dev_name(&dev.dev), sizeof(serio.phys));
    serio.port_data	= ps2if;
    serio.dev.parent	= &dev.dev;
    ps2if.io		= serio;
    ps2if.dev		= dev;
    ps2if.base		= pci_resource_start(dev, 0);
    pci_set_drvdata(dev, ps2if);
    serio_register_port(ps2if.io);
    return 0;
    release:
    kfree(ps2if);
    kfree(serio);
    pci_release_regions(dev);
    disable:
    pci_disable_device(dev);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pcips2_remove(dev: *mut pci_dev) {
    static void pcips2_remove(struct pci_dev *dev)
    {
    struct pcips2_data *ps2if = pci_get_drvdata(dev);
    serio_unregister_port(ps2if.io);
    kfree(ps2if);
    pci_release_regions(dev);
    pci_disable_device(dev);
    }
    static const struct pci_device_id pcips2_ids[] = {
    {
    .vendor		= 0x14f2,	/* MOBILITY */
    .device		= 0x0123,	/* Keyboard */
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    .class		= PCI_CLASS_INPUT_KEYBOARD << 8,
    .class_mask	= 0xffff00,
    },
    {
    .vendor		= 0x14f2,	/* MOBILITY */
    .device		= 0x0124,	/* Mouse */
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    .class		= PCI_CLASS_INPUT_MOUSE << 8,
    .class_mask	= 0xffff00,
    },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, pcips2_ids);
    static struct pci_driver pcips2_driver = {
    .name			= "pcips2",
    .id_table		= pcips2_ids,
    .probe			= pcips2_probe,
    .remove			= pcips2_remove,
    };
    module_pci_driver(pcips2_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Russell King <rmk@arm.linux.org.uk>");
    MODULE_DESCRIPTION("PCI PS/2 keyboard/mouse driver");
