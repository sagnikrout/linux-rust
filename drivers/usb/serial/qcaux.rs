//! Automatically rewritten from C to Rust
//! Source: drivers/usb/serial/qcaux.c
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
// Qualcomm USB Auxiliary Serial Port driver
//
// Copyright (C) 2008 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (C) 2010 Dan Williams <dcbw@redhat.com>
//
// Devices listed here usually provide a CDC ACM port on which normal modem
// AT commands and PPP can be used.  But when that port is in-use by PPP it
// cannot be used simultaneously for status or signal strength.  Instead, the
// ports here can be queried for that information using the Qualcomm DM
// protocol.
//

// NOTE: for now, only use this driver for devices that provide a CDC-ACM port
// for normal AT commands, but also provide secondary USB interfaces for the
// QCDM-capable ports.  Devices that do not provide a CDC-ACM port should
// probably be driven by option.ko.
//
// UTStarcom/Pantech/Curitel devices
pub const UTSTARCOM_VENDOR_ID: c_uint = 0x106c;
pub const UTSTARCOM_PRODUCT_PC5740: c_uint = 0x3701;
pub const UTSTARCOM_PRODUCT_PC5750: c_uint = 0x3702 /* aka Pantech PX-500 */;
pub const UTSTARCOM_PRODUCT_UM150: c_uint = 0x3711;
pub const UTSTARCOM_PRODUCT_UM175_V1: c_uint = 0x3712;
pub const UTSTARCOM_PRODUCT_UM175_V2: c_uint = 0x3714;
pub const UTSTARCOM_PRODUCT_UM175_ALLTEL: c_uint = 0x3715;
// CMOTECH devices
pub const CMOTECH_VENDOR_ID: c_uint = 0x16d8;
pub const CMOTECH_PRODUCT_CDU550: c_uint = 0x5553;
pub const CMOTECH_PRODUCT_CDX650: c_uint = 0x6512;
// LG devices
pub const LG_VENDOR_ID: c_uint = 0x1004;
pub const LG_PRODUCT_VX4400_6000: c_uint = 0x6000 /* VX4400/VX6000/Rumor */;
// Sanyo devices
pub const SANYO_VENDOR_ID: c_uint = 0x0474;
pub const SANYO_PRODUCT_KATANA_LX: c_uint = 0x0754 /* SCP-3800 (Katana LX) */;
// Samsung devices
pub const SAMSUNG_VENDOR_ID: c_uint = 0x04e8;
pub const SAMSUNG_PRODUCT_U520: c_uint = 0x6640 /* SCH-U520 */;
    static const struct usb_device_id id_table[] = {
    { USB_DEVICE_AND_INTERFACE_INFO(UTSTARCOM_VENDOR_ID, UTSTARCOM_PRODUCT_PC5740, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(UTSTARCOM_VENDOR_ID, UTSTARCOM_PRODUCT_PC5750, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(UTSTARCOM_VENDOR_ID, UTSTARCOM_PRODUCT_UM150, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(UTSTARCOM_VENDOR_ID, UTSTARCOM_PRODUCT_UM175_V1, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(UTSTARCOM_VENDOR_ID, UTSTARCOM_PRODUCT_UM175_V2, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(UTSTARCOM_VENDOR_ID, UTSTARCOM_PRODUCT_UM175_ALLTEL, 0xff, 0x00, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CDU550, 0xff, 0xff, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(CMOTECH_VENDOR_ID, CMOTECH_PRODUCT_CDX650, 0xff, 0xff, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(LG_VENDOR_ID, LG_PRODUCT_VX4400_6000, 0xff, 0xff, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(SANYO_VENDOR_ID, SANYO_PRODUCT_KATANA_LX, 0xff, 0xff, 0x00) },
    { USB_DEVICE_AND_INTERFACE_INFO(SAMSUNG_VENDOR_ID, SAMSUNG_PRODUCT_U520, 0xff, 0x00, 0x00) },
    { USB_VENDOR_AND_INTERFACE_INFO(UTSTARCOM_VENDOR_ID, 0xff, 0xfd, 0xff) },  /* NMEA */
    { USB_VENDOR_AND_INTERFACE_INFO(UTSTARCOM_VENDOR_ID, 0xff, 0xfe, 0xff) },  /* WMC */
    { USB_VENDOR_AND_INTERFACE_INFO(UTSTARCOM_VENDOR_ID, 0xff, 0xff, 0xff) },  /* DIAG */
    { USB_DEVICE_AND_INTERFACE_INFO(0x1fac, 0x0151, 0xff, 0xff, 0xff) },
    { },
    };
    MODULE_DEVICE_TABLE(usb, id_table);
    static struct usb_serial_driver qcaux_device = {
    .driver = {
    .name =		"qcaux",
    },
    .id_table =		id_table,
    .num_ports =		1,
    };
    static struct usb_serial_driver * const serial_drivers[] = {
    &qcaux_device, core::ptr::null_mut()
    };
    module_usb_serial_driver(serial_drivers, id_table);
    MODULE_DESCRIPTION("Qualcomm USB Auxiliary Serial Port driver");
    MODULE_LICENSE("GPL v2");
