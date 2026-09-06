//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/usb.h
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
// Copyright(c) 2009-2012  Realtek Corporation.

pub const RTL_RX_DESC_SIZE: c_int = 24;

pub const USB_HIGH_SPEED_BULK_SIZE: c_int = 512;
pub const USB_FULL_SPEED_BULK_SIZE: c_int = 64;

pub const RTL_USB_MAX_BULKOUT_NUM: c_int = 4;
pub const RTL_USB_MAX_TX_URBS_NUM: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_txq {
// These definitions shall be consistent with value
// returned by skb_get_queue_mapping
// ------------------------------------
    RTL_TXQ_BK,
    RTL_TXQ_BE,
    RTL_TXQ_VI,
    RTL_TXQ_VO,
// ------------------------------------
    RTL_TXQ_BCN,
    RTL_TXQ_MGT,
    RTL_TXQ_HI,

// Must be last
    __RTL_TXQ_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_ep_map {
    pub ep_mapping: [u32; __RTL_TXQ_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _trx_info {
    pub rtlusb: *mut rtl_usb,
    pub ep_num: u32,
}

// Add suspend/resume later
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl_usb_state {
    USB_STATE_STOP	= 0,
    USB_STATE_START	= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_usb {
    pub udev: *mut usb_device,
    pub intf: *mut usb_interface,
    pub state: rtl_usb_state,
// Bcn control register setting
    pub reg_bcn_ctrl_val: u32,
// for 88/92cu card disable
    pub disablehwsm: u8,
// QOS & EDCA
    pub acm_method: acm_method,
// irq  . HIMR,HIMR_EX
    pub irq_mask: [u32; 2],
    pub irq_enabled: bool,
    pub mac80211_queue_index): *mut *mut u16 (usb_mq_to_hwq)(__le16 fc, u16,
// Tx
    pub out_ep_nums: u8,
    pub out_eps: [u8; RTL_USB_MAX_BULKOUT_NUM],
    pub out_queue_sel: u8,
    pub ep_map: rtl_ep_map,
    pub max_bulk_out_size: u32,
    pub tx_submitted_urbs: u32,
    pub tx_skb_queue: [sk_buff_head; RTL_USB_MAX_EP_NUM],
    pub tx_pending: [usb_anchor; RTL_USB_MAX_EP_NUM],
    pub tx_submitted: usb_anchor,
    pub ): *mut sk_buff_head,
    pub ): *mut *mut urb , sk_buff,
    pub ): *mut *mut *mut void (usb_tx_cleanup)(struct ieee80211_hw , struct sk_buff,
// Rx
    pub in_ep_nums: u8,
    pub /: *mut *mut u32 in_ep; / Bulk IN endpoint number,
    pub /: *mut *mut u32 rx_max_size; / Bulk IN max buffer size,
    pub /: *mut *mut u32 rx_urb_num; / How many Bulk INs are submitted to host.,
    pub rx_submitted: usb_anchor,
    pub rx_cleanup_urbs: usb_anchor,
    pub rx_work_tasklet: tasklet_struct,
    pub rx_queue: sk_buff_head,
    pub ): *mut sk_buff_head,
    pub ): *mut *mut *mut void (usb_rx_hdl)(struct ieee80211_hw , struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl_usb_priv {
    pub bt_coexist: bt_coexist_info,
    pub dev: rtl_usb,
}

extern "C" {
    pub fn rtl_usb_disconnect(intf: *mut usb_interface);
}
