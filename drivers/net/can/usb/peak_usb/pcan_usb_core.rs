//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/usb/peak_usb/pcan_usb_core.h
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
// CAN driver for PEAK System USB adapters
// Derived from the PCAN project file driver/src/pcan_usb_core.c
//
// Copyright (C) 2003-2025 PEAK System-Technik GmbH
// Author: Stéphane Grosjean <s.grosjean@peak-system.fr>
//
// Many thanks to Klaus Hitschler <klaus.hitschler@gmx.de>
//
// PEAK-System vendor id.
pub const PCAN_USB_VENDOR_ID: c_uint = 0x0c72;
// supported device ids.
pub const PCAN_USB_PRODUCT_ID: c_uint = 0x000c;
pub const PCAN_USBPRO_PRODUCT_ID: c_uint = 0x000d;
pub const PCAN_USBPROFD_PRODUCT_ID: c_uint = 0x0011;
pub const PCAN_USBFD_PRODUCT_ID: c_uint = 0x0012;
pub const PCAN_USBCHIP_PRODUCT_ID: c_uint = 0x0013;
pub const PCAN_USBX6_PRODUCT_ID: c_uint = 0x0014;

// number of urbs that are submitted for rx/tx per channel
pub const PCAN_USB_MAX_RX_URBS: c_int = 4;
pub const PCAN_USB_MAX_TX_URBS: c_int = 10;
// usb adapters maximum channels per usb interface
pub const PCAN_USB_MAX_CHANNEL: c_int = 2;
// maximum length of the usb commands sent to/received from the devices
pub const PCAN_USB_MAX_CMD_LEN: c_int = 32;
// PEAK-System USB adapter descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peak_usb_adapter {
    pub name: *mut c_char,
    pub device_id: u32,
    pub ctrlmode_supported: u32,
    pub clock: can_clock,
    pub bittiming_const: *const *const can_bittiming_,
    pub data_bittiming_const: *const *const can_bittiming_,
    pub ctrl_count: c_uint,
    pub ethtool_ops: *const ethtool_ops,
    pub intf): *mut *mut int (intf_probe)(struct usb_interface,
    pub dev): *mut *mut int (dev_init)(struct peak_usb_device,
    pub dev): *mut *mut void (dev_exit)(struct peak_usb_device,
    pub dev): *mut *mut void (dev_free)(struct peak_usb_device,
    pub dev): *mut *mut int (dev_open)(struct peak_usb_device,
    pub dev): *mut *mut int (dev_close)(struct peak_usb_device,
    pub bt): *mut can_bittiming,
    pub bt): *mut can_bittiming,
    pub onoff): *mut *mut *mut int (dev_set_bus)(struct peak_usb_device dev, u8,
    pub can_ch_id): *mut *mut *mut int (dev_get_can_channel_id)(struct peak_usb_device dev, u32,
    pub can_ch_id): *mut *mut *mut int (dev_set_can_channel_id)(struct peak_usb_device dev, u32,
    pub urb): *mut *mut *mut int (dev_decode_buf)(struct peak_usb_device dev, struct urb,
    pub size): *mut *mut u8 obuf, size_t,
    pub dev): *mut *mut int (dev_start)(struct peak_usb_device,
    pub dev): *mut *mut int (dev_stop)(struct peak_usb_device,
    pub buf): *mut u8,
    pub bec): *mut can_berr_counter,
    pub ep_msg_in: u8,
    pub ep_msg_out: [u8; PCAN_USB_MAX_CHANNEL],
    pub ts_used_bits: u8,
    pub us_per_ts_shift: u8,
    pub us_per_ts_scale: u32,
    pub rx_buffer_size: c_int,
    pub tx_buffer_size: c_int,
    pub sizeof_dev_private: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct peak_time_ref {
    pub tv_host: ktime_t tv_host_0,,
    pub ts_dev_2: u32 ts_dev_1,,
    pub ts_total: u64,
    pub tick_count: u32,
    pub adapter: *const peak_usb_adapter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct peak_tx_urb_context {
    pub dev: *mut peak_usb_device,
    pub echo_index: u32,
    pub urb: *mut urb,
}

pub const PCAN_USB_STATE_CONNECTED: c_uint = 0x00000001;
pub const PCAN_USB_STATE_STARTED: c_uint = 0x00000002;
// PEAK-System USB device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peak_usb_device {
    pub can: can_priv,
    pub adapter: *const peak_usb_adapter,
    pub ctrl_idx: c_uint,
    pub state: u32,
    pub udev: *mut usb_device,
    pub netdev: *mut net_device,
    pub active_tx_urbs: core::sync::atomic::AtomicI32,
    pub tx_submitted: usb_anchor,
    pub tx_contexts: [peak_tx_urb_context; PCAN_USB_MAX_TX_URBS],
    pub cmd_buf: *mut u8,
    pub rx_submitted: usb_anchor,
// equivalent to the device ID in the Windows API
    pub can_channel_id: u32,
    pub device_rev: u8,
    pub ep_msg_in: u8,
    pub ep_msg_out: u8,
    pub prev_siblings: *mut peak_usb_device,
    pub next_siblings: *mut peak_usb_device,
}

extern "C" {
    pub fn pcan_dump_mem(prompt: *const c_char, p: *const c_void, l: c_int);
}
// common timestamp management
extern "C" {
    pub fn peak_usb_update_ts_now(time_ref: *mut peak_time_ref, ts_now: u32);
}
extern "C" {
    pub fn peak_usb_set_ts_now(time_ref: *mut peak_time_ref, ts_now: u32);
}
extern "C" {
    pub fn peak_usb_get_ts_time(time_ref: *mut peak_time_ref, ts: u32, tv: *mut ktime_t);
}
extern "C" {
    pub fn peak_usb_netif_rx_64(skb: *mut sk_buff, ts_low: u32, ts_high: u32) -> c_int;
}
extern "C" {
    pub fn peak_usb_async_complete(urb: *mut urb);
}
extern "C" {
    pub fn peak_usb_restart_complete(dev: *mut peak_usb_device);
}
extern "C" {
    pub fn pcan_get_ts_info(dev: *mut net_device, info: *mut kernel_ethtool_ts_info) -> c_int;
}
// common 32-bit CAN channel ID ethtool management
extern "C" {
    pub fn peak_usb_get_eeprom_len(netdev: *mut net_device) -> c_int;
}
