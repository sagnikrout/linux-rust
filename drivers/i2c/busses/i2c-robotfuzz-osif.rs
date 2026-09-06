//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-robotfuzz-osif.c
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
// Driver for RobotFuzz OSIF
//
// Copyright (c) 2013 Andrew Lunn <andrew@lunn.ch>
// Copyright (c) 2007 Barry Carter <Barry.Carter@robotfuzz.com>
//
// Based on the i2c-tiny-usb by
//
// Copyright (C) 2006 Til Harbaum (Till@Harbaum.org)
//

pub const OSIFI2C_READ: c_int = 20;
pub const OSIFI2C_WRITE: c_int = 21;
pub const OSIFI2C_STOP: c_int = 22;
pub const OSIFI2C_STATUS: c_int = 23;
pub const OSIFI2C_SET_BIT_RATE: c_int = 24;
pub const STATUS_ADDRESS_ACK: c_int = 0;
pub const STATUS_ADDRESS_NAK: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osif_priv {
    pub usb_dev: *mut usb_device,
    pub interface: *mut usb_interface,
    pub adapter: i2c_adapter,
    pub status: c_uchar,
}

    static int osif_usb_read(struct i2c_adapter *adapter, int cmd,
    int value, int index, void *data, int len)
    {
    struct osif_priv *priv = adapter.algo_data;
    return usb_control_msg(priv.usb_dev, usb_rcvctrlpipe(priv.usb_dev, 0),
    cmd, USB_TYPE_VENDOR | USB_RECIP_INTERFACE |
    USB_DIR_IN, value, index, data, len, 2000);
    }
    static int osif_usb_write(struct i2c_adapter *adapter, int cmd,
    int value, int index, void *data, int len)
    {
    struct osif_priv *priv = adapter.algo_data;
    return usb_control_msg(priv.usb_dev, usb_sndctrlpipe(priv.usb_dev, 0),
    cmd, USB_TYPE_VENDOR | USB_RECIP_INTERFACE,
    value, index, data, len, 2000);
    }
    static int osif_xfer(struct i2c_adapter *adapter, struct i2c_msg *msgs,
    int num)
    {
    struct osif_priv *priv = adapter.algo_data;
    struct i2c_msg *pmsg;
    int ret;
    int i;
    for (i = 0; i < num; i++) {
    pmsg = &msgs[i];
    if (pmsg.flags & I2C_M_RD) {
    ret = osif_usb_read(adapter, OSIFI2C_READ,
    pmsg.flags, pmsg.addr,
    pmsg.buf, pmsg.len);
    if (ret != pmsg.len) {
    dev_err(&adapter.dev, "failure reading data\n");
    return -EREMOTEIO;
    }
    } else {
    ret = osif_usb_write(adapter, OSIFI2C_WRITE,
    pmsg.flags, pmsg.addr,
    pmsg.buf, pmsg.len);
    if (ret != pmsg.len) {
    dev_err(&adapter.dev, "failure writing data\n");
    return -EREMOTEIO;
    }
    }
    ret = osif_usb_write(adapter, OSIFI2C_STOP, 0, 0, core::ptr::null_mut(), 0);
    if (ret) {
    dev_err(&adapter.dev, "failure sending STOP\n");
    return -EREMOTEIO;
    }
// read status
    ret = osif_usb_read(adapter, OSIFI2C_STATUS, 0, 0,
    &priv.status, 1);
    if (ret != 1) {
    dev_err(&adapter.dev, "failure reading status\n");
    return -EREMOTEIO;
    }
    if (priv.status != STATUS_ADDRESS_ACK) {
    dev_dbg(&adapter.dev, "status = %d\n", priv.status);
    return -EREMOTEIO;
    }
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn osif_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 osif_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
// prevent invalid 0-length usb_control_msg
    static const struct i2c_adapter_quirks osif_quirks = {
    .flags = I2C_AQ_NO_ZERO_LEN_READ,
    };
    static const struct i2c_algorithm osif_algorithm = {
    .xfer = osif_xfer,
    .functionality = osif_func,
    };
pub const USB_OSIF_VENDOR_ID: c_uint = 0x1964;
pub const USB_OSIF_PRODUCT_ID: c_uint = 0x0001;
    static const struct usb_device_id osif_table[] = {
    { USB_DEVICE(USB_OSIF_VENDOR_ID, USB_OSIF_PRODUCT_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(usb, osif_table);
    static int osif_probe(struct usb_interface *interface,
    const struct usb_device_id *id)
    {
    int ret;
    struct osif_priv *priv;
    u16 version;
    priv = devm_kzalloc(&interface.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.usb_dev = interface_to_usbdev(interface);
    priv.interface = interface;
    usb_set_intfdata(interface, priv);
    priv.adapter.owner = THIS_MODULE;
    priv.adapter.class = I2C_CLASS_HWMON;
    priv.adapter.quirks = &osif_quirks;
    priv.adapter.algo = &osif_algorithm;
    priv.adapter.algo_data = priv;
    snprintf(priv.adapter.name, sizeof(priv.adapter.name),
    "OSIF at bus %03d device %03d",
    priv.usb_dev.bus.busnum, priv.usb_dev.devnum);
//
// Set bus frequency. The frequency is:
// 120,000,000 / ( 16 + 2 * div * 4^prescale).
// Using dev = 52, prescale = 0 give 100KHz
    ret = osif_usb_write(&priv.adapter, OSIFI2C_SET_BIT_RATE, 52, 0,
    core::ptr::null_mut(), 0);
    if (ret) {
    dev_err(&interface.dev, "failure sending bit rate");
    return ret;
    }
    i2c_add_adapter(&(priv.adapter));
    version = le16_to_cpu(priv.usb_dev.descriptor.bcdDevice);
    dev_info(&interface.dev,
    "version %x.%02x found at bus %03d address %03d",
    version >> 8, version & 0xff,
    priv.usb_dev.bus.busnum, priv.usb_dev.devnum);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn osif_disconnect(interface: *mut usb_interface) {
    static void osif_disconnect(struct usb_interface *interface)
    {
    struct osif_priv *priv = usb_get_intfdata(interface);
    i2c_del_adapter(&(priv.adapter));
    usb_set_intfdata(interface, core::ptr::null_mut());
    }
    static struct usb_driver osif_driver = {
    .name		= "RobotFuzz Open Source InterFace, OSIF",
    .probe		= osif_probe,
    .disconnect	= osif_disconnect,
    .id_table	= osif_table,
    };
    module_usb_driver(osif_driver);
    MODULE_AUTHOR("Andrew Lunn <andrew@lunn.ch>");
    MODULE_AUTHOR("Barry Carter <barry.carter@robotfuzz.com>");
    MODULE_DESCRIPTION("RobotFuzz OSIF driver");
    MODULE_LICENSE("GPL v2");
