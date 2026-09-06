//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/ipw.c
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
// IPWireless 3G UMTS TDD Modem driver (USB connected)
//
// Copyright (C) 2004 Roelf Diedericks <roelfd@inet.co.za>
// Copyright (C) 2004 Greg Kroah-Hartman <greg@kroah.com>
//
// All information about the device was acquired using SnoopyPro
// on MSFT's O/S, and examing the MSFT drivers' debug output
// (insanely left _on_ in the enduser version)
//
// It was written out of frustration with the IPWireless USB modem
// supplied by Axity3G/Sentech South Africa not supporting
// Linux whatsoever.
//
// Nobody provided any proprietary information that was not already
// available for this device.
//
// The modem adheres to the "3GPP TS  27.007 AT command set for 3G
// User Equipment (UE)" standard, available from
// http://www.3gpp.org/ftp/Specs/html-info/27007.htm
//
// The code was only tested the IPWireless handheld modem distributed
// in South Africa by Sentech.
//
// It may work for Woosh Inc in .nz too, as it appears they use the
// same kit.
//
// There is still some work to be done in terms of handling
// DCD, DTR, RTS, CTS which are currently faked.
// It's good enough for PPP at this point. It's based off all kinds of
// code found in usb/serial and usb/class
//

pub const USB_IPW_MAGIC: c_uint = 0x6d02	/* magic number for ipw struct */;
// Message sizes
pub const EVENT_BUFFER_SIZE: c_uint = 0xFF;

// vendor/product pairs that are known work with this driver
pub const IPW_VID: c_uint = 0x0bc3;
pub const IPW_PID: c_uint = 0x0001;
// Vendor commands:
// baud rates
    enum {
    ipw_sio_b256000 = 0x000e,
    ipw_sio_b128000 = 0x001d,
    ipw_sio_b115200 = 0x0020,
    ipw_sio_b57600  = 0x0040,
    ipw_sio_b56000  = 0x0042,
    ipw_sio_b38400  = 0x0060,
    ipw_sio_b19200  = 0x00c0,
    ipw_sio_b14400  = 0x0100,
    ipw_sio_b9600   = 0x0180,
    ipw_sio_b4800   = 0x0300,
    ipw_sio_b2400   = 0x0600,
    ipw_sio_b1200   = 0x0c00,
    ipw_sio_b600    = 0x1800
    };
// data bits
pub const ipw_dtb_7: c_uint = 0x700;
pub const ipw_dtb_8: c_uint = 0x810	/* ok so the define is misleading, I know, but forces 8,n,1 */;
// I mean, is there a point to any other setting these days? :)
// usb control request types :
pub const IPW_SIO_RXCTL: c_uint = 0x00	/* control bulk rx channel transmissions, value=1/0 (on/off) */;
pub const IPW_SIO_SET_BAUD: c_uint = 0x01	/* set baud, value=requested ipw_sio_bxxxx */;
pub const IPW_SIO_SET_LINE: c_uint = 0x03	/* set databits, parity. value=ipw_dtb_x */;
pub const IPW_SIO_SET_PIN: c_uint = 0x03	/* set/clear dtr/rts value=ipw_pin_xxx */;
pub const IPW_SIO_POLL: c_uint = 0x08	/* get serial port status byte, call with value=0 */;
pub const IPW_SIO_INIT: c_uint = 0x11	/* initializes ? value=0 (appears as first thing todo on open) */;
pub const IPW_SIO_PURGE: c_uint = 0x12	/* purge all transmissions?, call with value=numchar_to_purge */;
pub const IPW_SIO_HANDFLOW: c_uint = 0x13	/* set xon/xoff limits value=0, and a buffer of 0x10 bytes */;
pub const IPW_SIO_SETCHARS: c_uint = 0x13	/* set the flowcontrol special chars, value=0, buf=6 bytes, */;
// last 2 bytes contain flowcontrol chars e.g. 00 00 00 00 11 13
// values used for request IPW_SIO_SET_PIN
pub const IPW_PIN_SETDTR: c_uint = 0x101;
pub const IPW_PIN_SETRTS: c_uint = 0x202;
pub const IPW_PIN_CLRDTR: c_uint = 0x100;
pub const IPW_PIN_CLRRTS: c_uint = 0x200 /* unconfirmed */;
// values used for request IPW_SIO_RXCTL
pub const IPW_RXBULK_ON: c_int = 1;
pub const IPW_RXBULK_OFF: c_int = 0;
// various 16 byte hardcoded transferbuffers used by flow control

    0, 0, 0, 0, 0, 0, 0, 0 }
// Interpretation of modem status lines
// These need sorting out by individually connecting pins and checking
// results. FIXME!
// When data is being sent we see 0x30 in the lower byte; this must
// contain DSR and CTS ...
//

pub const IPW_WANTS_TO_SEND: c_uint = 0x30;
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(IPW_VID, IPW_PID) },
    { },
    };
    MODULE_DEVICE_TABLE(usb, id_table);
#[no_mangle]
unsafe extern "C" fn ipw_open(tty: *mut tty_struct, port: *mut usb_serial_port) -> c_int {
    static int ipw_open(struct tty_struct *tty, struct usb_serial_port *port)
    {
    struct usb_device *udev = port.serial.dev;
    struct device *dev = &port.dev;
    u8 buf_flow_static[16] = IPW_BYTES_FLOWINIT;
    u8 *buf_flow_init;
    int result;
    buf_flow_init = kmemdup(buf_flow_static, 16, GFP_KERNEL);
    if (!buf_flow_init)
    return -ENOMEM;
// --1: Tell the modem to initialize (we think) From sniffs this is
// always the first thing that gets sent to the modem during
// opening of the device
    dev_dbg(dev, "%s: Sending SIO_INIT (we guess)\n", __func__);
    result = usb_control_msg(udev, usb_sndctrlpipe(udev, 0),
    IPW_SIO_INIT,
    USB_TYPE_VENDOR | USB_RECIP_INTERFACE | USB_DIR_OUT,
    0,
    0, /* index */
    core::ptr::null_mut(),
    0,
    100000);
    if (result < 0)
    dev_err(dev, "Init of modem failed (error = %d)\n", result);
// reset the bulk pipes
    usb_clear_halt(udev, usb_rcvbulkpipe(udev, port.bulk_in_endpointAddress));
    usb_clear_halt(udev, usb_sndbulkpipe(udev, port.bulk_out_endpointAddress));
// --2: Start reading from the device
    dev_dbg(dev, "%s: setting up bulk read callback\n", __func__);
    usb_wwan_open(tty, port);
// --3: Tell the modem to open the floodgates on the rx bulk channel
    dev_dbg(dev, "%s:asking modem for RxRead (RXBULK_ON)\n", __func__);
    result = usb_control_msg(udev, usb_sndctrlpipe(udev, 0),
    IPW_SIO_RXCTL,
    USB_TYPE_VENDOR | USB_RECIP_INTERFACE | USB_DIR_OUT,
    IPW_RXBULK_ON,
    0, /* index */
    core::ptr::null_mut(),
    0,
    100000);
    if (result < 0)
    dev_err(dev, "Enabling bulk RxRead failed (error = %d)\n", result);
// --4: setup the initial flowcontrol
    dev_dbg(dev, "%s:setting init flowcontrol (%s)\n", __func__, buf_flow_init);
    result = usb_control_msg(udev, usb_sndctrlpipe(udev, 0),
    IPW_SIO_HANDFLOW,
    USB_TYPE_VENDOR | USB_RECIP_INTERFACE | USB_DIR_OUT,
    0,
    0,
    buf_flow_init,
    0x10,
    200000);
    if (result < 0)
    dev_err(dev, "initial flowcontrol failed (error = %d)\n", result);
    kfree(buf_flow_init);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipw_attach(serial: *mut usb_serial) -> c_int {
    static int ipw_attach(struct usb_serial *serial)
    {
    struct usb_wwan_intf_private *data;
    data = kzalloc_obj(struct usb_wwan_intf_private);
    if (!data)
    return -ENOMEM;
    spin_lock_init(&data.susp_lock);
    usb_set_serial_data(serial, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ipw_release(serial: *mut usb_serial) {
    static void ipw_release(struct usb_serial *serial)
    {
    struct usb_wwan_intf_private *data = usb_get_serial_data(serial);
    usb_set_serial_data(serial, core::ptr::null_mut());
    kfree(data);
    }
#[no_mangle]
unsafe extern "C" fn ipw_dtr_rts(port: *mut usb_serial_port, on: c_int) {
    static void ipw_dtr_rts(struct usb_serial_port *port, int on)
    {
    struct usb_device *udev = port.serial.dev;
    struct device *dev = &port.dev;
    int result;
    dev_dbg(dev, "%s: on = %d\n", __func__, on);
    result = usb_control_msg(udev, usb_sndctrlpipe(udev, 0),
    IPW_SIO_SET_PIN,
    USB_TYPE_VENDOR | USB_RECIP_INTERFACE | USB_DIR_OUT,
    on ? IPW_PIN_SETDTR : IPW_PIN_CLRDTR,
    0,
    core::ptr::null_mut(),
    0,
    200000);
    if (result < 0)
    dev_err(dev, "setting dtr failed (error = %d)\n", result);
    result = usb_control_msg(udev, usb_sndctrlpipe(udev, 0),
    IPW_SIO_SET_PIN, USB_TYPE_VENDOR |
    USB_RECIP_INTERFACE | USB_DIR_OUT,
    on ? IPW_PIN_SETRTS : IPW_PIN_CLRRTS,
    0,
    core::ptr::null_mut(),
    0,
    200000);
    if (result < 0)
    dev_err(dev, "setting rts failed (error = %d)\n", result);
    }
#[no_mangle]
unsafe extern "C" fn ipw_close(port: *mut usb_serial_port) {
    static void ipw_close(struct usb_serial_port *port)
    {
    struct usb_device *udev = port.serial.dev;
    struct device *dev = &port.dev;
    int result;
// --3: purge
    dev_dbg(dev, "%s:sending purge\n", __func__);
    result = usb_control_msg(udev, usb_sndctrlpipe(udev, 0),
    IPW_SIO_PURGE, USB_TYPE_VENDOR |
    USB_RECIP_INTERFACE | USB_DIR_OUT,
    0x03,
    0,
    core::ptr::null_mut(),
    0,
    200000);
    if (result < 0)
    dev_err(dev, "purge failed (error = %d)\n", result);
// send RXBULK_off (tell modem to stop transmitting bulk data on
    rx chan) */
    result = usb_control_msg(udev, usb_sndctrlpipe(udev, 0),
    IPW_SIO_RXCTL,
    USB_TYPE_VENDOR | USB_RECIP_INTERFACE | USB_DIR_OUT,
    IPW_RXBULK_OFF,
    0, /* index */
    core::ptr::null_mut(),
    0,
    100000);
    if (result < 0)
    dev_err(dev, "Disabling bulk RxRead failed (error = %d)\n", result);
    usb_wwan_close(port);
    }
    static struct usb_serial_driver ipw_device = {
    .driver = {
    .name =		"ipw",
    },
    .description =		"IPWireless converter",
    .id_table =		id_table,
    .num_ports =		1,
    .open =			ipw_open,
    .close =		ipw_close,
    .attach =		ipw_attach,
    .release =		ipw_release,
    .port_probe =		usb_wwan_port_probe,
    .port_remove =		usb_wwan_port_remove,
    .dtr_rts =		ipw_dtr_rts,
    .write =		usb_wwan_write,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &ipw_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
// Module information
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
