//! Automatically rewritten from C to Rust
//! Source: drivers/input/tablet/kbtab.c
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


// SPDX-License-Identifier: GPL-2.0-only

//
// Pressure-threshold modules param code from Alex Perry <alex.perry@ieee.org>
//
    MODULE_AUTHOR("Josh Myer <josh@joshisanerd.com>");
    MODULE_DESCRIPTION("USB KB Gear JamStudio Tablet driver");
    MODULE_LICENSE("GPL");
pub const USB_VENDOR_ID_KBGEAR: c_uint = 0x084e;
    let mut kb_pressure_click: static int = 0x10;
    module_param(kb_pressure_click, int, 0);
    MODULE_PARM_DESC(kb_pressure_click, "pressure threshold for clicks");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbtab {
    pub data: *mut c_uchar,
    pub data_dma: dma_addr_t,
    pub dev: *mut input_dev,
    pub intf: *mut usb_interface,
    pub irq: *mut urb,
    pub phys: [c_char; 32],
}

#[no_mangle]
unsafe extern "C" fn kbtab_irq(urb: *mut urb) {
    static void kbtab_irq(struct urb *urb)
    {
    struct kbtab *kbtab = urb.context;
    unsigned char *data = kbtab.data;
    struct input_dev *dev = kbtab.dev;
    int pressure;
    int retval;
    switch (urb.status) {
    case 0:
// success
    break;
    case -ECONNRESET:
    case -ENOENT:
    case -ESHUTDOWN:
// this urb is terminated, clean up
    dev_dbg(&kbtab.intf.dev,
    "%s - urb shutting down with status: %d\n",
    __func__, urb.status);
    return;
    default:
    dev_dbg(&kbtab.intf.dev,
    "%s - nonzero urb status received: %d\n",
    __func__, urb.status);
    goto exit;
    }
    input_report_key(dev, BTN_TOOL_PEN, 1);
    input_report_abs(dev, ABS_X, get_unaligned_le16(&data[1]));
    input_report_abs(dev, ABS_Y, get_unaligned_le16(&data[3]));
// input_report_key(dev, BTN_TOUCH , data[0] & 0x01);
    input_report_key(dev, BTN_RIGHT, data[0] & 0x02);
    pressure = data[5];
    if (kb_pressure_click == -1)
    input_report_abs(dev, ABS_PRESSURE, pressure);
    else
    input_report_key(dev, BTN_LEFT, pressure > kb_pressure_click ? 1 : 0);
    input_sync(dev);
    exit:
    retval = usb_submit_urb(urb, GFP_ATOMIC);
    if (retval)
    dev_err(&kbtab.intf.dev,
    "%s - usb_submit_urb failed with result %d\n",
    __func__, retval);
    }
    static const struct usb_device_id kbtab_ids[] = {
    { USB_DEVICE(USB_VENDOR_ID_KBGEAR, 0x1001), .driver_info = 0 },
    { }
    };
    MODULE_DEVICE_TABLE(usb, kbtab_ids);
#[no_mangle]
unsafe extern "C" fn kbtab_open(dev: *mut input_dev) -> c_int {
    static int kbtab_open(struct input_dev *dev)
    {
    struct kbtab *kbtab = input_get_drvdata(dev);
    struct usb_device *udev = interface_to_usbdev(kbtab.intf);
    kbtab.irq.dev = udev;
    if (usb_submit_urb(kbtab.irq, GFP_KERNEL))
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kbtab_close(dev: *mut input_dev) {
    static void kbtab_close(struct input_dev *dev)
    {
    struct kbtab *kbtab = input_get_drvdata(dev);
    usb_kill_urb(kbtab.irq);
    }
#[no_mangle]
unsafe extern "C" fn kbtab_probe(intf: *mut usb_interface, id: *const usb_device_id) -> c_int {
    static int kbtab_probe(struct usb_interface *intf, const struct usb_device_id *id)
    {
    struct usb_device *dev = interface_to_usbdev(intf);
    struct usb_endpoint_descriptor *endpoint;
    struct kbtab *kbtab;
    struct input_dev *input_dev;
    let mut error: c_int = -ENOMEM;
    if (intf.cur_altsetting.desc.bNumEndpoints < 1)
    return -ENODEV;
    endpoint = &intf.cur_altsetting.endpoint[0].desc;
    if (!usb_endpoint_is_int_in(endpoint))
    return -ENODEV;
    kbtab = kzalloc_obj(*kbtab);
    input_dev = input_allocate_device();
    if (!kbtab || !input_dev)
    goto fail1;
    kbtab.data = usb_alloc_coherent(dev, 8, GFP_KERNEL, &kbtab.data_dma);
    if (!kbtab.data)
    goto fail1;
    kbtab.irq = usb_alloc_urb(0, GFP_KERNEL);
    if (!kbtab.irq)
    goto fail2;
    kbtab.intf = intf;
    kbtab.dev = input_dev;
    usb_make_path(dev, kbtab.phys, sizeof(kbtab.phys));
    strlcat(kbtab.phys, "/input0", sizeof(kbtab.phys));
    input_dev.name = "KB Gear Tablet";
    input_dev.phys = kbtab.phys;
    usb_to_input_id(dev, &input_dev.id);
    input_dev.dev.parent = &intf.dev;
    input_set_drvdata(input_dev, kbtab);
    input_dev.open = kbtab_open;
    input_dev.close = kbtab_close;
    input_dev.evbit[0] |= BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    input_dev.keybit[BIT_WORD(BTN_LEFT)] |=
    BIT_MASK(BTN_LEFT) | BIT_MASK(BTN_RIGHT);
    input_dev.keybit[BIT_WORD(BTN_DIGI)] |=
    BIT_MASK(BTN_TOOL_PEN) | BIT_MASK(BTN_TOUCH);
    input_set_abs_params(input_dev, ABS_X, 0, 0x2000, 4, 0);
    input_set_abs_params(input_dev, ABS_Y, 0, 0x1750, 4, 0);
    input_set_abs_params(input_dev, ABS_PRESSURE, 0, 0xff, 0, 0);
    usb_fill_int_urb(kbtab.irq, dev,
    usb_rcvintpipe(dev, endpoint.bEndpointAddress),
    kbtab.data, 8,
    kbtab_irq, kbtab, endpoint.bInterval);
    kbtab.irq.transfer_dma = kbtab.data_dma;
    kbtab.irq.transfer_flags |= URB_NO_TRANSFER_DMA_MAP;
    error = input_register_device(kbtab.dev);
    if (error)
    goto fail3;
    usb_set_intfdata(intf, kbtab);
    return 0;
    fail3:	usb_free_urb(kbtab.irq);
    fail2:	usb_free_coherent(dev, 8, kbtab.data, kbtab.data_dma);
    fail1:	input_free_device(input_dev);
    kfree(kbtab);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn kbtab_disconnect(intf: *mut usb_interface) {
    static void kbtab_disconnect(struct usb_interface *intf)
    {
    struct kbtab *kbtab = usb_get_intfdata(intf);
    struct usb_device *udev = interface_to_usbdev(intf);
    usb_set_intfdata(intf, core::ptr::null_mut());
    input_unregister_device(kbtab.dev);
    usb_free_urb(kbtab.irq);
    usb_free_coherent(udev, 8, kbtab.data, kbtab.data_dma);
    kfree(kbtab);
    }
    static struct usb_driver kbtab_driver = {
    .name =		"kbtab",
    .probe =	kbtab_probe,
    .disconnect =	kbtab_disconnect,
    .id_table =	kbtab_ids,
    };
    module_usb_driver(kbtab_driver);
