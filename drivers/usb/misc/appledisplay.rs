//! Automatically rewritten from C to Rust
//! Source: drivers/usb/misc/appledisplay.c
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
// Apple Cinema Display driver
//
// Copyright (C) 2006  Michael Hanselmann (linux-kernel@hansmi.ch)
//
// Thanks to Caskey L. Dickson for his work with acdctl.
//

pub const APPLE_VENDOR_ID: c_uint = 0x05AC;
pub const ACD_USB_TIMEOUT: c_int = 250;
pub const ACD_USB_EDID: c_uint = 0x0302;
pub const ACD_USB_BRIGHTNESS: c_uint = 0x0310;
pub const ACD_BTN_NONE: c_int = 0;
pub const ACD_BTN_BRIGHT_UP: c_int = 3;
pub const ACD_BTN_BRIGHT_DOWN: c_int = 4;
pub const ACD_URB_BUFFER_LEN: c_int = 2;
pub const ACD_MSG_BUFFER_LEN: c_int = 2;

    .match_flags = USB_DEVICE_ID_MATCH_DEVICE |		\
    USB_DEVICE_ID_MATCH_INT_CLASS |		\
    USB_DEVICE_ID_MATCH_INT_PROTOCOL,	\
    .idVendor = APPLE_VENDOR_ID,				\
    .idProduct = (prod),					\
    .bInterfaceClass = USB_CLASS_HID,			\
    .bInterfaceProtocol = 0x00
// table of devices that work with this driver
    static const struct usb_device_id appledisplay_table[] = {
    { APPLEDISPLAY_DEVICE(0x9218) },
    { APPLEDISPLAY_DEVICE(0x9219) },
    { APPLEDISPLAY_DEVICE(0x921c) },
    { APPLEDISPLAY_DEVICE(0x921d) },
    { APPLEDISPLAY_DEVICE(0x9222) },
    { APPLEDISPLAY_DEVICE(0x9226) },
    { APPLEDISPLAY_DEVICE(0x9236) },
// Terminating entry
    { }
    };
    MODULE_DEVICE_TABLE(usb, appledisplay_table);
// Structure to hold all of our device specific stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appledisplay {
    pub /: *mut *mut *mut usb_device udev; / usb device,
    pub /: *mut *mut *mut urb urb; / usb request block,
    pub /: *mut *mut *mut backlight_device bd; / backlight device,
    pub /: *mut *mut *mut u8 urbdata; / interrupt URB data buffer,
    pub /: *mut *mut *mut u8 msgdata; / control message data buffer,
    pub work: delayed_work,
    pub button_pressed: c_int,
    pub /: *mut *mut mutex sysfslock; / concurrent read and write,
}

    let mut count_displays: static atomic_t = ATOMIC_INIT(0);
#[no_mangle]
unsafe extern "C" fn appledisplay_complete(urb: *mut urb) {
    static void appledisplay_complete(struct urb *urb)
    {
    struct appledisplay *pdata = urb.context;
    struct device *dev = &pdata.udev.dev;
    let mut status: c_int = urb.status;
    int retval;
    switch (status) {
    case 0:
// success
    break;
    case -EOVERFLOW:
    dev_err(dev,
    "OVERFLOW with data length %d, actual length is %d\n",
    ACD_URB_BUFFER_LEN, pdata.urb.actual_length);
    fallthrough;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
// This urb is terminated, clean up
    dev_dbg(dev, "%s - urb shuttingdown with status: %d\n",
    __func__, status);
    return;
    default:
    dev_dbg(dev, "%s - nonzero urb status received: %d\n",
    __func__, status);
    goto exit;
    }
    switch(pdata.urbdata[1]) {
    case ACD_BTN_BRIGHT_UP:
    case ACD_BTN_BRIGHT_DOWN:
    pdata.button_pressed = 1;
//
// there is a window during which no device
// is registered
//
    if (pdata.bd )
    schedule_delayed_work(&pdata.work, 0);
    break;
    case ACD_BTN_NONE:
    default:
    pdata.button_pressed = 0;
    break;
    }
    exit:
    retval = usb_submit_urb(pdata.urb, GFP_ATOMIC);
    if (retval) {
    dev_err(dev, "%s - usb_submit_urb failed with result %d\n",
    __func__, retval);
    }
    }
#[no_mangle]
unsafe extern "C" fn appledisplay_bl_update_status(bd: *mut backlight_device) -> c_int {
    static int appledisplay_bl_update_status(struct backlight_device *bd)
    {
    struct appledisplay *pdata = bl_get_data(bd);
    int retval;
    mutex_lock(&pdata.sysfslock);
    pdata.msgdata[0] = 0x10;
    pdata.msgdata[1] = bd.props.brightness;
    retval = usb_control_msg(
    pdata.udev,
    usb_sndctrlpipe(pdata.udev, 0),
    HID_REQ_SET_REPORT,
    USB_DIR_OUT | USB_TYPE_CLASS | USB_RECIP_INTERFACE,
    ACD_USB_BRIGHTNESS,
    0,
    pdata.msgdata, 2,
    ACD_USB_TIMEOUT);
    mutex_unlock(&pdata.sysfslock);
    if (retval < 0)
    return retval;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn appledisplay_bl_get_brightness(bd: *mut backlight_device) -> c_int {
    static int appledisplay_bl_get_brightness(struct backlight_device *bd)
    {
    struct appledisplay *pdata = bl_get_data(bd);
    int retval, brightness;
    mutex_lock(&pdata.sysfslock);
    retval = usb_control_msg(
    pdata.udev,
    usb_rcvctrlpipe(pdata.udev, 0),
    HID_REQ_GET_REPORT,
    USB_DIR_IN | USB_TYPE_CLASS | USB_RECIP_INTERFACE,
    ACD_USB_BRIGHTNESS,
    0,
    pdata.msgdata, 2,
    ACD_USB_TIMEOUT);
    if (retval < 2) {
    if (retval >= 0)
    retval = -EMSGSIZE;
    } else {
    brightness = pdata.msgdata[1];
    }
    mutex_unlock(&pdata.sysfslock);
    if (retval < 0)
    return retval;
    else
    return brightness;
    }
    static const struct backlight_ops appledisplay_bl_data = {
    .get_brightness	= appledisplay_bl_get_brightness,
    .update_status	= appledisplay_bl_update_status,
    };
#[no_mangle]
unsafe extern "C" fn appledisplay_work(work: *mut work_struct) {
    static void appledisplay_work(struct work_struct *work)
    {
    struct appledisplay *pdata =
    container_of(work, struct appledisplay, work.work);
    int retval;
    retval = appledisplay_bl_get_brightness(pdata.bd);
    if (retval >= 0)
    pdata.bd.props.brightness = retval;
// Poll again in about 125ms if there's still a button pressed
    if (pdata.button_pressed)
    schedule_delayed_work(&pdata.work, HZ / 8);
    }
    static int appledisplay_probe(struct usb_interface *iface,
    const struct usb_device_id *id)
    {
    struct backlight_properties props;
    struct backlight_device *backlight;
    struct appledisplay *pdata;
    struct usb_device *udev = interface_to_usbdev(iface);
    struct usb_endpoint_descriptor *endpoint;
    let mut int_in_endpointAddr: c_int = 0;
    int retval, brightness;
    char bl_name[20];
// set up the endpoint information
// use only the first interrupt-in endpoint
    retval = usb_find_int_in_endpoint(iface.cur_altsetting, &endpoint);
    if (retval) {
    dev_err(&iface.dev, "Could not find int-in endpoint\n");
    return retval;
    }
    int_in_endpointAddr = endpoint.bEndpointAddress;
// allocate memory for our device state and initialize it
    pdata = kzalloc_obj(struct appledisplay);
    if (!pdata) {
    retval = -ENOMEM;
    goto error;
    }
    pdata.udev = udev;
    INIT_DELAYED_WORK(&pdata.work, appledisplay_work);
    mutex_init(&pdata.sysfslock);
// Allocate buffer for control messages
    pdata.msgdata = kmalloc(ACD_MSG_BUFFER_LEN, GFP_KERNEL);
    if (!pdata.msgdata) {
    retval = -ENOMEM;
    goto error;
    }
// Allocate interrupt URB
    pdata.urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!pdata.urb) {
    retval = -ENOMEM;
    goto error;
    }
// Allocate buffer for interrupt data
    pdata.urbdata = usb_alloc_coherent(pdata.udev, ACD_URB_BUFFER_LEN,
    GFP_KERNEL, &pdata.urb.transfer_dma);
    if (!pdata.urbdata) {
    retval = -ENOMEM;
    dev_err(&iface.dev, "Allocating URB buffer failed\n");
    goto error;
    }
// Configure interrupt URB
    usb_fill_int_urb(pdata.urb, udev,
    usb_rcvintpipe(udev, int_in_endpointAddr),
    pdata.urbdata, ACD_URB_BUFFER_LEN, appledisplay_complete,
    pdata, 1);
    pdata.urb.transfer_flags = URB_NO_TRANSFER_DMA_MAP;
    if (usb_submit_urb(pdata.urb, GFP_KERNEL)) {
    retval = -EIO;
    dev_err(&iface.dev, "Submitting URB failed\n");
    goto error;
    }
// Register backlight device
    snprintf(bl_name, sizeof(bl_name), "appledisplay%d",
    atomic_inc_return(&count_displays) - 1);
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = 0xff;
    backlight = backlight_device_register(bl_name, core::ptr::null_mut(), pdata,
    &appledisplay_bl_data, &props);
    if (IS_ERR(backlight)) {
    dev_err(&iface.dev, "Backlight registration failed\n");
    retval = PTR_ERR(backlight);
    goto error;
    }
    pdata.bd = backlight;
// Try to get brightness
    brightness = appledisplay_bl_get_brightness(pdata.bd);
    if (brightness < 0) {
    retval = brightness;
    dev_err(&iface.dev,
    "Error while getting initial brightness: %d\n", retval);
    goto error;
    }
// Set brightness in backlight device
    pdata.bd.props.brightness = brightness;
// save our data pointer in the interface device
    usb_set_intfdata(iface, pdata);
    printk(KERN_INFO "appledisplay: Apple Cinema Display connected\n");
    return 0;
    error:
    if (pdata) {
    if (pdata.urb) {
    usb_kill_urb(pdata.urb);
    cancel_delayed_work_sync(&pdata.work);
    usb_free_coherent(pdata.udev, ACD_URB_BUFFER_LEN,
    pdata.urbdata, pdata.urb.transfer_dma);
    usb_free_urb(pdata.urb);
    }
    if (!IS_ERR(pdata.bd))
    backlight_device_unregister(pdata.bd);
    kfree(pdata.msgdata);
    }
    usb_set_intfdata(iface, core::ptr::null_mut());
    kfree(pdata);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn appledisplay_disconnect(iface: *mut usb_interface) {
    static void appledisplay_disconnect(struct usb_interface *iface)
    {
    struct appledisplay *pdata = usb_get_intfdata(iface);
    if (pdata) {
    usb_kill_urb(pdata.urb);
    cancel_delayed_work_sync(&pdata.work);
    backlight_device_unregister(pdata.bd);
    usb_free_coherent(pdata.udev, ACD_URB_BUFFER_LEN,
    pdata.urbdata, pdata.urb.transfer_dma);
    usb_free_urb(pdata.urb);
    kfree(pdata.msgdata);
    kfree(pdata);
    }
    printk(KERN_INFO "appledisplay: Apple Cinema Display disconnected\n");
    }
    static struct usb_driver appledisplay_driver = {
    .name		= "appledisplay",
    .probe		= appledisplay_probe,
    .disconnect	= appledisplay_disconnect,
    .id_table	= appledisplay_table,
    };
    module_usb_driver(appledisplay_driver);
    MODULE_AUTHOR("Michael Hanselmann");
    MODULE_DESCRIPTION("Apple Cinema Display driver");
    MODULE_LICENSE("GPL");
