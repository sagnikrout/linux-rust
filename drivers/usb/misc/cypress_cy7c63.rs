//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/cypress_cy7c63.c
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
// cypress_cy7c63.c
//
// Copyright (c) 2006-2007 Oliver Bock (bock@tfh-berlin.de)
//
// This driver is based on the Cypress USB Driver by Marcus Maul
// (cyport) and the 2.0 version of Greg Kroah-Hartman's
// USB Skeleton driver.
//
// This is a generic driver for the Cypress CY7C63xxx family.
// For the time being it enables you to read from and write to
// the single I/O ports of the device.
//
// Supported vendors:	AK Modul-Bus Computer GmbH
// (Firmware "Port-Chip")
//
// Supported devices:	CY7C63001A-PC
// CY7C63001C-PXC
// CY7C63001C-SXC
//
// Supported functions:	Read/Write Ports
//
// For up-to-date information please visit:
// http://www.obock.de/kernel/cypress
//

pub const CYPRESS_VENDOR_ID: c_uint = 0xa2c;
pub const CYPRESS_PRODUCT_ID: c_uint = 0x8;
pub const CYPRESS_READ_PORT: c_uint = 0x4;
pub const CYPRESS_WRITE_PORT: c_uint = 0x5;
pub const CYPRESS_READ_RAM: c_uint = 0x2;
pub const CYPRESS_WRITE_RAM: c_uint = 0x3;
pub const CYPRESS_READ_ROM: c_uint = 0x1;
pub const CYPRESS_READ_PORT_ID0: c_int = 0;
pub const CYPRESS_WRITE_PORT_ID0: c_int = 0;
pub const CYPRESS_READ_PORT_ID1: c_uint = 0x2;
pub const CYPRESS_WRITE_PORT_ID1: c_int = 1;
pub const CYPRESS_MAX_REQSIZE: c_int = 8;
// table of devices that work with this driver
    static const struct usb_device_id cypress_table[] = {
    { USB_DEVICE(CYPRESS_VENDOR_ID, CYPRESS_PRODUCT_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(usb, cypress_table);
// structure to hold all of our device specific stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cypress {
    pub udev: *mut *mut usb_device,
    pub port: [c_uchar; 2],
}

// used to send usb control messages to device
    static int vendor_command(struct cypress *dev, unsigned char request,
    unsigned char address, unsigned char data)
    {
    let mut retval: c_int = 0;
    unsigned int pipe;
    unsigned char *iobuf;
// allocate some memory for the i/o buffer
    iobuf = kzalloc(CYPRESS_MAX_REQSIZE, GFP_KERNEL);
    if (!iobuf) {
    retval = -ENOMEM;
    goto error;
    }
    dev_dbg(&dev.udev.dev, "Sending usb_control_msg (data: %d)\n", data);
// prepare usb control message and send it upstream
    pipe = usb_rcvctrlpipe(dev.udev, 0);
    retval = usb_control_msg(dev.udev, pipe, request,
    USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_OTHER,
    address, data, iobuf, CYPRESS_MAX_REQSIZE,
    USB_CTRL_GET_TIMEOUT);
// we must not process garbage
    if (retval < 2) {
    if (retval >= 0)
    retval = -EIO;
    goto err_buf;
    }
// store returned data (more READs to be added)
    switch (request) {
    case CYPRESS_READ_PORT:
    if (address == CYPRESS_READ_PORT_ID0) {
    dev.port[0] = iobuf[1];
    dev_dbg(&dev.udev.dev,
    "READ_PORT0 returned: %d\n",
    dev.port[0]);
    }
#[no_mangle]
pub unsafe extern "C" fn if(CYPRESS_READ_PORT_ID1: address ==) -> else {
    dev.port[1] = iobuf[1];
    dev_dbg(&dev.udev.dev,
    "READ_PORT1 returned: %d\n",
    dev.port[1]);
    }
    break;
    }
    err_buf:
    kfree(iobuf);
    error:
    return retval;
    }
// write port value
    static ssize_t write_port(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count,
    int port_num, int write_id)
    {
    let mut value: c_int = -1;
    let mut result: c_int = 0;
    struct usb_interface *intf = to_usb_interface(dev);
    struct cypress *cyp = usb_get_intfdata(intf);
    dev_dbg(&cyp.udev.dev, "WRITE_PORT%d called\n", port_num);
// validate input data
    if (sscanf(buf, "%d", &value) < 1) {
    result = -EINVAL;
    goto error;
    }
    if (value < 0 || value > 255) {
    result = -EINVAL;
    goto error;
    }
    result = vendor_command(cyp, CYPRESS_WRITE_PORT, write_id,
    (unsigned char)value);
    dev_dbg(&cyp.udev.dev, "Result of vendor_command: %d\n\n", result);
    error:
    return result < 0 ? result : count;
    }
// attribute callback handler (write)
    static ssize_t port0_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    return write_port(dev, attr, buf, count, 0, CYPRESS_WRITE_PORT_ID0);
    }
// attribute callback handler (write)
    static ssize_t port1_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    return write_port(dev, attr, buf, count, 1, CYPRESS_WRITE_PORT_ID1);
    }
// read port value
    static ssize_t read_port(struct device *dev, struct device_attribute *attr,
    char *buf, int port_num, int read_id)
    {
    let mut result: c_int = 0;
    struct usb_interface *intf = to_usb_interface(dev);
    struct cypress *cyp = usb_get_intfdata(intf);
    dev_dbg(&cyp.udev.dev, "READ_PORT%d called\n", port_num);
    result = vendor_command(cyp, CYPRESS_READ_PORT, read_id, 0);
    dev_dbg(&cyp.udev.dev, "Result of vendor_command: %d\n\n", result);
    if (result < 0)
    return result;
    return sprintf(buf, "%d", cyp.port[port_num]);
    }
// attribute callback handler (read)
    static ssize_t port0_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return read_port(dev, attr, buf, 0, CYPRESS_READ_PORT_ID0);
    }
    static DEVICE_ATTR_RW(port0);
// attribute callback handler (read)
    static ssize_t port1_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return read_port(dev, attr, buf, 1, CYPRESS_READ_PORT_ID1);
    }
    static DEVICE_ATTR_RW(port1);
    static struct attribute *cypress_attrs[] = {
    &dev_attr_port0.attr,
    &dev_attr_port1.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(cypress);
    static int cypress_probe(struct usb_interface *interface,
    const struct usb_device_id *id)
    {
    struct cypress *dev;
    let mut retval: c_int = -ENOMEM;
// allocate memory for our device state and initialize it
    dev = kzalloc_obj(*dev);
    if (!dev)
    goto error_mem;
    dev.udev = interface_to_usbdev(interface);
// save our data pointer in this interface device
    usb_set_intfdata(interface, dev);
// let the user know that the device is now attached
    dev_info(&interface.dev,
    "Cypress CY7C63xxx device now attached\n");
    return 0;
    error_mem:
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn cypress_disconnect(interface: *mut usb_interface) {
    static void cypress_disconnect(struct usb_interface *interface)
    {
    struct cypress *dev;
    dev = usb_get_intfdata(interface);
// the intfdata can be set to NULL only after the
// device files have been removed
    usb_set_intfdata(interface, core::ptr::null_mut());
    dev_info(&interface.dev,
    "Cypress CY7C63xxx device now disconnected\n");
    kfree(dev);
    }
    static struct usb_driver cypress_driver = {
    .name = "cypress_cy7c63",
    .probe = cypress_probe,
    .disconnect = cypress_disconnect,
    .id_table = cypress_table,
    .dev_groups = cypress_groups,
    };
    module_usb_driver(cypress_driver);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
