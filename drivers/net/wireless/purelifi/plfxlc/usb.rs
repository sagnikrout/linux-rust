//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/purelifi/plfxlc/usb.h
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

pub const USB_BULK_MSG_TIMEOUT_MS: c_int = 2000;
pub const PURELIFI_X_VENDOR_ID_0: c_uint = 0x16C1;
pub const PURELIFI_X_PRODUCT_ID_0: c_uint = 0x1CDE;
pub const PURELIFI_XC_VENDOR_ID_0: c_uint = 0x2EF5;
pub const PURELIFI_XC_PRODUCT_ID_0: c_uint = 0x0008;
pub const PURELIFI_XL_VENDOR_ID_0: c_uint = 0x2EF5;
pub const PURELIFI_XL_PRODUCT_ID_0: c_uint = 0x000A /* Station */;
pub const PLF_FPGA_STATUS_LEN: c_int = 2;
pub const PLF_FPGA_STATE_LEN: c_int = 9;
pub const PLF_BULK_TLEN: c_int = 16384;

pub const PLF_XL_BUF_LEN: c_int = 64;
pub const PLF_MSG_STATUS_OFFSET: c_int = 7;
pub const PLF_USB_TIMEOUT: c_int = 1000;
pub const PLF_MSLEEP_TIME: c_int = 200;
pub const PURELIFI_URB_RETRY_MAX: c_int = 5;

// Tx retry backoff timer (in milliseconds)
pub const TX_RETRY_BACKOFF_MS: c_int = 10;
pub const STA_QUEUE_CLEANUP_MS: c_int = 5000;
// Tx retry backoff timer (in jiffies)

// Ensures that MAX_TRANSFER_SIZE is even.

pub const STATION_FIFO_ALMOST_FULL_MESSAGE: c_int = 0;
pub const STATION_FIFO_ALMOST_FULL_NOT_MESSAGE: c_int = 1;
pub const STATION_CONNECT_MESSAGE: c_int = 2;
pub const STATION_DISCONNECT_MESSAGE: c_int = 3;
extern "C" {
    pub fn plfxlc_tx_urb_complete(urb: *mut urb);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_usb_interrupt {
    pub /: *mut *mut spinlock_t lock; / spin lock for usb interrupt buffer,
    pub urb: *mut urb,
    pub buffer: *mut c_void,
    pub interval: c_int,
}

pub const RX_URBS_COUNT: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_usb_rx {
    pub /: *mut *mut spinlock_t lock; / spin lock for rx urb,
    pub /: *mut *mut mutex setup_mutex; / mutex lockt for rx urb,
    pub USB_MAX_RX_SIZE]: *mut *mut u8 fragment[2,
    pub fragment_length: c_uint,
    pub usb_packet_size: c_uint,
    pub urbs: *mut urb,
    pub urbs_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plf_station {
// 7...3    |    2      |     1     |     0	    |
// Reserved  | Heartbeat | FIFO full | Connected |
//
    pub flag: c_uchar,
    pub mac: [c_uchar; ETH_ALEN],
    pub data_list: sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_firmware_file {
    pub total_files: u32,
    pub total_size: u32,
    pub size: u32,
    pub start_addr: u32,
    pub control_packets: u32,
    pub __packed: },
pub const STATION_CONNECTED_FLAG: c_uint = 0x1;
pub const STATION_FIFO_FULL_FLAG: c_uint = 0x2;
pub const STATION_HEARTBEAT_FLAG: c_uint = 0x4;
pub const STATION_ACTIVE_FLAG: c_uint = 0xFD;
pub const PURELIFI_SERIAL_LEN: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_usb_tx {
    pub enabled: c_ulong,
    pub /: *mut *mut spinlock_t lock; / spinlock for USB tx,
    pub mac_fifo_full: u8,
    pub submitted_skbs: sk_buff_head,
    pub submitted: usb_anchor,
    pub submitted_urbs: c_int,
    pub stopped: bool,
    pub tx_retry_timer: timer_list,
    pub station: [plf_station; MAX_STA_NUM],
}

// Contains the usb parts. The structure doesn't require a lock because intf
// will not be changed after initialization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_usb {
    pub sta_queue_cleanup: timer_list,
    pub rx: plfxlc_usb_rx,
    pub tx: plfxlc_usb_tx,
    pub intf: *mut usb_interface,
    pub ez_usb: *mut usb_interface,
    pub /: *mut *mut u8 req_buf[64]; / plfxlc_usb_iowrite16v needs 62 bytes,
    pub /: *mut *mut u8 sidx; / store last served,
    pub rx_usb_enabled: bool,
    pub initialized: bool,
    pub was_running: bool,
    pub link_up: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum endpoints {
    EP_DATA_IN  = 2,
    EP_DATA_OUT = 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devicetype {
    DEVICE_LIFI_X  = 0,
    DEVICE_LIFI_XC  = 1,
    DEVICE_LIFI_XL  = 1,
}

extern "C" {
    pub fn interface_to_usbdev(_arg: usb->intf) -> return;
}
extern "C" {
    pub fn usb_get_intfdata(_arg: intf) -> return;
}
extern "C" {
    pub fn plfxlc_intf_to_hw(_arg: usb->intf) -> return;
}
extern "C" {
    pub fn plfxlc_send_packet_from_data_queue(usb: *mut plfxlc_usb);
}
extern "C" {
    pub fn plfxlc_usb_release(usb: *mut plfxlc_usb);
}
extern "C" {
    pub fn plfxlc_usb_disable_rx(usb: *mut plfxlc_usb);
}
extern "C" {
    pub fn plfxlc_usb_enable_tx(usb: *mut plfxlc_usb);
}
extern "C" {
    pub fn plfxlc_usb_disable_tx(usb: *mut plfxlc_usb);
}
extern "C" {
    pub fn plfxlc_usb_tx(usb: *mut plfxlc_usb, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn plfxlc_usb_enable_rx(usb: *mut plfxlc_usb) -> c_int;
}
extern "C" {
    pub fn plfxlc_usb_init_hw(usb: *mut plfxlc_usb) -> c_int;
}
// Firmware declarations
extern "C" {
    pub fn plfxlc_download_xl_firmware(intf: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn plfxlc_download_fpga(intf: *mut usb_interface) -> c_int;
}
