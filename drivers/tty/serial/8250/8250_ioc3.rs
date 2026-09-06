//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_ioc3.c
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
// SGI IOC3 8250 UART driver
//
// Copyright (C) 2019 Thomas Bogendoerfer <tbogendoerfer@suse.de>
//
// based on code Copyright (C) 2005 Stanislaw Skowronek <skylark@unaligned.org>
// Copyright (C) 2014 Joshua Kinard <linux@kumba.dev>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioc3_8250_data {
    pub line: c_int,
}

#[no_mangle]
unsafe extern "C" fn ioc3_serial_in(p: *mut uart_port, offset: c_uint) -> u32 {
    static u32 ioc3_serial_in(struct uart_port *p, unsigned int offset)
    {
    return readb(p.membase + (offset ^ 3));
    }
#[no_mangle]
unsafe extern "C" fn ioc3_serial_out(p: *mut uart_port, offset: c_uint, value: u32) {
    static void ioc3_serial_out(struct uart_port *p, unsigned int offset, u32 value)
    {
    writeb(value, p.membase + (offset ^ 3));
    }
#[no_mangle]
unsafe extern "C" fn serial8250_ioc3_probe(pdev: *mut platform_device) -> c_int {
    static int serial8250_ioc3_probe(struct platform_device *pdev)
    {
    struct ioc3_8250_data *data;
    struct uart_8250_port up;
    struct resource *r;
    void __iomem *membase;
    int irq, line;
    r = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!r)
    return -ENODEV;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    membase = devm_ioremap(&pdev.dev, r.start, resource_size(r));
    if (!membase)
    return -ENOMEM;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    irq = 0; /* no interrupt . use polling */
// Register serial ports with 8250.c
    memset(&up, 0, sizeof(struct uart_8250_port));
    up.port.iotype = UPIO_MEM;
    up.port.uartclk = IOC3_UARTCLK;
    up.port.type = PORT_16550A;
    up.port.irq = irq;
    up.port.flags = (UPF_BOOT_AUTOCONF | UPF_SHARE_IRQ);
    up.port.dev = &pdev.dev;
    up.port.membase = membase;
    up.port.mapbase = r.start;
    up.port.serial_in = ioc3_serial_in;
    up.port.serial_out = ioc3_serial_out;
    line = serial8250_register_8250_port(&up);
    if (line < 0)
    return line;
    platform_set_drvdata(pdev, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serial8250_ioc3_remove(pdev: *mut platform_device) {
    static void serial8250_ioc3_remove(struct platform_device *pdev)
    {
    struct ioc3_8250_data *data = platform_get_drvdata(pdev);
    serial8250_unregister_port(data.line);
    }
    static struct platform_driver serial8250_ioc3_driver = {
    .probe  = serial8250_ioc3_probe,
    .remove = serial8250_ioc3_remove,
    .driver = {
    .name = "ioc3-serial8250",
    }
    };
    module_platform_driver(serial8250_ioc3_driver);
    MODULE_AUTHOR("Thomas Bogendoerfer <tbogendoerfer@suse.de>");
    MODULE_DESCRIPTION("SGI IOC3 8250 UART driver");
    MODULE_LICENSE("GPL");
