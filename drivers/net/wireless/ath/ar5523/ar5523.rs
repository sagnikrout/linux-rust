//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ar5523/ar5523.h
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
// Copyright (c) 2006 Damien Bergamini <damien.bergamini@free.fr>
// Copyright (c) 2006 Sam Leffler, Errno Consulting
// Copyright (c) 2007 Christoph Hellwig <hch@lst.de>
// Copyright (c) 2008-2009 Weongyo Jeong <weongyo@freebsd.org>
// Copyright (c) 2012 Pontus Fuchs <pontus.fuchs@gmail.com>
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

pub const AR5523_CMD_TX_PIPE: c_uint = 0x01;
pub const AR5523_DATA_TX_PIPE: c_uint = 0x02;
pub const AR5523_CMD_RX_PIPE: c_uint = 0x81;
pub const AR5523_DATA_RX_PIPE: c_uint = 0x82;

pub const AR5523_DATA_TIMEOUT: c_int = 10000;
pub const AR5523_CMD_TIMEOUT: c_int = 1000;
pub const AR5523_TX_DATA_COUNT: c_int = 8;
pub const AR5523_TX_DATA_RESTART_COUNT: c_int = 2;
pub const AR5523_RX_DATA_COUNT: c_int = 16;
pub const AR5523_RX_DATA_REFILL_COUNT: c_int = 8;
pub const AR5523_CMD_ID: c_int = 1;
pub const AR5523_DATA_ID: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AR5523_flags {
    AR5523_HW_UP,
    AR5523_USB_DISCONNECTED,
    AR5523_CONNECTED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_tx_cmd {
    pub ar: *mut ar5523,
    pub urb_tx: *mut urb,
    pub buf_tx: *mut c_void,
    pub odata: *mut c_void,
    pub olen: c_int,
    pub flags: c_int,
    pub res: c_int,
    pub done: completion,
}

// This struct is placed in tx_info->driver_data. It must not be larger
// than IEEE80211_TX_INFO_DRIVER_DATA_SIZE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_tx_data {
    pub list: list_head,
    pub ar: *mut ar5523,
    pub urb: *mut urb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_rx_data {
    pub list: list_head,
    pub ar: *mut ar5523,
    pub urb: *mut urb,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523 {
    pub dev: *mut usb_device,
    pub hw: *mut ieee80211_hw,
    pub flags: c_ulong,
    pub mutex: mutex,
    pub wq: *mut workqueue_struct,
    pub tx_cmd: ar5523_tx_cmd,
    pub stat_work: delayed_work,
    pub tx_wd_timer: timer_list,
    pub tx_wd_work: work_struct,
    pub tx_work: work_struct,
    pub tx_queue_pending: list_head,
    pub tx_queue_submitted: list_head,
    pub tx_data_list_lock: spinlock_t,
    pub tx_flush_waitq: wait_queue_head_t,
// Queued + Submitted TX frames
    pub tx_nr_total: core::sync::atomic::AtomicI32,
// Submitted TX frames
    pub tx_nr_pending: core::sync::atomic::AtomicI32,
    pub rx_cmd_buf: *mut c_void,
    pub rx_cmd_urb: *mut urb,
    pub rx_data: [ar5523_rx_data; AR5523_RX_DATA_COUNT],
    pub rx_data_list_lock: spinlock_t,
    pub rx_data_free: list_head,
    pub rx_data_used: list_head,
    pub rx_data_free_cnt: core::sync::atomic::AtomicI32,
    pub rx_refill_work: work_struct,
    pub rxbufsz: c_uint,
    pub serial: [u8; 16],
    pub channels: [ieee80211_channel; 14],
    pub rates: [ieee80211_rate; 12],
    pub band: ieee80211_supported_band,
    pub vif: *mut ieee80211_vif,
}

// flags for sending firmware commands

// On USB hot-unplug there can be a lot of URBs in flight and they'll all
// fail. Instead of dealing with them in every possible place just surpress
// any messages on USB disconnect.
//

