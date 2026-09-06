//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/usb.h
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
// This file contains definitions for mwifiex USB interface driver.
//
// Copyright 2011-2020 NXP
//

pub const USB8XXX_VID: c_uint = 0x1286;
pub const USB8766_PID_1: c_uint = 0x2041;
pub const USB8766_PID_2: c_uint = 0x2042;
pub const USB8797_PID_1: c_uint = 0x2043;
pub const USB8797_PID_2: c_uint = 0x2044;
pub const USB8801_PID_1: c_uint = 0x2049;
pub const USB8801_PID_2: c_uint = 0x204a;
pub const USB8997_PID_1: c_uint = 0x2052;
pub const USB8997_PID_2: c_uint = 0x204e;
pub const USB8XXX_FW_DNLD: c_int = 1;
pub const USB8XXX_FW_READY: c_int = 2;
pub const USB8XXX_FW_MAX_RETRY: c_int = 3;
pub const MWIFIEX_TX_DATA_PORT: c_int = 2;
pub const MWIFIEX_TX_DATA_URB: c_int = 6;
pub const MWIFIEX_RX_DATA_URB: c_int = 6;
pub const MWIFIEX_USB_TIMEOUT: c_int = 100;

pub const FW_DNLD_TX_BUF_SIZE: c_int = 620;
pub const FW_DNLD_RX_BUF_SIZE: c_int = 2048;
pub const FW_HAS_LAST_BLOCK: c_uint = 0x00000004;
pub const FW_CMD_7: c_uint = 0x00000007;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb_context {
    pub adapter: *mut mwifiex_adapter,
    pub skb: *mut sk_buff,
    pub urb: *mut urb,
    pub ep: u8,
}

pub const MWIFIEX_USB_TX_AGGR_TMO_MIN: c_int = 1;
pub const MWIFIEX_USB_TX_AGGR_TMO_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_aggr_tmr_cnxt {
    pub adapter: *mut mwifiex_adapter,
    pub port: *mut usb_tx_data_port,
    pub hold_timer: timer_list,
    pub is_hold_timer_set: bool,
    pub hold_tmo_msecs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_tx_aggr {
    pub aggr_list: sk_buff_head,
    pub aggr_len: c_int,
    pub aggr_num: c_int,
    pub timer_cnxt: tx_aggr_tmr_cnxt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_tx_data_port {
    pub tx_data_ep: u8,
    pub block_status: u8,
    pub tx_data_urb_pending: core::sync::atomic::AtomicI32,
    pub tx_data_ix: c_int,
    pub tx_data_list: [urb_context; MWIFIEX_TX_DATA_URB],
// usb tx aggregation
    pub tx_aggr: usb_tx_aggr,
    pub skb_aggr: [*mut sk_buff; MWIFIEX_TX_DATA_URB],
// lock for protect tx aggregation data path
    pub tx_aggr_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_card_rec {
    pub adapter: *mut mwifiex_adapter,
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub fw_done: completion,
    pub rx_cmd_ep: u8,
    pub rx_cmd: urb_context,
    pub rx_cmd_urb_pending: core::sync::atomic::AtomicI32,
    pub rx_data_list: [urb_context; MWIFIEX_RX_DATA_URB],
    pub usb_boot_state: u8,
    pub rx_data_ep: u8,
    pub rx_data_urb_pending: core::sync::atomic::AtomicI32,
    pub tx_cmd_ep: u8,
    pub tx_cmd_urb_pending: core::sync::atomic::AtomicI32,
    pub bulk_out_maxpktsize: c_int,
    pub tx_cmd: urb_context,
    pub mc_resync_flag: u8,
    pub port: [usb_tx_data_port; MWIFIEX_TX_DATA_PORT],
    pub rx_cmd_ep_type: c_int,
    pub rx_cmd_interval: u8,
    pub tx_cmd_ep_type: c_int,
    pub tx_cmd_interval: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_header {
    pub dnld_cmd: __le32,
    pub base_addr: __le32,
    pub data_len: __le32,
    pub crc: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_sync_header {
    pub cmd: __le32,
    pub seq_num: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_data {
    pub fw_hdr: fw_header,
    pub seq_num: __le32,
    pub data: [u8; ],
    pub __packed: },
