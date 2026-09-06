//! Automatically rewritten from C to Rust
//! Source: drivers/input/mouse/synaptics_usb.c
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
// USB Synaptics device driver
//
// Copyright (c) 2002 Rob Miller (rob@inpharmatica . co . uk)
// Copyright (c) 2003 Ron Lee (ron@debian.org)
// cPad driver for kernel 2.4
//
// Copyright (c) 2004 Jan Steinhoff (cpad@jan-steinhoff . de)
// Copyright (c) 2004 Ron Lee (ron@debian.org)
// rewritten for kernel 2.6
//
// cPad display character device part is not included. It can be found at
// http://jan-steinhoff.de/linux/synaptics-usb.html
//
// Bases on:	usb_skeleton.c v2.2 by Greg Kroah-Hartman
// drivers/hid/usbhid/usbmouse.c by Vojtech Pavlik
// drivers/input/mouse/synaptics.c by Peter Osterlund
//
// Trademarks are the property of their respective owners.
//
// There are three different types of Synaptics USB devices: Touchpads,
// touchsticks (or trackpoints), and touchscreens. Touchpads are well supported
// by this driver, touchstick support has not been tested much yet, and
// touchscreens have not been tested at all.
//
// Up to three alternate settings are possible:
// setting 0: one int endpoint for relative movement (used by usbhid.ko)
// setting 1: one int endpoint for absolute finger position
// setting 2 (cPad only): one int endpoint for absolute finger position and
// two bulk endpoints for the display (in/out)
// This driver uses setting 1.
//

pub const USB_VENDOR_ID_SYNAPTICS: c_uint = 0x06cb;
pub const USB_DEVICE_ID_SYNAPTICS_TP: c_uint = 0x0001	/* Synaptics USB TouchPad */;
pub const USB_DEVICE_ID_SYNAPTICS_INT_TP: c_uint = 0x0002	/* Integrated USB TouchPad */;
pub const USB_DEVICE_ID_SYNAPTICS_CPAD: c_uint = 0x0003	/* Synaptics cPad */;
pub const USB_DEVICE_ID_SYNAPTICS_TS: c_uint = 0x0006	/* Synaptics TouchScreen */;
pub const USB_DEVICE_ID_SYNAPTICS_STICK: c_uint = 0x0007	/* Synaptics USB Styk */;
pub const USB_DEVICE_ID_SYNAPTICS_WP: c_uint = 0x0008	/* Synaptics USB WheelPad */;
pub const USB_DEVICE_ID_SYNAPTICS_COMP_TP: c_uint = 0x0009	/* Composite USB TouchPad */;
pub const USB_DEVICE_ID_SYNAPTICS_WTP: c_uint = 0x0010	/* Wireless TouchPad */;
pub const USB_DEVICE_ID_SYNAPTICS_DPAD: c_uint = 0x0013	/* DisplayPad */;

    USB_DEVICE(USB_VENDOR_ID_SYNAPTICS,		\
    USB_DEVICE_ID_SYNAPTICS_##prod),	\
    .driver_info = (kind),
pub const SYNUSB_RECV_SIZE: c_int = 8;
pub const XMIN_NOMINAL: c_int = 1472;
pub const XMAX_NOMINAL: c_int = 5472;
pub const YMIN_NOMINAL: c_int = 1408;
pub const YMAX_NOMINAL: c_int = 4448;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct synusb {
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub urb: *mut urb,
    pub data: *mut c_uchar,
// serialize access to open/suspend
    pub pm_mutex: mutex,
    pub is_open: bool,
// input device related data structures
    pub input: *mut input_dev,
    pub name: [c_char; 128],
    pub phys: [c_char; 64],
// characteristics of the device
    pub flags: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn synusb_report_buttons(synusb: *mut synusb) {
    static void synusb_report_buttons(struct synusb *synusb)
    {
    struct input_dev *input_dev = synusb.input;
    input_report_key(input_dev, BTN_LEFT, synusb.data[1] & 0x04);
    input_report_key(input_dev, BTN_RIGHT, synusb.data[1] & 0x01);
    input_report_key(input_dev, BTN_MIDDLE, synusb.data[1] & 0x02);
    }
#[no_mangle]
unsafe extern "C" fn synusb_report_stick(synusb: *mut synusb) {
    static void synusb_report_stick(struct synusb *synusb)
    {
    struct input_dev *input_dev = synusb.input;
    int x, y;
    unsigned int pressure;
    pressure = synusb.data[6];
    x = (s16)(be16_to_cpup((__be16 *)&synusb.data[2]) << 3) >> 7;
    y = (s16)(be16_to_cpup((__be16 *)&synusb.data[4]) << 3) >> 7;
    if (pressure > 0) {
    input_report_rel(input_dev, REL_X, x);
    input_report_rel(input_dev, REL_Y, -y);
    }
    input_report_abs(input_dev, ABS_PRESSURE, pressure);
    synusb_report_buttons(synusb);
    input_sync(input_dev);
    }
#[no_mangle]
unsafe extern "C" fn synusb_report_touchpad(synusb: *mut synusb) {
    static void synusb_report_touchpad(struct synusb *synusb)
    {
    struct input_dev *input_dev = synusb.input;
    unsigned int num_fingers, tool_width;
    unsigned int x, y;
    unsigned int pressure, w;
    pressure = synusb.data[6];
    x = be16_to_cpup((__be16 *)&synusb.data[2]);
    y = be16_to_cpup((__be16 *)&synusb.data[4]);
    w = synusb.data[0] & 0x0f;
    if (pressure > 0) {
    num_fingers = 1;
    tool_width = 5;
    switch (w) {
    case 0 ... 1:
    num_fingers = 2 + w;
    break;
    case 2:	                /* pen, pretend its a finger */
    break;
    case 4 ... 15:
    tool_width = w;
    break;
    }
    } else {
    num_fingers = 0;
    tool_width = 0;
    }
//
// Post events
// BTN_TOUCH has to be first as mousedev relies on it when doing
// absolute -> relative conversion
//
    if (pressure > 30)
    input_report_key(input_dev, BTN_TOUCH, 1);
    if (pressure < 25)
    input_report_key(input_dev, BTN_TOUCH, 0);
    if (num_fingers > 0) {
    input_report_abs(input_dev, ABS_X, x);
    input_report_abs(input_dev, ABS_Y,
    YMAX_NOMINAL + YMIN_NOMINAL - y);
    }
    input_report_abs(input_dev, ABS_PRESSURE, pressure);
    input_report_abs(input_dev, ABS_TOOL_WIDTH, tool_width);
    input_report_key(input_dev, BTN_TOOL_FINGER, num_fingers == 1);
    input_report_key(input_dev, BTN_TOOL_DOUBLETAP, num_fingers == 2);
    input_report_key(input_dev, BTN_TOOL_TRIPLETAP, num_fingers == 3);
    synusb_report_buttons(synusb);
    if (synusb.flags & SYNUSB_AUXDISPLAY)
    input_report_key(input_dev, BTN_MIDDLE, synusb.data[1] & 0x08);
    input_sync(input_dev);
    }
#[no_mangle]
unsafe extern "C" fn synusb_irq(urb: *mut urb) {
    static void synusb_irq(struct urb *urb)
    {
    struct synusb *synusb = urb.context;
    int error;
// Check our status in case we need to bail out early.
    switch (urb.status) {
    case 0:
    usb_mark_last_busy(synusb.udev);
    break;
// Device went away so don't keep trying to read from it.
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    return;
    default:
    goto resubmit;
    break;
    }
    if (synusb.flags & SYNUSB_STICK)
    synusb_report_stick(synusb);
    else
    synusb_report_touchpad(synusb);
    resubmit:
    error = usb_submit_urb(urb, GFP_ATOMIC);
    if (error && error != -EPERM)
    dev_err(&synusb.intf.dev,
    "%s - usb_submit_urb failed with result: %d",
    __func__, error);
    }
#[no_mangle]
unsafe extern "C" fn synusb_open(dev: *mut input_dev) -> c_int {
    static int synusb_open(struct input_dev *dev)
    {
    struct synusb *synusb = input_get_drvdata(dev);
    int retval;
    retval = usb_autopm_get_interface(synusb.intf);
    if (retval) {
    dev_err(&synusb.intf.dev,
    "%s - usb_autopm_get_interface failed, error: %d\n",
    __func__, retval);
    return retval;
    }
    mutex_lock(&synusb.pm_mutex);
    retval = usb_submit_urb(synusb.urb, GFP_KERNEL);
    if (retval) {
    dev_err(&synusb.intf.dev,
    "%s - usb_submit_urb failed, error: %d\n",
    __func__, retval);
    retval = -EIO;
    goto out;
    }
    synusb.intf.needs_remote_wakeup = 1;
    synusb.is_open = true;
    out:
    mutex_unlock(&synusb.pm_mutex);
    usb_autopm_put_interface(synusb.intf);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn synusb_close(dev: *mut input_dev) {
    static void synusb_close(struct input_dev *dev)
    {
    struct synusb *synusb = input_get_drvdata(dev);
    int autopm_error;
    autopm_error = usb_autopm_get_interface(synusb.intf);
    mutex_lock(&synusb.pm_mutex);
    usb_kill_urb(synusb.urb);
    synusb.intf.needs_remote_wakeup = 0;
    synusb.is_open = false;
    mutex_unlock(&synusb.pm_mutex);
    if (!autopm_error)
    usb_autopm_put_interface(synusb.intf);
    }
    static int synusb_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    struct usb_endpoint_descriptor *ep;
    struct synusb *synusb;
    struct input_dev *input_dev;
    let mut intf_num: c_uint = intf.cur_altsetting.desc.bInterfaceNumber;
    let mut altsetting: c_uint = min(intf.num_altsetting, 1U);
    int error;
    error = usb_set_interface(udev, intf_num, altsetting);
    if (error) {
    dev_err(&udev.dev,
    "Can not set alternate setting to %i, error: %i",
    altsetting, error);
    return error;
    }
    error = usb_find_int_in_endpoint(intf.cur_altsetting, &ep);
    if (error)
    return -ENODEV;
    synusb = kzalloc_obj(*synusb);
    input_dev = input_allocate_device();
    if (!synusb || !input_dev) {
    error = -ENOMEM;
    goto err_free_mem;
    }
    synusb.udev = udev;
    synusb.intf = intf;
    synusb.input = input_dev;
    mutex_init(&synusb.pm_mutex);
    synusb.flags = id.driver_info;
    if (synusb.flags & SYNUSB_COMBO) {
//
// This is a combo device, we need to set proper
// capability, depending on the interface.
//
    synusb.flags |= intf_num == 1 ?
    SYNUSB_STICK : SYNUSB_TOUCHPAD;
    }
    synusb.urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!synusb.urb) {
    error = -ENOMEM;
    goto err_free_mem;
    }
    synusb.data = usb_alloc_coherent(udev, SYNUSB_RECV_SIZE, GFP_KERNEL,
    &synusb.urb.transfer_dma);
    if (!synusb.data) {
    error = -ENOMEM;
    goto err_free_urb;
    }
    usb_fill_int_urb(synusb.urb, udev,
    usb_rcvintpipe(udev, ep.bEndpointAddress),
    synusb.data, SYNUSB_RECV_SIZE,
    synusb_irq, synusb,
    ep.bInterval);
    synusb.urb.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    if (udev.manufacturer)
    strscpy(synusb.name, udev.manufacturer,
    sizeof(synusb.name));
    if (udev.product) {
    if (udev.manufacturer)
    strlcat(synusb.name, " ", sizeof(synusb.name));
    strlcat(synusb.name, udev.product, sizeof(synusb.name));
    }
    if (!strlen(synusb.name))
    snprintf(synusb.name, sizeof(synusb.name),
    "USB Synaptics Device %04x:%04x",
    le16_to_cpu(udev.descriptor.idVendor),
    le16_to_cpu(udev.descriptor.idProduct));
    if (synusb.flags & SYNUSB_STICK)
    strlcat(synusb.name, " (Stick)", sizeof(synusb.name));
    usb_make_path(udev, synusb.phys, sizeof(synusb.phys));
    strlcat(synusb.phys, "/input0", sizeof(synusb.phys));
    input_dev.name = synusb.name;
    input_dev.phys = synusb.phys;
    usb_to_input_id(udev, &input_dev.id);
    input_dev.dev.parent = &synusb.intf.dev;
    if (!(synusb.flags & SYNUSB_IO_ALWAYS)) {
    input_dev.open = synusb_open;
    input_dev.close = synusb_close;
    }
    input_set_drvdata(input_dev, synusb);
    __set_bit(EV_ABS, input_dev.evbit);
    __set_bit(EV_KEY, input_dev.evbit);
    if (synusb.flags & SYNUSB_STICK) {
    __set_bit(EV_REL, input_dev.evbit);
    __set_bit(REL_X, input_dev.relbit);
    __set_bit(REL_Y, input_dev.relbit);
    __set_bit(INPUT_PROP_POINTING_STICK, input_dev.propbit);
    input_set_abs_params(input_dev, ABS_PRESSURE, 0, 127, 0, 0);
    } else {
    input_set_abs_params(input_dev, ABS_X,
    XMIN_NOMINAL, XMAX_NOMINAL, 0, 0);
    input_set_abs_params(input_dev, ABS_Y,
    YMIN_NOMINAL, YMAX_NOMINAL, 0, 0);
    input_set_abs_params(input_dev, ABS_PRESSURE, 0, 255, 0, 0);
    input_set_abs_params(input_dev, ABS_TOOL_WIDTH, 0, 15, 0, 0);
    __set_bit(BTN_TOUCH, input_dev.keybit);
    __set_bit(BTN_TOOL_FINGER, input_dev.keybit);
    __set_bit(BTN_TOOL_DOUBLETAP, input_dev.keybit);
    __set_bit(BTN_TOOL_TRIPLETAP, input_dev.keybit);
    }
    if (synusb.flags & SYNUSB_TOUCHSCREEN)
    __set_bit(INPUT_PROP_DIRECT, input_dev.propbit);
    else
    __set_bit(INPUT_PROP_POINTER, input_dev.propbit);
    __set_bit(BTN_LEFT, input_dev.keybit);
    __set_bit(BTN_RIGHT, input_dev.keybit);
    __set_bit(BTN_MIDDLE, input_dev.keybit);
    usb_set_intfdata(intf, synusb);
    if (synusb.flags & SYNUSB_IO_ALWAYS) {
    error = synusb_open(input_dev);
    if (error)
    goto err_free_dma;
    }
    error = input_register_device(input_dev);
    if (error) {
    dev_err(&udev.dev,
    "Failed to register input device, error %d\n",
    error);
    goto err_stop_io;
    }
    return 0;
    err_stop_io:
    if (synusb.flags & SYNUSB_IO_ALWAYS)
    synusb_close(synusb.input);
    err_free_dma:
    usb_free_coherent(udev, SYNUSB_RECV_SIZE, synusb.data,
    synusb.urb.transfer_dma);
    err_free_urb:
    usb_free_urb(synusb.urb);
    err_free_mem:
    input_free_device(input_dev);
    kfree(synusb);
    usb_set_intfdata(intf, core::ptr::null_mut());
    return error;
    }
#[no_mangle]
unsafe extern "C" fn synusb_disconnect(intf: *mut usb_interface) {
    static void synusb_disconnect(struct usb_interface *intf)
    {
    struct synusb *synusb = usb_get_intfdata(intf);
    struct usb_device *udev = interface_to_usbdev(intf);
    if (synusb.flags & SYNUSB_IO_ALWAYS)
    synusb_close(synusb.input);
    input_unregister_device(synusb.input);
    usb_free_coherent(udev, SYNUSB_RECV_SIZE, synusb.data,
    synusb.urb.transfer_dma);
    usb_free_urb(synusb.urb);
    kfree(synusb);
    usb_set_intfdata(intf, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn synusb_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int {
    static int synusb_suspend(struct usb_interface *intf, pm_message_t message)
    {
    struct synusb *synusb = usb_get_intfdata(intf);
    mutex_lock(&synusb.pm_mutex);
    usb_kill_urb(synusb.urb);
    mutex_unlock(&synusb.pm_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synusb_resume(intf: *mut usb_interface) -> c_int {
    static int synusb_resume(struct usb_interface *intf)
    {
    struct synusb *synusb = usb_get_intfdata(intf);
    let mut retval: c_int = 0;
    mutex_lock(&synusb.pm_mutex);
    if ((synusb.is_open || (synusb.flags & SYNUSB_IO_ALWAYS)) &&
    usb_submit_urb(synusb.urb, GFP_NOIO) < 0) {
    retval = -EIO;
    }
    mutex_unlock(&synusb.pm_mutex);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn synusb_pre_reset(intf: *mut usb_interface) -> c_int {
    static int synusb_pre_reset(struct usb_interface *intf)
    {
    struct synusb *synusb = usb_get_intfdata(intf);
    mutex_lock(&synusb.pm_mutex);
    usb_kill_urb(synusb.urb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn synusb_post_reset(intf: *mut usb_interface) -> c_int {
    static int synusb_post_reset(struct usb_interface *intf)
    {
    struct synusb *synusb = usb_get_intfdata(intf);
    let mut retval: c_int = 0;
    if ((synusb.is_open || (synusb.flags & SYNUSB_IO_ALWAYS)) &&
    usb_submit_urb(synusb.urb, GFP_NOIO) < 0) {
    retval = -EIO;
    }
    mutex_unlock(&synusb.pm_mutex);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn synusb_reset_resume(intf: *mut usb_interface) -> c_int {
    static int synusb_reset_resume(struct usb_interface *intf)
    {
    return synusb_resume(intf);
    }
    static const struct usb_device_id synusb_idtable[] = {
    { USB_DEVICE_SYNAPTICS(TP, SYNUSB_TOUCHPAD) },
    { USB_DEVICE_SYNAPTICS(INT_TP, SYNUSB_TOUCHPAD) },
    { USB_DEVICE_SYNAPTICS(CPAD,
    SYNUSB_TOUCHPAD | SYNUSB_AUXDISPLAY | SYNUSB_IO_ALWAYS) },
    { USB_DEVICE_SYNAPTICS(TS, SYNUSB_TOUCHSCREEN) },
    { USB_DEVICE_SYNAPTICS(STICK, SYNUSB_STICK) },
    { USB_DEVICE_SYNAPTICS(WP, SYNUSB_TOUCHPAD) },
    { USB_DEVICE_SYNAPTICS(COMP_TP, SYNUSB_COMBO) },
    { USB_DEVICE_SYNAPTICS(WTP, SYNUSB_TOUCHPAD) },
    { USB_DEVICE_SYNAPTICS(DPAD, SYNUSB_TOUCHPAD) },
    { }
    };
    MODULE_DEVICE_TABLE(usb, synusb_idtable);
    static struct usb_driver synusb_driver = {
    .name		= "synaptics_usb",
    .probe		= synusb_probe,
    .disconnect	= synusb_disconnect,
    .id_table	= synusb_idtable,
    .suspend	= synusb_suspend,
    .resume		= synusb_resume,
    .pre_reset	= synusb_pre_reset,
    .post_reset	= synusb_post_reset,
    .reset_resume	= synusb_reset_resume,
    .supports_autosuspend = 1,
    };
    module_usb_driver(synusb_driver);
    MODULE_AUTHOR("Rob Miller <rob@inpharmatica.co.uk>, "
    "Ron Lee <ron@debian.org>, "
    "Jan Steinhoff <cpad@jan-steinhoff.de>");
    MODULE_DESCRIPTION("Synaptics USB device driver");
    MODULE_LICENSE("GPL");
