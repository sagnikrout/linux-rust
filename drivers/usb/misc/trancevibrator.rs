//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/trancevibrator.c
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
// PlayStation 2 Trance Vibrator driver
//
// Copyright (C) 2006 Sam Hocevar <sam@zoy.org>
//
// Standard include files

pub const TRANCEVIBRATOR_VENDOR_ID: c_uint = 0x0b49	/* ASCII Corporation */;
pub const TRANCEVIBRATOR_PRODUCT_ID: c_uint = 0x064f	/* Trance Vibrator */;
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(TRANCEVIBRATOR_VENDOR_ID, TRANCEVIBRATOR_PRODUCT_ID) },
    { },
    };
    MODULE_DEVICE_TABLE (usb, id_table);
// Driver-local specific stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trancevibrator {
    pub udev: *mut usb_device,
    pub speed: c_uint,
}

    static ssize_t speed_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct usb_interface *intf = to_usb_interface(dev);
    struct trancevibrator *tv = usb_get_intfdata(intf);
    return sprintf(buf, "%d\n", tv.speed);
    }
    static ssize_t speed_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct usb_interface *intf = to_usb_interface(dev);
    struct trancevibrator *tv = usb_get_intfdata(intf);
    int temp, retval, old;
    retval = kstrtoint(buf, 10, &temp);
    if (retval)
    return retval;
    if (temp > 255)
    temp = 255;
#[no_mangle]
pub unsafe extern "C" fn if(0: temp <) -> else {
    else if (temp < 0)
    temp = 0;
    old = tv.speed;
    tv.speed = temp;
    dev_dbg(&tv.udev.dev, "speed = %d\n", tv.speed);
// Set speed
    retval = usb_control_msg(tv.udev, usb_sndctrlpipe(tv.udev, 0),
    0x01, /* vendor request: set speed */
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_OTHER,
    tv.speed, /* speed value */
    0, core::ptr::null_mut(), 0, USB_CTRL_SET_TIMEOUT);
    if (retval) {
    tv.speed = old;
    dev_dbg(&tv.udev.dev, "retval = %d\n", retval);
    return retval;
    }
    return count;
    }
    static DEVICE_ATTR_RW(speed);
    static struct attribute *tv_attrs[] = {
    &dev_attr_speed.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(tv);
    static int tv_probe(struct usb_interface *interface,
    const struct usb_device_id *id)
    {
    struct usb_device *udev = interface_to_usbdev(interface);
    struct trancevibrator *dev;
    int retval;
    dev = kzalloc_obj(struct trancevibrator);
    if (!dev) {
    retval = -ENOMEM;
    goto error;
    }
    dev.udev = udev;
    usb_set_intfdata(interface, dev);
    return 0;
    error:
    kfree(dev);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn tv_disconnect(interface: *mut usb_interface) {
    static void tv_disconnect(struct usb_interface *interface)
    {
    struct trancevibrator *dev;
    dev = usb_get_intfdata (interface);
    usb_set_intfdata(interface, core::ptr::null_mut());
    kfree(dev);
    }
// USB subsystem object
    static struct usb_driver tv_driver = {
    .name =		"trancevibrator",
    .probe =	tv_probe,
    .disconnect =	tv_disconnect,
    .id_table =	id_table,
    .dev_groups =	tv_groups,
    };
    module_usb_driver(tv_driver);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
