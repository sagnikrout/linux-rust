//! Automatically rewritten from C to Rust
//! Source: drivers/input/tablet/acecad.c
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
// Copyright (c) 2001-2005 Edouard TISSERANT   <edouard.tisserant@wanadoo.fr>
// Copyright (c) 2004-2005 Stephane VOLTZ      <svoltz@numericable.fr>
//
// USB Acecad "Acecad Flair" tablet support
//
// Changelog:
// v3.2 - Added sysfs support
//

    MODULE_AUTHOR("Edouard TISSERANT <edouard.tisserant@wanadoo.fr>");
    MODULE_DESCRIPTION("USB Acecad Flair tablet driver");
    MODULE_LICENSE("GPL");
pub const USB_VENDOR_ID_ACECAD: c_uint = 0x0460;
pub const USB_DEVICE_ID_FLAIR: c_uint = 0x0004;
pub const USB_DEVICE_ID_302: c_uint = 0x0008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_acecad {
    pub name: [c_char; 128],
    pub phys: [c_char; 64],
    pub intf: *mut usb_interface,
    pub input: *mut input_dev,
    pub irq: *mut urb,
    pub data: *mut c_uchar,
    pub data_dma: dma_addr_t,
}

#[no_mangle]
unsafe extern "C" fn usb_acecad_irq(urb: *mut urb) {
    static void usb_acecad_irq(struct urb *urb)
    {
    struct usb_acecad *acecad = urb.context;
    unsigned char *data = acecad.data;
    struct input_dev *dev = acecad.input;
    struct usb_interface *intf = acecad.intf;
    struct usb_device *udev = interface_to_usbdev(intf);
    int prox, status;
    switch (urb.status) {
    case 0:
// success
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
// this urb is terminated, clean up
    dev_dbg(&intf.dev, "%s - urb shutting down with status: %d\n",
    __func__, urb.status);
    return;
    default:
    dev_dbg(&intf.dev, "%s - nonzero urb status received: %d\n",
    __func__, urb.status);
    goto resubmit;
    }
    prox = (data[0] & 0x04) >> 2;
    input_report_key(dev, BTN_TOOL_PEN, prox);
    if (prox) {
    let mut x: c_int = data[1] | (data[2] << 8);
    let mut y: c_int = data[3] | (data[4] << 8);
// Pressure should compute the same way for flair and 302
    let mut pressure: c_int = data[5] | (data[6] << 8);
    let mut touch: c_int = data[0] & 0x01;
    let mut stylus: c_int = (data[0] & 0x10) >> 4;
    let mut stylus2: c_int = (data[0] & 0x20) >> 5;
    input_report_abs(dev, ABS_X, x);
    input_report_abs(dev, ABS_Y, y);
    input_report_abs(dev, ABS_PRESSURE, pressure);
    input_report_key(dev, BTN_TOUCH, touch);
    input_report_key(dev, BTN_STYLUS, stylus);
    input_report_key(dev, BTN_STYLUS2, stylus2);
    }
// event termination
    input_sync(dev);
    resubmit:
    status = usb_submit_urb(urb, GFP_ATOMIC);
    if (status)
    dev_err(&intf.dev,
    "can't resubmit intr, %s-%s/input0, status %d\n",
    udev.bus.bus_name,
    udev.devpath, status);
    }
#[no_mangle]
unsafe extern "C" fn usb_acecad_open(dev: *mut input_dev) -> c_int {
    static int usb_acecad_open(struct input_dev *dev)
    {
    struct usb_acecad *acecad = input_get_drvdata(dev);
    acecad.irq.dev = interface_to_usbdev(acecad.intf);
    if (usb_submit_urb(acecad.irq, GFP_KERNEL))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_acecad_close(dev: *mut input_dev) {
    static void usb_acecad_close(struct input_dev *dev)
    {
    struct usb_acecad *acecad = input_get_drvdata(dev);
    usb_kill_urb(acecad.irq);
    }
#[no_mangle]
unsafe extern "C" fn usb_acecad_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int usb_acecad_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    struct usb_device *dev = interface_to_usbdev(intf);
    struct usb_host_interface *interface = intf.cur_altsetting;
    struct usb_endpoint_descriptor *endpoint;
    struct usb_acecad *acecad;
    struct input_dev *input_dev;
    int pipe, maxp;
    int err;
    if (interface.desc.bNumEndpoints != 1)
    return -ENODEV;
    endpoint = &interface.endpoint[0].desc;
    if (!usb_endpoint_is_int_in(endpoint))
    return -ENODEV;
    pipe = usb_rcvintpipe(dev, endpoint.bEndpointAddress);
    maxp = usb_maxpacket(dev, pipe);
    acecad = kzalloc_obj(*acecad);
    input_dev = input_allocate_device();
    if (!acecad || !input_dev) {
    err = -ENOMEM;
    goto fail1;
    }
    acecad.data = usb_alloc_coherent(dev, 8, GFP_KERNEL, &acecad.data_dma);
    if (!acecad.data) {
    err= -ENOMEM;
    goto fail1;
    }
    acecad.irq = usb_alloc_urb(0, GFP_KERNEL);
    if (!acecad.irq) {
    err = -ENOMEM;
    goto fail2;
    }
    acecad.intf = intf;
    acecad.input = input_dev;
    if (dev.manufacturer)
    strscpy(acecad.name, dev.manufacturer, sizeof(acecad.name));
    if (dev.product) {
    if (dev.manufacturer)
    strlcat(acecad.name, " ", sizeof(acecad.name));
    strlcat(acecad.name, dev.product, sizeof(acecad.name));
    }
    usb_make_path(dev, acecad.phys, sizeof(acecad.phys));
    strlcat(acecad.phys, "/input0", sizeof(acecad.phys));
    input_dev.name = acecad.name;
    input_dev.phys = acecad.phys;
    usb_to_input_id(dev, &input_dev.id);
    input_dev.dev.parent = &intf.dev;
    input_set_drvdata(input_dev, acecad);
    input_dev.open = usb_acecad_open;
    input_dev.close = usb_acecad_close;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_DIGI)] = BIT_MASK(BTN_TOOL_PEN) |
    BIT_MASK(BTN_TOUCH) | BIT_MASK(BTN_STYLUS) |
    BIT_MASK(BTN_STYLUS2);
    switch (id.driver_info) {
    case 0:
    input_set_abs_params(input_dev, ABS_X, 0, 5000, 4, 0);
    input_set_abs_params(input_dev, ABS_Y, 0, 3750, 4, 0);
    input_set_abs_params(input_dev, ABS_PRESSURE, 0, 512, 0, 0);
    if (!strlen(acecad.name))
    snprintf(acecad.name, sizeof(acecad.name),
    "USB Acecad Flair Tablet %04x:%04x",
    le16_to_cpu(dev.descriptor.idVendor),
    le16_to_cpu(dev.descriptor.idProduct));
    break;
    case 1:
    input_set_abs_params(input_dev, ABS_X, 0, 53000, 4, 0);
    input_set_abs_params(input_dev, ABS_Y, 0, 2250, 4, 0);
    input_set_abs_params(input_dev, ABS_PRESSURE, 0, 1024, 0, 0);
    if (!strlen(acecad.name))
    snprintf(acecad.name, sizeof(acecad.name),
    "USB Acecad 302 Tablet %04x:%04x",
    le16_to_cpu(dev.descriptor.idVendor),
    le16_to_cpu(dev.descriptor.idProduct));
    break;
    }
    usb_fill_int_urb(acecad.irq, dev, pipe,
    acecad.data, maxp > 8 ? 8 : maxp,
    usb_acecad_irq, acecad, endpoint.bInterval);
    acecad.irq.transfer_dma = acecad.data_dma;
    acecad.irq.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    err = input_register_device(acecad.input);
    if (err)
    goto fail3;
    usb_set_intfdata(intf, acecad);
    return 0;
    fail3:	usb_free_urb(acecad.irq);
    fail2:	usb_free_coherent(dev, 8, acecad.data, acecad.data_dma);
    fail1: input_free_device(input_dev);
    kfree(acecad);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn usb_acecad_disconnect(intf: *mut usb_interface) {
    static void usb_acecad_disconnect(struct usb_interface *intf)
    {
    struct usb_acecad *acecad = usb_get_intfdata(intf);
    struct usb_device *udev = interface_to_usbdev(intf);
    usb_set_intfdata(intf, core::ptr::null_mut());
    input_unregister_device(acecad.input);
    usb_free_urb(acecad.irq);
    usb_free_coherent(udev, 8, acecad.data, acecad.data_dma);
    kfree(acecad);
    }
    static const struct usb_device_id usb_acecad_id_table[] = {
    { USB_DEVICE(USB_VENDOR_ID_ACECAD, USB_DEVICE_ID_FLAIR), .driver_info = 0 },
    { USB_DEVICE(USB_VENDOR_ID_ACECAD, USB_DEVICE_ID_302),	 .driver_info = 1 },
    { }
    };
    MODULE_DEVICE_TABLE(usb, usb_acecad_id_table);
    static struct usb_driver usb_acecad_driver = {
    .name =		"usb_acecad",
    .probe =	usb_acecad_probe,
    .disconnect =	usb_acecad_disconnect,
    .id_table =	usb_acecad_id_table,
    };
    module_usb_driver(usb_acecad_driver);
