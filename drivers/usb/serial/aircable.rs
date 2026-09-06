//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/aircable.c
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
// AIRcable USB Bluetooth Dongle Driver.
//
// Copyright (C) 2010 Johan Hovold <jhovold@gmail.com>
// Copyright (C) 2006 Manuel Francisco Naranjo (naranjo.manuel@gmail.com)
//
// The device works as an standard CDC device, it has 2 interfaces, the first
// one is for firmware access and the second is the serial one.
// The protocol is very simply, there are two possibilities reading or writing.
// When writing the first urb must have a Header that starts with 0x20 0x29 the
// next two bytes must say how much data will be sent.
// When reading the process is almost equal except that the header starts with
// 0x00 0x20.
//
// The device simply need some stuff to understand data coming from the usb
// buffer: The First and Second byte is used for a Header, the Third and Fourth
// tells the  device the amount of information the package holds.
// Packages are 60 bytes long Header Stuff.
// When writing to the device the first two bytes of the header are 0x20 0x29
// When reading the bytes are 0x00 0x20, or 0x00 0x10, there is an strange
// situation, when too much data arrives to the device because it sends the data
// but with out the header. I will use a simply hack to override this situation,
// if there is data coming that does not contain any header, then that is data
// that must go directly to the tty, as there is no documentation about if there
// is any other control code, I will simply check for the first
// one.
//
// I have taken some info from a Greg Kroah-Hartman article:
// http://www.linuxjournal.com/article/6573
// And from Linux Device Driver Kit CD, which is a great work, the authors taken
// the work to recompile lots of information an knowledge in drivers development
// and made it all available inside a cd.
// URL: http://kernel.org/pub/linux/kernel/people/gregkh/ddk
//

// Vendor and Product ID
pub const AIRCABLE_VID: c_uint = 0x16CA;
pub const AIRCABLE_USB_PID: c_uint = 0x1502;
// Protocol Stuff
pub const HCI_HEADER_LENGTH: c_uint = 0x4;
pub const TX_HEADER_0: c_uint = 0x20;
pub const TX_HEADER_1: c_uint = 0x29;
pub const RX_HEADER_0: c_uint = 0x00;
pub const RX_HEADER_1: c_uint = 0x20;
pub const HCI_COMPLETE_FRAME: c_int = 64;
// rx_flags
pub const THROTTLED: c_uint = 0x01;
pub const ACTUALLY_THROTTLED: c_uint = 0x02;

// ID table that will be registered with USB core
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(AIRCABLE_VID, AIRCABLE_USB_PID) },
    { },
    };
    MODULE_DEVICE_TABLE(usb, id_table);
    static int aircable_prepare_write_buffer(struct usb_serial_port *port,
    void *dest, size_t size)
    {
    int count;
    unsigned char *buf = dest;
    count = kfifo_out_locked(&port.write_fifo, buf + HCI_HEADER_LENGTH,
    size - HCI_HEADER_LENGTH, &port.lock);
    buf[0] = TX_HEADER_0;
    buf[1] = TX_HEADER_1;
    put_unaligned_le16(count, &buf[2]);
    return count + HCI_HEADER_LENGTH;
    }
    static int aircable_calc_num_ports(struct usb_serial *serial,
    struct usb_serial_endpoints *epds)
    {
// Ignore the first interface, which has no bulk endpoints.
    if (epds.num_bulk_out == 0) {
    dev_dbg(&serial.interface.dev,
    "ignoring interface with no bulk-out endpoints\n");
    return -ENODEV;
    }
    return 1;
    }
    static int aircable_process_packet(struct usb_serial_port *port,
    int has_headers, char *packet, int len)
    {
    if (has_headers) {
    len -= HCI_HEADER_LENGTH;
    packet += HCI_HEADER_LENGTH;
    }
    if (len <= 0) {
    dev_dbg(&port.dev, "%s - malformed packet\n", __func__);
    return 0;
    }
    tty_insert_flip_string(&port.port, packet, len);
    return len;
    }
#[no_mangle]
unsafe extern "C" fn aircable_process_read_urb(urb: *mut urb) {
    static void aircable_process_read_urb(struct urb *urb)
    {
    struct usb_serial_port *port = urb.context;
    char *data = urb.transfer_buffer;
    int has_headers;
    int count;
    int len;
    int i;
    has_headers = (urb.actual_length > 2 && data[0] == RX_HEADER_0);
    count = 0;
    for (i = 0; i < urb.actual_length; i += HCI_COMPLETE_FRAME) {
    len = min_t(int, urb.actual_length - i, HCI_COMPLETE_FRAME);
    count += aircable_process_packet(port, has_headers,
    &data[i], len);
    }
    if (count)
    tty_flip_buffer_push(&port.port);
    }
    static struct usb_serial_driver aircable_device = {
    .driver = {
    .name =		"aircable",
    },
    .id_table = 		id_table,
    .bulk_out_size =	HCI_COMPLETE_FRAME,
    .calc_num_ports =	aircable_calc_num_ports,
    .process_read_urb =	aircable_process_read_urb,
    .prepare_write_buffer =	aircable_prepare_write_buffer,
    .throttle =		usb_serial_generic_throttle,
    .unthrottle =		usb_serial_generic_unthrottle,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &aircable_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL v2");
