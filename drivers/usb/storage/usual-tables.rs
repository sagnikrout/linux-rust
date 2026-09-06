//! Automatically rewritten from C to Rust
//! Source: drivers/usb/storage/usual-tables.c
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
// Driver for USB Mass Storage devices
// Usual Tables File for usb-storage and libusual
//
// Copyright (C) 2009 Alan Stern (stern@rowland.harvard.edu)
//

//
// The table of devices
//

    vendorName, productName, useProtocol, useTransport, \
    initFunction, flags) \
    { USB_DEVICE_VER(id_vendor, id_product, bcdDeviceMin, bcdDeviceMax), \
    .driver_info = (kernel_ulong_t)(flags) }

    { USB_INTERFACE_INFO(USB_CLASS_MASS_STORAGE, useProto, useTrans) }
    const struct usb_device_id usb_storage_usb_ids[] = {

    { }		/* Terminating entry */
    };
    MODULE_DEVICE_TABLE(usb, usb_storage_usb_ids);

//
// The table of devices to ignore
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ignore_entry {
    pub bcdmax: u16 vid, pid, bcdmin,,
}

    vendorName, productName, useProtocol, useTransport, \
    initFunction, flags) \
    {					\
    .vid	= id_vendor,		\
    .pid 	= id_product,		\
    .bcdmin	= bcdDeviceMin,		\
    .bcdmax = bcdDeviceMax,		\
    }
    static const struct ignore_entry ignore_ids[] = {

    { }		/* Terminating entry */
    };

// Return an error if a device is in the ignore_ids list
#[no_mangle]
pub unsafe extern "C" fn usb_usual_ignore_device(intf: *mut usb_interface) -> c_int {
    int usb_usual_ignore_device(struct usb_interface *intf)
    {
    struct usb_device *udev;
    unsigned vid, pid, bcd;
    const struct ignore_entry *p;
    udev = interface_to_usbdev(intf);
    vid = le16_to_cpu(udev.descriptor.idVendor);
    pid = le16_to_cpu(udev.descriptor.idProduct);
    bcd = le16_to_cpu(udev.descriptor.bcdDevice);
    for (p = ignore_ids; p.vid; ++p) {
    if (p.vid == vid && p.pid == pid &&
    p.bcdmin <= bcd && p.bcdmax >= bcd)
    return -ENXIO;
    }
    return 0;
    }
