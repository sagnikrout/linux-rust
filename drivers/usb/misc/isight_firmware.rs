//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/isight_firmware.c
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
// Driver for loading USB isight firmware
//
// Copyright (C) 2008 Matthew Garrett <mjg@redhat.com>
//
// The USB isight cameras in recent Apples are roughly compatible with the USB
// video class specification, and can be driven by uvcvideo. However, they
// need firmware to be loaded beforehand. After firmware loading, the device
// detaches from the USB bus and reattaches with a new device ID. It can then
// be claimed by the uvc driver.
//
// The firmware is non-free and must be extracted by the user. Tools to do this
// are available at http://bersace03.free.fr/ift
//
// The isight firmware loading was reverse engineered by Johannes Berg
// <johannes@sipsolutions.de>, and this driver is based on code by Ronald
// Bultje <rbultje@ronald.bitfreak.net>
//

    static const struct usb_device_id id_table[] = {
    {USB_DEVICE(0x05ac, 0x8300)},
    {},
    };
    MODULE_DEVICE_TABLE(usb, id_table);
    static int isight_firmware_load(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *dev = interface_to_usbdev(intf);
    int llen, len, req, ret = 0;
    const struct firmware *firmware;
    unsigned char *buf = kmalloc(50, GFP_KERNEL);
    unsigned char data[4];
    const u8 *ptr;
    if (!buf)
    return -ENOMEM;
    if (request_firmware(&firmware, "isight.fw", &dev.dev) != 0) {
    printk(KERN_ERR "Unable to load isight firmware\n");
    ret = -ENODEV;
    goto out;
    }
    ptr = firmware.data;
    buf[0] = 0x01;
    if (usb_control_msg
    (dev, usb_sndctrlpipe(dev, 0), 0xa0, 0x40, 0xe600, 0, buf, 1,
    300) != 1) {
    printk(KERN_ERR
    "Failed to initialise isight firmware loader\n");
    ret = -ENODEV;
    goto out;
    }
    while (ptr+4 <= firmware.data+firmware.size) {
    memcpy(data, ptr, 4);
    len = (data[0] << 8 | data[1]);
    req = (data[2] << 8 | data[3]);
    ptr += 4;
    if (len == 0x8001)
    break;	/* success */
#[no_mangle]
pub unsafe extern "C" fn if(0: len ==) -> else {
    else if (len == 0)
    continue;
    for (; len > 0; req += 50) {
    llen = min(len, 50);
    len -= llen;
    if (ptr+llen > firmware.data+firmware.size) {
    printk(KERN_ERR
    "Malformed isight firmware");
    ret = -ENODEV;
    goto out;
    }
    memcpy(buf, ptr, llen);
    ptr += llen;
    if (usb_control_msg
    (dev, usb_sndctrlpipe(dev, 0), 0xa0, 0x40, req, 0,
    buf, llen, 300) != llen) {
    printk(KERN_ERR
    "Failed to load isight firmware\n");
    ret = -ENODEV;
    goto out;
    }
    }
    }
    buf[0] = 0x00;
    if (usb_control_msg
    (dev, usb_sndctrlpipe(dev, 0), 0xa0, 0x40, 0xe600, 0, buf, 1,
    300) != 1) {
    printk(KERN_ERR "isight firmware loading completion failed\n");
    ret = -ENODEV;
    }
    out:
    kfree(buf);
    release_firmware(firmware);
    return ret;
    }
    MODULE_FIRMWARE("isight.fw");
#[no_mangle]
unsafe extern "C" fn isight_firmware_disconnect(intf: *mut usb_interface) {
    static void isight_firmware_disconnect(struct usb_interface *intf)
    {
    }
    static struct usb_driver isight_firmware_driver = {
    .name = "isight_firmware",
    .probe = isight_firmware_load,
    .disconnect = isight_firmware_disconnect,
    .id_table = id_table,
    };
    module_usb_driver(isight_firmware_driver);
    MODULE_DESCRIPTION("iSight firmware loading support");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Matthew Garrett <mjg@redhat.com>");
