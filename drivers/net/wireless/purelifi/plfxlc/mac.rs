//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/purelifi/plfxlc/mac.h
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

pub const PURELIFI_CCK: c_uint = 0x00;
pub const PURELIFI_OFDM: c_uint = 0x10;
pub const PURELIFI_CCK_PREA_SHORT: c_uint = 0x20;
pub const PURELIFI_OFDM_PLCP_RATE_6M: c_uint = 0xb;
pub const PURELIFI_OFDM_PLCP_RATE_9M: c_uint = 0xf;
pub const PURELIFI_OFDM_PLCP_RATE_12M: c_uint = 0xa;
pub const PURELIFI_OFDM_PLCP_RATE_18M: c_uint = 0xe;
pub const PURELIFI_OFDM_PLCP_RATE_24M: c_uint = 0x9;
pub const PURELIFI_OFDM_PLCP_RATE_36M: c_uint = 0xd;
pub const PURELIFI_OFDM_PLCP_RATE_48M: c_uint = 0x8;
pub const PURELIFI_OFDM_PLCP_RATE_54M: c_uint = 0xc;

pub const PURELIFI_RX_ERROR: c_uint = 0x80;
pub const PURELIFI_RX_CRC32_ERROR: c_uint = 0x10;
pub const PLF_REGDOMAIN_FCC: c_uint = 0x10;
pub const PLF_REGDOMAIN_IC: c_uint = 0x20;
pub const PLF_REGDOMAIN_ETSI: c_uint = 0x30;
pub const PLF_REGDOMAIN_SPAIN: c_uint = 0x31;
pub const PLF_REGDOMAIN_FRANCE: c_uint = 0x32;
pub const PLF_REGDOMAIN_JAPAN_2: c_uint = 0x40;
pub const PLF_REGDOMAIN_JAPAN: c_uint = 0x41;
pub const PLF_REGDOMAIN_JAPAN_3: c_uint = 0x49;
pub const PLF_RX_ERROR: c_uint = 0x80;
pub const PLF_RX_CRC32_ERROR: c_uint = 0x10;

pub const PURELIFI_MAC_STATS_BUFFER_SIZE: c_int = 16;
pub const PURELIFI_MAC_MAX_ACK_WAITERS: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_ctrlset {
// id should be plf_usb_req_enum
    pub id: __be32,
    pub len: __be32,
    pub modulation: u8,
    pub control: u8,
    pub service: u8,
    pub pad: u8,
    pub packet_length: __le16,
    pub current_length: __le16,
    pub next_frame_length: __le16,
    pub tx_length: __le16,
    pub payload_len_nw: __be32,
    pub __packed: },
// overlay
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_header {
    pub plf_ctrl: plfxlc_ctrlset,
    pub frametype: u32,
    pub dmac: *mut u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_status {
    pub type: u8,
    pub id: u8,
    pub rate: u8,
    pub pad: u8,
    pub mac: [u8; ETH_ALEN],
    pub retry: u8,
    pub failure: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct beacon {
    pub watchdog_work: delayed_work,
    pub cur_beacon: *mut sk_buff,
    pub last_update: c_ulong,
    pub interval: u16,
    pub period: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum plfxlc_device_flags {
    PURELIFI_DEVICE_RUNNING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plfxlc_mac {
    pub hw: *mut ieee80211_hw,
    pub vif: *mut ieee80211_vif,
    pub beacon: beacon,
    pub set_rts_cts_work: work_struct,
    pub process_intr: work_struct,
    pub multicast_hash: plfxlc_mc_hash,
    pub ack_wait_queue: sk_buff_head,
    pub channels: [ieee80211_channel; 14],
    pub rates: [ieee80211_rate; 12],
    pub band: ieee80211_supported_band,
    pub chip: plfxlc_chip,
    pub /: *mut *mut spinlock_t lock; / lock for mac data,
    pub intr_buffer: [u8; USB_MAX_EP_INT_BUFFER],
    pub serial_number: [c_char; PURELIFI_SERIAL_LEN],
    pub hw_address: [c_uchar; ETH_ALEN],
    pub default_regdomain: u8,
    pub flags: c_ulong,
    pub pass_failed_fcs: bool,
    pub pass_ctrl: bool,
    pub ack_pending: bool,
    pub ack_signal: c_int,
    pub associated: c_int,
    pub regdomain: u8,
    pub channel: u8,
    pub type: c_int,
    pub crc_errors: u64,
    pub rssi: u64,
}

extern "C" {
    pub fn container_of(_arg: chip, plfxlc_mac: struct, _arg: chip) -> return;
}
extern "C" {
    pub fn plfxlc_chip_to_mac(_arg: plfxlc_usb_to_chip(usb)) -> return;
}
extern "C" {
    pub fn plfxlc_mac_release_hw(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn plfxlc_mac_preinit_hw(hw: *mut ieee80211_hw, hw_address: *const u8) -> c_int;
}
extern "C" {
    pub fn plfxlc_mac_init_hw(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn plfxlc_mac_tx_failed(urb: *mut urb);
}
extern "C" {
    pub fn plfxlc_mac_tx_to_dev(skb: *mut sk_buff, error: c_int);
}
extern "C" {
    pub fn plfxlc_op_start(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn plfxlc_op_stop(hw: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn plfxlc_restore_settings(mac: *mut plfxlc_mac) -> c_int;
}
