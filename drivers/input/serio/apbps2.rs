//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/apbps2.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2013 Aeroflex Gaisler
//
// This driver supports the APBPS2 PS/2 core available in the GRLIB
// VHDL IP core library.
//
// Full documentation of the APBPS2 core can be found here:
// http://www.gaisler.com/products/grlib/grip.pdf
//
// See "Documentation/devicetree/bindings/input/ps2keyb-mouse-apbps2.txt" for
// information on open firmware properties.
//
// Contributors: Daniel Hellstrom <daniel@gaisler.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apbps2_regs {
    pub /: *mut *mut u32 __iomem data; / 0x00,
    pub /: *mut *mut u32 __iomem status; / 0x04,
    pub /: *mut *mut u32 __iomem ctrl; / 0x08,
    pub /: *mut *mut u32 __iomem reload; / 0x0c,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apbps2_priv {
    pub io: *mut serio,
    pub regs: *mut apbps2_regs __iomem,
}

    static int apbps2_idx;
#[no_mangle]
unsafe extern "C" fn apbps2_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t apbps2_isr(int irq, void *dev_id)
    {
    struct apbps2_priv *priv = dev_id;
    unsigned long status, data, rxflags;
    let mut ret: irqreturn_t = IRQ_NONE;
    while ((status = ioread32be(&priv.regs.status)) & APBPS2_STATUS_DR) {
    data = ioread32be(&priv.regs.data);
    rxflags = (status & APBPS2_STATUS_PE) ? SERIO_PARITY : 0;
    rxflags |= (status & APBPS2_STATUS_FE) ? SERIO_FRAME : 0;
// Clear error bits
    if (rxflags)
    iowrite32be(0, &priv.regs.status);
    serio_interrupt(priv.io, data, rxflags);
    ret = IRQ_HANDLED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn apbps2_write(io: *mut serio, val: c_uchar) -> c_int {
    static int apbps2_write(struct serio *io, unsigned char val)
    {
    struct apbps2_priv *priv = io.port_data;
    unsigned int tleft = 10000; /* Timeout in 100ms */
// Delay until PS/2 controller has room for more chars
    while ((ioread32be(&priv.regs.status) & APBPS2_STATUS_TF) && tleft--)
    udelay(10);
    if ((ioread32be(&priv.regs.status) & APBPS2_STATUS_TF) == 0) {
    iowrite32be(val, &priv.regs.data);
    iowrite32be(APBPS2_CTRL_RE | APBPS2_CTRL_RI | APBPS2_CTRL_TE,
    &priv.regs.ctrl);
    return 0;
    }
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn apbps2_open(io: *mut serio) -> c_int {
    static int apbps2_open(struct serio *io)
    {
    struct apbps2_priv *priv = io.port_data;
    int limit;
// Clear error flags
    iowrite32be(0, &priv.regs.status);
// Clear old data if available (unlikely)
    limit = 1024;
    while ((ioread32be(&priv.regs.status) & APBPS2_STATUS_DR) && --limit)
    ioread32be(&priv.regs.data);
// Enable receiver and its interrupt
    iowrite32be(APBPS2_CTRL_RE | APBPS2_CTRL_RI, &priv.regs.ctrl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apbps2_close(io: *mut serio) {
    static void apbps2_close(struct serio *io)
    {
    struct apbps2_priv *priv = io.port_data;
// Stop interrupts at PS/2 HW level
    iowrite32be(0, &priv.regs.ctrl);
    }
// Initialize one APBPS2 PS/2 core
#[no_mangle]
unsafe extern "C" fn apbps2_of_probe(ofdev: *mut platform_device) -> c_int {
    static int apbps2_of_probe(struct platform_device *ofdev)
    {
    struct apbps2_priv *priv;
    int irq, err;
    u32 freq_hz;
    priv = devm_kzalloc(&ofdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv) {
    dev_err(&ofdev.dev, "memory allocation failed\n");
    return -ENOMEM;
    }
// Find device address
    priv.regs = devm_platform_ioremap_resource(ofdev, 0);
    if (IS_ERR(priv.regs))
    return PTR_ERR(priv.regs);
// Reset hardware, disable interrupt
    iowrite32be(0, &priv.regs.ctrl);
// IRQ
    irq = platform_get_irq(ofdev, 0);
    if (irq < 0)
    return irq;
    err = devm_request_irq(&ofdev.dev, irq, apbps2_isr,
    IRQF_SHARED, "apbps2", priv);
    if (err) {
    dev_err(&ofdev.dev, "request IRQ%d failed\n", irq);
    return err;
    }
// Get core frequency
    if (of_property_read_u32(ofdev.dev.of_node, "freq", &freq_hz)) {
    dev_err(&ofdev.dev, "unable to get core frequency\n");
    return -EINVAL;
    }
// Set reload register to core freq in kHz/10
    iowrite32be(freq_hz / 10000, &priv.regs.reload);
    priv.io = kzalloc_obj(*priv.io);
    if (!priv.io)
    return -ENOMEM;
    priv.io.id.type = SERIO_8042;
    priv.io.open = apbps2_open;
    priv.io.close = apbps2_close;
    priv.io.write = apbps2_write;
    priv.io.port_data = priv;
    strscpy(priv.io.name, "APBPS2 PS/2", sizeof(priv.io.name));
    snprintf(priv.io.phys, sizeof(priv.io.phys),
    "apbps2_%d", apbps2_idx++);
    dev_info(&ofdev.dev, "irq = %d, base = 0x%p\n", irq, priv.regs);
    serio_register_port(priv.io);
    platform_set_drvdata(ofdev, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apbps2_of_remove(of_dev: *mut platform_device) {
    static void apbps2_of_remove(struct platform_device *of_dev)
    {
    struct apbps2_priv *priv = platform_get_drvdata(of_dev);
    serio_unregister_port(priv.io);
    }
    static const struct of_device_id apbps2_of_match[] = {
    { .name = "GAISLER_APBPS2", },
    { .name = "01_060", },
    {}
    };
    MODULE_DEVICE_TABLE(of, apbps2_of_match);
    static struct platform_driver apbps2_of_driver = {
    .driver = {
    .name = "grlib-apbps2",
    .of_match_table = apbps2_of_match,
    },
    .probe = apbps2_of_probe,
    .remove = apbps2_of_remove,
    };
    module_platform_driver(apbps2_of_driver);
    MODULE_AUTHOR("Aeroflex Gaisler AB.");
    MODULE_DESCRIPTION("GRLIB APBPS2 PS/2 serial I/O");
    MODULE_LICENSE("GPL");
