//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/cyberjack.c
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
// REINER SCT cyberJack pinpad/e-com USB Chipcard Reader Driver
//
// Copyright (C) 2001  REINER SCT
// Author: Matthias Bruestle
//
// Contact: support@reiner-sct.com (see MAINTAINERS)
//
// This program is largely derived from work by the linux-usb group
// and associated source files.  Please see the usb/serial files for
// individual credits and copyrights.
//
// Thanks to Greg Kroah-Hartman (greg@kroah.com) for his help and
// patience.
//
// In case of problems, please write to the contact e-mail address
// mentioned above.
//
// Please note that later models of the cyberjack reader family are
// supported by a libusb-based userspace device driver.
//
// Homepage: http://www.reiner-sct.de/support/treiber_cyberjack.php#linux
//

pub const CYBERJACK_LOCAL_BUF_SIZE: c_int = 32;

pub const CYBERJACK_VENDOR_ID: c_uint = 0x0C4B;
pub const CYBERJACK_PRODUCT_ID: c_uint = 0x0100;
// Function prototypes
    static int cyberjack_port_probe(struct usb_serial_port *port);
    static void cyberjack_port_remove(struct usb_serial_port *port);
    static int  cyberjack_open(struct tty_struct *tty,
    struct usb_serial_port *port);
    static void cyberjack_close(struct usb_serial_port *port);
    static int cyberjack_write(struct tty_struct *tty,
    struct usb_serial_port *port, const unsigned char *buf, int count);
    static unsigned int cyberjack_write_room(struct tty_struct *tty);
    static void cyberjack_read_int_callback(struct urb *urb);
    static void cyberjack_read_bulk_callback(struct urb *urb);
    static void cyberjack_write_bulk_callback(struct urb *urb);
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(CYBERJACK_VENDOR_ID, CYBERJACK_PRODUCT_ID) },
    { }			/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, id_table);
    static struct usb_serial_driver cyberjack_device = {
    .driver = {
    .name =		"cyberjack",
    },
    .description =		"Reiner SCT Cyberjack USB card reader",
    .id_table =		id_table,
    .num_ports =		1,
    .num_bulk_out =		1,
    .port_probe =		cyberjack_port_probe,
    .port_remove =		cyberjack_port_remove,
    .open =			cyberjack_open,
    .close =		cyberjack_close,
    .write =		cyberjack_write,
    .write_room =		cyberjack_write_room,
    .read_int_callback =	cyberjack_read_int_callback,
    .read_bulk_callback =	cyberjack_read_bulk_callback,
    .write_bulk_callback =	cyberjack_write_bulk_callback,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &cyberjack_device, core::ptr::null_mut()
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyberjack_private {
    pub /: *mut *mut spinlock_t lock; / Lock for SMP,
    pub /: *mut *mut short rdtodo; / Bytes still to read,
    pub /: *mut *mut *mut unsigned char wrbuf[564]; / Buffer for collecting data to write,
    pub /: *mut *mut short wrfilled; / Overall data size we already got,
    pub /: *mut *mut short wrsent; / Data already sent,
}

#[no_mangle]
unsafe extern "C" fn cyberjack_port_probe(port: *mut usb_serial_port) -> c_int {
    static int cyberjack_port_probe(struct usb_serial_port *port)
    {
    struct cyberjack_private *priv;
    int result;
    priv = kmalloc_obj(struct cyberjack_private);
    if (!priv)
    return -ENOMEM;
    spin_lock_init(&priv.lock);
    priv.rdtodo = 0;
    priv.wrfilled = 0;
    priv.wrsent = 0;
    usb_set_serial_port_data(port, priv);
    result = usb_submit_urb(port.interrupt_in_urb, GFP_KERNEL);
    if (result)
    dev_err(&port.dev, "usb_submit_urb(read int) failed\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cyberjack_port_remove(port: *mut usb_serial_port) {
    static void cyberjack_port_remove(struct usb_serial_port *port)
    {
    struct cyberjack_private *priv;
    usb_kill_urb(port.interrupt_in_urb);
    priv = usb_get_serial_port_data(port);
    kfree(priv);
    }
    static int  cyberjack_open(struct tty_struct *tty,
    struct usb_serial_port *port)
    {
    struct cyberjack_private *priv;
    unsigned long flags;
    dev_dbg(&port.dev, "%s - usb_clear_halt\n", __func__);
    usb_clear_halt(port.serial.dev, port.write_urb.pipe);
    priv = usb_get_serial_port_data(port);
    spin_lock_irqsave(&priv.lock, flags);
    priv.rdtodo = 0;
    priv.wrfilled = 0;
    priv.wrsent = 0;
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cyberjack_close(port: *mut usb_serial_port) {
    static void cyberjack_close(struct usb_serial_port *port)
    {
    usb_kill_urb(port.write_urb);
    usb_kill_urb(port.read_urb);
    }
    static int cyberjack_write(struct tty_struct *tty,
    struct usb_serial_port *port, const unsigned char *buf, int count)
    {
    struct device *dev = &port.dev;
    struct cyberjack_private *priv = usb_get_serial_port_data(port);
    unsigned long flags;
    int result;
    int wrexpected;
    if (count == 0) {
    dev_dbg(dev, "%s - write request of 0 bytes\n", __func__);
    return 0;
    }
    if (!test_and_clear_bit(0, &port.write_urbs_free)) {
    dev_dbg(dev, "%s - already writing\n", __func__);
    return 0;
    }
    spin_lock_irqsave(&priv.lock, flags);
    if (count+priv.wrfilled > sizeof(priv.wrbuf)) {
// To much data for buffer. Reset buffer.
    priv.wrfilled = 0;
    spin_unlock_irqrestore(&priv.lock, flags);
    set_bit(0, &port.write_urbs_free);
    return 0;
    }
// Copy data
    memcpy(priv.wrbuf + priv.wrfilled, buf, count);
    usb_serial_debug_data(dev, __func__, count, priv.wrbuf + priv.wrfilled);
    priv.wrfilled += count;
    if (priv.wrfilled >= 3) {
    wrexpected = ((int)priv.wrbuf[2]<<8)+priv.wrbuf[1]+3;
    dev_dbg(dev, "%s - expected data: %d\n", __func__, wrexpected);
    } else
    wrexpected = sizeof(priv.wrbuf);
    if (priv.wrfilled >= wrexpected) {
// We have enough data to begin transmission
    int length;
    dev_dbg(dev, "%s - transmitting data (frame 1)\n", __func__);
    length = (wrexpected > port.bulk_out_size) ?
    port.bulk_out_size : wrexpected;
    memcpy(port.write_urb.transfer_buffer, priv.wrbuf, length);
    priv.wrsent = length;
// set up our urb
    port.write_urb.transfer_buffer_length = length;
// send the data out the bulk port
    result = usb_submit_urb(port.write_urb, GFP_ATOMIC);
    if (result) {
    dev_err(&port.dev,
    "%s - failed submitting write urb, error %d\n",
    __func__, result);
// Throw away data. No better idea what to do with it.
    priv.wrfilled = 0;
    priv.wrsent = 0;
    spin_unlock_irqrestore(&priv.lock, flags);
    set_bit(0, &port.write_urbs_free);
    return 0;
    }
    dev_dbg(dev, "%s - priv.wrsent=%d\n", __func__, priv.wrsent);
    dev_dbg(dev, "%s - priv.wrfilled=%d\n", __func__, priv.wrfilled);
    if (priv.wrsent >= priv.wrfilled) {
    dev_dbg(dev, "%s - buffer cleaned\n", __func__);
    memset(priv.wrbuf, 0, sizeof(priv.wrbuf));
    priv.wrfilled = 0;
    priv.wrsent = 0;
    }
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn cyberjack_write_room(tty: *mut tty_struct) -> c_uint {
    static unsigned int cyberjack_write_room(struct tty_struct *tty)
    {
// FIXME: ....
    return CYBERJACK_LOCAL_BUF_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn cyberjack_read_int_callback(urb: *mut urb) {
    static void cyberjack_read_int_callback(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    struct cyberjack_private *priv = usb_get_serial_port_data(port);
    struct device *dev = &port.dev;
    unsigned char *data = urb.transfer_buffer;
    let mut status: c_int = urb.status;
    unsigned long flags;
    int result;
// the urb might have been killed.
    if (status)
    return;
    usb_serial_debug_data(dev, __func__, urb.actual_length, data);
// React only to interrupts signaling a bulk_in transfer
    if (urb.actual_length == 4 && data[0] == 0x01) {
    short old_rdtodo;
// This is a announcement of coming bulk_ins.
    let mut size: c_ushort = ((unsigned short)data[3]<<8)+data[2]+3;
    spin_lock_irqsave(&priv.lock, flags);
    old_rdtodo = priv.rdtodo;
    if (old_rdtodo > SHRT_MAX - size) {
    dev_dbg(dev, "Too many bulk_in urbs to do.\n");
    spin_unlock_irqrestore(&priv.lock, flags);
    goto resubmit;
    }
// "+=" is probably more fault tolerant than "="
    priv.rdtodo += size;
    dev_dbg(dev, "%s - rdtodo: %d\n", __func__, priv.rdtodo);
    spin_unlock_irqrestore(&priv.lock, flags);
    if (!old_rdtodo) {
    result = usb_submit_urb(port.read_urb, GFP_ATOMIC);
    if (result)
    dev_err(dev, "%s - failed resubmitting read urb, error %d\n",
    __func__, result);
    dev_dbg(dev, "%s - usb_submit_urb(read urb)\n", __func__);
    }
    }
    resubmit:
    result = usb_submit_urb(port.interrupt_in_urb, GFP_ATOMIC);
    if (result)
    dev_err(&port.dev, "usb_submit_urb(read int) failed\n");
    dev_dbg(dev, "%s - usb_submit_urb(int urb)\n", __func__);
    }
#[no_mangle]
unsafe extern "C" fn cyberjack_read_bulk_callback(urb: *mut urb) {
    static void cyberjack_read_bulk_callback(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    struct cyberjack_private *priv = usb_get_serial_port_data(port);
    struct device *dev = &port.dev;
    unsigned char *data = urb.transfer_buffer;
    unsigned long flags;
    short todo;
    int result;
    let mut status: c_int = urb.status;
    usb_serial_debug_data(dev, __func__, urb.actual_length, data);
    if (status) {
    dev_dbg(dev, "%s - nonzero read bulk status received: %d\n",
    __func__, status);
    return;
    }
    if (urb.actual_length) {
    tty_insert_flip_string(&port.port, data, urb.actual_length);
    tty_flip_buffer_push(&port.port);
    }
    spin_lock_irqsave(&priv.lock, flags);
// Reduce urbs to do by one.
    priv.rdtodo -= urb.actual_length;
// Just to be sure
    if (priv.rdtodo < 0)
    priv.rdtodo = 0;
    todo = priv.rdtodo;
    spin_unlock_irqrestore(&priv.lock, flags);
    dev_dbg(dev, "%s - rdtodo: %d\n", __func__, todo);
// Continue to read if we have still urbs to do.
    if (todo /* || (urb.actual_length==port.bulk_in_endpointAddress)*/) {
    result = usb_submit_urb(port.read_urb, GFP_ATOMIC);
    if (result)
    dev_err(dev, "%s - failed resubmitting read urb, error %d\n",
    __func__, result);
    dev_dbg(dev, "%s - usb_submit_urb(read urb)\n", __func__);
    }
    }
#[no_mangle]
unsafe extern "C" fn cyberjack_write_bulk_callback(urb: *mut urb) {
    static void cyberjack_write_bulk_callback(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    struct cyberjack_private *priv = usb_get_serial_port_data(port);
    struct device *dev = &port.dev;
    let mut status: c_int = urb.status;
    unsigned long flags;
    let mut resubmitted: bool = false;
    if (status) {
    dev_dbg(dev, "%s - nonzero write bulk status received: %d\n",
    __func__, status);
    set_bit(0, &port.write_urbs_free);
    return;
    }
    spin_lock_irqsave(&priv.lock, flags);
// only do something if we have more data to send
    if (priv.wrfilled) {
    int length, blksize, result;
    dev_dbg(dev, "%s - transmitting data (frame n)\n", __func__);
    length = ((priv.wrfilled - priv.wrsent) > port.bulk_out_size) ?
    port.bulk_out_size : (priv.wrfilled - priv.wrsent);
    memcpy(port.write_urb.transfer_buffer,
    priv.wrbuf + priv.wrsent, length);
    priv.wrsent += length;
// set up our urb
    port.write_urb.transfer_buffer_length = length;
// send the data out the bulk port
    result = usb_submit_urb(port.write_urb, GFP_ATOMIC);
    if (result) {
    dev_err(dev, "%s - failed submitting write urb, error %d\n",
    __func__, result);
// Throw away data. No better idea what to do with it.
    priv.wrfilled = 0;
    priv.wrsent = 0;
    goto exit;
    }
    resubmitted = true;
    dev_dbg(dev, "%s - priv.wrsent=%d\n", __func__, priv.wrsent);
    dev_dbg(dev, "%s - priv.wrfilled=%d\n", __func__, priv.wrfilled);
    blksize = ((int)priv.wrbuf[2]<<8)+priv.wrbuf[1]+3;
    if (priv.wrsent >= priv.wrfilled ||
    priv.wrsent >= blksize) {
    dev_dbg(dev, "%s - buffer cleaned\n", __func__);
    memset(priv.wrbuf, 0, sizeof(priv.wrbuf));
    priv.wrfilled = 0;
    priv.wrsent = 0;
    }
    }
    exit:
    spin_unlock_irqrestore(&priv.lock, flags);
    if (!resubmitted)
    set_bit(0, &port.write_urbs_free);
    usb_serial_port_softint(port);
    }
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
