//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/maceps2.c
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
// SGI O2 MACE PS2 controller driver for linux
//
// Copyright (C) 2002 Vivien Chappelier
//

    MODULE_AUTHOR("Vivien Chappelier <vivien.chappelier@linux-mips.org");
    MODULE_DESCRIPTION("SGI O2 MACE PS2 controller driver");
    MODULE_LICENSE("GPL");

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maceps2_data {
    pub port: *mut mace_ps2port,
    pub irq: c_int,
}

    static struct maceps2_data port_data[2];
    static struct serio *maceps2_port[2];
    static struct platform_device *maceps2_device;
#[no_mangle]
unsafe extern "C" fn maceps2_write(dev: *mut serio, val: c_uchar) -> c_int {
    static int maceps2_write(struct serio *dev, unsigned char val)
    {
    struct mace_ps2port *port = ((struct maceps2_data *)dev.port_data).port;
    let mut timeout: c_uint = MACE_PS2_TIMEOUT;
    do {
    if (port.status & PS2_STATUS_TX_EMPTY) {
    port.tx = val;
    return 0;
    }
    udelay(50);
    } while (timeout--);
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn maceps2_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t maceps2_interrupt(int irq, void *dev_id)
    {
    struct serio *dev = dev_id;
    struct mace_ps2port *port = ((struct maceps2_data *)dev.port_data).port;
    unsigned long byte;
    if (port.status & PS2_STATUS_RX_FULL) {
    byte = port.rx;
    serio_interrupt(dev, byte & 0xff, 0);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn maceps2_open(dev: *mut serio) -> c_int {
    static int maceps2_open(struct serio *dev)
    {
    struct maceps2_data *data = (struct maceps2_data *)dev.port_data;
    if (request_irq(data.irq, maceps2_interrupt, 0, "PS2 port", dev)) {
    printk(KERN_ERR "Could not allocate PS/2 IRQ\n");
    return -EBUSY;
    }
// Reset port
    data.port.control = PS2_CONTROL_TX_CLOCK_DISABLE | PS2_CONTROL_RESET;
    udelay(100);
// Enable interrupts
    data.port.control = PS2_CONTROL_RX_CLOCK_ENABLE |
    PS2_CONTROL_TX_ENABLE |
    PS2_CONTROL_RX_INT_ENABLE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn maceps2_close(dev: *mut serio) {
    static void maceps2_close(struct serio *dev)
    {
    struct maceps2_data *data = (struct maceps2_data *)dev.port_data;
    data.port.control = PS2_CONTROL_TX_CLOCK_DISABLE | PS2_CONTROL_RESET;
    udelay(100);
    free_irq(data.irq, dev);
    }
    static struct serio *maceps2_allocate_port(int idx)
    {
    struct serio *serio;
    serio = kzalloc_obj(*serio);
    if (serio) {
    serio.id.type		= SERIO_8042;
    serio.write		= maceps2_write;
    serio.open		= maceps2_open;
    serio.close		= maceps2_close;
    snprintf(serio.name, sizeof(serio.name), "MACE PS/2 port%d", idx);
    snprintf(serio.phys, sizeof(serio.phys), "mace/serio%d", idx);
    serio.port_data	= &port_data[idx];
    serio.dev.parent	= &maceps2_device.dev;
    }
    return serio;
    }
#[no_mangle]
unsafe extern "C" fn maceps2_probe(dev: *mut platform_device) -> c_int {
    static int maceps2_probe(struct platform_device *dev)
    {
    maceps2_port[0] = maceps2_allocate_port(0);
    maceps2_port[1] = maceps2_allocate_port(1);
    if (!maceps2_port[0] || !maceps2_port[1]) {
    kfree(maceps2_port[0]);
    kfree(maceps2_port[1]);
    return -ENOMEM;
    }
    serio_register_port(maceps2_port[0]);
    serio_register_port(maceps2_port[1]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn maceps2_remove(dev: *mut platform_device) {
    static void maceps2_remove(struct platform_device *dev)
    {
    serio_unregister_port(maceps2_port[0]);
    serio_unregister_port(maceps2_port[1]);
    }
    static struct platform_driver maceps2_driver = {
    .driver		= {
    .name	= "maceps2",
    },
    .probe		= maceps2_probe,
    .remove		= maceps2_remove,
    };
#[no_mangle]
unsafe extern "C" fn maceps2_init() -> int __init {
    static int __init maceps2_init(void)
    {
    int error;
    error = platform_driver_register(&maceps2_driver);
    if (error)
    return error;
    maceps2_device = platform_device_alloc("maceps2", -1);
    if (!maceps2_device) {
    error = -ENOMEM;
    goto err_unregister_driver;
    }
    port_data[0].port = &mace.perif.ps2.keyb;
    port_data[0].irq  = MACEISA_KEYB_IRQ;
    port_data[1].port = &mace.perif.ps2.mouse;
    port_data[1].irq  = MACEISA_MOUSE_IRQ;
    error = platform_device_add(maceps2_device);
    if (error)
    goto err_free_device;
    return 0;
    err_free_device:
    platform_device_put(maceps2_device);
    err_unregister_driver:
    platform_driver_unregister(&maceps2_driver);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn maceps2_exit() -> void __exit {
    static void __exit maceps2_exit(void)
    {
    platform_device_unregister(maceps2_device);
    platform_driver_unregister(&maceps2_driver);
    }
    module_init(maceps2_init);
    module_exit(maceps2_exit);
