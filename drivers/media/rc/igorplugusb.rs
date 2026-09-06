//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/igorplugusb.c
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
// IgorPlug-USB IR Receiver
//
// Copyright (C) 2014 Sean Young <sean@mess.org>
//
// Supports the standard homebrew IgorPlugUSB receiver with Igor's firmware.
// See http://www.cesko.host.sk/IgorPlugUSB/IgorPlug-USB%20(AVR)_eng.htm
//
// Based on the lirc_igorplugusb.c driver:
// Copyright (C) 2004 Jan M. Hochstein
// <hochstein@algo.informatik.tu-darmstadt.de>
//

pub const HEADERLEN: c_int = 3;
pub const BUFLEN: c_int = 36;

pub const SET_INFRABUFFER_EMPTY: c_int = 1;
pub const GET_INFRACODE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igorplugusb {
    pub rc: *mut rc_dev,
    pub dev: *mut device,
    pub urb: *mut urb,
    pub request: *mut usb_ctrlrequest,
    pub timer: timer_list,
    pub buf_in: *mut u8,
    pub phys: [c_char; 64],
}

    static void igorplugusb_cmd(struct igorplugusb *ir, int cmd);
#[no_mangle]
unsafe extern "C" fn igorplugusb_irdata(ir: *mut igorplugusb, len: unsigned) {
    static void igorplugusb_irdata(struct igorplugusb *ir, unsigned len)
    {
    let mut rawir: ir_raw_event = {};
    unsigned i, start, overflow;
    dev_dbg(ir.dev, "irdata: %*ph (len=%u)", len, ir.buf_in, len);
//
// If more than 36 pulses and spaces follow each other, the igorplugusb
// overwrites its buffer from the beginning. The overflow value is the
// last offset which was not overwritten. Everything from this offset
// onwards occurred before everything until this offset.
//
    overflow = ir.buf_in[2];
    i = start = overflow + HEADERLEN;
    if (start >= len) {
    dev_err(ir.dev, "receive overflow invalid: %u", overflow);
    } else {
    if (overflow > 0) {
    dev_warn(ir.dev, "receive overflow, at least %u lost",
    overflow);
    ir_raw_event_overflow(ir.rc);
    }
    do {
    rawir.duration = ir.buf_in[i] * 85;
    rawir.pulse = i & 1;
    ir_raw_event_store_with_filter(ir.rc, &rawir);
    if (++i == len)
    i = HEADERLEN;
    } while (i != start);
// add a trailing space
    rawir.duration = ir.rc.timeout;
    rawir.pulse = false;
    ir_raw_event_store_with_filter(ir.rc, &rawir);
    ir_raw_event_handle(ir.rc);
    }
    igorplugusb_cmd(ir, SET_INFRABUFFER_EMPTY);
    }
#[no_mangle]
unsafe extern "C" fn igorplugusb_callback(urb: *mut urb) {
    static void igorplugusb_callback(struct urb *urb)
    {
    struct usb_ctrlrequest *req;
    struct igorplugusb *ir = urb.context;
    req = (struct usb_ctrlrequest *)urb.setup_packet;
    switch (urb.status) {
    case 0:
    if (req.bRequest == GET_INFRACODE &&
    urb.actual_length > HEADERLEN)
    igorplugusb_irdata(ir, urb.actual_length);
    else /* request IR */
    mod_timer(&ir.timer, jiffies + msecs_to_jiffies(50));
    break;
    case -EPROTO:
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
    return;
    default:
    dev_warn(ir.dev, "Error: urb status = %d\n", urb.status);
    igorplugusb_cmd(ir, SET_INFRABUFFER_EMPTY);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn igorplugusb_cmd(ir: *mut igorplugusb, cmd: c_int) {
    static void igorplugusb_cmd(struct igorplugusb *ir, int cmd)
    {
    int ret;
    ir.request.bRequest = cmd;
    ir.urb.transfer_flags = 0;
    ret = usb_submit_urb(ir.urb, GFP_ATOMIC);
    if (ret && ret != -EPERM)
    dev_err(ir.dev, "submit urb failed: %d", ret);
    }
#[no_mangle]
unsafe extern "C" fn igorplugusb_timer(t: *mut timer_list) {
    static void igorplugusb_timer(struct timer_list *t)
    {
    struct igorplugusb *ir = timer_container_of(ir, t, timer);
    igorplugusb_cmd(ir, GET_INFRACODE);
    }
    static int igorplugusb_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *udev;
    struct usb_host_interface *idesc;
    struct usb_endpoint_descriptor *ep;
    struct igorplugusb *ir;
    struct rc_dev *rc;
    let mut ret: c_int = -ENOMEM;
    udev = interface_to_usbdev(intf);
    idesc = intf.cur_altsetting;
    if (idesc.desc.bNumEndpoints != 1) {
    dev_err(&intf.dev, "incorrect number of endpoints");
    return -ENODEV;
    }
    ep = &idesc.endpoint[0].desc;
    if (!usb_endpoint_dir_in(ep) || !usb_endpoint_xfer_control(ep)) {
    dev_err(&intf.dev, "endpoint incorrect");
    return -ENODEV;
    }
    ir = devm_kzalloc(&intf.dev, sizeof(*ir), GFP_KERNEL);
    if (!ir)
    return -ENOMEM;
    ir.request = kzalloc_obj(*ir.request);
    if (!ir.request)
    goto fail;
    ir.dev = &intf.dev;
    timer_setup(&ir.timer, igorplugusb_timer, 0);
    ir.request.bRequest = GET_INFRACODE;
    ir.request.bRequestType = USB_TYPE_VENDOR | USB_DIR_IN;
    ir.request.wLength = cpu_to_le16(MAX_PACKET);
    ir.urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!ir.urb)
    goto fail;
    ir.buf_in = kmalloc(MAX_PACKET, GFP_KERNEL);
    if (!ir.buf_in)
    goto fail;
    usb_fill_control_urb(ir.urb, udev,
    usb_rcvctrlpipe(udev, 0), (uint8_t *)ir.request,
    ir.buf_in, MAX_PACKET, igorplugusb_callback, ir);
    usb_make_path(udev, ir.phys, sizeof(ir.phys));
    rc = rc_allocate_device(RC_DRIVER_IR_RAW);
    if (!rc)
    goto fail;
    rc.device_name = DRIVER_DESC;
    rc.input_phys = ir.phys;
    usb_to_input_id(udev, &rc.input_id);
    rc.dev.parent = &intf.dev;
//
// This device can only store 36 pulses + spaces, which is not enough
// for the NEC protocol and many others.
//
    rc.allowed_protocols = RC_PROTO_BIT_ALL_IR_DECODER &
    ~(RC_PROTO_BIT_NEC | RC_PROTO_BIT_NECX | RC_PROTO_BIT_NEC32 |
    RC_PROTO_BIT_RC6_6A_20 | RC_PROTO_BIT_RC6_6A_24 |
    RC_PROTO_BIT_RC6_6A_32 | RC_PROTO_BIT_RC6_MCE |
    RC_PROTO_BIT_SONY20 | RC_PROTO_BIT_SANYO);
    rc.priv = ir;
    rc.driver_name = DRIVER_NAME;
    rc.map_name = RC_MAP_HAUPPAUGE;
    rc.timeout = MS_TO_US(100);
    rc.rx_resolution = 85;
    ir.rc = rc;
    ret = rc_register_device(rc);
    if (ret) {
    dev_err(&intf.dev, "failed to register rc device: %d", ret);
    goto fail;
    }
    usb_set_intfdata(intf, ir);
    igorplugusb_cmd(ir, SET_INFRABUFFER_EMPTY);
    return 0;
    fail:
    usb_poison_urb(ir.urb);
    timer_delete(&ir.timer);
    usb_unpoison_urb(ir.urb);
    usb_free_urb(ir.urb);
    rc_free_device(ir.rc);
    kfree(ir.buf_in);
    kfree(ir.request);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn igorplugusb_disconnect(intf: *mut usb_interface) {
    static void igorplugusb_disconnect(struct usb_interface *intf)
    {
    struct igorplugusb *ir = usb_get_intfdata(intf);
    rc_unregister_device(ir.rc);
    usb_poison_urb(ir.urb);
    timer_delete_sync(&ir.timer);
    usb_set_intfdata(intf, core::ptr::null_mut());
    usb_unpoison_urb(ir.urb);
    usb_free_urb(ir.urb);
    rc_free_device(ir.rc);
    kfree(ir.buf_in);
    kfree(ir.request);
    }
    static const struct usb_device_id igorplugusb_table[] = {
// Igor Plug USB (Atmel's Manufact. ID)
    { USB_DEVICE(0x03eb, 0x0002) },
// Fit PC2 Infrared Adapter
    { USB_DEVICE(0x03eb, 0x21fe) },
// Terminating entry
    { }
    };
    static struct usb_driver igorplugusb_driver = {
    .name =	DRIVER_NAME,
    .probe = igorplugusb_probe,
    .disconnect = igorplugusb_disconnect,
    .id_table = igorplugusb_table
    };
    module_usb_driver(igorplugusb_driver);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_AUTHOR("Sean Young <sean@mess.org>");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(usb, igorplugusb_table);
