//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/carl9170/wlan.h
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
// Shared Atheros AR9170 Header
//
// RX/TX meta descriptor format
//
// Copyright 2008, Johannes Berg <johannes@sipsolutions.net>
// Copyright 2009-2011 Christian Lamparter <chunkeey@googlemail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; see the file COPYING.  If not, see
// http://www.gnu.org/licenses/.
//
// This file incorporates work covered by the following copyright and
// permission notice:
// Copyright (c) 2007-2008 Atheros Communications, Inc.
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

pub const AR9170_RX_PHY_RATE_CCK_1M: c_uint = 0x0a;
pub const AR9170_RX_PHY_RATE_CCK_2M: c_uint = 0x14;
pub const AR9170_RX_PHY_RATE_CCK_5M: c_uint = 0x37;
pub const AR9170_RX_PHY_RATE_CCK_11M: c_uint = 0x6e;
pub const AR9170_ENC_ALG_NONE: c_uint = 0x0;
pub const AR9170_ENC_ALG_WEP64: c_uint = 0x1;
pub const AR9170_ENC_ALG_TKIP: c_uint = 0x2;
pub const AR9170_ENC_ALG_AESCCMP: c_uint = 0x4;
pub const AR9170_ENC_ALG_WEP128: c_uint = 0x5;
pub const AR9170_ENC_ALG_WEP256: c_uint = 0x6;
pub const AR9170_ENC_ALG_CENC: c_uint = 0x7;
pub const AR9170_RX_ENC_SOFTWARE: c_uint = 0x8;
pub const AR9170_RX_STATUS_MODULATION: c_uint = 0x03;
pub const AR9170_RX_STATUS_MODULATION_S: c_int = 0;
pub const AR9170_RX_STATUS_MODULATION_CCK: c_uint = 0x00;
pub const AR9170_RX_STATUS_MODULATION_OFDM: c_uint = 0x01;
pub const AR9170_RX_STATUS_MODULATION_HT: c_uint = 0x02;
pub const AR9170_RX_STATUS_MODULATION_DUPOFDM: c_uint = 0x03;
// depends on modulation
pub const AR9170_RX_STATUS_SHORT_PREAMBLE: c_uint = 0x08;
pub const AR9170_RX_STATUS_GREENFIELD: c_uint = 0x08;
pub const AR9170_RX_STATUS_MPDU: c_uint = 0x30;
pub const AR9170_RX_STATUS_MPDU_S: c_int = 4;
pub const AR9170_RX_STATUS_MPDU_SINGLE: c_uint = 0x00;
pub const AR9170_RX_STATUS_MPDU_FIRST: c_uint = 0x20;
pub const AR9170_RX_STATUS_MPDU_MIDDLE: c_uint = 0x30;
pub const AR9170_RX_STATUS_MPDU_LAST: c_uint = 0x10;
pub const AR9170_RX_STATUS_CONT_AGGR: c_uint = 0x40;
pub const AR9170_RX_STATUS_TOTAL_ERROR: c_uint = 0x80;
pub const AR9170_RX_ERROR_RXTO: c_uint = 0x01;
pub const AR9170_RX_ERROR_OVERRUN: c_uint = 0x02;
pub const AR9170_RX_ERROR_DECRYPT: c_uint = 0x04;
pub const AR9170_RX_ERROR_FCS: c_uint = 0x08;
pub const AR9170_RX_ERROR_WRONG_RA: c_uint = 0x10;
pub const AR9170_RX_ERROR_PLCP: c_uint = 0x20;
pub const AR9170_RX_ERROR_MMIC: c_uint = 0x40;
// these are either-or
pub const AR9170_TX_MAC_PROT_RTS: c_uint = 0x0001;
pub const AR9170_TX_MAC_PROT_CTS: c_uint = 0x0002;
pub const AR9170_TX_MAC_PROT: c_uint = 0x0003;
pub const AR9170_TX_MAC_NO_ACK: c_uint = 0x0004;
// if unset, MAC will only do SIFS space before frame
pub const AR9170_TX_MAC_BACKOFF: c_uint = 0x0008;
pub const AR9170_TX_MAC_BURST: c_uint = 0x0010;
pub const AR9170_TX_MAC_AGGR: c_uint = 0x0020;
// encryption is a two-bit field
pub const AR9170_TX_MAC_ENCR_NONE: c_uint = 0x0000;
pub const AR9170_TX_MAC_ENCR_RC4: c_uint = 0x0040;
pub const AR9170_TX_MAC_ENCR_CENC: c_uint = 0x0080;
pub const AR9170_TX_MAC_ENCR_AES: c_uint = 0x00c0;
pub const AR9170_TX_MAC_MMIC: c_uint = 0x0100;
pub const AR9170_TX_MAC_HW_DURATION: c_uint = 0x0200;
pub const AR9170_TX_MAC_QOS_S: c_int = 10;
pub const AR9170_TX_MAC_QOS: c_uint = 0x0c00;
pub const AR9170_TX_MAC_DISABLE_TXOP: c_uint = 0x1000;
pub const AR9170_TX_MAC_TXOP_RIFS: c_uint = 0x2000;
pub const AR9170_TX_MAC_IMM_BA: c_uint = 0x4000;
// either-or
pub const AR9170_TX_PHY_MOD_CCK: c_uint = 0x00000000;
pub const AR9170_TX_PHY_MOD_OFDM: c_uint = 0x00000001;
pub const AR9170_TX_PHY_MOD_HT: c_uint = 0x00000002;
// depends on modulation
pub const AR9170_TX_PHY_SHORT_PREAMBLE: c_uint = 0x00000004;
pub const AR9170_TX_PHY_GREENFIELD: c_uint = 0x00000004;
pub const AR9170_TX_PHY_BW_S: c_int = 3;

pub const AR9170_TX_PHY_BW_20MHZ: c_int = 0;
pub const AR9170_TX_PHY_BW_40MHZ: c_int = 2;
pub const AR9170_TX_PHY_BW_40MHZ_DUP: c_int = 3;
pub const AR9170_TX_PHY_TX_HEAVY_CLIP_S: c_int = 6;

pub const AR9170_TX_PHY_TX_PWR_S: c_int = 9;

pub const AR9170_TX_PHY_TXCHAIN_S: c_int = 15;

pub const AR9170_TX_PHY_TXCHAIN_1: c_int = 1;
// use for cck, ofdm 6/9/12/18/24 and HT if capable
pub const AR9170_TX_PHY_TXCHAIN_2: c_int = 5;
pub const AR9170_TX_PHY_MCS_S: c_int = 18;

pub const AR9170_TX_PHY_RATE_CCK_1M: c_uint = 0x0;
pub const AR9170_TX_PHY_RATE_CCK_2M: c_uint = 0x1;
pub const AR9170_TX_PHY_RATE_CCK_5M: c_uint = 0x2;
pub const AR9170_TX_PHY_RATE_CCK_11M: c_uint = 0x3;
// same as AR9170_RX_PHY_RATE
pub const AR9170_TXRX_PHY_RATE_OFDM_6M: c_uint = 0xb;
pub const AR9170_TXRX_PHY_RATE_OFDM_9M: c_uint = 0xf;
pub const AR9170_TXRX_PHY_RATE_OFDM_12M: c_uint = 0xa;
pub const AR9170_TXRX_PHY_RATE_OFDM_18M: c_uint = 0xe;
pub const AR9170_TXRX_PHY_RATE_OFDM_24M: c_uint = 0x9;
pub const AR9170_TXRX_PHY_RATE_OFDM_36M: c_uint = 0xd;
pub const AR9170_TXRX_PHY_RATE_OFDM_48M: c_uint = 0x8;
pub const AR9170_TXRX_PHY_RATE_OFDM_54M: c_uint = 0xc;
pub const AR9170_TXRX_PHY_RATE_HT_MCS0: c_uint = 0x0;
pub const AR9170_TXRX_PHY_RATE_HT_MCS1: c_uint = 0x1;
pub const AR9170_TXRX_PHY_RATE_HT_MCS2: c_uint = 0x2;
pub const AR9170_TXRX_PHY_RATE_HT_MCS3: c_uint = 0x3;
pub const AR9170_TXRX_PHY_RATE_HT_MCS4: c_uint = 0x4;
pub const AR9170_TXRX_PHY_RATE_HT_MCS5: c_uint = 0x5;
pub const AR9170_TXRX_PHY_RATE_HT_MCS6: c_uint = 0x6;
pub const AR9170_TXRX_PHY_RATE_HT_MCS7: c_uint = 0x7;
pub const AR9170_TXRX_PHY_RATE_HT_MCS8: c_uint = 0x8;
pub const AR9170_TXRX_PHY_RATE_HT_MCS9: c_uint = 0x9;
pub const AR9170_TXRX_PHY_RATE_HT_MCS10: c_uint = 0xa;
pub const AR9170_TXRX_PHY_RATE_HT_MCS11: c_uint = 0xb;
pub const AR9170_TXRX_PHY_RATE_HT_MCS12: c_uint = 0xc;
pub const AR9170_TXRX_PHY_RATE_HT_MCS13: c_uint = 0xd;
pub const AR9170_TXRX_PHY_RATE_HT_MCS14: c_uint = 0xe;
pub const AR9170_TXRX_PHY_RATE_HT_MCS15: c_uint = 0xf;
pub const AR9170_TX_PHY_SHORT_GI: c_uint = 0x80000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_tx_hw_mac_control {
//
// Beware of compiler bugs in all gcc pre 4.4!
//
    pub erp_prot:2: u8,
    pub no_ack:1: u8,
    pub backoff:1: u8,
    pub burst:1: u8,
    pub ampdu:1: u8,
    pub enc_mode:2: u8,
    pub hw_mmic:1: u8,
    pub hw_duration:1: u8,
    pub qos_queue:2: u8,
    pub disable_txop:1: u8,
    pub txop_rifs:1: u8,
    pub ba_end:1: u8,
    pub probe:1: u8,
    pub __packed: },
    pub set: __le16,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_tx_hw_phy_control {
//
// Beware of compiler bugs in all gcc pre 4.4!
//
    pub modulation:2: u8,
    pub preamble:1: u8,
    pub bandwidth:2: u8,
    pub heavy_clip:3: u8,
    pub tx_power:6: u8,
    pub chains:3: u8,
    pub mcs:7: u8,
    pub short_gi:1: u8,
    pub __packed: },
    pub set: __le32,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_tx_rate_info {
    pub tries:3: u8,
    pub erp_prot:2: u8,
    pub ampdu:1: u8,
    pub /: *mut *mut u8 free:2; / free for use (e.g.:RIFS/TXOP/AMPDU),
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_tx_superdesc {
    pub len: __le16,
    pub rix: u8,
    pub cnt: u8,
    pub cookie: u8,
    pub ampdu_density:3: u8,
    pub ampdu_factor:2: u8,
    pub ampdu_commit_density:1: u8,
    pub ampdu_commit_factor:1: u8,
    pub ampdu_unused_bit:1: u8,
    pub queue:2: u8,
    pub assign_seq:1: u8,
    pub vif_id:3: u8,
    pub fill_in_tsf:1: u8,
    pub cab:1: u8,
    pub padding2: u8,
    pub ri: [ar9170_tx_rate_info; CARL9170_TX_MAX_RATES],
    pub rr: [ar9170_tx_hw_phy_control; CARL9170_TX_MAX_RETRY_RATES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_tx_hwdesc {
    pub length: __le16,
    pub mac: ar9170_tx_hw_mac_control,
    pub phy: ar9170_tx_hw_phy_control,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_tx_frame {
    pub hdr: ar9170_tx_hwdesc,
    pub i3e: ieee80211_hdr,
    pub payload): DECLARE_FLEX_ARRAY(u8,,
    pub data: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_tx_superframe {
    pub s: carl9170_tx_superdesc,
    pub f: ar9170_tx_frame,
    pub __aligned(4): } __packed,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ar9170_tx_hwdesc {
    pub length: __le16,
    pub mac_control: __le16,
    pub phy_control: __le32,
    pub __packed: },
pub const CARL9170_TX_SUPER_AMPDU_DENSITY_S: c_int = 0;
pub const CARL9170_TX_SUPER_AMPDU_DENSITY: c_uint = 0x7;
pub const CARL9170_TX_SUPER_AMPDU_FACTOR: c_uint = 0x18;
pub const CARL9170_TX_SUPER_AMPDU_FACTOR_S: c_int = 3;
pub const CARL9170_TX_SUPER_AMPDU_COMMIT_DENSITY: c_uint = 0x20;
pub const CARL9170_TX_SUPER_AMPDU_COMMIT_DENSITY_S: c_int = 5;
pub const CARL9170_TX_SUPER_AMPDU_COMMIT_FACTOR: c_uint = 0x40;
pub const CARL9170_TX_SUPER_AMPDU_COMMIT_FACTOR_S: c_int = 6;
pub const CARL9170_TX_SUPER_MISC_QUEUE: c_uint = 0x3;
pub const CARL9170_TX_SUPER_MISC_QUEUE_S: c_int = 0;
pub const CARL9170_TX_SUPER_MISC_ASSIGN_SEQ: c_uint = 0x4;
pub const CARL9170_TX_SUPER_MISC_VIF_ID: c_uint = 0x38;
pub const CARL9170_TX_SUPER_MISC_VIF_ID_S: c_int = 3;
pub const CARL9170_TX_SUPER_MISC_FILL_IN_TSF: c_uint = 0x40;
pub const CARL9170_TX_SUPER_MISC_CAB: c_uint = 0x80;
pub const CARL9170_TX_SUPER_RI_TRIES: c_uint = 0x7;
pub const CARL9170_TX_SUPER_RI_TRIES_S: c_int = 0;
pub const CARL9170_TX_SUPER_RI_ERP_PROT: c_uint = 0x18;
pub const CARL9170_TX_SUPER_RI_ERP_PROT_S: c_int = 3;
pub const CARL9170_TX_SUPER_RI_AMPDU: c_uint = 0x20;
pub const CARL9170_TX_SUPER_RI_AMPDU_S: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _carl9170_tx_superdesc {
    pub len: __le16,
    pub rix: u8,
    pub cnt: u8,
    pub cookie: u8,
    pub ampdu_settings: u8,
    pub misc: u8,
    pub padding: u8,
    pub ri: [u8; CARL9170_TX_MAX_RATES],
    pub rr: [__le32; CARL9170_TX_MAX_RETRY_RATES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _carl9170_tx_superframe {
    pub s: _carl9170_tx_superdesc,
    pub f: _ar9170_tx_hwdesc,
    pub frame_data: [u8; ],
    pub __aligned(4): } __packed,
pub const CARL9170_TX_SUPERDESC_LEN: c_int = 24;
pub const AR9170_TX_HWDESC_LEN: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_rx_head {
    pub plcp: [u8; 12],
    pub __packed: },
pub const AR9170_RX_HEAD_LEN: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_rx_phystatus {
    pub __packed: },
    pub rssi: [u8; 7],
    pub __packed: },
    pub evm_stream1: [u8 evm_stream0[6],; 6],
    pub phy_err: u8,
    pub __packed: },
pub const AR9170_RX_PHYSTATUS_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_rx_macstatus {
    pub DAidx: u8 SAidx,,
    pub error: u8,
    pub status: u8,
    pub __packed: },
pub const AR9170_RX_MACSTATUS_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_rx_frame_single {
    pub phy_head: ar9170_rx_head,
    pub __aligned(2): ieee80211_hdr i3e __packed,
    pub phy_tail: ar9170_rx_phystatus,
    pub macstatus: ar9170_rx_macstatus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_rx_frame_head {
    pub phy_head: ar9170_rx_head,
    pub __aligned(2): ieee80211_hdr i3e __packed,
    pub macstatus: ar9170_rx_macstatus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_rx_frame_middle {
    pub __aligned(2): ieee80211_hdr i3e __packed,
    pub macstatus: ar9170_rx_macstatus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_rx_frame_tail {
    pub __aligned(2): ieee80211_hdr i3e __packed,
    pub phy_tail: ar9170_rx_phystatus,
    pub macstatus: ar9170_rx_macstatus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar9170_rx_frame {
    pub single: ar9170_rx_frame_single,
    pub head: ar9170_rx_frame_head,
    pub middle: ar9170_rx_frame_middle,
    pub tail: ar9170_rx_frame_tail,
}

//
// This is an workaround for several undocumented bugs.
// Don't mess with the QoS/AC <-> HW Queue map, if you don't
// know what you are doing.
//
// Known problems [hardware]:
// * The MAC does not aggregate frames on anything other
// than the first HW queue.
// * when an AMPDU is placed [in the first hw queue] and
// additional frames are already queued on a different
// hw queue, the MAC will ALWAYS freeze.
//
// In a nutshell: The hardware can either do QoS or
// Aggregation but not both at the same time. As a
// result, this makes the device pretty much useless
// for any serious 802.11n setup.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ar9170_txq {
    AR9170_TXQ_BK = 0,	/* TXQ0 */
    AR9170_TXQ_BE,		/* TXQ1	*/
    AR9170_TXQ_VI,		/* TXQ2	*/
    AR9170_TXQ_VO,		/* TXQ3 */

    __AR9170_NUM_TXQ,
}

pub const AR9170_TXQ_DEPTH: c_int = 32;
