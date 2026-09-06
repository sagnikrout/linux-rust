//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/purelifi/plfxlc/intf.h
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
// Copyright (c) 2021 pureLiFi
//
pub const PURELIFI_BYTE_NUM_ALIGNMENT: c_int = 4;
pub const ETH_ALEN: c_int = 6;
pub const AP_USER_LIMIT: c_int = 8;
pub const PLF_VNDR_FPGA_STATE_REQ: c_uint = 0x30;
pub const PLF_VNDR_FPGA_SET_REQ: c_uint = 0x33;
pub const PLF_VNDR_FPGA_SET_CMD: c_uint = 0x34;
pub const PLF_VNDR_FPGA_STATE_CMD: c_uint = 0x35;
pub const PLF_VNDR_XL_FW_CMD: c_uint = 0x80;
pub const PLF_VNDR_XL_DATA_CMD: c_uint = 0x81;
pub const PLF_VNDR_XL_FILE_CMD: c_uint = 0x82;
pub const PLF_VNDR_XL_EX_CMD: c_uint = 0x83;
pub const PLF_MAC_VENDOR_REQUEST: c_uint = 0x36;
pub const PLF_SERIAL_NUMBER_VENDOR_REQUEST: c_uint = 0x37;
pub const PLF_FIRMWARE_VERSION_VENDOR_REQUEST: c_uint = 0x39;
pub const PLF_SERIAL_LEN: c_int = 14;
pub const PLF_FW_VER_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_status {
    pub rssi: __be16,
    pub rate_idx: u8,
    pub pad: u8,
    pub crc_error_count: __be64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum plf_usb_req_enum {
    USB_REQ_TEST_WR            = 0,
    USB_REQ_MAC_WR             = 1,
    USB_REQ_POWER_WR           = 2,
    USB_REQ_RXTX_WR            = 3,
    USB_REQ_BEACON_WR          = 4,
    USB_REQ_BEACON_INTERVAL_WR = 5,
    USB_REQ_RTS_CTS_RATE_WR    = 6,
    USB_REQ_HASH_WR            = 7,
    USB_REQ_DATA_TX            = 8,
    USB_REQ_RATE_WR            = 9,
    USB_REQ_SET_FREQ           = 15
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plf_usb_req {
    pub /: *mut *mut __be32 id; / should be plf_usb_req_enum,
    pub len: __be32,
    pub buf: [u8; 512],
}
