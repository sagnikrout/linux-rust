//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/8250/8250_acorn.c
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
// linux/drivers/serial/acorn.c
//
// Copyright (C) 1996-2003 Russell King.
//

pub const MAX_PORTS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_card_type {
    pub num_ports: c_uint,
    pub uartclk: c_uint,
    pub type: c_uint,
    pub offset: [c_uint; MAX_PORTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_card_info {
    pub num_ports: c_uint,
    pub ports: [c_int; MAX_PORTS],
    pub vaddr: *mut void __iomem,
}

    static int
    serial_card_probe(struct expansion_card *ec, const struct ecard_id *id)
    {
    struct serial_card_info *info;
    struct serial_card_type *type = id.data;
    struct uart_8250_port uart;
    unsigned long bus_addr;
    unsigned int i;
    info = kzalloc_obj(struct serial_card_info);
    if (!info)
    return -ENOMEM;
    info.num_ports = type.num_ports;
    bus_addr = ecard_resource_start(ec, type.type);
    info.vaddr = ecardm_iomap(ec, type.type, 0, 0);
    if (!info.vaddr) {
    kfree(info);
    return -ENOMEM;
    }
    ecard_set_drvdata(ec, info);
    memset(&uart, 0, sizeof(struct uart_8250_port));
    uart.port.irq	= ec.irq;
    uart.port.flags	= UPF_BOOT_AUTOCONF | UPF_SHARE_IRQ;
    uart.port.uartclk	= type.uartclk;
    uart.port.iotype	= UPIO_MEM;
    uart.port.regshift	= 2;
    uart.port.dev	= &ec.dev;
    for (i = 0; i < info.num_ports; i++) {
    uart.port.membase = info.vaddr + type.offset[i];
    uart.port.mapbase = bus_addr + type.offset[i];
    info.ports[i] = serial8250_register_8250_port(&uart);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serial_card_remove(ec: *mut expansion_card) {
    static void serial_card_remove(struct expansion_card *ec)
    {
    struct serial_card_info *info = ecard_get_drvdata(ec);
    int i;
    ecard_set_drvdata(ec, core::ptr::null_mut());
    for (i = 0; i < info.num_ports; i++)
    if (info.ports[i] > 0)
    serial8250_unregister_port(info.ports[i]);
    kfree(info);
    }
    static struct serial_card_type atomwide_type = {
    .num_ports	= 3,
    .uartclk	= 7372800,
    .type		= ECARD_RES_IOCSLOW,
    .offset		= { 0x2800, 0x2400, 0x2000 },
    };
    static struct serial_card_type serport_type = {
    .num_ports	= 2,
    .uartclk	= 3686400,
    .type		= ECARD_RES_IOCSLOW,
    .offset		= { 0x2000, 0x2020 },
    };
    static const struct ecard_id serial_cids[] = {
    { MANU_ATOMWIDE,	PROD_ATOMWIDE_3PSERIAL,	&atomwide_type	},
    { MANU_SERPORT,		PROD_SERPORT_DSPORT,	&serport_type	},
    { 0xffff, 0xffff }
    };
    static struct ecard_driver serial_card_driver = {
    .probe		= serial_card_probe,
    .remove		= serial_card_remove,
    .id_table	= serial_cids,
    .drv = {
    .name	= "8250_acorn",
    },
    };
#[no_mangle]
unsafe extern "C" fn serial_card_init() -> int __init {
    static int __init serial_card_init(void)
    {
    return ecard_register_driver(&serial_card_driver);
    }
#[no_mangle]
unsafe extern "C" fn serial_card_exit() -> void __exit {
    static void __exit serial_card_exit(void)
    {
    ecard_remove_driver(&serial_card_driver);
    }
    MODULE_AUTHOR("Russell King");
    MODULE_DESCRIPTION("Acorn 8250-compatible serial port expansion card driver");
    MODULE_LICENSE("GPL");
    module_init(serial_card_init);
    module_exit(serial_card_exit);
