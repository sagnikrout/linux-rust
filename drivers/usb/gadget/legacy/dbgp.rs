//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/legacy/dbgp.c
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
// dbgp.c -- EHCI Debug Port device gadget
//
// Copyright (C) 2010 Stephane Duverger
//
// Released under the GPLv2.
//
// verbose messages

pub const DRIVER_VENDOR_ID: c_uint = 0x0525 /* NetChip */;
pub const DRIVER_PRODUCT_ID: c_uint = 0xc0de /* undefined */;
pub const USB_DEBUG_MAX_PACKET_SIZE: c_int = 8;
pub const DBGP_REQ_EP0_LEN: c_int = 128;
pub const DBGP_REQ_LEN: c_int = 512;
    static struct dbgp {
    struct usb_gadget  *gadget;
    struct usb_request *req;
    struct usb_ep      *i_ep;
    struct usb_ep      *o_ep;

    struct gserial     *serial;

    } dbgp;
    static struct usb_device_descriptor device_desc = {
    .bLength = sizeof device_desc,
    .bDescriptorType = USB_DT_DEVICE,
    .bcdUSB = cpu_to_le16(0x0200),
    .bDeviceClass =	USB_CLASS_VENDOR_SPEC,
    .idVendor = cpu_to_le16(DRIVER_VENDOR_ID),
    .idProduct = cpu_to_le16(DRIVER_PRODUCT_ID),
    .bNumConfigurations = 1,
    };
    static struct usb_debug_descriptor dbg_desc = {
    .bLength = sizeof dbg_desc,
    .bDescriptorType = USB_DT_DEBUG,
    };
    static struct usb_endpoint_descriptor i_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bmAttributes = USB_ENDPOINT_XFER_BULK,
    .bEndpointAddress = USB_DIR_IN,
    };
    static struct usb_endpoint_descriptor o_desc = {
    .bLength = USB_DT_ENDPOINT_SIZE,
    .bDescriptorType = USB_DT_ENDPOINT,
    .bmAttributes = USB_ENDPOINT_XFER_BULK,
    .bEndpointAddress = USB_DIR_OUT,
    };

#[no_mangle]
unsafe extern "C" fn dbgp_consume(buf: *mut c_char, len: unsigned) -> c_int {
    static int dbgp_consume(char *buf, unsigned len)
    {
    char c;
    if (!len)
    return 0;
    c = buf[len-1];
    if (c != 0)
    buf[len-1] = 0;
    printk(KERN_NOTICE "%s%c", buf, c);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __disable_ep(ep: *mut usb_ep) {
    static void __disable_ep(struct usb_ep *ep)
    {
    usb_ep_disable(ep);
    }
#[no_mangle]
unsafe extern "C" fn dbgp_disable_ep() {
    static void dbgp_disable_ep(void)
    {
    __disable_ep(dbgp.i_ep);
    __disable_ep(dbgp.o_ep);
    }
#[no_mangle]
unsafe extern "C" fn dbgp_complete(ep: *mut usb_ep, req: *mut usb_request) {
    static void dbgp_complete(struct usb_ep *ep, struct usb_request *req)
    {
    int stp;
    let mut err: c_int = 0;
    let mut status: c_int = req.status;
    if (ep == dbgp.i_ep) {
    stp = 1;
    goto fail;
    }
    if (status != 0) {
    stp = 2;
    goto release_req;
    }
    dbgp_consume(req.buf, req.actual);
    req.length = DBGP_REQ_LEN;
    err = usb_ep_queue(ep, req, GFP_ATOMIC);
    if (err < 0) {
    stp = 3;
    goto release_req;
    }
    return;
    release_req:
    kfree(req.buf);
    usb_ep_free_request(dbgp.o_ep, req);
    dbgp_disable_ep();
    fail:
    dev_dbg(&dbgp.gadget.dev,
    "complete: failure (%d:%d) ==> %d\n", stp, err, status);
    }
#[no_mangle]
unsafe extern "C" fn dbgp_enable_ep_req(ep: *mut usb_ep) -> c_int {
    static int dbgp_enable_ep_req(struct usb_ep *ep)
    {
    int err, stp;
    struct usb_request *req;
    req = usb_ep_alloc_request(ep, GFP_KERNEL);
    if (!req) {
    err = -ENOMEM;
    stp = 1;
    goto fail_1;
    }
    req.buf = kzalloc(DBGP_REQ_LEN, GFP_KERNEL);
    if (!req.buf) {
    err = -ENOMEM;
    stp = 2;
    goto fail_2;
    }
    req.complete = dbgp_complete;
    req.length = DBGP_REQ_LEN;
    err = usb_ep_queue(ep, req, GFP_ATOMIC);
    if (err < 0) {
    stp = 3;
    goto fail_3;
    }
    return 0;
    fail_3:
    kfree(req.buf);
    fail_2:
    usb_ep_free_request(dbgp.o_ep, req);
    fail_1:
    dev_dbg(&dbgp.gadget.dev,
    "enable ep req: failure (%d:%d)\n", stp, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn __enable_ep(ep: *mut usb_ep, desc: *mut usb_endpoint_descriptor) -> c_int {
    static int __enable_ep(struct usb_ep *ep, struct usb_endpoint_descriptor *desc)
    {
    int err;
    ep.desc = desc;
    err = usb_ep_enable(ep);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dbgp_enable_ep() -> c_int {
    static int dbgp_enable_ep(void)
    {
    int err, stp;
    err = __enable_ep(dbgp.i_ep, &i_desc);
    if (err < 0) {
    stp = 1;
    goto fail_1;
    }
    err = __enable_ep(dbgp.o_ep, &o_desc);
    if (err < 0) {
    stp = 2;
    goto fail_2;
    }
    err = dbgp_enable_ep_req(dbgp.o_ep);
    if (err < 0) {
    stp = 3;
    goto fail_3;
    }
    return 0;
    fail_3:
    __disable_ep(dbgp.o_ep);
    fail_2:
    __disable_ep(dbgp.i_ep);
    fail_1:
    dev_dbg(&dbgp.gadget.dev, "enable ep: failure (%d:%d)\n", stp, err);
    return err;
    }

#[no_mangle]
unsafe extern "C" fn dbgp_disconnect(gadget: *mut usb_gadget) {
    static void dbgp_disconnect(struct usb_gadget *gadget)
    {

    dbgp_disable_ep();

    gserial_disconnect(dbgp.serial);

    }
#[no_mangle]
unsafe extern "C" fn dbgp_unbind(gadget: *mut usb_gadget) {
    static void dbgp_unbind(struct usb_gadget *gadget)
    {

    kfree(dbgp.serial);
    dbgp.serial = core::ptr::null_mut();

    if (dbgp.req) {
    kfree(dbgp.req.buf);
    usb_ep_free_request(gadget.ep0, dbgp.req);
    dbgp.req = core::ptr::null_mut();
    }
    }

    static unsigned char tty_line;

#[no_mangle]
unsafe extern "C" fn dbgp_configure_endpoints(gadget: *mut usb_gadget) -> c_int {
    static int dbgp_configure_endpoints(struct usb_gadget *gadget)
    {
    int stp;
    usb_ep_autoconfig_reset(gadget);
    dbgp.i_ep = usb_ep_autoconfig(gadget, &i_desc);
    if (!dbgp.i_ep) {
    stp = 1;
    goto fail_1;
    }
    i_desc.wMaxPacketSize =
    cpu_to_le16(USB_DEBUG_MAX_PACKET_SIZE);
    dbgp.o_ep = usb_ep_autoconfig(gadget, &o_desc);
    if (!dbgp.o_ep) {
    stp = 2;
    goto fail_1;
    }
    o_desc.wMaxPacketSize =
    cpu_to_le16(USB_DEBUG_MAX_PACKET_SIZE);
    dbg_desc.bDebugInEndpoint = i_desc.bEndpointAddress;
    dbg_desc.bDebugOutEndpoint = o_desc.bEndpointAddress;

    dbgp.serial.in = dbgp.i_ep;
    dbgp.serial.out = dbgp.o_ep;
    dbgp.serial.in.desc = &i_desc;
    dbgp.serial.out.desc = &o_desc;

    return 0;
    fail_1:
    dev_dbg(&dbgp.gadget.dev, "ep config: failure (%d)\n", stp);
    return -ENODEV;
    }
    static int dbgp_bind(struct usb_gadget *gadget,
    struct usb_gadget_driver *driver)
    {
    int err, stp;
    dbgp.gadget = gadget;
    dbgp.req = usb_ep_alloc_request(gadget.ep0, GFP_KERNEL);
    if (!dbgp.req) {
    err = -ENOMEM;
    stp = 1;
    goto fail;
    }
    dbgp.req.buf = kmalloc(DBGP_REQ_EP0_LEN, GFP_KERNEL);
    if (!dbgp.req.buf) {
    err = -ENOMEM;
    stp = 2;
    goto fail;
    }
    dbgp.req.length = DBGP_REQ_EP0_LEN;

    dbgp.serial = kzalloc_obj(struct gserial);
    if (!dbgp.serial) {
    stp = 3;
    err = -ENOMEM;
    goto fail;
    }
    if (gserial_alloc_line(&tty_line)) {
    stp = 4;
    err = -ENODEV;
    goto fail;
    }

    err = dbgp_configure_endpoints(gadget);
    if (err < 0) {
    stp = 5;
    goto fail;
    }
    dev_dbg(&dbgp.gadget.dev, "bind: success\n");
    return 0;
    fail:
    dev_dbg(&gadget.dev, "bind: failure (%d:%d)\n", stp, err);
    dbgp_unbind(gadget);
    return err;
    }
    static void dbgp_setup_complete(struct usb_ep *ep,
    struct usb_request *req)
    {
    dev_dbg(&dbgp.gadget.dev, "setup complete: %d, %d/%d\n",
    req.status, req.actual, req.length);
    }
    static int dbgp_setup(struct usb_gadget *gadget,
    const struct usb_ctrlrequest *ctrl)
    {
    struct usb_request *req = dbgp.req;
    let mut request: u8 = ctrl.bRequest;
    let mut value: u16 = le16_to_cpu(ctrl.wValue);
    let mut length: u16 = le16_to_cpu(ctrl.wLength);
    let mut err: c_int = -EOPNOTSUPP;
    void *data = core::ptr::null_mut();
    let mut len: u16 = 0;
    if (length > DBGP_REQ_LEN) {
    if (ctrl.bRequestType & USB_DIR_IN) {
// Cast away the const, we are going to overwrite on purpose.
    __le16 *temp = (__le16 *)&ctrl.wLength;
// temp = cpu_to_le16(DBGP_REQ_LEN);
    length = DBGP_REQ_LEN;
    } else {
    return err;
    }
    }
    if (request == USB_REQ_GET_DESCRIPTOR) {
    switch (value>>8) {
    case USB_DT_DEVICE:
    dev_dbg(&dbgp.gadget.dev, "setup: desc device\n");
    len = sizeof device_desc;
    data = &device_desc;
    device_desc.bMaxPacketSize0 = gadget.ep0.maxpacket;
    break;
    case USB_DT_DEBUG:
    dev_dbg(&dbgp.gadget.dev, "setup: desc debug\n");
    len = sizeof dbg_desc;
    data = &dbg_desc;
    break;
    default:
    goto fail;
    }
    err = 0;
    } else if (request == USB_REQ_SET_FEATURE &&
    value == USB_DEVICE_DEBUG_MODE) {
    dev_dbg(&dbgp.gadget.dev, "setup: feat debug\n");

    err = dbgp_enable_ep();

    err = dbgp_configure_endpoints(gadget);
    if (err < 0) {
    goto fail;
    }
    err = gserial_connect(dbgp.serial, tty_line);

    if (err < 0)
    goto fail;
    } else
    goto fail;
    req.length = min(length, len);
    req.zero = len < req.length;
    if (data && req.length)
    memcpy(req.buf, data, req.length);
    req.complete = dbgp_setup_complete;
    return usb_ep_queue(gadget.ep0, req, GFP_ATOMIC);
    fail:
    dev_dbg(&dbgp.gadget.dev,
    "setup: failure req %x v %x\n", request, value);
    return err;
    }
    static struct usb_gadget_driver dbgp_driver = {
    .function = "dbgp",
    .max_speed = USB_SPEED_HIGH,
    .bind = dbgp_bind,
    .unbind = dbgp_unbind,
    .setup = dbgp_setup,
    .reset = dbgp_disconnect,
    .disconnect = dbgp_disconnect,
    .driver	= {
    .owner = THIS_MODULE,
    .name = "dbgp"
    },
    };
#[no_mangle]
unsafe extern "C" fn dbgp_init() -> int __init {
    static int __init dbgp_init(void)
    {
    return usb_gadget_register_driver(&dbgp_driver);
    }
#[no_mangle]
unsafe extern "C" fn dbgp_exit() -> void __exit {
    static void __exit dbgp_exit(void)
    {
    usb_gadget_unregister_driver(&dbgp_driver);

    gserial_free_line(tty_line);

    }
    MODULE_AUTHOR("Stephane Duverger");
    MODULE_DESCRIPTION("EHCI Debug Port device gadget");
    MODULE_LICENSE("GPL");
    module_init(dbgp_init);
    module_exit(dbgp_exit);
