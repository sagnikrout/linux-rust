//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/mctp-usb.h
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
// mctp-usb.h - MCTP USB transport binding: common definitions,
// based on DMTF0283 specification:
// https://www.dmtf.org/sites/default/files/standards/documents/DSP0283_1.1.0.pdf
//
// These are protocol-level definitions, that may be shared between host
// and gadget drivers.
//
// Copyright (C) 2024-2025 Code Construct Pty Ltd
//

//
// MCTP-over-USB transport header. DSP0283 v1.0 has an 8-bit length field
// (preceded by 8 reserved bits), v1.1 has a 13-bit length field (preceded by
// 3 reserved bits). We use a be16 for our length to handle the larger v1.1
// representation, and mask as appropriate.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_usb_hdr {
    pub id: __be16,
    pub len: __be16,
    pub __packed: },
// max transfer size for DSP0283 v1.0
pub const MCTP_USB_1_0_XFER_SIZE: c_int = 512;
pub const MCTP_USB_BTU: c_int = 68;

pub const MCTP_USB_DMTF_ID: c_uint = 0x1ab4;
// mctp-usblib
//
// RX handle: drivers will typically create one on init, which persists for
// the life of the driver. The same handle is used for progressive
// prepare -> complete operations (for each incoming USB transfer), which
// result in netif_rx()-ing the MCTP packets received
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_usblib_rx {
    pub skb: *mut sk_buff,
    pub ep_pktlen: u16,
    pub span: bool,
}

extern "C" {
    pub fn mctp_usblib_rx_init(rx: *mut mctp_usblib_rx, ep_pktlen: u16, span: bool) -> c_int;
}
extern "C" {
    pub fn mctp_usblib_rx_fini(rx: *mut mctp_usblib_rx);
}
extern "C" {
    pub fn mctp_usblib_rx_cancel(rx: *mut mctp_usblib_rx);
}
//
// TX handle: created by mctp_usblib_tx_push() during the tx path, and
// may persist across multiple packet transmits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_usblib_tx_ops {
// Start a USB TX for @data. On returning success, the implementation
// must arrange for mctp_usblib_tx_send_complete() to be called at some
// later point (eg., on urb completion).
//
    pub len): *mut *mut *mut *mut int (send)(struct mctp_usblib_tx_ctx tx_ctx, void data, size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_usblib_tx {
    pub ops: mctp_usblib_tx_ops,
    pub priv: *mut c_void,
    pub span: bool,
// protects access to cur_ctx
    pub lock: spinlock_t,
// context to which we are adding packets, cleared on send
    pub cur_ctx: *mut mctp_usblib_tx_ctx,
}

extern "C" {
    pub fn mctp_usblib_tx_fini(tx: *mut mctp_usblib_tx);
}
