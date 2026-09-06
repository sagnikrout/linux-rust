//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-viewsonic.c
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
// HID driver for ViewSonic devices not fully compliant with HID standard
//
// Copyright (c) 2017 Nikolai Kondrashov
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//

// Size of the original descriptor of PD1011 signature pad
pub const PD1011_RDESC_ORIG_SIZE: c_int = 408;
// Fixed report descriptor of PD1011 signature pad
    static const __u8 pd1011_rdesc_fixed[] = {
    0x05, 0x0D,             /*  Usage Page (Digitizer),             */
    0x09, 0x01,             /*  Usage (Digitizer),                  */
    0xA1, 0x01,             /*  Collection (Application),           */
    0x85, 0x02,             /*      Report ID (2),                  */
    0x09, 0x20,             /*      Usage (Stylus),                 */
    0xA0,                   /*      Collection (Physical),          */
    0x75, 0x10,             /*          Report Size (16),           */
    0x95, 0x01,             /*          Report Count (1),           */
    0xA4,                   /*          Push,                       */
    0x05, 0x01,             /*          Usage Page (Desktop),       */
    0x65, 0x13,             /*          Unit (Inch),                */
    0x55, 0xFD,             /*          Unit Exponent (-3),         */
    0x34,                   /*          Physical Minimum (0),       */
    0x09, 0x30,             /*          Usage (X),                  */
    0x46, 0x5D, 0x21,       /*          Physical Maximum (8541),    */
    0x27, 0x80, 0xA9,
    0x00, 0x00,     /*          Logical Maximum (43392),    */
    0x81, 0x02,             /*          Input (Variable),           */
    0x09, 0x31,             /*          Usage (Y),                  */
    0x46, 0xDA, 0x14,       /*          Physical Maximum (5338),    */
    0x26, 0xF0, 0x69,       /*          Logical Maximum (27120),    */
    0x81, 0x02,             /*          Input (Variable),           */
    0xB4,                   /*          Pop,                        */
    0x14,                   /*          Logical Minimum (0),        */
    0x25, 0x01,             /*          Logical Maximum (1),        */
    0x75, 0x01,             /*          Report Size (1),            */
    0x95, 0x01,             /*          Report Count (1),           */
    0x81, 0x03,             /*          Input (Constant, Variable), */
    0x09, 0x32,             /*          Usage (In Range),           */
    0x09, 0x42,             /*          Usage (Tip Switch),         */
    0x95, 0x02,             /*          Report Count (2),           */
    0x81, 0x02,             /*          Input (Variable),           */
    0x95, 0x05,             /*          Report Count (5),           */
    0x81, 0x03,             /*          Input (Constant, Variable), */
    0x75, 0x10,             /*          Report Size (16),           */
    0x95, 0x01,             /*          Report Count (1),           */
    0x09, 0x30,             /*          Usage (Tip Pressure),       */
    0x15, 0x05,             /*          Logical Minimum (5),        */
    0x26, 0xFF, 0x07,       /*          Logical Maximum (2047),     */
    0x81, 0x02,             /*          Input (Variable),           */
    0x75, 0x10,             /*          Report Size (16),           */
    0x95, 0x01,             /*          Report Count (1),           */
    0x81, 0x03,             /*          Input (Constant, Variable), */
    0xC0,                   /*      End Collection,                 */
    0xC0                    /*  End Collection                      */
    };
    static const __u8 *viewsonic_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    switch (hdev.product) {
    case USB_DEVICE_ID_VIEWSONIC_PD1011:
    case USB_DEVICE_ID_SIGNOTEC_VIEWSONIC_PD1011:
    if (*rsize == PD1011_RDESC_ORIG_SIZE) {
// rsize = sizeof(pd1011_rdesc_fixed);
    return pd1011_rdesc_fixed;
    }
    break;
    }
    return rdesc;
    }
    static const struct hid_device_id viewsonic_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_VIEWSONIC,
    USB_DEVICE_ID_VIEWSONIC_PD1011) },
    { HID_USB_DEVICE(USB_VENDOR_ID_SIGNOTEC,
    USB_DEVICE_ID_SIGNOTEC_VIEWSONIC_PD1011) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, viewsonic_devices);
    static struct hid_driver viewsonic_driver = {
    .name = "viewsonic",
    .id_table = viewsonic_devices,
    .report_fixup = viewsonic_report_fixup,
    };
    module_hid_driver(viewsonic_driver);
    MODULE_DESCRIPTION("HID driver for ViewSonic devices not fully compliant with HID standard");
    MODULE_LICENSE("GPL");
