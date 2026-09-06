//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/webusb.h
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
// WebUSB descriptors and constants
//
// Copyright (C) 2023 Jó Ágila Bitsch <jgilab@gmail.com>
//

//
// Little Endian PlatformCapablityUUID for WebUSB
// 3408b638-09a9-47a0-8bfd-a0768815b665
// to identify Platform Device Capability descriptors as referring to WebUSB.
//

//
// WebUSB Platform Capability data
//
// A device announces support for the
// WebUSB command set by including the following Platform Descriptor Data in its
// Binary Object Store associated with the WebUSB_UUID above.
// See: https://wicg.github.io/webusb/#webusb-platform-capability-descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_webusb_cap_data {
    pub bcdVersion: __le16,

    pub bVendorCode: u8,
    pub iLandingPage: u8,
pub const WEBUSB_LANDING_PAGE_NOT_PRESENT: c_int = 0;

    pub __packed: },
pub const USB_WEBUSB_CAP_DATA_SIZE: c_int = 4;
//
// Get URL Request
//
// The request to fetch an URL is defined in https://wicg.github.io/webusb/#get-url as:
// bmRequestType: (USB_DIR_IN | USB_TYPE_VENDOR) = 11000000B
// bRequest: bVendorCode
// wValue: iLandingPage
// wIndex: GET_URL = 2
// wLength: Descriptor Length (typically U8_MAX = 255)
// Data: URL Descriptor
//
pub const WEBUSB_GET_URL: c_int = 2;
//
// This descriptor contains a single URL and is returned by the Get URL request.
//
// See: https://wicg.github.io/webusb/#url-descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct webusb_url_descriptor {
    pub bLength: u8,
pub const WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH: c_int = 3;
    pub bDescriptorType: u8,
pub const WEBUSB_URL_DESCRIPTOR_TYPE: c_int = 3;
    pub bScheme: u8,
pub const WEBUSB_URL_SCHEME_HTTP: c_int = 0;
pub const WEBUSB_URL_SCHEME_HTTPS: c_int = 1;
pub const WEBUSB_URL_SCHEME_NONE: c_int = 255;
    pub WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH]: u8 URL[U8_MAX -,
    pub __packed: },
//
// Buffer size to hold the longest URL that can be in an URL descriptor
//
// The descriptor can be U8_MAX  bytes long.
// WEBUSB_URL_DESCRIPTOR_HEADER_LENGTH bytes are used for a header.
// Since the longest prefix that might be stripped is "https://", we may accommodate an additional
// 8 bytes.
//

