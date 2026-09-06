//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/irda.h
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
// USB IrDA Bridge Device Definition
//
// This device should use Application-specific class
pub const USB_SUBCLASS_IRDA: c_uint = 0x02;
// -------------------------------------------------------------------------
// Class-Specific requests (bRequest field)
pub const USB_REQ_CS_IRDA_RECEIVING: c_int = 1;
pub const USB_REQ_CS_IRDA_CHECK_MEDIA_BUSY: c_int = 3;
pub const USB_REQ_CS_IRDA_RATE_SNIFF: c_int = 4;
pub const USB_REQ_CS_IRDA_UNICAST_LIST: c_int = 5;
pub const USB_REQ_CS_IRDA_GET_CLASS_DESC: c_int = 6;
// -------------------------------------------------------------------------
// Class-Specific descriptor
pub const USB_DT_CS_IRDA: c_uint = 0x21;
// -------------------------------------------------------------------------
// Data sizes

// Window sizes

// Min turnaround times in usecs

// Baud rates

// Additional BOFs

// IRDA Rate Sniff
pub const USB_IRDA_RATE_SNIFF: c_int = 1;
// -------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_irda_cs_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bcdSpecRevision: __le16,
    pub bmDataSize: __u8,
    pub bmWindowSize: __u8,
    pub bmMinTurnaroundTime: __u8,
    pub wBaudRate: __le16,
    pub bmAdditionalBOFs: __u8,
    pub bIrdaRateSniff: __u8,
    pub bMaxUnicastList: __u8,
// C attribute field omitted
// -------------------------------------------------------------------------
// Data Format

// The following is a 4-bit value used for both
// inbound and outbound headers:
//
// 0 - speed ignored
// 1 - 2400 bps
// 2 - 9600 bps
// 3 - 19200 bps
// 4 - 38400 bps
// 5 - 57600 bps
// 6 - 115200 bps
// 7 - 576000 bps
// 8 - 1.152 Mbps
// 9 - 4 Mbps
// 10..15 - Reserved
//
pub const USB_IRDA_STATUS_LINK_SPEED: c_uint = 0x0f;
pub const USB_IRDA_LS_NO_CHANGE: c_int = 0;
pub const USB_IRDA_LS_2400: c_int = 1;
pub const USB_IRDA_LS_9600: c_int = 2;
pub const USB_IRDA_LS_19200: c_int = 3;
pub const USB_IRDA_LS_38400: c_int = 4;
pub const USB_IRDA_LS_57600: c_int = 5;
pub const USB_IRDA_LS_115200: c_int = 6;
pub const USB_IRDA_LS_576000: c_int = 7;
pub const USB_IRDA_LS_1152000: c_int = 8;
pub const USB_IRDA_LS_4000000: c_int = 9;
// The following is a 4-bit value used only for
// outbound header:
//
// 0 - No change (BOF ignored)
// 1 - 48 BOFs
// 2 - 24 BOFs
// 3 - 12 BOFs
// 4 - 6 BOFs
// 5 - 3 BOFs
// 6 - 2 BOFs
// 7 - 1 BOFs
// 8 - 0 BOFs
// 9..15 - Reserved
//
pub const USB_IRDA_EXTRA_BOFS: c_uint = 0xf0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_irda_inbound_header {
    pub bmStatus: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_irda_outbound_header {
    pub bmChange: __u8,
}
