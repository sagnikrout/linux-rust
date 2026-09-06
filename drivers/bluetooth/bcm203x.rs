//! Automatically rewritten from C to Rust
//! Source: drivers/bluetooth/bcm203x.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Broadcom Blutonium firmware driver
//
// Copyright (C) 2003  Maxim Krasnyansky <maxk@qualcomm.com>
// Copyright (C) 2003  Marcel Holtmann <marcel@holtmann.org>
//

    static const struct usb_device_id bcm203x_table[] = {
// Broadcom Blutonium (BCM2033)
    { USB_DEVICE(0x0a5c, 0x2033) },
    { }	/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, bcm203x_table);
pub const BCM203X_ERROR: c_int = 0;
pub const BCM203X_RESET: c_int = 1;
pub const BCM203X_LOAD_MINIDRV: c_int = 2;
pub const BCM203X_SELECT_MEMORY: c_int = 3;
pub const BCM203X_CHECK_MEMORY: c_int = 4;
pub const BCM203X_LOAD_FIRMWARE: c_int = 5;
pub const BCM203X_CHECK_FIRMWARE: c_int = 6;
pub const BCM203X_IN_EP: c_uint = 0x81;
pub const BCM203X_OUT_EP: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm203x_data {
    pub udev: *mut usb_device,
    pub state: c_ulong,
    pub work: work_struct,
    pub shutdown: core::sync::atomic::AtomicI32,
    pub urb: *mut urb,
    pub buffer: *mut c_uchar,
    pub fw_data: *mut c_uchar,
    pub fw_size: c_uint,
    pub fw_sent: c_uint,
}

#[no_mangle]
unsafe extern "C" fn bcm203x_complete(urb: *mut urb) {
    static void bcm203x_complete(struct urb *urb)
    {
    struct bcm203x_data *data = urb.context;
    struct usb_device *udev = urb.dev;
    int len;
    BT_DBG("udev %p urb %p", udev, urb);
    if (urb.status) {
    BT_ERR("URB failed with status %d", urb.status);
    data.state = BCM203X_ERROR;
    return;
    }
    switch (data.state) {
    case BCM203X_LOAD_MINIDRV:
    memcpy(data.buffer, "#", 1);
    usb_fill_bulk_urb(urb, udev, usb_sndbulkpipe(udev, BCM203X_OUT_EP),
    data.buffer, 1, bcm203x_complete, data);
    data.state = BCM203X_SELECT_MEMORY;
// use workqueue to have a small delay
    schedule_work(&data.work);
    break;
    case BCM203X_SELECT_MEMORY:
    usb_fill_int_urb(urb, udev, usb_rcvintpipe(udev, BCM203X_IN_EP),
    data.buffer, 32, bcm203x_complete, data, 1);
    data.state = BCM203X_CHECK_MEMORY;
    if (usb_submit_urb(data.urb, GFP_ATOMIC) < 0)
    BT_ERR("Can't submit URB");
    break;
    case BCM203X_CHECK_MEMORY:
    if (data.buffer[0] != '#') {
    BT_ERR("Memory select failed");
    data.state = BCM203X_ERROR;
    break;
    }
    data.state = BCM203X_LOAD_FIRMWARE;
    fallthrough;
    case BCM203X_LOAD_FIRMWARE:
    if (data.fw_sent == data.fw_size) {
    usb_fill_int_urb(urb, udev, usb_rcvintpipe(udev, BCM203X_IN_EP),
    data.buffer, 32, bcm203x_complete, data, 1);
    data.state = BCM203X_CHECK_FIRMWARE;
    } else {
    len = min_t(uint, data.fw_size - data.fw_sent, 4096);
    usb_fill_bulk_urb(urb, udev, usb_sndbulkpipe(udev, BCM203X_OUT_EP),
    data.fw_data + data.fw_sent, len, bcm203x_complete, data);
    data.fw_sent += len;
    }
    if (usb_submit_urb(data.urb, GFP_ATOMIC) < 0)
    BT_ERR("Can't submit URB");
    break;
    case BCM203X_CHECK_FIRMWARE:
    if (data.buffer[0] != '.') {
    BT_ERR("Firmware loading failed");
    data.state = BCM203X_ERROR;
    break;
    }
    data.state = BCM203X_RESET;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm203x_work(work: *mut work_struct) {
    static void bcm203x_work(struct work_struct *work)
    {
    struct bcm203x_data *data =
    container_of(work, struct bcm203x_data, work);
    if (atomic_read(&data.shutdown))
    return;
    if (usb_submit_urb(data.urb, GFP_KERNEL) < 0)
    BT_ERR("Can't submit URB");
    }
#[no_mangle]
unsafe extern "C" fn bcm203x_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int bcm203x_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    const struct firmware *firmware;
    struct usb_device *udev = interface_to_usbdev(intf);
    struct bcm203x_data *data;
    int size;
    BT_DBG("intf %p id %p", intf, id);
    if (intf.cur_altsetting.desc.bInterfaceNumber != 0)
    return -ENODEV;
    data = devm_kzalloc(&intf.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.udev  = udev;
    data.state = BCM203X_LOAD_MINIDRV;
    data.urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!data.urb)
    return -ENOMEM;
    if (request_firmware(&firmware, "BCM2033-MD.hex", &udev.dev) < 0) {
    BT_ERR("Mini driver request failed");
    usb_free_urb(data.urb);
    return -EIO;
    }
    BT_DBG("minidrv data %p size %zu", firmware.data, firmware.size);
    size = max_t(uint, firmware.size, 4096);
    data.buffer = kmalloc(size, GFP_KERNEL);
    if (!data.buffer) {
    BT_ERR("Can't allocate memory for mini driver");
    release_firmware(firmware);
    usb_free_urb(data.urb);
    return -ENOMEM;
    }
    memcpy(data.buffer, firmware.data, firmware.size);
    usb_fill_bulk_urb(data.urb, udev, usb_sndbulkpipe(udev, BCM203X_OUT_EP),
    data.buffer, firmware.size, bcm203x_complete, data);
    release_firmware(firmware);
    if (request_firmware(&firmware, "BCM2033-FW.bin", &udev.dev) < 0) {
    BT_ERR("Firmware request failed");
    usb_free_urb(data.urb);
    kfree(data.buffer);
    return -EIO;
    }
    BT_DBG("firmware data %p size %zu", firmware.data, firmware.size);
    data.fw_data = kmemdup(firmware.data, firmware.size, GFP_KERNEL);
    if (!data.fw_data) {
    BT_ERR("Can't allocate memory for firmware image");
    release_firmware(firmware);
    usb_free_urb(data.urb);
    kfree(data.buffer);
    return -ENOMEM;
    }
    data.fw_size = firmware.size;
    data.fw_sent = 0;
    release_firmware(firmware);
    INIT_WORK(&data.work, bcm203x_work);
    usb_set_intfdata(intf, data);
// use workqueue to have a small delay
    schedule_work(&data.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm203x_disconnect(intf: *mut usb_interface) {
    static void bcm203x_disconnect(struct usb_interface *intf)
    {
    struct bcm203x_data *data = usb_get_intfdata(intf);
    BT_DBG("intf %p", intf);
    atomic_inc(&data.shutdown);
    cancel_work_sync(&data.work);
    usb_kill_urb(data.urb);
    usb_set_intfdata(intf, core::ptr::null_mut());
    usb_free_urb(data.urb);
    kfree(data.fw_data);
    kfree(data.buffer);
    }
    static struct usb_driver bcm203x_driver = {
    .name		= "bcm203x",
    .probe		= bcm203x_probe,
    .disconnect	= bcm203x_disconnect,
    .id_table	= bcm203x_table,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(bcm203x_driver);
    MODULE_AUTHOR("Marcel Holtmann <marcel@holtmann.org>");
    MODULE_DESCRIPTION("Broadcom Blutonium firmware driver ver " VERSION);
    MODULE_VERSION(VERSION);
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE("BCM2033-MD.hex");
    MODULE_FIRMWARE("BCM2033-FW.bin");
