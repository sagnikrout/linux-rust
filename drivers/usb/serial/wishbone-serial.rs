//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/wishbone-serial.c
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
// USB Wishbone-Serial adapter driver
//
// Copyright (C) 2013 Wesley W. Terpstra <w.terpstra@gsi.de>
// Copyright (C) 2013 GSI Helmholtz Centre for Heavy Ion Research GmbH
//

pub const GSI_VENDOR_OPENCLOSE: c_uint = 0xB0;
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE_AND_INTERFACE_INFO(0x1D50, 0x6062, 0xFF, 0xFF, 0xFF) },
    { },
    };
    MODULE_DEVICE_TABLE(usb, id_table);
//
// Etherbone must be told that a new stream has begun before data arrives.
// This is necessary to restart the negotiation of Wishbone bus parameters.
// Similarly, when the stream ends, Etherbone must be told so that the cycle
// line can be driven low in the case that userspace failed to do so.
//
#[no_mangle]
unsafe extern "C" fn usb_gsi_openclose(port: *mut usb_serial_port, value: c_int) -> c_int {
    static int usb_gsi_openclose(struct usb_serial_port *port, int value)
    {
    struct usb_device *dev = port.serial.dev;
    return usb_control_msg(
    dev,
    usb_sndctrlpipe(dev, 0), /* Send to EP0OUT */
    GSI_VENDOR_OPENCLOSE,
    USB_DIR_OUT|USB_TYPE_VENDOR|USB_RECIP_INTERFACE,
    value, /* wValue = device is open(1) or closed(0) */
    port.serial.interface.cur_altsetting.desc.bInterfaceNumber,
    core::ptr::null_mut(), 0,  /* There is no data stage */
    5000); /* Timeout till operation fails */
    }
    static int wishbone_serial_open(struct tty_struct *tty,
    struct usb_serial_port *port)
    {
    int retval;
    retval = usb_gsi_openclose(port, 1);
    if (retval) {
    dev_err(&port.serial.dev.dev,
    "Could not mark device as open (%d)\n",
    retval);
    return retval;
    }
    retval = usb_serial_generic_open(tty, port);
    if (retval)
    usb_gsi_openclose(port, 0);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn wishbone_serial_close(port: *mut usb_serial_port) {
    static void wishbone_serial_close(struct usb_serial_port *port)
    {
    usb_serial_generic_close(port);
    usb_gsi_openclose(port, 0);
    }
    static struct usb_serial_driver wishbone_serial_device = {
    .driver = {
    .name =		"wishbone_serial",
    },
    .id_table =		id_table,
    .num_ports =		1,
    .open =			&wishbone_serial_open,
    .close =		&wishbone_serial_close,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &wishbone_serial_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_AUTHOR("Wesley W. Terpstra <w.terpstra@gsi.de>");
    MODULE_DESCRIPTION("USB Wishbone-Serial adapter");
    MODULE_LICENSE("GPL");
