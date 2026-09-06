//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ar5523/ar5523_hw.h
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
// all fields are big endian
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_fwblock {
    pub flags: __be32,

    pub len: __be32,
pub const AR5523_MAX_FWBLOCK_SIZE: c_int = 2048;
    pub total: __be32,
    pub remain: __be32,
    pub rxtotal: __be32,
    pub pad: [__be32; 123],
    pub __packed: },
pub const AR5523_MAX_RXCMDSZ: c_int = 1024;
pub const AR5523_MAX_TXCMDSZ: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_hdr {
    pub len: __be32,
    pub code: __be32,
// NB: these are defined for rev 1.5 firmware; rev 1.6 is different
// messages from Host -> Target
pub const WDCMSG_HOST_AVAILABLE: c_uint = 0x01;
pub const WDCMSG_BIND: c_uint = 0x02;
pub const WDCMSG_TARGET_RESET: c_uint = 0x03;
pub const WDCMSG_TARGET_GET_CAPABILITY: c_uint = 0x04;
pub const WDCMSG_TARGET_SET_CONFIG: c_uint = 0x05;
pub const WDCMSG_TARGET_GET_STATUS: c_uint = 0x06;
pub const WDCMSG_TARGET_GET_STATS: c_uint = 0x07;
pub const WDCMSG_TARGET_START: c_uint = 0x08;
pub const WDCMSG_TARGET_STOP: c_uint = 0x09;
pub const WDCMSG_TARGET_ENABLE: c_uint = 0x0a;
pub const WDCMSG_TARGET_DISABLE: c_uint = 0x0b;
pub const WDCMSG_CREATE_CONNECTION: c_uint = 0x0c;
pub const WDCMSG_UPDATE_CONNECT_ATTR: c_uint = 0x0d;
pub const WDCMSG_DELETE_CONNECT: c_uint = 0x0e;
pub const WDCMSG_SEND: c_uint = 0x0f;
pub const WDCMSG_FLUSH: c_uint = 0x10;
// messages from Target -> Host
pub const WDCMSG_STATS_UPDATE: c_uint = 0x11;
pub const WDCMSG_BMISS: c_uint = 0x12;
pub const WDCMSG_DEVICE_AVAIL: c_uint = 0x13;
pub const WDCMSG_SEND_COMPLETE: c_uint = 0x14;
pub const WDCMSG_DATA_AVAIL: c_uint = 0x15;
pub const WDCMSG_SET_PWR_MODE: c_uint = 0x16;
pub const WDCMSG_BMISS_ACK: c_uint = 0x17;
pub const WDCMSG_SET_LED_STEADY: c_uint = 0x18;
pub const WDCMSG_SET_LED_BLINK: c_uint = 0x19;
// more messages
pub const WDCMSG_SETUP_BEACON_DESC: c_uint = 0x1a;
pub const WDCMSG_BEACON_INIT: c_uint = 0x1b;
pub const WDCMSG_RESET_KEY_CACHE: c_uint = 0x1c;
pub const WDCMSG_RESET_KEY_CACHE_ENTRY: c_uint = 0x1d;
pub const WDCMSG_SET_KEY_CACHE_ENTRY: c_uint = 0x1e;
pub const WDCMSG_SET_DECOMP_MASK: c_uint = 0x1f;
pub const WDCMSG_SET_REGULATORY_DOMAIN: c_uint = 0x20;
pub const WDCMSG_SET_LED_STATE: c_uint = 0x21;
pub const WDCMSG_WRITE_ASSOCID: c_uint = 0x22;
pub const WDCMSG_SET_STA_BEACON_TIMERS: c_uint = 0x23;
pub const WDCMSG_GET_TSF: c_uint = 0x24;
pub const WDCMSG_RESET_TSF: c_uint = 0x25;
pub const WDCMSG_SET_ADHOC_MODE: c_uint = 0x26;
pub const WDCMSG_SET_BASIC_RATE: c_uint = 0x27;
pub const WDCMSG_MIB_CONTROL: c_uint = 0x28;
pub const WDCMSG_GET_CHANNEL_DATA: c_uint = 0x29;
pub const WDCMSG_GET_CUR_RSSI: c_uint = 0x2a;
pub const WDCMSG_SET_ANTENNA_SWITCH: c_uint = 0x2b;
pub const WDCMSG_USE_SHORT_SLOT_TIME: c_uint = 0x2f;
pub const WDCMSG_SET_POWER_MODE: c_uint = 0x30;
pub const WDCMSG_SETUP_PSPOLL_DESC: c_uint = 0x31;
pub const WDCMSG_SET_RX_MULTICAST_FILTER: c_uint = 0x32;
pub const WDCMSG_RX_FILTER: c_uint = 0x33;
pub const WDCMSG_PER_CALIBRATION: c_uint = 0x34;
pub const WDCMSG_RESET: c_uint = 0x35;
pub const WDCMSG_DISABLE: c_uint = 0x36;
pub const WDCMSG_PHY_DISABLE: c_uint = 0x37;
pub const WDCMSG_SET_TX_POWER_LIMIT: c_uint = 0x38;
pub const WDCMSG_SET_TX_QUEUE_PARAMS: c_uint = 0x39;
pub const WDCMSG_SETUP_TX_QUEUE: c_uint = 0x3a;
pub const WDCMSG_RELEASE_TX_QUEUE: c_uint = 0x3b;
pub const WDCMSG_SET_DEFAULT_KEY: c_uint = 0x43;
    pub data,: *mut *mut __u32 priv; / driver private,
    pub magic: __be32,
    pub reserved2: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_host_available {
    pub sw_ver_major: __be32,
    pub sw_ver_minor: __be32,
    pub sw_ver_patch: __be32,
    pub sw_ver_build: __be32,
    pub __packed: },
pub const ATH_SW_VER_MAJOR: c_int = 1;
pub const ATH_SW_VER_MINOR: c_int = 5;
pub const ATH_SW_VER_PATCH: c_int = 0;
pub const ATH_SW_VER_BUILD: c_int = 9999;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_chunk {
    pub /: *mut *mut u8 seqnum; / sequence number for ordering,
    pub flags: u8,
pub const UATH_CFLAGS_FINAL: c_uint = 0x01	/* final chunk of a msg */;
pub const UATH_CFLAGS_RXMSG: c_uint = 0x02	/* chunk contains rx completion */;
pub const UATH_CFLAGS_DEBUG: c_uint = 0x04	/* for debugging */;
    pub /: *mut *mut __be16 length; / chunk size in bytes,
// chunk data follows
    pub __packed: },
//
// Message format for a WDCMSG_DATA_AVAIL message from Target to Host.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_rx_desc {
    pub /: *mut *mut __be32 len; / msg length including header,
    pub /: *mut *mut __be32 code; / WDCMSG_DATA_AVAIL,
    pub /: *mut *mut __be32 gennum; / generation number,
    pub /: *mut *mut __be32 status; / start of RECEIVE_INFO,
pub const UATH_STATUS_OK: c_int = 0;
pub const UATH_STATUS_STOP_IN_PROGRESS: c_int = 1;
pub const UATH_STATUS_CRC_ERR: c_int = 2;
pub const UATH_STATUS_PHY_ERR: c_int = 3;
pub const UATH_STATUS_DECRYPT_CRC_ERR: c_int = 4;
pub const UATH_STATUS_DECRYPT_MIC_ERR: c_int = 5;
pub const UATH_STATUS_DECOMP_ERR: c_int = 6;
pub const UATH_STATUS_KEY_ERR: c_int = 7;
pub const UATH_STATUS_ERR: c_int = 8;
    pub /: *mut *mut __be32 tstamp_low; / low-order 32-bits of rx timestamp,
    pub /: *mut *mut __be32 tstamp_high; / high-order 32-bits of rx timestamp,
    pub /: *mut *mut __be32 framelen; / frame length,
    pub /: *mut *mut __be32 rate; / rx rate code,
    pub antenna: __be32,
    pub rssi: __be32,
    pub channel: __be32,
    pub phyerror: __be32,
    pub /: *mut *mut __be32 connix; / key table ix for bss traffic,
    pub decrypterror: __be32,
    pub keycachemiss: __be32,
    pub /: *mut *mut __be32 pad; / XXX?,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_tx_desc {
    pub msglen: __be32,
    pub /: *mut *mut u32 msgid; / msg id (supplied by host),
    pub /: *mut *mut __be32 type; / opcode: WDMSG_SEND or WDCMSG_FLUSH,
    pub /: *mut *mut __be32 txqid; / tx queue id and flags,
pub const UATH_TXQID_MASK: c_uint = 0x0f;
pub const UATH_TXQID_MINRATE: c_uint = 0x10	/* use min tx rate */;
pub const UATH_TXQID_FF: c_uint = 0x20	/* content is fast frame */;
    pub /: *mut *mut __be32 connid; / tx connection id,
pub const UATH_ID_INVALID: c_uint = 0xffffffff	/* for sending prior to connection */;
    pub /: *mut *mut __be32 flags; / non-zero if response desired,

    pub /: *mut *mut __be32 buflen; / payload length,
    pub __packed: },
pub const AR5523_ID_BSS: c_int = 2;
pub const AR5523_ID_BROADCAST: c_uint = 0xffffffff;
// structure for command UATH_CMD_WRITE_MAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_write_mac {
    pub reg: __be32,
    pub len: __be32,
    pub data: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_rateset {
    pub length: __u8,
pub const AR5523_MAX_NRATES: c_int = 32;
    pub set: [__u8; AR5523_MAX_NRATES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_set_associd {
    pub defaultrateix: __be32,
    pub associd: __be32,
    pub timoffset: __be32,
    pub turboprime: __be32,
    pub bssid: [__u8; 6],
    pub __packed: },
// structure for command WDCMSG_RESET
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_reset {
    pub /: *mut *mut __be32 flags; / channel flags,
pub const UATH_CHAN_TURBO: c_uint = 0x0100;
pub const UATH_CHAN_CCK: c_uint = 0x0200;
pub const UATH_CHAN_OFDM: c_uint = 0x0400;
pub const UATH_CHAN_2GHZ: c_uint = 0x1000;
pub const UATH_CHAN_5GHZ: c_uint = 0x2000;
    pub /: *mut *mut __be32 freq; / channel frequency,
    pub maxrdpower: __be32,
    pub cfgctl: __be32,
    pub twiceantennareduction: __be32,
    pub channelchange: __be32,
    pub keeprccontent: __be32,
    pub __packed: },
// structure for command WDCMSG_SET_BASIC_RATE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_rates {
    pub connid: __be32,
    pub keeprccontent: __be32,
    pub size: __be32,
    pub rateset: ar5523_cmd_rateset,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_connection_attr {
    pub longpreambleonly: __be32,
    pub rateset: ar5523_cmd_rateset,
    pub wlanmode: __be32,
    pub __packed: },
// structure for command AR5523_CREATE_CONNECTION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_create_connection {
    pub connid: __be32,
    pub bssid: __be32,
    pub size: __be32,
    pub connattr: ar5523_cmd_connection_attr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_ledsteady {
    pub lednum: __be32,
pub const UATH_LED_LINK: c_int = 0;
pub const UATH_LED_ACTIVITY: c_int = 1;
    pub ledmode: __be32,
pub const UATH_LED_OFF: c_int = 0;
pub const UATH_LED_ON: c_int = 1;
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_ledblink {
    pub lednum: __be32,
    pub ledmode: __be32,
    pub blinkrate: __be32,
    pub slowmode: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_ledstate {
    pub connected: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_txq_attr {
    pub priority: __be32,
    pub aifs: __be32,
    pub logcwmin: __be32,
    pub logcwmax: __be32,
    pub bursttime: __be32,
    pub mode: __be32,
    pub qflags: __be32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_txq_setup {
    pub qid: __be32,
    pub len: __be32,
    pub attr: ar5523_cmd_txq_attr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar5523_cmd_rx_filter {
    pub bits: __be32,
pub const UATH_FILTER_RX_UCAST: c_uint = 0x00000001;
pub const UATH_FILTER_RX_MCAST: c_uint = 0x00000002;
pub const UATH_FILTER_RX_BCAST: c_uint = 0x00000004;
pub const UATH_FILTER_RX_CONTROL: c_uint = 0x00000008;
pub const UATH_FILTER_RX_BEACON: c_uint = 0x00000010	/* beacon frames */;
pub const UATH_FILTER_RX_PROM: c_uint = 0x00000020	/* promiscuous mode */;
pub const UATH_FILTER_RX_PHY_ERR: c_uint = 0x00000040	/* phy errors */;
pub const UATH_FILTER_RX_PHY_RADAR: c_uint = 0x00000080	/* radar phy errors */;
pub const UATH_FILTER_RX_XR_POOL: c_uint = 0x00000400	/* XR group polls */;
pub const UATH_FILTER_RX_PROBE_REQ: c_uint = 0x00000800;
    pub op: __be32,
pub const UATH_FILTER_OP_INIT: c_uint = 0x0;
pub const UATH_FILTER_OP_SET: c_uint = 0x1;
pub const UATH_FILTER_OP_CLEAR: c_uint = 0x2;
pub const UATH_FILTER_OP_TEMP: c_uint = 0x3;
pub const UATH_FILTER_OP_RESTORE: c_uint = 0x4;
    pub __packed: },
// MAC Address to use.  Overrides EEPROM
// An ID for use in error & debug messages
}

// Sentinal to indicate "no capability"
// Target supports WDC message debug features
// this is in net/ieee80211.h, but that conflicts with the mac80211 headers
pub const IEEE80211_2ADDR_LEN: c_int = 16;
