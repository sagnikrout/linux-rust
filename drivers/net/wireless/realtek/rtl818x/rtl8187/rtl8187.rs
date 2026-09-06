//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtl818x/rtl8187/rtl8187.h
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
// Definitions for RTL8187 hardware
//
// Copyright 2007 Michael Wu <flamingice@sourmilk.net>
// Copyright 2007 Andrea Merello <andrea.merello@gmail.com>
//
// Based on the r8187 driver, which is:
// Copyright 2005 Andrea Merello <andrea.merello@gmail.com>, et al.
//

pub const RTL8187_EEPROM_TXPWR_BASE: c_uint = 0x05;
pub const RTL8187_EEPROM_MAC_ADDR: c_uint = 0x07;
pub const RTL8187_EEPROM_TXPWR_CHAN_1: c_uint = 0x16	/* 3 channels */;
pub const RTL8187_EEPROM_TXPWR_CHAN_6: c_uint = 0x1B	/* 2 channels */;
pub const RTL8187_EEPROM_TXPWR_CHAN_4: c_uint = 0x3D	/* 2 channels */;
pub const RTL8187_EEPROM_SELECT_GPIO: c_uint = 0x3B;
pub const RTL8187_REQT_READ: c_uint = 0xC0;
pub const RTL8187_REQT_WRITE: c_uint = 0x40;
pub const RTL8187_REQ_GET_REG: c_uint = 0x05;
pub const RTL8187_REQ_SET_REG: c_uint = 0x05;
pub const RTL8187_MAX_RX: c_uint = 0x9C4;
pub const RFKILL_MASK_8187_89_97: c_uint = 0x2;
pub const RFKILL_MASK_8198: c_uint = 0x4;
pub const RETRY_COUNT: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187_rx_info {
    pub urb: *mut urb,
    pub dev: *mut ieee80211_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187_rx_hdr {
    pub flags: __le32,
    pub noise: u8,
    pub signal: u8,
    pub agc: u8,
    pub reserved: u8,
    pub mac_time: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187b_rx_hdr {
    pub flags: __le32,
    pub mac_time: __le64,
    pub sq: u8,
    pub rssi: u8,
    pub agc: u8,
    pub flags2: u8,
    pub snr_long2end: __le16,
    pub pwdb_g12: i8,
    pub fot: u8,
    pub __packed: },
// {rtl8187,rtl8187b}_tx_info is in skb
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187_tx_hdr {
    pub flags: __le32,
    pub rts_duration: __le16,
    pub len: __le16,
    pub retry: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187b_tx_hdr {
    pub flags: __le32,
    pub rts_duration: __le16,
    pub len: __le16,
    pub unused_1: __le32,
    pub unused_2: __le16,
    pub tx_duration: __le16,
    pub unused_3: __le32,
    pub retry: __le32,
    pub unused_4: [__le32; 2],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187_vif {
    pub dev: *mut ieee80211_hw,
// beaconing
    pub beacon_work: delayed_work,
    pub enable_beacon: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187_priv {
// common between rtl818x drivers
    pub map: *mut rtl818x_csr,
    pub rf: *const rtl818x_rf_ops,
    pub vif: *mut ieee80211_vif,
// The mutex protects the TX loopback state.
// Any attempt to set channels concurrently locks the device.
//
    pub conf_mutex: mutex,
// rtl8187 specific
    pub channels: [ieee80211_channel; 14],
    pub rates: [ieee80211_rate; 12],
    pub band: ieee80211_supported_band,
    pub udev: *mut usb_device,
    pub rx_conf: u32,
    pub anchored: usb_anchor,
    pub work: delayed_work,
    pub dev: *mut ieee80211_hw,

    pub led_radio: rtl8187_led,
    pub led_tx: rtl8187_led,
    pub led_rx: rtl8187_led,
    pub led_on: delayed_work,
    pub led_off: delayed_work,

    pub txpwr_base: u16,
    pub asic_rev: u8,
    pub is_rtl8187b: u8,
    pub hw_rev: },
    pub rx_queue: sk_buff_head,
    pub signal: u8,
    pub noise: u8,
    pub slot_time: u8,
    pub aifsn: [u8; 4],
    pub rfkill_mask: u8,
    pub buf: __le64,
    pub dummy1: [u8; L1_CACHE_BYTES],
    pub ____cacheline_aligned: },
    pub queue: sk_buff_head,
    pub /: *mut *mut } b_tx_status; / This queue is used by both -b and non-b devices,
    pub io_mutex: mutex,
    pub bits8: u8,
    pub bits16: __le16,
    pub bits32: __le32,
    pub dummy2: [u8; L1_CACHE_BYTES],
    pub ____cacheline_aligned: *mut *mut } io_dmabuf,
    pub rfkill_off: bool,
    pub seqno: u16,
}

extern "C" {
    pub fn rtl8187_write_phy(dev: *mut ieee80211_hw, addr: u8, data: u32);
}
extern "C" {
    pub fn rtl818x_ioread8_idx(_arg: priv, _arg: addr, _arg: 0) -> return;
}
extern "C" {
    pub fn rtl818x_ioread16_idx(_arg: priv, _arg: addr, _arg: 0) -> return;
}
extern "C" {
    pub fn rtl818x_ioread32_idx(_arg: priv, _arg: addr, _arg: 0) -> return;
}
