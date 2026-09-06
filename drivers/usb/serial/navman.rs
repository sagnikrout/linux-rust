//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/navman.c
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
// Navman Serial USB driver
//
// Copyright (C) 2006 Greg Kroah-Hartman <gregkh@suse.de>
//
// TODO:
// Add termios method that uses copy_hw but also kills all echo
// flags as the navman is rx only so cannot echo.
//

    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(0x0a99, 0x0001) },	/* Talon Technology device */
    { USB_DEVICE(0x0df7, 0x0900) },	/* Mobile Action i-gotU */
    { },
    };
    MODULE_DEVICE_TABLE(usb, id_table);
#[no_mangle]
unsafe extern "C" fn navman_read_int_callback(urb: *mut urb) {
    static void navman_read_int_callback(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    unsigned char *data = urb.transfer_buffer;
    let mut status: c_int = urb.status;
    int result;
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
    if (urb.actual_length) {
    tty_insert_flip_string(&port.port, data, urb.actual_length);
    tty_flip_buffer_push(&port.port);
    }
    exit:
    result = usb_submit_urb(urb, GFP_ATOMIC);
    if (result)
    dev_err(&urb.dev.dev,
    "%s - Error %d submitting interrupt urb\n",
    __func__, result);
    }
#[no_mangle]
unsafe extern "C" fn navman_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int navman_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    let mut result: c_int = 0;
    if (port.interrupt_in_urb) {
    dev_dbg(&port.dev, "%s - adding interrupt input for treo\n",
    __func__);
    result = usb_submit_urb(port.interrupt_in_urb, GFP_KERNEL);
    if (result)
    dev_err(&port.dev,
    "%s - failed submitting interrupt urb, error %d\n",
    __func__, result);
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn navman_close(port: *mut usb_serial_port) {
    static void navman_close(struct usb_serial_port *port)
    {
    usb_kill_urb(port.interrupt_in_urb);
    }
    static int navman_write(struct tty_struct *tty, struct usb_serial_port *port,
    const unsigned char *buf, int count)
    {
//
// This device can't write any data, only read from the device
//
    return -EOPNOTSUPP;
    }
    static struct usb_serial_driver navman_device = {
    .driver = {
    .name =		"navman",
    },
    .id_table =		id_table,
    .num_ports =		1,
    .open =			navman_open,
    .close = 		navman_close,
    .write = 		navman_write,
    .read_int_callback =	navman_read_int_callback,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &navman_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_DESCRIPTION("Navman USB Serial driver");
    MODULE_LICENSE("GPL v2");
