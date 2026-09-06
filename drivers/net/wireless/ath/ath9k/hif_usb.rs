//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath9k/hif_usb.h
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
// Copyright (c) 2010-2011 Atheros Communications Inc.
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
// old firmware images

// supported Major FW version
pub const MAJOR_VERSION_REQ: c_int = 1;
pub const MINOR_VERSION_REQ: c_int = 3;
// minimal and maximal supported Minor FW version.
pub const FIRMWARE_MINOR_IDX_MAX: c_int = 4;
pub const FIRMWARE_MINOR_IDX_MIN: c_int = 3;

pub const AR9271_FIRMWARE: c_uint = 0x501000;
pub const AR9271_FIRMWARE_TEXT: c_uint = 0x903000;
pub const AR7010_FIRMWARE_TEXT: c_uint = 0x906000;
pub const FIRMWARE_DOWNLOAD: c_uint = 0x30;
pub const FIRMWARE_DOWNLOAD_COMP: c_uint = 0x31;
pub const ATH_USB_RX_STREAM_MODE_TAG: c_uint = 0x4e00;
pub const ATH_USB_TX_STREAM_MODE_TAG: c_uint = 0x697e;
// FIXME: Verify these numbers (with Windows)
pub const MAX_TX_URB_NUM: c_int = 8;
pub const MAX_TX_BUF_NUM: c_int = 256;
pub const MAX_TX_BUF_SIZE: c_int = 32768;
pub const MAX_TX_AGGR_NUM: c_int = 20;
pub const MAX_RX_URB_NUM: c_int = 8;
pub const MAX_RX_BUF_SIZE: c_int = 16384;
pub const MAX_PKT_NUM_IN_TRANSFER: c_int = 10;
pub const MAX_REG_OUT_URB_NUM: c_int = 1;
pub const MAX_REG_IN_URB_NUM: c_int = 64;
pub const MAX_REG_IN_BUF_SIZE: c_int = 64;
// USB Endpoint definition
pub const USB_WLAN_TX_PIPE: c_int = 1;
pub const USB_WLAN_RX_PIPE: c_int = 2;
pub const USB_REG_IN_PIPE: c_int = 3;
pub const USB_REG_OUT_PIPE: c_int = 4;

pub const HIF_USB_MAX_RXPIPES: c_int = 2;
pub const HIF_USB_MAX_TXPIPES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_buf {
    pub len: u16,
    pub offset: u16,
    pub urb: *mut urb,
    pub skb_queue: sk_buff_head,
    pub hif_dev: *mut hif_device_usb,
    pub list: list_head,
    pub buf: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_buf {
    pub skb: *mut sk_buff,
    pub hif_dev: *mut hif_device_usb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hif_usb_tx {
    pub flags: u8,
    pub tx_buf_cnt: u8,
    pub tx_skb_cnt: u16,
    pub tx_skb_queue: sk_buff_head,
    pub tx_buf: list_head,
    pub tx_pending: list_head,
    pub tx_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_buf {
    pub skb: *mut sk_buff,
    pub hif_dev: *mut hif_device_usb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hif_device_usb {
    pub udev: *mut usb_device,
    pub interface: *mut usb_interface,
    pub id_info: c_int,
    pub fw_data: *const c_void,
    pub fw_size: usize,
    pub fw_done: completion,
    pub htc_handle: *mut htc_target,
    pub tx: hif_usb_tx,
    pub regout_submitted: usb_anchor,
    pub rx_submitted: usb_anchor,
    pub reg_in_submitted: usb_anchor,
    pub mgmt_submitted: usb_anchor,
    pub remain_skb: *mut sk_buff,
    pub fw_name: [c_char; 64],
    pub fw_minor_index: c_int,
    pub rx_remain_len: c_int,
    pub rx_pkt_len: c_int,
    pub rx_transfer_len: c_int,
    pub rx_pad_len: c_int,
    pub rx_lock: spinlock_t,
    pub /: *mut *mut *mut u8 flags; / HIF_USB_,
}

extern "C" {
    pub fn ath9k_hif_usb_dealloc_urbs(hif_dev: *mut hif_device_usb);
}
