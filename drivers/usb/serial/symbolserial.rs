//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/symbolserial.c
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
// Symbol USB barcode to serial driver
//
// Copyright (C) 2013 Johan Hovold <jhovold@gmail.com>
// Copyright (C) 2009 Greg Kroah-Hartman <gregkh@suse.de>
// Copyright (C) 2009 Novell Inc.
//

    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(0x05e0, 0x0600) },
    { },
    };
    MODULE_DEVICE_TABLE(usb, id_table);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct symbol_private {
    pub /: *mut *mut spinlock_t lock; / protects the following flags,
    pub throttled: bool,
    pub actually_throttled: bool,
}

#[no_mangle]
unsafe extern "C" fn symbol_int_callback(urb: *mut urb) {
    static void symbol_int_callback(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    struct symbol_private *priv = usb_get_serial_port_data(port);
    unsigned char *data = urb.transfer_buffer;
    let mut status: c_int = urb.status;
    unsigned long flags;
    int result;
    int data_length;
    switch (status) {
    case 0:
// success
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
// this urb is terminated, clean up
    dev_dbg(&port.dev, "%s - urb shutting down with status: %d\n",
    __func__, status);
    return;
    default:
    dev_dbg(&port.dev, "%s - nonzero urb status received: %d\n",
    __func__, status);
    goto exit;
    }
    usb_serial_debug_data(&port.dev, __func__, urb.actual_length, data);
//
// Data from the device comes with a 1 byte header:
//
// <size of data> <data>...
//
    if (urb.actual_length > 1) {
    data_length = data[0];
    if (data_length > (urb.actual_length - 1))
    data_length = urb.actual_length - 1;
    tty_insert_flip_string(&port.port, &data[1], data_length);
    tty_flip_buffer_push(&port.port);
    } else {
    dev_dbg(&port.dev, "%s - short packet\n", __func__);
    }
    exit:
    spin_lock_irqsave(&priv.lock, flags);
// Continue trying to always read if we should
    if (!priv.throttled) {
    result = usb_submit_urb(port.interrupt_in_urb, GFP_ATOMIC);
    if (result)
    dev_err(&port.dev,
    "%s - failed resubmitting read urb, error %d\n",
    __func__, result);
    } else
    priv.actually_throttled = true;
    spin_unlock_irqrestore(&priv.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn symbol_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int symbol_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    struct symbol_private *priv = usb_get_serial_port_data(port);
    unsigned long flags;
    let mut result: c_int = 0;
    spin_lock_irqsave(&priv.lock, flags);
    priv.throttled = false;
    priv.actually_throttled = false;
    spin_unlock_irqrestore(&priv.lock, flags);
// Start reading from the device
    result = usb_submit_urb(port.interrupt_in_urb, GFP_KERNEL);
    if (result)
    dev_err(&port.dev,
    "%s - failed resubmitting read urb, error %d\n",
    __func__, result);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn symbol_close(port: *mut usb_serial_port) {
    static void symbol_close(struct usb_serial_port *port)
    {
    usb_kill_urb(port.interrupt_in_urb);
    }
#[no_mangle]
unsafe extern "C" fn symbol_throttle(tty: *mut tty_struct) {
    static void symbol_throttle(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct symbol_private *priv = usb_get_serial_port_data(port);
    spin_lock_irq(&priv.lock);
    priv.throttled = true;
    spin_unlock_irq(&priv.lock);
    }
#[no_mangle]
unsafe extern "C" fn symbol_unthrottle(tty: *mut tty_struct) {
    static void symbol_unthrottle(struct tty_struct *tty)
    {
    struct usb_serial_port *port = tty.driver_data;
    struct symbol_private *priv = usb_get_serial_port_data(port);
    int result;
    bool was_throttled;
    spin_lock_irq(&priv.lock);
    priv.throttled = false;
    was_throttled = priv.actually_throttled;
    priv.actually_throttled = false;
    spin_unlock_irq(&priv.lock);
    if (was_throttled) {
    result = usb_submit_urb(port.interrupt_in_urb, GFP_KERNEL);
    if (result)
    dev_err(&port.dev,
    "%s - failed submitting read urb, error %d\n",
    __func__, result);
    }
    }
#[no_mangle]
unsafe extern "C" fn symbol_port_probe(port: *mut usb_serial_port) -> c_int {
    static int symbol_port_probe(struct usb_serial_port *port)
    {
    struct symbol_private *priv;
    priv = kzalloc_obj(*priv);
    if (!priv)
    return -ENOMEM;
    spin_lock_init(&priv.lock);
    usb_set_serial_port_data(port, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn symbol_port_remove(port: *mut usb_serial_port) {
    static void symbol_port_remove(struct usb_serial_port *port)
    {
    struct symbol_private *priv = usb_get_serial_port_data(port);
    kfree(priv);
    }
    static struct usb_serial_driver symbol_device = {
    .driver = {
    .name =		"symbol",
    },
    .id_table =		id_table,
    .num_ports =		1,
    .num_interrupt_in =	1,
    .port_probe =		symbol_port_probe,
    .port_remove =		symbol_port_remove,
    .open =			symbol_open,
    .close =		symbol_close,
    .throttle = 		symbol_throttle,
    .unthrottle =		symbol_unthrottle,
    .read_int_callback =	symbol_int_callback,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &symbol_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_DESCRIPTION("Symbol USB barcode to serial driver");
    MODULE_LICENSE("GPL v2");
