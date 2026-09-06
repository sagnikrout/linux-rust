//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/rpckbd.c
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
// Copyright (c) 2000-2001 Vojtech Pavlik
// Copyright (c) 2002 Russell King
//
// Acorn RiscPC PS/2 keyboard controller driver for Linux/ARM
//

    MODULE_AUTHOR("Vojtech Pavlik, Russell King");
    MODULE_DESCRIPTION("Acorn RiscPC PS/2 keyboard controller driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:kart");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpckbd_data {
    pub tx_irq: c_int,
    pub rx_irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn rpckbd_write(port: *mut serio, val: c_uchar) -> c_int {
    static int rpckbd_write(struct serio *port, unsigned char val)
    {
    while (!(iomd_readb(IOMD_KCTRL) & (1 << 7)))
    cpu_relax();
    iomd_writeb(val, IOMD_KARTTX);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpckbd_rx(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rpckbd_rx(int irq, void *dev_id)
    {
    struct serio *port = dev_id;
    unsigned int byte;
    let mut handled: c_int = IRQ_NONE;
    while (iomd_readb(IOMD_KCTRL) & (1 << 5)) {
    byte = iomd_readb(IOMD_KARTRX);
    serio_interrupt(port, byte, 0);
    handled = IRQ_HANDLED;
    }
    return handled;
    }
#[no_mangle]
unsafe extern "C" fn rpckbd_tx(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rpckbd_tx(int irq, void *dev_id)
    {
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rpckbd_open(port: *mut serio) -> c_int {
    static int rpckbd_open(struct serio *port)
    {
    struct rpckbd_data *rpckbd = port.port_data;
// Reset the keyboard state machine.
    iomd_writeb(0, IOMD_KCTRL);
    iomd_writeb(8, IOMD_KCTRL);
    iomd_readb(IOMD_KARTRX);
    if (request_irq(rpckbd.rx_irq, rpckbd_rx, 0, "rpckbd", port) != 0) {
    printk(KERN_ERR "rpckbd.c: Could not allocate keyboard receive IRQ\n");
    return -EBUSY;
    }
    if (request_irq(rpckbd.tx_irq, rpckbd_tx, 0, "rpckbd", port) != 0) {
    printk(KERN_ERR "rpckbd.c: Could not allocate keyboard transmit IRQ\n");
    free_irq(rpckbd.rx_irq, port);
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpckbd_close(port: *mut serio) {
    static void rpckbd_close(struct serio *port)
    {
    struct rpckbd_data *rpckbd = port.port_data;
    free_irq(rpckbd.rx_irq, port);
    free_irq(rpckbd.tx_irq, port);
    }
//
// Allocate and initialize serio structure for subsequent registration
// with serio core.
//
#[no_mangle]
unsafe extern "C" fn rpckbd_probe(dev: *mut platform_device) -> c_int {
    static int rpckbd_probe(struct platform_device *dev)
    {
    struct rpckbd_data *rpckbd;
    struct serio *serio;
    int tx_irq, rx_irq;
    rx_irq = platform_get_irq(dev, 0);
    if (rx_irq < 0)
    return rx_irq;
    tx_irq = platform_get_irq(dev, 1);
    if (tx_irq < 0)
    return tx_irq;
    serio = kzalloc_obj(*serio);
    rpckbd = kzalloc_obj(*rpckbd);
    if (!serio || !rpckbd) {
    kfree(rpckbd);
    kfree(serio);
    return -ENOMEM;
    }
    rpckbd.rx_irq = rx_irq;
    rpckbd.tx_irq = tx_irq;
    serio.id.type		= SERIO_8042;
    serio.write		= rpckbd_write;
    serio.open		= rpckbd_open;
    serio.close		= rpckbd_close;
    serio.dev.parent	= &dev.dev;
    serio.port_data	= rpckbd;
    strscpy(serio.name, "RiscPC PS/2 kbd port", sizeof(serio.name));
    strscpy(serio.phys, "rpckbd/serio0", sizeof(serio.phys));
    platform_set_drvdata(dev, serio);
    serio_register_port(serio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rpckbd_remove(dev: *mut platform_device) {
    static void rpckbd_remove(struct platform_device *dev)
    {
    struct serio *serio = platform_get_drvdata(dev);
    struct rpckbd_data *rpckbd = serio.port_data;
    serio_unregister_port(serio);
    kfree(rpckbd);
    }
    static struct platform_driver rpckbd_driver = {
    .probe		= rpckbd_probe,
    .remove		= rpckbd_remove,
    .driver		= {
    .name	= "kart",
    },
    };
    module_platform_driver(rpckbd_driver);
