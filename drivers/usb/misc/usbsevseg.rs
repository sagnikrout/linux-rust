//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/usbsevseg.c
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
// USB 7 Segment Driver
//
// Copyright (C) 2008 Harrison Metzger <harrisonmetz@gmail.com>
// Based on usbled.c by Greg Kroah-Hartman (greg@kroah.com)
//

pub const VENDOR_ID: c_uint = 0x0fc5;
pub const PRODUCT_ID: c_uint = 0x1227;
pub const MAXLEN: c_int = 8;
// table of devices that work with this driver
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE(VENDOR_ID, PRODUCT_ID) },
    { },
    };
    MODULE_DEVICE_TABLE(usb, id_table);
// the different text display modes the device is capable of
    static const char *display_textmodes[] = {"raw", "hex", "ascii"};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_sevsegdev {
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub powered: u8,
    pub mode_msb: u8,
    pub mode_lsb: u8,
    pub decimals: [u8; MAXLEN],
    pub textmode: u8,
    pub text: [u8; MAXLEN],
    pub textlength: u16,
    pub /: *mut *mut u8 shadow_power; / for PM,
    pub has_interface_pm: u8,
}

// sysfs_streq can't replace this completely
// If the device was in hex mode, and the user wanted a 0,
// if str commands are used, we would assume the end of string
// so mem commands are used.
//
#[no_mangle]
pub unsafe extern "C" fn my_memlen(buf: *const c_char, count: usize) -> usize {
    static inline size_t my_memlen(const char *buf, size_t count)
    {
    if (count > 0 && buf[count-1] == '\n')
    return count - 1;
    else
    return count;
    }
#[no_mangle]
unsafe extern "C" fn update_display_powered(mydev: *mut usb_sevsegdev) {
    static void update_display_powered(struct usb_sevsegdev *mydev)
    {
    int rc;
    if (mydev.powered && !mydev.has_interface_pm) {
    rc = usb_autopm_get_interface(mydev.intf);
    if (rc < 0)
    return;
    mydev.has_interface_pm = 1;
    }
    if (mydev.shadow_power != 1)
    return;
    rc = usb_control_msg_send(mydev.udev, 0, 0x12, 0x48,
    (80 * 0x100) + 10, /*  (power mode) */
    (0x00 * 0x100) + (mydev.powered ? 1 : 0),
    core::ptr::null_mut(), 0, 2000, GFP_KERNEL);
    if (rc < 0)
    dev_dbg(&mydev.udev.dev, "power retval = %d\n", rc);
    if (!mydev.powered && mydev.has_interface_pm) {
    usb_autopm_put_interface(mydev.intf);
    mydev.has_interface_pm = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn update_display_mode(mydev: *mut usb_sevsegdev) {
    static void update_display_mode(struct usb_sevsegdev *mydev)
    {
    int rc;
    if(mydev.shadow_power != 1)
    return;
    rc = usb_control_msg_send(mydev.udev, 0, 0x12, 0x48,
    (82 * 0x100) + 10, /* (set mode) */
    (mydev.mode_msb * 0x100) + mydev.mode_lsb,
    core::ptr::null_mut(), 0, 2000, GFP_NOIO);
    if (rc < 0)
    dev_dbg(&mydev.udev.dev, "mode retval = %d\n", rc);
    }
#[no_mangle]
unsafe extern "C" fn update_display_visual(mydev: *mut usb_sevsegdev, mf: gfp_t) {
    static void update_display_visual(struct usb_sevsegdev *mydev, gfp_t mf)
    {
    int rc;
    int i;
    unsigned char buffer[MAXLEN] = {0};
    let mut decimals: u8 = 0;
    if(mydev.shadow_power != 1)
    return;
// The device is right to left, where as you write left to right
    for (i = 0; i < mydev.textlength; i++)
    buffer[i] = mydev.text[mydev.textlength-1-i];
    rc = usb_control_msg_send(mydev.udev, 0, 0x12, 0x48,
    (85 * 0x100) + 10, /* (write text) */
    (0 * 0x100) + mydev.textmode, /* mode  */
    &buffer, mydev.textlength, 2000, mf);
    if (rc < 0)
    dev_dbg(&mydev.udev.dev, "write retval = %d\n", rc);
// The device is right to left, where as you write left to right
    for (i = 0; i < sizeof(mydev.decimals); i++)
    decimals |= mydev.decimals[i] << i;
    rc = usb_control_msg_send(mydev.udev, 0, 0x12, 0x48,
    (86 * 0x100) + 10, /* (set decimal) */
    (0 * 0x100) + decimals, /* decimals */
    core::ptr::null_mut(), 0, 2000, mf);
    if (rc < 0)
    dev_dbg(&mydev.udev.dev, "decimal retval = %d\n", rc);
    }

    static ssize_t name##_show(struct device *dev,			\
    struct device_attribute *attr, char *buf) 		\
    {								\
    struct usb_interface *intf = to_usb_interface(dev);	\
    struct usb_sevsegdev *mydev = usb_get_intfdata(intf);	\
    \
    return sprintf(buf, "%u\n", mydev.name);		\
    }								\
    \
    static ssize_t name##_store(struct device *dev,			\
    struct device_attribute *attr, const char *buf, size_t count) \
    {								\
    struct usb_interface *intf = to_usb_interface(dev);	\
    struct usb_sevsegdev *mydev = usb_get_intfdata(intf);	\
    \
    mydev.name = simple_strtoul(buf, core::ptr::null_mut(), 10);		\
    update_fcn(mydev); 					\
    \
    return count;						\
    }								\
    static DEVICE_ATTR_RW(name);
    static ssize_t text_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct usb_interface *intf = to_usb_interface(dev);
    struct usb_sevsegdev *mydev = usb_get_intfdata(intf);
    return sysfs_emit(buf, "%s\n", mydev.text);
    }
    static ssize_t text_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct usb_interface *intf = to_usb_interface(dev);
    struct usb_sevsegdev *mydev = usb_get_intfdata(intf);
    let mut end: usize = my_memlen(buf, count);
    if (end > sizeof(mydev.text))
    return -EINVAL;
    memset(mydev.text, 0, sizeof(mydev.text));
    mydev.textlength = end;
    if (end > 0)
    memcpy(mydev.text, buf, end);
    update_display_visual(mydev, GFP_KERNEL);
    return count;
    }
    static DEVICE_ATTR_RW(text);
    static ssize_t decimals_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct usb_interface *intf = to_usb_interface(dev);
    struct usb_sevsegdev *mydev = usb_get_intfdata(intf);
    int i;
    int pos;
    for (i = 0; i < sizeof(mydev.decimals); i++) {
    pos = sizeof(mydev.decimals) - 1 - i;
    if (mydev.decimals[i] == 0)
    buf[pos] = '0';
#[no_mangle]
pub unsafe extern "C" fn if(1: mydev->decimals[i] ==) -> else {
    else if (mydev.decimals[i] == 1)
    buf[pos] = '1';
    else
    buf[pos] = 'x';
    }
    buf[sizeof(mydev.decimals)] = '\n';
    return sizeof(mydev.decimals) + 1;
    }
    static ssize_t decimals_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct usb_interface *intf = to_usb_interface(dev);
    struct usb_sevsegdev *mydev = usb_get_intfdata(intf);
    let mut end: usize = my_memlen(buf, count);
    int i;
    if (end > sizeof(mydev.decimals))
    return -EINVAL;
    for (i = 0; i < end; i++)
    if (buf[i] != '0' && buf[i] != '1')
    return -EINVAL;
    memset(mydev.decimals, 0, sizeof(mydev.decimals));
    for (i = 0; i < end; i++)
    if (buf[i] == '1')
    mydev.decimals[end-1-i] = 1;
    update_display_visual(mydev, GFP_KERNEL);
    return count;
    }
    static DEVICE_ATTR_RW(decimals);
    static ssize_t textmode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct usb_interface *intf = to_usb_interface(dev);
    struct usb_sevsegdev *mydev = usb_get_intfdata(intf);
    int i;
    buf[0] = 0;
    for (i = 0; i < ARRAY_SIZE(display_textmodes); i++) {
    if (mydev.textmode == i) {
    strcat(buf, " [");
    strcat(buf, display_textmodes[i]);
    strcat(buf, "] ");
    } else {
    strcat(buf, " ");
    strcat(buf, display_textmodes[i]);
    strcat(buf, " ");
    }
    }
    strcat(buf, "\n");
    return strlen(buf);
    }
    static ssize_t textmode_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct usb_interface *intf = to_usb_interface(dev);
    struct usb_sevsegdev *mydev = usb_get_intfdata(intf);
    int i;
    i = sysfs_match_string(display_textmodes, buf);
    if (i < 0)
    return i;
    mydev.textmode = i;
    update_display_visual(mydev, GFP_KERNEL);
    return count;
    }
    static DEVICE_ATTR_RW(textmode);
    MYDEV_ATTR_SIMPLE_UNSIGNED(powered, update_display_powered);
    MYDEV_ATTR_SIMPLE_UNSIGNED(mode_msb, update_display_mode);
    MYDEV_ATTR_SIMPLE_UNSIGNED(mode_lsb, update_display_mode);
    static struct attribute *sevseg_attrs[] = {
    &dev_attr_powered.attr,
    &dev_attr_text.attr,
    &dev_attr_textmode.attr,
    &dev_attr_decimals.attr,
    &dev_attr_mode_msb.attr,
    &dev_attr_mode_lsb.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(sevseg);
    static int sevseg_probe(struct usb_interface *interface,
    const struct usb_device_id *id)
    {
    struct usb_device *udev = interface_to_usbdev(interface);
    struct usb_sevsegdev *mydev;
    let mut rc: c_int = -ENOMEM;
    mydev = kzalloc_obj(struct usb_sevsegdev);
    if (!mydev)
    goto error_mem;
    mydev.udev = udev;
    mydev.intf = interface;
    usb_set_intfdata(interface, mydev);
// PM
    mydev.shadow_power = 1; /* currently active */
    mydev.has_interface_pm = 0; /* have not issued autopm_get */
// set defaults
    mydev.textmode = 0x02; /* ascii mode */
    mydev.mode_msb = 0x06; /* 6 characters */
    mydev.mode_lsb = 0x3f; /* scanmode for 6 chars */
    dev_info(&interface.dev, "USB 7 Segment device now attached\n");
    return 0;
    error_mem:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn sevseg_disconnect(interface: *mut usb_interface) {
    static void sevseg_disconnect(struct usb_interface *interface)
    {
    struct usb_sevsegdev *mydev;
    mydev = usb_get_intfdata(interface);
    usb_set_intfdata(interface, core::ptr::null_mut());
    kfree(mydev);
    dev_info(&interface.dev, "USB 7 Segment now disconnected\n");
    }
#[no_mangle]
unsafe extern "C" fn sevseg_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int {
    static int sevseg_suspend(struct usb_interface *intf, pm_message_t message)
    {
    struct usb_sevsegdev *mydev;
    mydev = usb_get_intfdata(intf);
    mydev.shadow_power = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sevseg_resume(intf: *mut usb_interface) -> c_int {
    static int sevseg_resume(struct usb_interface *intf)
    {
    struct usb_sevsegdev *mydev;
    mydev = usb_get_intfdata(intf);
    mydev.shadow_power = 1;
    update_display_mode(mydev);
    update_display_visual(mydev, GFP_NOIO);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sevseg_reset_resume(intf: *mut usb_interface) -> c_int {
    static int sevseg_reset_resume(struct usb_interface *intf)
    {
    struct usb_sevsegdev *mydev;
    mydev = usb_get_intfdata(intf);
    mydev.shadow_power = 1;
    update_display_mode(mydev);
    update_display_visual(mydev, GFP_NOIO);
    return 0;
    }
    static struct usb_driver sevseg_driver = {
    .name =		"usbsevseg",
    .probe =	sevseg_probe,
    .disconnect =	sevseg_disconnect,
    .suspend =	sevseg_suspend,
    .resume =	sevseg_resume,
    .reset_resume =	sevseg_reset_resume,
    .id_table =	id_table,
    .dev_groups =	sevseg_groups,
    .supports_autosuspend = 1,
    };
    module_usb_driver(sevseg_driver);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
