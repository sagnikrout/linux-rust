//! Automatically rewritten from C to Rust
//! Source: drivers/usb/core/endpoint.c
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
// drivers/usb/core/endpoint.c
//
// (C) Copyright 2002,2004,2006 Greg Kroah-Hartman
// (C) Copyright 2002,2004 IBM Corp.
// (C) Copyright 2006 Novell Inc.
//
// Released under the GPLv2 only.
//
// Endpoint sysfs stuff
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep_device {
    pub desc: *mut usb_endpoint_descriptor,
    pub udev: *mut usb_device,
    pub dev: device,
}

    container_of(_dev, struct ep_device, dev)

    static ssize_t field##_show(struct device *dev,			\
    struct device_attribute *attr,	\
    char *buf)			\
    {								\
    struct ep_device *ep = to_ep_device(dev);		\
    return sysfs_emit(buf, format_string, ep.desc.field);	\
    }								\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: field) -> static {
    static DEVICE_ATTR_RO(field)
    usb_ep_attr(bLength, "%02x\n");
    usb_ep_attr(bEndpointAddress, "%02x\n");
    usb_ep_attr(bmAttributes, "%02x\n");
    usb_ep_attr(bInterval, "%02x\n");
    static ssize_t wMaxPacketSize_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ep_device *ep = to_ep_device(dev);
    return sysfs_emit(buf, "%04x\n", usb_endpoint_maxp(ep.desc));
    }
    static DEVICE_ATTR_RO(wMaxPacketSize);
    static ssize_t type_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ep_device *ep = to_ep_device(dev);
    char *type = "unknown";
    switch (usb_endpoint_type(ep.desc)) {
    case USB_ENDPOINT_XFER_CONTROL:
    type = "Control";
    break;
    case USB_ENDPOINT_XFER_ISOC:
    type = "Isoc";
    break;
    case USB_ENDPOINT_XFER_BULK:
    type = "Bulk";
    break;
    case USB_ENDPOINT_XFER_INT:
    type = "Interrupt";
    break;
    }
    return sysfs_emit(buf, "%s\n", type);
    }
    static DEVICE_ATTR_RO(type);
    static ssize_t interval_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ep_device *ep = to_ep_device(dev);
    unsigned int interval;
    char unit;
    interval = usb_decode_interval(ep.desc, ep.udev.speed);
    if (interval % 1000) {
    unit = 'u';
    } else {
    unit = 'm';
    interval /= 1000;
    }
    return sysfs_emit(buf, "%d%cs\n", interval, unit);
    }
    static DEVICE_ATTR_RO(interval);
    static ssize_t direction_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ep_device *ep = to_ep_device(dev);
    char *direction;
    if (usb_endpoint_xfer_control(ep.desc))
    direction = "both";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: usb_endpoint_dir_in(ep->desc)) -> else {
    else if (usb_endpoint_dir_in(ep.desc))
    direction = "in";
    else
    direction = "out";
    return sysfs_emit(buf, "%s\n", direction);
    }
    static DEVICE_ATTR_RO(direction);
    static struct attribute *ep_dev_attrs[] = {
    &dev_attr_bLength.attr,
    &dev_attr_bEndpointAddress.attr,
    &dev_attr_bmAttributes.attr,
    &dev_attr_bInterval.attr,
    &dev_attr_wMaxPacketSize.attr,
    &dev_attr_interval.attr,
    &dev_attr_type.attr,
    &dev_attr_direction.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ep_dev_attr_grp = {
    .attrs = ep_dev_attrs,
    };
    static const struct attribute_group *ep_dev_groups[] = {
    &ep_dev_attr_grp,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn ep_device_release(dev: *mut device) {
    static void ep_device_release(struct device *dev)
    {
    struct ep_device *ep_dev = to_ep_device(dev);
    kfree(ep_dev);
    }
    const struct device_type usb_ep_device_type = {
    .name =		"usb_endpoint",
    .release = ep_device_release,
    };
    int usb_create_ep_devs(struct device *parent,
    struct usb_host_endpoint *endpoint,
    struct usb_device *udev)
    {
    struct ep_device *ep_dev;
    int retval;
    ep_dev = kzalloc_obj(*ep_dev);
    if (!ep_dev) {
    retval = -ENOMEM;
    goto exit;
    }
    ep_dev.desc = &endpoint.desc;
    ep_dev.udev = udev;
    ep_dev.dev.groups = ep_dev_groups;
    ep_dev.dev.type = &usb_ep_device_type;
    ep_dev.dev.parent = parent;
    dev_set_name(&ep_dev.dev, "ep_%02x", endpoint.desc.bEndpointAddress);
    retval = device_register(&ep_dev.dev);
    if (retval)
    goto error_register;
    device_enable_async_suspend(&ep_dev.dev);
    endpoint.ep_dev = ep_dev;
    return retval;
    error_register:
    put_device(&ep_dev.dev);
    exit:
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn usb_remove_ep_devs(endpoint: *mut usb_host_endpoint) {
    void usb_remove_ep_devs(struct usb_host_endpoint *endpoint)
    {
    struct ep_device *ep_dev = endpoint.ep_dev;
    if (ep_dev) {
    device_unregister(&ep_dev.dev);
    endpoint.ep_dev = core::ptr::null_mut();
    }
    }
