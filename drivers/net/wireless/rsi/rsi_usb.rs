//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/rsi/rsi_usb.h
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


//
// @section LICENSE
// Copyright (c) 2014 Redpine Signals Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const RSI_USB_VENDOR_ID: c_uint = 0x1618;
pub const RSI_USB_PID_9113: c_uint = 0x9113;
pub const RSI_USB_PID_9116: c_uint = 0x9116;
pub const USB_INTERNAL_REG_1: c_uint = 0x25000;
pub const RSI_USB_READY_MAGIC_NUM: c_uint = 0xab;
pub const FW_STATUS_REG: c_uint = 0x41050012;
pub const RSI_TA_HOLD_REG: c_uint = 0x22000844;
pub const RSI_FW_WDT_DISABLE_REQ: c_uint = 0x69;
pub const USB_VENDOR_REGISTER_READ: c_uint = 0x15;
pub const USB_VENDOR_REGISTER_WRITE: c_uint = 0x16;
pub const RSI_USB_TX_HEAD_ROOM: c_int = 128;
pub const MAX_RX_URBS: c_int = 2;
pub const MAX_BULK_EP: c_int = 8;
pub const WLAN_EP: c_int = 1;
pub const BT_EP: c_int = 2;
pub const RSI_USB_BUF_SIZE: c_int = 4096;
pub const RSI_USB_CTRL_BUF_SIZE: c_uint = 0x04;
pub const RSI_MAX_RX_USB_PKT_SIZE: c_int = 3000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_usb_ctrl_block {
    pub data: *mut u8,
    pub rx_urb: *mut urb,
    pub rx_skb: *mut sk_buff,
    pub ep_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_91x_usbdev {
    pub priv: *mut c_void,
    pub rx_thread: rsi_thread,
    pub endpoint: u8,
    pub usbdev: *mut usb_device,
    pub pfunction: *mut usb_interface,
    pub rx_cb: [rx_usb_ctrl_block; MAX_RX_URBS],
    pub tx_buffer: *mut u8,
    pub bulkin_size: [__le16; MAX_BULK_EP],
    pub bulkin_endpoint_addr: [u8; MAX_BULK_EP],
    pub bulkout_size: [__le16; MAX_BULK_EP],
    pub bulkout_endpoint_addr: [u8; MAX_BULK_EP],
    pub tx_blk_size: u32,
    pub write_fail: u8,
    pub rx_q: sk_buff_head,
}

// In USB, there isn't any need to check the queue status
extern "C" {
    pub fn rsi_usb_rx_thread(data: *mut c_void) -> c_int;
}
