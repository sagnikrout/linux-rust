//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/zydas/zd1211rw/zd_mac.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// ZD1211 USB-WLAN driver for Linux
//
// Copyright (C) 2005-2007 Ulrich Kunitz <kune@deine-taler.de>
// Copyright (C) 2006-2007 Daniel Drake <dsd@gentoo.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_ctrlset {
    pub modulation: u8,
    pub tx_length: __le16,
    pub control: u8,
// stores only the difference to tx_length on ZD1211B
    pub packet_length: __le16,
    pub current_length: __le16,
    pub service: u8,
    pub next_frame_length: __le16,
    pub __packed: },
pub const ZD_CS_RESERVED_SIZE: c_int = 25;
// The field modulation of struct zd_ctrlset controls the bit rate, the use
// of short or long preambles in 802.11b (CCK mode) or the use of 802.11a or
// 802.11g in OFDM mode.
//
// The term zd-rate is used for the combination of the modulation type flag
// and the "pure" rate value.
//
pub const ZD_PURE_RATE_MASK: c_uint = 0x0f;
pub const ZD_MODULATION_TYPE_MASK: c_uint = 0x10;

// The two possible modulation types. Notify that 802.11b doesn't use the CCK
// codeing for the 1 and 2 MBit/s rate. We stay with the term here to remain
// consistent with uses the term at other places.
//
pub const ZD_CCK: c_uint = 0x00;
pub const ZD_OFDM: c_uint = 0x10;
// The ZD1211 firmware uses proprietary encodings of the 802.11b (CCK) rates.
// For OFDM the PLCP rate encodings are used. We combine these "pure" rates
// with the modulation type flag and call the resulting values zd-rates.
//

// The bit 5 of the zd_ctrlset modulation field controls the preamble in CCK
// mode or the 802.11a/802.11g selection in OFDM mode.
//
pub const ZD_CCK_PREA_LONG: c_uint = 0x00;
pub const ZD_CCK_PREA_SHORT: c_uint = 0x20;
pub const ZD_OFDM_MODE_11G: c_uint = 0x00;
pub const ZD_OFDM_MODE_11A: c_uint = 0x20;
// zd_ctrlset control field
pub const ZD_CS_NEED_RANDOM_BACKOFF: c_uint = 0x01;
pub const ZD_CS_NO_ACK: c_uint = 0x02;
pub const ZD_CS_FRAME_TYPE_MASK: c_uint = 0x0c;
pub const ZD_CS_DATA_FRAME: c_uint = 0x00;
pub const ZD_CS_PS_POLL_FRAME: c_uint = 0x04;
pub const ZD_CS_MANAGEMENT_FRAME: c_uint = 0x08;
pub const ZD_CS_NO_SEQUENCE_CTL_FRAME: c_uint = 0x0c;
pub const ZD_CS_WAKE_DESTINATION: c_uint = 0x10;
pub const ZD_CS_RTS: c_uint = 0x20;
pub const ZD_CS_ENCRYPT: c_uint = 0x40;
pub const ZD_CS_SELF_CTS: c_uint = 0x80;
// Incoming frames are prepended by a PLCP header
pub const ZD_PLCP_HEADER_SIZE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_length_info {
    pub length: [__le16; 3],
    pub tag: __le16,
    pub __packed: },
pub const RX_LENGTH_INFO_TAG: c_uint = 0x697e;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_status {
    pub signal_quality_cck: u8,
// rssi
    pub signal_strength: u8,
    pub signal_quality_ofdm: u8,
    pub decryption_type: u8,
    pub frame_status: u8,
    pub __packed: },
// rx_status field decryption_type
pub const ZD_RX_NO_WEP: c_int = 0;
pub const ZD_RX_WEP64: c_int = 1;
pub const ZD_RX_TKIP: c_int = 2;
pub const ZD_RX_AES: c_int = 4;
pub const ZD_RX_WEP128: c_int = 5;
pub const ZD_RX_WEP256: c_int = 6;
// rx_status field frame_status
pub const ZD_RX_FRAME_MODULATION_MASK: c_uint = 0x01;
pub const ZD_RX_CCK: c_uint = 0x00;
pub const ZD_RX_OFDM: c_uint = 0x01;
pub const ZD_RX_TIMEOUT_ERROR: c_uint = 0x02;
pub const ZD_RX_FIFO_OVERRUN_ERROR: c_uint = 0x04;
pub const ZD_RX_DECRYPTION_ERROR: c_uint = 0x08;
pub const ZD_RX_CRC32_ERROR: c_uint = 0x10;
pub const ZD_RX_NO_ADDR1_MATCH_ERROR: c_uint = 0x20;
pub const ZD_RX_CRC16_ERROR: c_uint = 0x40;
pub const ZD_RX_ERROR: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_retry_rate {
    pub /: *mut *mut int count; / number of valid element in rate[] array,
    pub /: *mut *mut int rate[10]; / retry rates, described by an index in zd_rates[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_status {
    pub /: *mut *mut u8 type; / must always be 0x01 : USB_INT_TYPE,
    pub /: *mut *mut u8 id; / must always be 0xa0 : USB_INT_ID_RETRY_FAILED,
    pub rate: u8,
    pub pad: u8,
    pub mac: [u8; ETH_ALEN],
    pub retry: u8,
    pub failure: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_flags {
    MAC_FIXED_CHANNEL = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct housekeeping {
    pub link_led_work: delayed_work,
}

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
pub enum zd_device_flags {
    ZD_DEVICE_RUNNING,
}

pub const ZD_MAC_STATS_BUFFER_SIZE: c_int = 16;
pub const ZD_MAC_MAX_ACK_WAITERS: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zd_mac {
    pub chip: zd_chip,
    pub lock: spinlock_t,
    pub intr_lock: spinlock_t,
    pub hw: *mut ieee80211_hw,
    pub vif: *mut ieee80211_vif,
    pub housekeeping: housekeeping,
    pub beacon: beacon,
    pub set_rts_cts_work: work_struct,
    pub process_intr: work_struct,
    pub multicast_hash: zd_mc_hash,
    pub intr_buffer: [u8; USB_MAX_EP_INT_BUFFER],
    pub regdomain: u8,
    pub default_regdomain: u8,
    pub channel: u8,
    pub type: c_int,
    pub associated: c_int,
    pub flags: c_ulong,
    pub ack_wait_queue: sk_buff_head,
    pub channels: [ieee80211_channel; 14],
    pub rates: [ieee80211_rate; 12],
    pub band: ieee80211_supported_band,
// Short preamble (used for RTS/CTS)
    pub short_preamble:1: c_uint,
// whether to pass frames with CRC errors to stack
    pub pass_failed_fcs:1: c_uint,
// whether to pass control frames to stack
    pub pass_ctrl:1: c_uint,
// whether we have received a 802.11 ACK that is pending
    pub ack_pending:1: c_uint,
// signal strength of the last 802.11 ACK received
    pub ack_signal: c_int,
}

pub const ZD_REGDOMAIN_FCC: c_uint = 0x10;
pub const ZD_REGDOMAIN_IC: c_uint = 0x20;
pub const ZD_REGDOMAIN_ETSI: c_uint = 0x30;
pub const ZD_REGDOMAIN_SPAIN: c_uint = 0x31;
pub const ZD_REGDOMAIN_FRANCE: c_uint = 0x32;
pub const ZD_REGDOMAIN_JAPAN_2: c_uint = 0x40;
pub const ZD_REGDOMAIN_JAPAN: c_uint = 0x41;
pub const ZD_REGDOMAIN_JAPAN_3: c_uint = 0x49;
pub const ZD_PLCP_SERVICE_LENGTH_EXTENSION: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ofdm_plcp_header {
    pub prefix: [u8; 3],
    pub service: __le16,
    pub __packed: },
    pub 0xf: return header->prefix[0] &,
// The following defines give the encoding of the 4-bit rate field in the
// OFDM (802.11a/802.11g) PLCP header. Notify that these values are used to
// define the zd-rate values for OFDM.
//
// See the struct zd_ctrlset definition in zd_mac.h.
//
pub const ZD_OFDM_PLCP_RATE_6M: c_uint = 0xb;
pub const ZD_OFDM_PLCP_RATE_9M: c_uint = 0xf;
pub const ZD_OFDM_PLCP_RATE_12M: c_uint = 0xa;
pub const ZD_OFDM_PLCP_RATE_18M: c_uint = 0xe;
pub const ZD_OFDM_PLCP_RATE_24M: c_uint = 0x9;
pub const ZD_OFDM_PLCP_RATE_36M: c_uint = 0xd;
pub const ZD_OFDM_PLCP_RATE_48M: c_uint = 0x8;
pub const ZD_OFDM_PLCP_RATE_54M: c_uint = 0xc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cck_plcp_header {
    pub signal: u8,
    pub service: u8,
    pub length: __le16,
    pub crc16: __le16,
    pub __packed: },
    pub header->signal: return,
// These defines give the encodings of the signal field in the 802.11b PLCP
// header. The signal field gives the bit rate of the following packet. Even
// if technically wrong we use CCK here also for the 1 MBit/s and 2 MBit/s
// rate to stay consistent with Zydas and our use of the term.
//
// Notify that these values are *not* used in the zd-rates.
//
pub const ZD_CCK_PLCP_SIGNAL_1M: c_uint = 0x0a;
pub const ZD_CCK_PLCP_SIGNAL_2M: c_uint = 0x14;
pub const ZD_CCK_PLCP_SIGNAL_5M5: c_uint = 0x37;
pub const ZD_CCK_PLCP_SIGNAL_11M: c_uint = 0x6e;
    pub hw->priv: return,
    pub chip): return container_of(chip, struct zd_mac,,
    pub zd_chip_to_mac(zd_usb_to_chip(usb)): return,
    pub mac->hw->wiphy->perm_addr: return,

    pub intf): *mut *mut ieee80211_hw zd_mac_alloc_hw(usb_interface,
    pub mac): *mut void zd_mac_clear(struct zd_mac,
    pub hw): *mut int zd_mac_preinit_hw(struct ieee80211_hw,
    pub hw): *mut int zd_mac_init_hw(struct ieee80211_hw,
    pub length): *const *const *const int zd_mac_rx(struct ieee80211_hw hw, u8 buffer, unsigned int,
    pub urb): *mut void zd_mac_tx_failed(struct urb,
    pub error): *mut *mut void zd_mac_tx_to_dev(struct sk_buff skb, int,
    pub hw): *mut int zd_op_start(struct ieee80211_hw,
    pub suspend): *mut *mut void zd_op_stop(struct ieee80211_hw hw, bool,
    pub mac): *mut int zd_restore_settings(struct zd_mac,

    pub status): *const void zd_dump_rx_status(struct rx_status,

// Macro flag: #define zd_dump_rx_status(status)

