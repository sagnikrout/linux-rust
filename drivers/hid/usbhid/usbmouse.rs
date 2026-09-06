//! Automatically rewritten from C to Rust
//! Source: drivers/hid/usbhid/usbmouse.c
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
// Copyright (c) 1999-2001 Vojtech Pavlik
//
// USB HIDBP Mouse support
//
// Should you need to contact me, the author, you can do so either by
// e-mail - mail your message to <vojtech@ucw.cz>, or by paper mail:
// Vojtech Pavlik, Simunkova 1594, Prague 8, 182 00 Czech Republic
//

// for apple IDs

//
// Version Information
//

    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_mouse {
    pub name: [c_char; 128],
    pub phys: [c_char; 64],
    pub usbdev: *mut usb_device,
    pub dev: *mut input_dev,
    pub irq: *mut urb,
    pub data: *mut signed char,
    pub data_dma: dma_addr_t,
}

#[no_mangle]
unsafe extern "C" fn usb_mouse_irq(urb: *mut urb) {
    static void usb_mouse_irq(struct urb *urb)
    {
    struct usb_mouse *mouse = urb.context;
    signed char *data = mouse.data;
    struct input_dev *dev = mouse.dev;
    int status;
    switch (urb.status) {
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
    input_report_key(dev, BTN_LEFT,   data[0] & 0x01);
    input_report_key(dev, BTN_RIGHT,  data[0] & 0x02);
    input_report_key(dev, BTN_MIDDLE, data[0] & 0x04);
    input_report_key(dev, BTN_SIDE,   data[0] & 0x08);
    input_report_key(dev, BTN_EXTRA,  data[0] & 0x10);
    input_report_rel(dev, REL_X,     data[1]);
    input_report_rel(dev, REL_Y,     data[2]);
    input_report_rel(dev, REL_WHEEL, data[3]);
    input_sync(dev);
    resubmit:
    status = usb_submit_urb (urb, GFP_ATOMIC);
    if (status)
    dev_err(&mouse.usbdev.dev,
    "can't resubmit intr, %s-%s/input0, status %d\n",
    mouse.usbdev.bus.bus_name,
    mouse.usbdev.devpath, status);
    }
#[no_mangle]
unsafe extern "C" fn usb_mouse_open(dev: *mut input_dev) -> c_int {
    static int usb_mouse_open(struct input_dev *dev)
    {
    struct usb_mouse *mouse = input_get_drvdata(dev);
    mouse.irq.dev = mouse.usbdev;
    if (usb_submit_urb(mouse.irq, GFP_KERNEL))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb_mouse_close(dev: *mut input_dev) {
    static void usb_mouse_close(struct input_dev *dev)
    {
    struct usb_mouse *mouse = input_get_drvdata(dev);
    usb_kill_urb(mouse.irq);
    }
#[no_mangle]
unsafe extern "C" fn usb_mouse_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int usb_mouse_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    struct usb_device *dev = interface_to_usbdev(intf);
    struct usb_host_interface *interface;
    struct usb_endpoint_descriptor *endpoint;
    struct usb_mouse *mouse;
    struct input_dev *input_dev;
    struct seq_buf mouse_name;
    int pipe, maxp;
    let mut error: c_int = -ENOMEM;
    size_t len;
    interface = intf.cur_altsetting;
    if (interface.desc.bNumEndpoints != 1)
    return -ENODEV;
    endpoint = &interface.endpoint[0].desc;
    if (!usb_endpoint_is_int_in(endpoint))
    return -ENODEV;
    pipe = usb_rcvintpipe(dev, endpoint.bEndpointAddress);
    maxp = usb_maxpacket(dev, pipe);
    mouse = kzalloc_obj(struct usb_mouse);
    input_dev = input_allocate_device();
    if (!mouse || !input_dev)
    goto fail1;
    mouse.data = usb_alloc_coherent(dev, 8, GFP_KERNEL, &mouse.data_dma);
    if (!mouse.data)
    goto fail1;
    mouse.irq = usb_alloc_urb(0, GFP_KERNEL);
    if (!mouse.irq)
    goto fail2;
    mouse.usbdev = dev;
    mouse.dev = input_dev;
    seq_buf_init(&mouse_name, mouse.name, sizeof(mouse.name));
    if (dev.manufacturer)
    seq_buf_puts(&mouse_name, dev.manufacturer);
    if (dev.product) {
    if (dev.manufacturer)
    seq_buf_puts(&mouse_name, " ");
    seq_buf_puts(&mouse_name, dev.product);
    }
    if (!seq_buf_used(&mouse_name))
    snprintf(mouse.name, sizeof(mouse.name),
    "USB HIDBP Mouse %04x:%04x",
    le16_to_cpu(dev.descriptor.idVendor),
    le16_to_cpu(dev.descriptor.idProduct));
    usb_make_path(dev, mouse.phys, sizeof(mouse.phys));
    len = strnlen(mouse.phys, sizeof(mouse.phys));
    strscpy(mouse.phys + len, "/input0", sizeof(mouse.phys) - len);
    input_dev.name = mouse.name;
    input_dev.phys = mouse.phys;
    usb_to_input_id(dev, &input_dev.id);
    input_dev.dev.parent = &intf.dev;
    input_dev.evbit[0] = BIT_MASK(EV_KEY) | BIT_MASK(EV_REL);
    input_dev.keybit[BIT_WORD(BTN_MOUSE)] = BIT_MASK(BTN_LEFT) |
    BIT_MASK(BTN_RIGHT) | BIT_MASK(BTN_MIDDLE);
    input_dev.relbit[0] = BIT_MASK(REL_X) | BIT_MASK(REL_Y);
    input_dev.keybit[BIT_WORD(BTN_MOUSE)] |= BIT_MASK(BTN_SIDE) |
    BIT_MASK(BTN_EXTRA);
    input_dev.relbit[0] |= BIT_MASK(REL_WHEEL);
    input_set_drvdata(input_dev, mouse);
    input_dev.open = usb_mouse_open;
    input_dev.close = usb_mouse_close;
    usb_fill_int_urb(mouse.irq, dev, pipe, mouse.data,
    (maxp > 8 ? 8 : maxp),
    usb_mouse_irq, mouse, endpoint.bInterval);
    mouse.irq.transfer_dma = mouse.data_dma;
    mouse.irq.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    error = input_register_device(mouse.dev);
    if (error)
    goto fail3;
    usb_set_intfdata(intf, mouse);
    return 0;
    fail3:
    usb_free_urb(mouse.irq);
    fail2:
    usb_free_coherent(dev, 8, mouse.data, mouse.data_dma);
    fail1:
    input_free_device(input_dev);
    kfree(mouse);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn usb_mouse_disconnect(intf: *mut usb_interface) {
    static void usb_mouse_disconnect(struct usb_interface *intf)
    {
    struct usb_mouse *mouse = usb_get_intfdata (intf);
    usb_set_intfdata(intf, core::ptr::null_mut());
    if (mouse) {
    usb_kill_urb(mouse.irq);
    input_unregister_device(mouse.dev);
    usb_free_urb(mouse.irq);
    usb_free_coherent(interface_to_usbdev(intf), 8, mouse.data, mouse.data_dma);
    kfree(mouse);
    }
    }
    static const struct usb_device_id usb_mouse_id_table[] = {
    { USB_INTERFACE_INFO(USB_INTERFACE_CLASS_HID, USB_INTERFACE_SUBCLASS_BOOT,
    USB_INTERFACE_PROTOCOL_MOUSE) },
    { }	/* Terminating entry */
    };
    MODULE_DEVICE_TABLE (usb, usb_mouse_id_table);
    static struct usb_driver usb_mouse_driver = {
    .name		= "usbmouse",
    .probe		= usb_mouse_probe,
    .disconnect	= usb_mouse_disconnect,
    .id_table	= usb_mouse_id_table,
    };
    module_usb_driver(usb_mouse_driver);
