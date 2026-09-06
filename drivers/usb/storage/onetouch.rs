//! Automatically rewritten from C to Rust
//! Source: drivers/usb/storage/onetouch.c
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
// Support for the Maxtor OneTouch USB hard drive's button
//
// Current development and maintenance by:
// Copyright (c) 2005 Nick Sillik <n.sillik@temple.edu>
//
// Initial work by:
// Copyright (c) 2003 Erik Thyren <erth7411@student.uu.se>
//
// Based on usbmouse.c (Vojtech Pavlik) and xpad.c (Marko Friedemann)
//

    MODULE_DESCRIPTION("Maxtor USB OneTouch hard drive button driver");
    MODULE_AUTHOR("Nick Sillik <n.sillik@temple.edu>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("USB_STORAGE");
pub const ONETOUCH_PKT_LEN: c_uint = 0x02;

    static int onetouch_connect_input(struct us_data *ss);
    static void onetouch_release_input(void *onetouch_);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_onetouch {
    pub name: [c_char; 128],
    pub phys: [c_char; 64],
    pub /: *mut *mut *mut input_dev dev; / input device interface,
    pub /: *mut *mut *mut usb_device udev; / usb device,
    pub /: *mut *mut *mut urb irq; / urb for interrupt in report,
    pub /: *mut *mut *mut unsigned char data; / input data,
    pub data_dma: dma_addr_t,
    pub is_open:1: c_uint,
}

//
// The table of devices
//

    vendorName, productName, useProtocol, useTransport, \
    initFunction, flags) \
    { USB_DEVICE_VER(id_vendor, id_product, bcdDeviceMin, bcdDeviceMax), \
    .driver_info = (flags) }
    static const struct usb_device_id onetouch_usb_ids[] = {

    { }		/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, onetouch_usb_ids);

//
// The flags table
//

    vendor_name, product_name, use_protocol, use_transport, \
    init_function, Flags) \
    { \
    .vendorName = vendor_name,	\
    .productName = product_name,	\
    .useProtocol = use_protocol,	\
    .useTransport = use_transport,	\
    .initFunction = init_function,	\
    }
    static const struct us_unusual_dev onetouch_unusual_dev_list[] = {

    { }		/* Terminating entry */
    };

#[no_mangle]
unsafe extern "C" fn usb_onetouch_irq(urb: *mut urb) {
    static void usb_onetouch_irq(struct urb *urb)
    {
    struct usb_onetouch *onetouch = urb.context;
    signed char *data = onetouch.data;
    struct input_dev *dev = onetouch.dev;
    let mut status: c_int = urb.status;
    int retval;
    switch (status) {
    case 0:			/* success */
    break;
    case -ECONNRESET:	/* unlink */
    case -ENOENT:
    case -ESHUTDOWN:
    return;
// -EPIPE:  should clear the halt
    default:		/* error */
    goto resubmit;
    }
    input_report_key(dev, ONETOUCH_BUTTON, data[0] & 0x02);
    input_sync(dev);
    resubmit:
    retval = usb_submit_urb (urb, GFP_ATOMIC);
    if (retval)
    dev_err(&dev.dev, "can't resubmit intr, %s-%s/input0, "
    "retval %d\n", onetouch.udev.bus.bus_name,
    onetouch.udev.devpath, retval);
    }
#[no_mangle]
unsafe extern "C" fn usb_onetouch_open(dev: *mut input_dev) -> c_int {
    static int usb_onetouch_open(struct input_dev *dev)
    {
    struct usb_onetouch *onetouch = input_get_drvdata(dev);
    onetouch.is_open = 1;
    onetouch.irq.dev = onetouch.udev;
    if (usb_submit_urb(onetouch.irq, GFP_KERNEL)) {
    dev_err(&dev.dev, "usb_submit_urb failed\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_onetouch_close(dev: *mut input_dev) {
    static void usb_onetouch_close(struct input_dev *dev)
    {
    struct usb_onetouch *onetouch = input_get_drvdata(dev);
    usb_kill_urb(onetouch.irq);
    onetouch.is_open = 0;
    }

#[no_mangle]
unsafe extern "C" fn usb_onetouch_pm_hook(us: *mut us_data, action: c_int) {
    static void usb_onetouch_pm_hook(struct us_data *us, int action)
    {
    struct usb_onetouch *onetouch = (struct usb_onetouch *) us.extra;
    if (onetouch.is_open) {
    switch (action) {
    case US_SUSPEND:
    usb_kill_urb(onetouch.irq);
    break;
    case US_RESUME:
    if (usb_submit_urb(onetouch.irq, GFP_NOIO) != 0)
    dev_err(&onetouch.irq.dev.dev,
    "usb_submit_urb failed\n");
    break;
    default:
    break;
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn onetouch_connect_input(ss: *mut us_data) -> c_int {
    static int onetouch_connect_input(struct us_data *ss)
    {
    struct usb_device *udev = ss.pusb_dev;
    struct usb_host_interface *interface;
    struct usb_endpoint_descriptor *endpoint;
    struct usb_onetouch *onetouch;
    struct input_dev *input_dev;
    int pipe, maxp;
    let mut error: c_int = -ENOMEM;
    interface = ss.pusb_intf.cur_altsetting;
    if (interface.desc.bNumEndpoints != 3)
    return -ENODEV;
    endpoint = &interface.endpoint[2].desc;
    if (!usb_endpoint_is_int_in(endpoint))
    return -ENODEV;
    pipe = usb_rcvintpipe(udev, endpoint.bEndpointAddress);
    maxp = usb_maxpacket(udev, pipe);
    maxp = min(maxp, ONETOUCH_PKT_LEN);
    onetouch = kzalloc_obj(struct usb_onetouch);
    input_dev = input_allocate_device();
    if (!onetouch || !input_dev)
    goto fail1;
    onetouch.data = usb_alloc_coherent(udev, ONETOUCH_PKT_LEN,
    GFP_KERNEL, &onetouch.data_dma);
    if (!onetouch.data)
    goto fail1;
    onetouch.irq = usb_alloc_urb(0, GFP_KERNEL);
    if (!onetouch.irq)
    goto fail2;
    onetouch.udev = udev;
    onetouch.dev = input_dev;
    if (udev.manufacturer)
    strscpy(onetouch.name, udev.manufacturer,
    sizeof(onetouch.name));
    if (udev.product) {
    if (udev.manufacturer)
    strlcat(onetouch.name, " ", sizeof(onetouch.name));
    strlcat(onetouch.name, udev.product, sizeof(onetouch.name));
    }
    if (!strlen(onetouch.name))
    snprintf(onetouch.name, sizeof(onetouch.name),
    "Maxtor Onetouch %04x:%04x",
    le16_to_cpu(udev.descriptor.idVendor),
    le16_to_cpu(udev.descriptor.idProduct));
    usb_make_path(udev, onetouch.phys, sizeof(onetouch.phys));
    strlcat(onetouch.phys, "/input0", sizeof(onetouch.phys));
    input_dev.name = onetouch.name;
    input_dev.phys = onetouch.phys;
    usb_to_input_id(udev, &input_dev.id);
    input_dev.dev.parent = &udev.dev;
    set_bit(EV_KEY, input_dev.evbit);
    set_bit(ONETOUCH_BUTTON, input_dev.keybit);
    clear_bit(0, input_dev.keybit);
    input_set_drvdata(input_dev, onetouch);
    input_dev.open = usb_onetouch_open;
    input_dev.close = usb_onetouch_close;
    usb_fill_int_urb(onetouch.irq, udev, pipe, onetouch.data, maxp,
    usb_onetouch_irq, onetouch, endpoint.bInterval);
    onetouch.irq.transfer_dma = onetouch.data_dma;
    onetouch.irq.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    ss.extra_destructor = onetouch_release_input;
    ss.extra = onetouch;

    ss.suspend_resume_hook = usb_onetouch_pm_hook;

    error = input_register_device(onetouch.dev);
    if (error)
    goto fail3;
    return 0;
    fail3:	usb_free_urb(onetouch.irq);
    fail2:	usb_free_coherent(udev, ONETOUCH_PKT_LEN,
    onetouch.data, onetouch.data_dma);
    fail1:	kfree(onetouch);
    input_free_device(input_dev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn onetouch_release_input(onetouch_: *mut c_void) {
    static void onetouch_release_input(void *onetouch_)
    {
    struct usb_onetouch *onetouch = (struct usb_onetouch *) onetouch_;
    if (onetouch) {
    usb_kill_urb(onetouch.irq);
    input_unregister_device(onetouch.dev);
    usb_free_urb(onetouch.irq);
    usb_free_coherent(onetouch.udev, ONETOUCH_PKT_LEN,
    onetouch.data, onetouch.data_dma);
    }
    }
    static struct scsi_host_template onetouch_host_template;
    static int onetouch_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct us_data *us;
    int result;
    result = usb_stor_probe1(&us, intf, id,
    (id - onetouch_usb_ids) + onetouch_unusual_dev_list,
    &onetouch_host_template);
    if (result)
    return result;
// Use default transport and protocol
    result = usb_stor_probe2(us);
    return result;
    }
    static struct usb_driver onetouch_driver = {
    .name =		DRV_NAME,
    .probe =	onetouch_probe,
    .disconnect =	usb_stor_disconnect,
    .suspend =	usb_stor_suspend,
    .resume =	usb_stor_resume,
    .reset_resume =	usb_stor_reset_resume,
    .pre_reset =	usb_stor_pre_reset,
    .post_reset =	usb_stor_post_reset,
    .id_table =	onetouch_usb_ids,
    .soft_unbind =	1,
    .no_dynamic_id = 1,
    };
    module_usb_stor_driver(onetouch_driver, onetouch_host_template, DRV_NAME);
