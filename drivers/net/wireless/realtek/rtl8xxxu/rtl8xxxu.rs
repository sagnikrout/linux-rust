//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtl8xxxu/rtl8xxxu.h
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
// Copyright (c) 2014 - 2017 Jes Sorensen <Jes.Sorensen@gmail.com>
//
// Register definitions taken from original Realtek rtl8723au driver
//

pub const RTL8XXXU_DEBUG_REG_WRITE: c_uint = 0x01;
pub const RTL8XXXU_DEBUG_REG_READ: c_uint = 0x02;
pub const RTL8XXXU_DEBUG_RFREG_WRITE: c_uint = 0x04;
pub const RTL8XXXU_DEBUG_RFREG_READ: c_uint = 0x08;
pub const RTL8XXXU_DEBUG_CHANNEL: c_uint = 0x10;
pub const RTL8XXXU_DEBUG_TX: c_uint = 0x20;
pub const RTL8XXXU_DEBUG_TX_DUMP: c_uint = 0x40;
pub const RTL8XXXU_DEBUG_RX: c_uint = 0x80;
pub const RTL8XXXU_DEBUG_RX_DUMP: c_uint = 0x100;
pub const RTL8XXXU_DEBUG_USB: c_uint = 0x200;
pub const RTL8XXXU_DEBUG_KEY: c_uint = 0x400;
pub const RTL8XXXU_DEBUG_H2C: c_uint = 0x800;
pub const RTL8XXXU_DEBUG_ACTION: c_uint = 0x1000;
pub const RTL8XXXU_DEBUG_EFUSE: c_uint = 0x2000;
pub const RTL8XXXU_DEBUG_INTERRUPT: c_uint = 0x4000;
pub const RTW_USB_CONTROL_MSG_TIMEOUT: c_int = 500;
pub const RTL8XXXU_MAX_REG_POLL: c_int = 500;
pub const USB_INTR_CONTENT_LENGTH: c_int = 56;
pub const RTL8XXXU_OUT_ENDPOINTS: c_int = 6;
pub const REALTEK_USB_READ: c_uint = 0xc0;
pub const REALTEK_USB_WRITE: c_uint = 0x40;
pub const REALTEK_USB_CMD_REQ: c_uint = 0x05;
pub const REALTEK_USB_CMD_IDX: c_uint = 0x00;
pub const TX_TOTAL_PAGE_NUM: c_uint = 0xf8;
pub const TX_TOTAL_PAGE_NUM_8188F: c_uint = 0xf7;
pub const TX_TOTAL_PAGE_NUM_8188E: c_uint = 0xa9;
pub const TX_TOTAL_PAGE_NUM_8192E: c_uint = 0xf3;
pub const TX_TOTAL_PAGE_NUM_8723B: c_uint = 0xf7;
pub const TX_TOTAL_PAGE_NUM_8192F: c_uint = 0xf7;
// (HPQ + LPQ + NPQ + PUBQ) = TX_TOTAL_PAGE_NUM
pub const TX_PAGE_NUM_PUBQ: c_uint = 0xe7;
pub const TX_PAGE_NUM_HI_PQ: c_uint = 0x0c;
pub const TX_PAGE_NUM_LO_PQ: c_uint = 0x02;
pub const TX_PAGE_NUM_NORM_PQ: c_uint = 0x02;
pub const TX_PAGE_NUM_PUBQ_8188F: c_uint = 0xe5;
pub const TX_PAGE_NUM_HI_PQ_8188F: c_uint = 0x0c;
pub const TX_PAGE_NUM_LO_PQ_8188F: c_uint = 0x02;
pub const TX_PAGE_NUM_NORM_PQ_8188F: c_uint = 0x02;
pub const TX_PAGE_NUM_PUBQ_8188E: c_uint = 0x47;
pub const TX_PAGE_NUM_HI_PQ_8188E: c_uint = 0x29;
pub const TX_PAGE_NUM_LO_PQ_8188E: c_uint = 0x1c;
pub const TX_PAGE_NUM_NORM_PQ_8188E: c_uint = 0x1c;
pub const TX_PAGE_NUM_PUBQ_8192E: c_uint = 0xe7;
pub const TX_PAGE_NUM_HI_PQ_8192E: c_uint = 0x08;
pub const TX_PAGE_NUM_LO_PQ_8192E: c_uint = 0x0c;
pub const TX_PAGE_NUM_NORM_PQ_8192E: c_uint = 0x00;
pub const TX_PAGE_NUM_PUBQ_8723B: c_uint = 0xe7;
pub const TX_PAGE_NUM_HI_PQ_8723B: c_uint = 0x0c;
pub const TX_PAGE_NUM_LO_PQ_8723B: c_uint = 0x02;
pub const TX_PAGE_NUM_NORM_PQ_8723B: c_uint = 0x02;
pub const TX_PAGE_NUM_PUBQ_8192F: c_uint = 0xde;
pub const TX_PAGE_NUM_HI_PQ_8192F: c_uint = 0x08;
pub const TX_PAGE_NUM_LO_PQ_8192F: c_uint = 0x08;
pub const TX_PAGE_NUM_NORM_PQ_8192F: c_uint = 0x08;
pub const RTL_FW_PAGE_SIZE: c_int = 4096;
pub const RTL8XXXU_FIRMWARE_POLL_MAX: c_int = 1000;
pub const RTL8723A_CHANNEL_GROUPS: c_int = 3;
pub const RTL8723A_MAX_RF_PATHS: c_int = 2;
pub const RTL8723B_CHANNEL_GROUPS: c_int = 6;
pub const RTL8723B_TX_COUNT: c_int = 4;
pub const RTL8723B_MAX_RF_PATHS: c_int = 4;
pub const RTL8XXXU_MAX_CHANNEL_GROUPS: c_int = 6;
pub const RF6052_MAX_TX_PWR: c_uint = 0x3f;
pub const EFUSE_MAP_LEN: c_int = 512;
pub const EFUSE_MAX_SECTION_8723A: c_int = 64;
pub const EFUSE_REAL_CONTENT_LEN_8723A: c_int = 512;
pub const EFUSE_BT_MAP_LEN_8723A: c_int = 1024;
pub const EFUSE_MAX_WORD_UNIT: c_int = 4;
pub const EFUSE_UNDEFINED: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8xxxu_rtl_chip {
    RTL8192S = 0x81920,
    RTL8191S = 0x81910,
    RTL8192C = 0x8192c,
    RTL8191C = 0x8191c,
    RTL8188C = 0x8188c,
    RTL8188R = 0x81889,
    RTL8192D = 0x8192d,
    RTL8723A = 0x8723a,
    RTL8188E = 0x8188e,
    RTL8812  = 0x88120,
    RTL8821  = 0x88210,
    RTL8192E = 0x8192e,
    RTL8191E = 0x8191e,
    RTL8723B = 0x8723b,
    RTL8814A = 0x8814a,
    RTL8881A = 0x8881a,
    RTL8821B = 0x8821b,
    RTL8822B = 0x8822b,
    RTL8703B = 0x8703b,
    RTL8195A = 0x8195a,
    RTL8188F = 0x8188f,
    RTL8710B = 0x8710b,
    RTL8192F = 0x8192f,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8xxxu_rx_type {
    RX_TYPE_DATA_PKT = 0,
    RX_TYPE_C2H = 1,
    RX_TYPE_ERROR = -1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8xxxu_rx_desc_enc {
    RX_DESC_ENC_NONE	= 0,
    RX_DESC_ENC_WEP40	= 1,
    RX_DESC_ENC_TKIP_WO_MIC	= 2,
    RX_DESC_ENC_TKIP_MIC	= 3,
    RX_DESC_ENC_AES		= 4,
    RX_DESC_ENC_WEP104	= 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_rxdesc16 {

    pub pktlen:14: u32,
    pub crc32:1: u32,
    pub icverr:1: u32,
    pub drvinfo_sz:4: u32,
    pub security:3: u32,
    pub qos:1: u32,
    pub shift:2: u32,
    pub phy_stats:1: u32,
    pub swdec:1: u32,
    pub ls:1: u32,
    pub fs:1: u32,
    pub eor:1: u32,
    pub own:1: u32,
    pub macid:5: u32,
    pub tid:4: u32,
    pub hwrsvd:4: u32,
    pub amsdu:1: u32,
    pub paggr:1: u32,
    pub faggr:1: u32,
    pub a1fit:4: u32,
    pub a2fit:4: u32,
    pub pam:1: u32,
    pub pwr:1: u32,
    pub md:1: u32,
    pub mf:1: u32,
    pub type:2: u32,
    pub mc:1: u32,
    pub bc:1: u32,
    pub seq:12: u32,
    pub frag:4: u32,
    pub pkt_cnt:8: u32,
    pub reserved:6: u32,
    pub nextind:1: u32,
    pub reserved0:1: u32,
    pub rxmcs:6: u32,
    pub rxht:1: u32,
    pub gf:1: u32,
    pub splcp:1: u32,
    pub bw:1: u32,
    pub htc:1: u32,
    pub eosp:1: u32,
    pub bssidfit:2: u32,
    pub /: *mut *mut u32 rpt_sel:2; / 8188e,
    pub reserved1:14: u32,
    pub unicastwake:1: u32,
    pub magicwake:1: u32,
    pub pattern0match:1: u32,
    pub pattern1match:1: u32,
    pub pattern2match:1: u32,
    pub pattern3match:1: u32,
    pub pattern4match:1: u32,
    pub pattern5match:1: u32,
    pub pattern6match:1: u32,
    pub pattern7match:1: u32,
    pub pattern8match:1: u32,
    pub pattern9match:1: u32,
    pub patternamatch:1: u32,
    pub patternbmatch:1: u32,
    pub patterncmatch:1: u32,
    pub reserved2:19: u32,

    pub own:1: u32,
    pub eor:1: u32,
    pub fs:1: u32,
    pub ls:1: u32,
    pub swdec:1: u32,
    pub phy_stats:1: u32,
    pub shift:2: u32,
    pub qos:1: u32,
    pub security:3: u32,
    pub drvinfo_sz:4: u32,
    pub icverr:1: u32,
    pub crc32:1: u32,
    pub pktlen:14: u32,
    pub bc:1: u32,
    pub mc:1: u32,
    pub type:2: u32,
    pub mf:1: u32,
    pub md:1: u32,
    pub pwr:1: u32,
    pub pam:1: u32,
    pub a2fit:4: u32,
    pub a1fit:4: u32,
    pub faggr:1: u32,
    pub paggr:1: u32,
    pub amsdu:1: u32,
    pub hwrsvd:4: u32,
    pub tid:4: u32,
    pub macid:5: u32,
    pub reserved0:1: u32,
    pub nextind:1: u32,
    pub reserved:6: u32,
    pub pkt_cnt:8: u32,
    pub frag:4: u32,
    pub seq:12: u32,
    pub magicwake:1: u32,
    pub unicastwake:1: u32,
    pub reserved1:14: u32,
    pub /: *mut *mut u32 rpt_sel:2; / 8188e,
    pub bssidfit:2: u32,
    pub eosp:1: u32,
    pub htc:1: u32,
    pub bw:1: u32,
    pub splcp:1: u32,
    pub gf:1: u32,
    pub rxht:1: u32,
    pub rxmcs:6: u32,
    pub reserved2:19: u32,
    pub patterncmatch:1: u32,
    pub patternbmatch:1: u32,
    pub patternamatch:1: u32,
    pub pattern9match:1: u32,
    pub pattern8match:1: u32,
    pub pattern7match:1: u32,
    pub pattern6match:1: u32,
    pub pattern5match:1: u32,
    pub pattern4match:1: u32,
    pub pattern3match:1: u32,
    pub pattern2match:1: u32,
    pub pattern1match:1: u32,
    pub pattern0match:1: u32,

    pub tsfl: u32,

    pub bassn:12: u32,
    pub bavld:1: u32,
    pub reserved3:19: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_rxdesc24 {

    pub pktlen:14: u32,
    pub crc32:1: u32,
    pub icverr:1: u32,
    pub drvinfo_sz:4: u32,
    pub security:3: u32,
    pub qos:1: u32,
    pub shift:2: u32,
    pub phy_stats:1: u32,
    pub swdec:1: u32,
    pub ls:1: u32,
    pub fs:1: u32,
    pub eor:1: u32,
    pub own:1: u32,
    pub macid:7: u32,
    pub dummy1_0:1: u32,
    pub tid:4: u32,
    pub dummy1_1:1: u32,
    pub amsdu:1: u32,
    pub rxid_match:1: u32,
    pub paggr:1: u32,
    pub /: *mut *mut u32 a1fit:4; / 16,
    pub chkerr:1: u32,
    pub ipver:1: u32,
    pub tcpudp:1: u32,
    pub chkvld:1: u32,
    pub pam:1: u32,
    pub pwr:1: u32,
    pub more_data:1: u32,
    pub more_frag:1: u32,
    pub type:2: u32,
    pub mc:1: u32,
    pub bc:1: u32,
    pub seq:12: u32,
    pub frag:4: u32,
    pub /: *mut *mut u32 rx_is_qos:1; / 16,
    pub dummy2_0:1: u32,
    pub wlanhd_iv_len:6: u32,
    pub dummy2_1:4: u32,
    pub rpt_sel:1: u32,
    pub dummy2_2:3: u32,
    pub rxmcs:7: u32,
    pub dummy3_0:3: u32,
    pub htc:1: u32,
    pub eosp:1: u32,
    pub bssidfit:2: u32,
    pub dummy3_1:2: u32,
    pub /: *mut *mut u32 usb_agg_pktnum:8; / 16,
    pub dummy3_2:5: u32,
    pub pattern_match:1: u32,
    pub unicast_match:1: u32,
    pub magic_match:1: u32,
    pub splcp:1: u32,
    pub ldcp:1: u32,
    pub stbc:1: u32,
    pub dummy4_0:1: u32,
    pub bw:2: u32,
    pub dummy4_1:26: u32,

    pub own:1: u32,
    pub eor:1: u32,
    pub fs:1: u32,
    pub ls:1: u32,
    pub swdec:1: u32,
    pub phy_stats:1: u32,
    pub shift:2: u32,
    pub qos:1: u32,
    pub security:3: u32,
    pub drvinfo_sz:4: u32,
    pub icverr:1: u32,
    pub crc32:1: u32,
    pub pktlen:14: u32,
    pub bc:1: u32,
    pub mc:1: u32,
    pub type:2: u32,
    pub mf:1: u32,
    pub md:1: u32,
    pub pwr:1: u32,
    pub pam:1: u32,
    pub a2fit:4: u32,
    pub a1fit:4: u32,
    pub faggr:1: u32,
    pub paggr:1: u32,
    pub amsdu:1: u32,
    pub hwrsvd:4: u32,
    pub tid:4: u32,
    pub macid:5: u32,
    pub dummy2_2:3: u32,
    pub rpt_sel:1: u32,
    pub dummy2_1:4: u32,
    pub wlanhd_iv_len:6: u32,
    pub dummy2_0:1: u32,
    pub rx_is_qos:1: u32,
    pub /: *mut *mut u32 frag:4; / 16,
    pub seq:12: u32,
    pub magic_match:1: u32,
    pub unicast_match:1: u32,
    pub pattern_match:1: u32,
    pub dummy3_2:5: u32,
    pub usb_agg_pktnum:8: u32,
    pub /: *mut *mut u32 dummy3_1:2; / 16,
    pub bssidfit:2: u32,
    pub eosp:1: u32,
    pub htc:1: u32,
    pub dummy3_0:3: u32,
    pub rxmcs:7: u32,
    pub dumm4_1:26: u32,
    pub bw:2: u32,
    pub dummy4_0:1: u32,
    pub stbc:1: u32,
    pub ldcp:1: u32,
    pub splcp:1: u32,

    pub tsfl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_txdesc32 {
    pub pkt_size: __le16,
    pub pkt_offset: u8,
    pub txdw0: u8,
    pub txdw1: __le32,
    pub txdw2: __le32,
    pub txdw3: __le32,
    pub txdw4: __le32,
    pub txdw5: __le32,
    pub txdw6: __le32,
    pub csum: __le16,
    pub txdw7: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_txdesc40 {
    pub pkt_size: __le16,
    pub pkt_offset: u8,
    pub txdw0: u8,
    pub txdw1: __le32,
    pub txdw2: __le32,
    pub txdw3: __le32,
    pub txdw4: __le32,
    pub txdw5: __le32,
    pub txdw6: __le32,
    pub csum: __le16,
    pub txdw7: __le16,
    pub txdw8: __le32,
    pub txdw9: __le32,
}

// CCK Rates, TxHT = 0
pub const DESC_RATE_1M: c_uint = 0x00;
pub const DESC_RATE_2M: c_uint = 0x01;
pub const DESC_RATE_5_5M: c_uint = 0x02;
pub const DESC_RATE_11M: c_uint = 0x03;
// OFDM Rates, TxHT = 0
pub const DESC_RATE_6M: c_uint = 0x04;
pub const DESC_RATE_9M: c_uint = 0x05;
pub const DESC_RATE_12M: c_uint = 0x06;
pub const DESC_RATE_18M: c_uint = 0x07;
pub const DESC_RATE_24M: c_uint = 0x08;
pub const DESC_RATE_36M: c_uint = 0x09;
pub const DESC_RATE_48M: c_uint = 0x0a;
pub const DESC_RATE_54M: c_uint = 0x0b;
// MCS Rates, TxHT = 1
pub const DESC_RATE_MCS0: c_uint = 0x0c;
pub const DESC_RATE_MCS1: c_uint = 0x0d;
pub const DESC_RATE_MCS2: c_uint = 0x0e;
pub const DESC_RATE_MCS3: c_uint = 0x0f;
pub const DESC_RATE_MCS4: c_uint = 0x10;
pub const DESC_RATE_MCS5: c_uint = 0x11;
pub const DESC_RATE_MCS6: c_uint = 0x12;
pub const DESC_RATE_MCS7: c_uint = 0x13;
pub const DESC_RATE_MCS8: c_uint = 0x14;
pub const DESC_RATE_MCS9: c_uint = 0x15;
pub const DESC_RATE_MCS10: c_uint = 0x16;
pub const DESC_RATE_MCS11: c_uint = 0x17;
pub const DESC_RATE_MCS12: c_uint = 0x18;
pub const DESC_RATE_MCS13: c_uint = 0x19;
pub const DESC_RATE_MCS14: c_uint = 0x1a;
pub const DESC_RATE_MCS15: c_uint = 0x1b;
pub const DESC_RATE_MCS15_SG: c_uint = 0x1c;
pub const DESC_RATE_MCS32: c_uint = 0x20;
pub const TXDESC_OFFSET_SZ: c_int = 0;
pub const TXDESC_OFFSET_SHT: c_int = 16;

// Word 1
//
// Bits 0-7 differ dependent on chip generation. For 8723au bits 5/6 are
// aggregation enable and break respectively. For 8723bu, bits 0-7 are macid.
//
pub const TXDESC_PKT_OFFSET_SZ: c_int = 0;

pub const TXDESC40_MACID_SHIFT: c_int = 0;
pub const TXDESC40_MACID_MASK: c_uint = 0x00f0;
pub const TXDESC_QUEUE_SHIFT: c_int = 8;
pub const TXDESC_QUEUE_MASK: c_uint = 0x1f00;
pub const TXDESC_QUEUE_BK: c_uint = 0x2;
pub const TXDESC_QUEUE_BE: c_uint = 0x0;
pub const TXDESC_QUEUE_VI: c_uint = 0x5;
pub const TXDESC_QUEUE_VO: c_uint = 0x7;
pub const TXDESC_QUEUE_BEACON: c_uint = 0x10;
pub const TXDESC_QUEUE_HIGH: c_uint = 0x11;
pub const TXDESC_QUEUE_MGNT: c_uint = 0x12;
pub const TXDESC_QUEUE_CMD: c_uint = 0x13;

pub const DESC_RATE_ID_SHIFT: c_int = 16;
pub const DESC_RATE_ID_MASK: c_uint = 0xf;

pub const TXDESC_SEC_RC4: c_uint = 0x00400000;
pub const TXDESC_SEC_AES: c_uint = 0x00c00000;
pub const TXDESC_PKT_OFFSET_SHIFT: c_int = 26;

// Word 2
pub const TXDESC40_PAID_SHIFT: c_int = 0;
pub const TXDESC40_PAID_MASK: c_uint = 0x1ff;
pub const TXDESC40_CCA_RTS_SHIFT: c_int = 10;
pub const TXDESC40_CCA_RTS_MASK: c_uint = 0xc00;

pub const TXDESC_AMPDU_DENSITY_SHIFT: c_int = 20;

pub const TXDESC40_GID_SHIFT: c_int = 24;

// Word 3

pub const TXDESC32_SEQ_SHIFT: c_int = 16;
pub const TXDESC32_SEQ_MASK: c_uint = 0x0fff0000;
// Word 4
pub const TXDESC32_RTS_RATE_SHIFT: c_int = 0;
pub const TXDESC32_RTS_RATE_MASK: c_uint = 0x3f;

pub const TXDESC40_DATA_RATE_FB_SHIFT: c_int = 8;
pub const TXDESC40_DATA_RATE_FB_MASK: c_uint = 0x00001f00;

pub const TXDESC40_RETRY_LIMIT_SHIFT: c_int = 18;
pub const TXDESC40_RETRY_LIMIT_MASK: c_uint = 0x00fc0000;
pub const TXDESC40_RTS_RATE_SHIFT: c_int = 24;
pub const TXDESC40_RTS_RATE_MASK: c_uint = 0x3f000000;
// Word 5

pub const TXDESC32_RETRY_LIMIT_SHIFT: c_int = 18;
pub const TXDESC32_RETRY_LIMIT_MASK: c_uint = 0x00fc0000;
// Word 6
pub const TXDESC_MAX_AGG_SHIFT: c_int = 11;
pub const TXDESC_USB_TX_AGG_SHIT: c_int = 24;
// Word 7

// Word 8

// Word 9
pub const TXDESC40_SEQ_SHIFT: c_int = 12;
pub const TXDESC40_SEQ_MASK: c_uint = 0x00fff000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_rx_agc_info {

    pub trsw:1: u8 gain:7,,

    pub gain:7: u8 trsw:1,,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8723au_phy_stats {
    pub path_agc: [phy_rx_agc_info; RTL8723A_MAX_RF_PATHS],
    pub ch_corr: [u8; RTL8723A_MAX_RF_PATHS],
    pub cck_sig_qual_ofdm_pwdb_all: u8,
    pub cck_agc_rpt_ofdm_cfosho_a: u8,
    pub cck_rpt_b_ofdm_cfosho_b: u8,
    pub reserved_1: u8,
    pub noise_power_db_msb: u8,
    pub path_cfotail: [i8; RTL8723A_MAX_RF_PATHS],
    pub pcts_mask: [u8; RTL8723A_MAX_RF_PATHS],
    pub stream_rxevm: [i8; RTL8723A_MAX_RF_PATHS],
    pub path_rxsnr: [u8; RTL8723A_MAX_RF_PATHS],
    pub noise_power_db_lsb: u8,
    pub reserved_2: [u8; 3],
    pub stream_csi: [u8; RTL8723A_MAX_RF_PATHS],
    pub stream_target_csi: [u8; RTL8723A_MAX_RF_PATHS],
    pub sig_evm: i8,
    pub reserved_3: u8,

    pub /: *mut *mut u8 antsel_rx_keep_2:1; / ex_intf_flg:1;,
    pub sgi_en:1: u8,
    pub rxsc:2: u8,
    pub idle_long:1: u8,
    pub r_ant_train_en:1: u8,
    pub antenna_select_b:1: u8,
    pub antenna_select:1: u8,

    pub antenna_select:1: u8,
    pub antenna_select_b:1: u8,
    pub r_ant_train_en:1: u8,
    pub idle_long:1: u8,
    pub rxsc:2: u8,
    pub sgi_en:1: u8,
    pub /: *mut *mut u8 antsel_rx_keep_2:1; / ex_intf_flg:1;,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jaguar2_phy_stats_type0 {
// DW0
    pub page_num: u8,
    pub pwdb: u8,

    pub 6: u8 gain:,
    pub 1: u8 rsvd_0:,
    pub 1: u8 trsw:,

    pub 1: u8 trsw:,
    pub 1: u8 rsvd_0:,
    pub 6: u8 gain:,

    pub rsvd_1: u8,
// DW1
    pub rsvd_2: u8,

    pub 4: u8 rxsc:,
    pub 4: u8 agc_table:,

    pub 4: u8 agc_table:,
    pub 4: u8 rxsc:,

    pub channel: u8,
    pub band: u8,
// DW2
    pub length: u16,

    pub 3: u8 antidx_a:,
    pub 3: u8 antidx_b:,
    pub 2: u8 rsvd_3:,
    pub 3: u8 antidx_c:,
    pub 3: u8 antidx_d:,
    pub rsvd_4:2: u8,

    pub 2: u8 rsvd_3:,
    pub 3: u8 antidx_b:,
    pub 3: u8 antidx_a:,
    pub rsvd_4:2: u8,
    pub 3: u8 antidx_d:,
    pub 3: u8 antidx_c:,

// DW3
    pub signal_quality: u8,

    pub vga:5: u8,
    pub lna_l:3: u8,
    pub bb_power:6: u8,
    pub rsvd_9:1: u8,
    pub lna_h:1: u8,

    pub lna_l:3: u8,
    pub vga:5: u8,
    pub lna_h:1: u8,
    pub rsvd_9:1: u8,
    pub bb_power:6: u8,

    pub rsvd_5: u8,
// DW4
    pub rsvd_6: u32,
// DW5
    pub rsvd_7: u32,
// DW6
    pub rsvd_8: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jaguar2_phy_stats_type1 {
// DW0 and DW1
    pub page_num: u8,
    pub pwdb: [u8; 4],
    pub 4: u8 l_rxsc:,
    pub 4: u8 ht_rxsc:,

    pub 4: u8 ht_rxsc:,
    pub 4: u8 l_rxsc:,

    pub channel: u8,

    pub 2: u8 band:,
    pub 1: u8 rsvd_0:,
    pub 1: u8 hw_antsw_occu:,
    pub 1: u8 gnt_bt:,
    pub 1: u8 ldpc:,
    pub 1: u8 stbc:,
    pub 1: u8 beamformed:,

    pub 1: u8 beamformed:,
    pub 1: u8 stbc:,
    pub 1: u8 ldpc:,
    pub 1: u8 gnt_bt:,
    pub 1: u8 hw_antsw_occu:,
    pub 1: u8 rsvd_0:,
    pub 2: u8 band:,

// DW2
    pub lsig_length: u16,

    pub 3: u8 antidx_a:,
    pub 3: u8 antidx_b:,
    pub 2: u8 rsvd_1:,
    pub 3: u8 antidx_c:,
    pub 3: u8 antidx_d:,
    pub 2: u8 rsvd_2:,

    pub 2: u8 rsvd_1:,
    pub 3: u8 antidx_b:,
    pub 3: u8 antidx_a:,
    pub 2: u8 rsvd_2:,
    pub 3: u8 antidx_d:,
    pub 3: u8 antidx_c:,

// DW3
    pub paid: u8,

    pub 1: u8 paid_msb:,
    pub 6: u8 gid:,
    pub 1: u8 rsvd_3:,

    pub 1: u8 rsvd_3:,
    pub 6: u8 gid:,
    pub 1: u8 paid_msb:,

    pub intf_pos: u8,

    pub 1: u8 intf_pos_msb:,
    pub 2: u8 rsvd_4:,
    pub 1: u8 nb_intf_flag:,
    pub 2: u8 rf_mode:,
    pub 2: u8 rsvd_5:,

    pub 2: u8 rsvd_5:,
    pub 2: u8 rf_mode:,
    pub 1: u8 nb_intf_flag:,
    pub 2: u8 rsvd_4:,
    pub 1: u8 intf_pos_msb:,

// DW4
    pub /: *mut *mut s8 rxevm[4]; / s(8,1),
// DW5
    pub /: *mut *mut s8 cfo_tail[4]; / s(8,7),
// DW6
    pub /: *mut *mut s8 rxsnr[4]; / s(8,1),
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jaguar2_phy_stats_type2 {
// DW0 ane DW1
    pub page_num: u8,
    pub pwdb: [u8; 4],
    pub 4: u8 l_rxsc:,
    pub 4: u8 ht_rxsc:,

    pub 4: u8 ht_rxsc:,
    pub 4: u8 l_rxsc:,

    pub channel: u8,

    pub 2: u8 band:,
    pub 1: u8 rsvd_0:,
    pub 1: u8 hw_antsw_occu:,
    pub 1: u8 gnt_bt:,
    pub 1: u8 ldpc:,
    pub 1: u8 stbc:,
    pub 1: u8 beamformed:,

    pub 1: u8 beamformed:,
    pub 1: u8 stbc:,
    pub 1: u8 ldpc:,
    pub 1: u8 gnt_bt:,
    pub 1: u8 hw_antsw_occu:,
    pub 1: u8 rsvd_0:,
    pub 2: u8 band:,

// DW2

    pub 6: u8 shift_l_map:,
    pub 2: u8 rsvd_1:,

    pub 2: u8 rsvd_1:,
    pub 6: u8 shift_l_map:,

    pub cnt_pw2cca: u8,

    pub 4: u8 agc_table_a:,
    pub 4: u8 agc_table_b:,
    pub 4: u8 agc_table_c:,
    pub 4: u8 agc_table_d:,

    pub 4: u8 agc_table_b:,
    pub 4: u8 agc_table_a:,
    pub 4: u8 agc_table_d:,
    pub 4: u8 agc_table_c:,

// DW3 ~ DW6
    pub cnt_cca2agc_rdy: u8,

    pub 6: u8 gain_a:,
    pub 1: u8 rsvd_2:,
    pub 1: u8 trsw_a:,
    pub 6: u8 gain_b:,
    pub 1: u8 rsvd_3:,
    pub 1: u8 trsw_b:,
    pub 6: u8 gain_c:,
    pub 1: u8 rsvd_4:,
    pub 1: u8 trsw_c:,
    pub 6: u8 gain_d:,
    pub 1: u8 rsvd_5:,
    pub 1: u8 trsw_d:,
    pub 2: u8 aagc_step_a:,
    pub 2: u8 aagc_step_b:,
    pub 2: u8 aagc_step_c:,
    pub 2: u8 aagc_step_d:,

    pub 1: u8 trsw_a:,
    pub 1: u8 rsvd_2:,
    pub 6: u8 gain_a:,
    pub 1: u8 trsw_b:,
    pub 1: u8 rsvd_3:,
    pub 6: u8 gain_b:,
    pub 1: u8 trsw_c:,
    pub 1: u8 rsvd_4:,
    pub 6: u8 gain_c:,
    pub 1: u8 trsw_d:,
    pub 1: u8 rsvd_5:,
    pub 6: u8 gain_d:,
    pub 2: u8 aagc_step_d:,
    pub 2: u8 aagc_step_c:,
    pub 2: u8 aagc_step_b:,
    pub 2: u8 aagc_step_a:,
    pub ht_aagc_gain: [u8; 4],
    pub dagc_gain: [u8; 4],
    pub 6: u8 counter:,
    pub 2: u8 rsvd_6:,
    pub 5: u8 syn_count:,
    pub rsvd_7:3: u8,

    pub 2: u8 rsvd_6:,
    pub 6: u8 counter:,
    pub rsvd_7:3: u8,
    pub 5: u8 syn_count:,

    pub __packed: },
//
// Regs to backup
//
pub const RTL8XXXU_ADDA_REGS: c_int = 16;
pub const RTL8XXXU_MAC_REGS: c_int = 4;
pub const RTL8XXXU_BB_REGS: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_firmware_header {
    pub 92C,: *mut *mut __le16 signature; / 92C0: test chip;,
    pub chip: 88C0: test,
    pub A-cut: 88C1: MP,
    pub /: *mut *mut u8 category; / AP/NIC and USB/PCI,
    pub function: u8,
    pub /: *mut *mut __le16 major_version; / FW Version,
    pub /: *mut *mut u8 minor_version; / FW Subversion, default 0x00,
    pub reserved1: u8,
    pub /: *mut *mut u8 month; / Release time Month field,
    pub /: *mut *mut u8 date; / Release time Date field,
    pub /: *mut *mut u8 hour; / Release time Hour field,
    pub /: *mut *mut u8 minute; / Release time Minute field,
    pub /: *mut *mut __le16 ramcodesize; / Size of RAM code,
    pub reserved2: u16,
    pub /: *mut *mut __le32 svn_idx; / SVN entry index,
    pub reserved3: u32,
    pub reserved4: u32,
    pub reserved5: u32,
    pub data: [u8; ],
}

//
// 8723au/8192cu/8188ru required base power index offset tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_power_base {
    pub reg_0e00: u32,
    pub reg_0e04: u32,
    pub reg_0e08: u32,
    pub reg_086c: u32,
    pub reg_0e10: u32,
    pub reg_0e14: u32,
    pub reg_0e18: u32,
    pub reg_0e1c: u32,
    pub reg_0830: u32,
    pub reg_0834: u32,
    pub reg_0838: u32,
    pub reg_086c_2: u32,
    pub reg_083c: u32,
    pub reg_0848: u32,
    pub reg_084c: u32,
    pub reg_0868: u32,
}

//
// The 8723au has 3 channel groups: 1-3, 4-9, and 10-14
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8723au_idx {

    pub a:4: c_int,
    pub b:4: c_int,

    pub b:4: c_int,
    pub a:4: c_int,

    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8723au_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 0xe],
    pub /: *mut *mut u8 cck_tx_power_index_A[3]; / 0x10,
    pub cck_tx_power_index_B: [u8; 3],
    pub /: *mut *mut u8 ht40_1s_tx_power_index_A[3]; / 0x16,
    pub ht40_1s_tx_power_index_B: [u8; 3],
//
// The following entries are half-bytes split as:
// bits 0-3: path A, bits 4-7: path B, all values 4 bits signed
//
    pub ht20_tx_power_index_diff: [rtl8723au_idx; 3],
    pub ofdm_tx_power_index_diff: [rtl8723au_idx; 3],
    pub ht40_max_power_offset: [rtl8723au_idx; 3],
    pub ht20_max_power_offset: [rtl8723au_idx; 3],
    pub /: *mut *mut u8 channel_plan; / 0x28,
    pub tssi_a: u8,
    pub thermal_meter: u8,
    pub rf_regulatory: u8,
    pub rf_option_2: u8,
    pub rf_option_3: u8,
    pub rf_option_4: u8,
    pub res7: u8,
    pub /: *mut *mut u8 version / 0x30,
    pub customer_id_major: u8,
    pub customer_id_minor: u8,
    pub xtal_k: u8,
    pub /: *mut *mut u8 chipset; / 0x34,
    pub res8: [u8; 0x82],
    pub /: *mut *mut u8 vid; / 0xb7,
    pub res9: u8,
    pub /: *mut *mut u8 pid; / 0xb9,
    pub res10: [u8; 0x0c],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0xc6,
    pub res11: [u8; 2],
    pub vendor_name: [u8; 7],
    pub res12: [u8; 2],
    pub /: *mut *mut u8 device_name[0x29]; / 0xd7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8192cu_efuse {
    pub rtl_id: __le16,
    pub hpon: __le16,
    pub res0: [u8; 2],
    pub clk: __le16,
    pub testr: __le16,
    pub vid: __le16,
    pub did: __le16,
    pub svid: __le16,
    pub /: *mut *mut __le16 smid; / 0x10,
    pub res1: [u8; 4],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x16,
    pub res2: [u8; 2],
    pub vendor_name: [u8; 7],
    pub res3: [u8; 3],
    pub /: *mut *mut u8 device_name[0x14]; / 0x28,
    pub /: *mut *mut u8 res4[0x1e]; / 0x3c,
    pub /: *mut *mut u8 cck_tx_power_index_A[3]; / 0x5a,
    pub cck_tx_power_index_B: [u8; 3],
    pub /: *mut *mut u8 ht40_1s_tx_power_index_A[3]; / 0x60,
    pub ht40_1s_tx_power_index_B: [u8; 3],
//
// The following entries are half-bytes split as:
// bits 0-3: path A, bits 4-7: path B, all values 4 bits signed
//
    pub ht40_2s_tx_power_index_diff: [rtl8723au_idx; 3],
    pub /: *mut *mut rtl8723au_idx ht20_tx_power_index_diff[3]; / 0x69,
    pub ofdm_tx_power_index_diff: [rtl8723au_idx; 3],
    pub /: *mut *mut rtl8723au_idx ht40_max_power_offset[3]; / 0x6f,
    pub ht20_max_power_offset: [rtl8723au_idx; 3],
    pub /: *mut *mut u8 channel_plan; / 0x75,
    pub tssi_a: u8,
    pub tssi_b: u8,
    pub /: *mut *mut *mut *mut u8 thermal_meter; / xtal_k / / 0x78,
    pub rf_regulatory: u8,
    pub rf_option_2: u8,
    pub rf_option_3: u8,
    pub rf_option_4: u8,
    pub /: *mut *mut u8 res5[1]; / 0x7d,
    pub version: u8,
    pub customer_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8723bu_pwr_idx {

    pub ht20:4: c_int,
    pub ht40:4: c_int,
    pub ofdm:4: c_int,
    pub cck:4: c_int,

    pub cck:4: c_int,
    pub ofdm:4: c_int,
    pub ht40:4: c_int,
    pub ht20:4: c_int,

    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8723bu_efuse_tx_power {
    pub cck_base: [u8; 6],
    pub ht40_base: [u8; 5],
    pub ht20_ofdm_1s_diff: rtl8723au_idx,
    pub pwr_diff: [rtl8723bu_pwr_idx; 3],
    pub /: *mut *mut u8 dummy5g[24]; / max channel group (14) + power diff offset (10),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8723bu_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 0x0e],
    pub /: *mut *mut rtl8723bu_efuse_tx_power tx_power_index_A; / 0x10,
    pub /: *mut *mut rtl8723bu_efuse_tx_power tx_power_index_B; / 0x3a,
    pub /: *mut *mut rtl8723bu_efuse_tx_power tx_power_index_C; / 0x64,
    pub /: *mut *mut rtl8723bu_efuse_tx_power tx_power_index_D; / 0x8e,
    pub /: *mut *mut u8 channel_plan; / 0xb8,
    pub xtal_k: u8,
    pub thermal_meter: u8,
    pub iqk_lck: u8,
    pub /: *mut *mut u8 pa_type; / 0xbc,
    pub /: *mut *mut u8 lna_type_2g; / 0xbd,
    pub res2: [u8; 3],
    pub rf_board_option: u8,
    pub rf_feature_option: u8,
    pub rf_bt_setting: u8,
    pub eeprom_version: u8,
    pub eeprom_customer_id: u8,
    pub res3: [u8; 2],
    pub tx_pwr_calibrate_rate: u8,
    pub /: *mut *mut u8 rf_antenna_option; / 0xc9,
    pub rfe_option: u8,
    pub res4: [u8; 9],
    pub usb_optional_function: u8,
    pub res5: [u8; 0x1e],
    pub res6: [u8; 2],
    pub /: *mut *mut u8 serial[0x0b]; / 0xf5,
    pub /: *mut *mut u8 vid; / 0x100,
    pub res7: u8,
    pub pid: u8,
    pub res8: [u8; 4],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x107,
    pub res9: [u8; 2],
    pub vendor_name: [u8; 0x07],
    pub res10: [u8; 2],
    pub device_name: [u8; 0x14],
    pub res11: [u8; 0xcf],
    pub /: *mut *mut u8 package_type; / 0x1fb,
    pub res12: [u8; 0x4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8192eu_efuse_tx_power {
    pub cck_base: [u8; 6],
    pub ht40_base: [u8; 5],
    pub ht20_ofdm_1s_diff: rtl8723au_idx,
    pub pwr_diff: [rtl8723bu_pwr_idx; 3],
    pub /: *mut *mut u8 dummy5g[24]; / max channel group (14) + power diff offset (10),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8192eu_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 0x0e],
    pub /: *mut *mut rtl8192eu_efuse_tx_power tx_power_index_A; / 0x10,
    pub /: *mut *mut rtl8192eu_efuse_tx_power tx_power_index_B; / 0x3a,
    pub res2: [u8; 0x54],
    pub /: *mut *mut u8 channel_plan; / 0xb8,
    pub xtal_k: u8,
    pub thermal_meter: u8,
    pub iqk_lck: u8,
    pub /: *mut *mut u8 pa_type; / 0xbc,
    pub /: *mut *mut u8 lna_type_2g; / 0xbd,
    pub res3: [u8; 1],
    pub /: *mut *mut u8 lna_type_5g; / 0xbf,
    pub res4: [u8; 1],
    pub rf_board_option: u8,
    pub rf_feature_option: u8,
    pub rf_bt_setting: u8,
    pub eeprom_version: u8,
    pub eeprom_customer_id: u8,
    pub res5: [u8; 3],
    pub /: *mut *mut u8 rf_antenna_option; / 0xc9,
    pub res6: [u8; 6],
    pub /: *mut *mut u8 vid; / 0xd0,
    pub res7: [u8; 1],
    pub /: *mut *mut u8 pid; / 0xd2,
    pub res8: [u8; 1],
    pub usb_optional_function: u8,
    pub res9: [u8; 2],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0xd7,
    pub device_info: [u8; 80],
    pub res11: [u8; 3],
    pub /: *mut *mut u8 unknown[0x0d]; / 0x130,
    pub res12: [u8; 0xc3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8188fu_efuse_tx_power {
    pub cck_base: [u8; 6],
    pub ht40_base: [u8; 5],
// a: ofdm; b: ht20
    pub ht20_ofdm_1s_diff: rtl8723au_idx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8188fu_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 0x0e],
    pub /: *mut *mut rtl8188fu_efuse_tx_power tx_power_index_A; / 0x10,
    pub /: *mut *mut u8 res1[0x9c]; / 0x1c,
    pub /: *mut *mut u8 channel_plan; / 0xb8,
    pub xtal_k: u8,
    pub thermal_meter: u8,
    pub iqk_lck: u8,
    pub res2: [u8; 5],
    pub rf_board_option: u8,
    pub rf_feature_option: u8,
    pub rf_bt_setting: u8,
    pub eeprom_version: u8,
    pub eeprom_customer_id: u8,
    pub res3: [u8; 2],
    pub kfree_thermal_k_on: u8,
    pub /: *mut *mut u8 rf_antenna_option; / 0xc9,
    pub rfe_option: u8,
    pub country_code: u8,
    pub res4: [u8; 4],
    pub /: *mut *mut u8 vid; / 0xd0,
    pub res5: [u8; 1],
    pub /: *mut *mut u8 pid; / 0xd2,
    pub res6: [u8; 1],
    pub usb_optional_function: u8,
    pub res7: [u8; 2],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0xd7,
    pub res8: [u8; 2],
    pub vendor_name: [u8; 7],
    pub res9: [u8; 2],
    pub /: *mut *mut u8 device_name[7]; / 0xe8,
    pub res10: [u8; 0x41],
    pub /: *mut *mut u8 unknown[0x0d]; / 0x130,
    pub res11: [u8; 0xc3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8188eu_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 0x0e],
    pub /: *mut *mut rtl8192eu_efuse_tx_power tx_power_index_A; / 0x10,
    pub /: *mut *mut u8 res1[0x7e]; / 0x3a,
    pub /: *mut *mut u8 channel_plan; / 0xb8,
    pub xtal_k: u8,
    pub thermal_meter: u8,
    pub iqk_lck: u8,
    pub res2: [u8; 5],
    pub rf_board_option: u8,
    pub rf_feature_option: u8,
    pub rf_bt_setting: u8,
    pub eeprom_version: u8,
    pub eeprom_customer_id: u8,
    pub res3: [u8; 3],
    pub /: *mut *mut u8 rf_antenna_option; / 0xc9,
    pub res4: [u8; 6],
    pub /: *mut *mut u8 vid; / 0xd0,
    pub res5: [u8; 1],
    pub /: *mut *mut u8 pid; / 0xd2,
    pub res6: [u8; 1],
    pub usb_optional_function: u8,
    pub res7: [u8; 2],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0xd7,
    pub res8: [u8; 2],
    pub vendor_name: [u8; 7],
    pub res9: [u8; 2],
    pub /: *mut *mut u8 device_name[0x0b]; / 0xe8,
    pub res10: [u8; 2],
    pub /: *mut *mut u8 serial[0x0b]; / 0xf5,
    pub res11: [u8; 0x30],
    pub /: *mut *mut u8 unknown[0x0d]; / 0x130,
    pub res12: [u8; 0xc3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8710bu_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 0x1e],
    pub /: *mut *mut rtl8188fu_efuse_tx_power tx_power_index_A; / 0x20,
    pub /: *mut *mut u8 res1[0x9c]; / 0x2c,
    pub /: *mut *mut u8 channel_plan; / 0xc8,
    pub /: *mut *mut u8 xtal_k; / 0xc9,
    pub /: *mut *mut u8 thermal_meter; / 0xca,
    pub res2: [u8; 0x4f],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x11a,
    pub res3: [u8; 0x11],
    pub /: *mut *mut u8 rf_board_option; / 0x131,
    pub res4: [u8; 2],
    pub /: *mut *mut u8 eeprom_version; / 0x134,
    pub /: *mut *mut u8 eeprom_customer_id; / 0x135,
    pub res5: [u8; 5],
    pub /: *mut *mut u8 country_code; / 0x13b,
    pub res6: [u8; 0x84],
    pub /: *mut *mut u8 vid[2]; / 0x1c0,
    pub /: *mut *mut u8 pid[2]; / 0x1c2,
    pub res7: [u8; 0x3c],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8192fu_efuse {
    pub rtl_id: __le16,
    pub res0: [u8; 0x0e],
    pub /: *mut *mut rtl8192eu_efuse_tx_power tx_power_index_A; / 0x10,
    pub /: *mut *mut rtl8192eu_efuse_tx_power tx_power_index_B; / 0x3a,
    pub res2: [u8; 0x54],
    pub /: *mut *mut u8 channel_plan; / 0xb8,
    pub /: *mut *mut u8 xtal_k; / 0xb9,
    pub /: *mut *mut u8 thermal_meter; / 0xba,
    pub /: *mut *mut u8 iqk_lck; / 0xbb,
    pub /: *mut *mut u8 pa_type; / 0xbc,
    pub /: *mut *mut u8 lna_type_2g; / 0xbd,
    pub res3: [u8; 1],
    pub /: *mut *mut u8 lna_type_5g; / 0xbf,
    pub res4: [u8; 1],
    pub /: *mut *mut u8 rf_board_option; / 0xc1,
    pub /: *mut *mut u8 rf_feature_option; / 0xc2,
    pub /: *mut *mut u8 rf_bt_setting; / 0xc3,
    pub /: *mut *mut u8 eeprom_version; / 0xc4,
    pub /: *mut *mut u8 eeprom_customer_id; / 0xc5,
    pub res5: [u8; 3],
    pub /: *mut *mut u8 rf_antenna_option; / 0xc9,
    pub /: *mut *mut u8 rfe_option; / 0xca,
    pub /: *mut *mut u8 country_code; / 0xcb,
    pub res6: [u8; 52],
    pub /: *mut *mut u8 vid[2]; / 0x100,
    pub /: *mut *mut u8 pid[2]; / 0x102,
    pub /: *mut *mut u8 usb_optional_function; / 0x104,
    pub res7: [u8; 2],
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / 0x107,
    pub /: *mut *mut u8 device_info[80]; / 0x10d,
    pub res9: [u8; 163],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_reg8val {
    pub reg: u16,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_reg32val {
    pub reg: u16,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_rfregval {
    pub reg: u8,
    pub val: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8xxxu_rfpath {
    RF_A = 0,
    RF_B = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_rfregs {
    pub hssiparm1: u16,
    pub hssiparm2: u16,
    pub lssiparm: u16,
    pub hspiread: u16,
    pub lssiread: u16,
    pub rf_sw_ctrl: u16,
}

pub const H2C_MAX_MBOX: c_int = 4;

pub const H2C_JOIN_BSS_DISCONNECT: c_int = 0;
pub const H2C_JOIN_BSS_CONNECT: c_int = 1;
pub const H2C_MACID_ROLE_STA: c_int = 1;
pub const H2C_MACID_ROLE_AP: c_int = 2;
//
// H2C (firmware) commands differ between the older generation chips
// 8188[cr]u, 819[12]cu, and 8723au, and the more recent chips 8723bu,
// 8192[de]u, 8192eu, and 8812.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum h2c_cmd_8723a {
    H2C_SET_POWER_MODE = 1,
    H2C_JOIN_BSS_REPORT = 2,
    H2C_SET_RSSI = 5,
    H2C_SET_RATE_MASK = (6 | H2C_EXT),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum h2c_cmd_8723b {
//
// Common Class: 000
//
    H2C_8723B_RSVD_PAGE = 0x00,
    H2C_8723B_MEDIA_STATUS_RPT = 0x01,
    H2C_8723B_SCAN_ENABLE = 0x02,
    H2C_8723B_KEEP_ALIVE = 0x03,
    H2C_8723B_DISCON_DECISION = 0x04,
    H2C_8723B_PSD_OFFLOAD = 0x05,
    H2C_8723B_AP_OFFLOAD = 0x08,
    H2C_8723B_BCN_RSVDPAGE = 0x09,
    H2C_8723B_PROBERSP_RSVDPAGE = 0x0A,
    H2C_8723B_FCS_RSVDPAGE = 0x10,
    H2C_8723B_FCS_INFO = 0x11,
    H2C_8723B_AP_WOW_GPIO_CTRL = 0x13,

//
// PoweSave Class: 001
//
    H2C_8723B_SET_PWR_MODE = 0x20,
    H2C_8723B_PS_TUNING_PARA = 0x21,
    H2C_8723B_PS_TUNING_PARA2 = 0x22,
    H2C_8723B_P2P_LPS_PARAM = 0x23,
    H2C_8723B_P2P_PS_OFFLOAD = 0x24,
    H2C_8723B_PS_SCAN_ENABLE = 0x25,
    H2C_8723B_SAP_PS_ = 0x26,
    H2C_8723B_INACTIVE_PS_ = 0x27,
    H2C_8723B_FWLPS_IN_IPS_ = 0x28,

//
// Dynamic Mechanism Class: 010
//
    H2C_8723B_MACID_CFG_RAID = 0x40,
    H2C_8723B_TXBF = 0x41,
    H2C_8723B_RSSI_SETTING = 0x42,
    H2C_8723B_AP_REQ_TXRPT = 0x43,
    H2C_8723B_INIT_RATE_COLLECT = 0x44,

//
// BT Class: 011
//
    H2C_8723B_B_TYPE_TDMA = 0x60,
    H2C_8723B_BT_INFO = 0x61,
    H2C_8723B_FORCE_BT_TXPWR = 0x62,
    H2C_8723B_BT_IGNORE_WLANACT = 0x63,
    H2C_8723B_DAC_SWING_VALUE = 0x64,
    H2C_8723B_ANT_SEL_RSV = 0x65,
    H2C_8723B_WL_OPMODE = 0x66,
    H2C_8723B_BT_MP_OPER = 0x67,
    H2C_8723B_BT_CONTROL = 0x68,
    H2C_8723B_BT_WIFI_CTRL = 0x69,
    H2C_8723B_BT_FW_PATCH = 0x6a,
    H2C_8723B_BT_WLAN_CALIBRATION = 0x6d,
    H2C_8723B_BT_GRANT = 0x6e,

//
// WOWLAN Class: 100
//
    H2C_8723B_WOWLAN = 0x80,
    H2C_8723B_REMOTE_WAKE_CTRL = 0x81,
    H2C_8723B_AOAC_GLOBAL_INFO = 0x82,
    H2C_8723B_AOAC_RSVD_PAGE = 0x83,
    H2C_8723B_AOAC_RSVD_PAGE2 = 0x84,
    H2C_8723B_D0_SCAN_OFFLOAD_CTRL = 0x85,
    H2C_8723B_D0_SCAN_OFFLOAD_INFO = 0x86,
    H2C_8723B_CHNL_SWITCH_OFFLOAD = 0x87,

    H2C_8723B_RESET_TSF = 0xC0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h2c_cmd {
    pub cmd: u8,
    pub data: [u8; 7],
    pub cmd: } __packed,
    pub data: __le32,
    pub ext: __le16,
    pub raw: } __packed,
    pub data: __le32,
    pub ext: __le32,
    pub raw_wide: } __packed,
    pub cmd: u8,
    pub data: u8,
    pub joinbss: } __packed,
    pub cmd: u8,
    pub mask_hi: __le16,
    pub arg: u8,
    pub mask_lo: __le16,
    pub ramask: } __packed,
    pub cmd: u8,
    pub parm: u8,
    pub macid: u8,
    pub macid_end: u8,
    pub media_status_rpt: } __packed,
    pub cmd: u8,
    pub macid: u8,
//
// [0:4] - RAID
// [7]   - SGI
//
    pub data1: u8,
//
// [0:1] - Bandwidth
// [3]   - No Update
// [4:5] - VHT enable
// [6]   - DISPT
// [7]   - DISRA
//
    pub data2: u8,
    pub ramask0: u8,
    pub ramask1: u8,
    pub ramask2: u8,
    pub ramask3: u8,
    pub b_macid_cfg: } __packed,
    pub cmd: u8,
    pub data1: u8,
    pub data2: u8,
    pub data3: u8,
    pub data4: u8,
    pub data5: u8,
    pub b_type_dma: } __packed,
    pub cmd: u8,
    pub data: u8,
    pub bt_info: } __packed,
    pub cmd: u8,
    pub operreq: u8,
    pub opcode: u8,
    pub data: u8,
    pub addr: u8,
    pub bt_mp_oper: } __packed,
    pub cmd: u8,
    pub data: u8,
    pub bt_wlan_calibration: } __packed,
    pub cmd: u8,
    pub data: u8,
    pub ignore_wlan: } __packed,
    pub cmd: u8,
    pub ant_inverse: u8,
    pub int_switch_type: u8,
    pub ant_sel_rsv: } __packed,
    pub cmd: u8,
    pub data: u8,
    pub bt_grant: } __packed,
    pub cmd: u8,
    pub macid: u8,
    pub unknown0: u8,
    pub rssi: u8,
//
// [0]   - is_rx
// [1]   - stbc_en
// [2]   - noisy_decision
// [6]   - bf_en
//
    pub data: u8,
//
// [0:6] - ra_th_offset
// [7]   - ra_offset_direction
//
    pub ra_th_offset: u8,
    pub unknown1: u8,
    pub unknown2: u8,
    pub rssi_report: } __packed,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum c2h_evt_8723b {
    C2H_8723B_DEBUG = 0,
    C2H_8723B_TSF = 1,
    C2H_8723B_AP_RPT_RSP = 2,
    C2H_8723B_CCX_TX_RPT = 3,
    C2H_8723B_BT_RSSI = 4,
    C2H_8723B_BT_OP_MODE = 5,
    C2H_8723B_EXT_RA_RPT = 6,
    C2H_8723B_BT_INFO = 9,
    C2H_8723B_HW_INFO_EXCH = 0x0a,
    C2H_8723B_BT_MP_INFO = 0x0b,
    C2H_8723B_RA_REPORT = 0x0c,
    C2H_8723B_FW_DEBUG = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_info_src_8723b {
    BT_INFO_SRC_8723B_WIFI_FW = 0x0,
    BT_INFO_SRC_8723B_BT_RSP = 0x1,
    BT_INFO_SRC_8723B_BT_ACTIVE_SEND = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_mp_oper_opcode_8723b {
    BT_MP_OP_GET_BT_VERSION	= 0x00,
    BT_MP_OP_RESET = 0x01,
    BT_MP_OP_TEST_CTRL = 0x02,
    BT_MP_OP_SET_BT_MODE = 0x03,
    BT_MP_OP_SET_CHNL_TX_GAIN = 0x04,
    BT_MP_OP_SET_PKT_TYPE_LEN = 0x05,
    BT_MP_OP_SET_PKT_CNT_L_PL_TYPE = 0x06,
    BT_MP_OP_SET_PKT_CNT_H_PKT_INTV = 0x07,
    BT_MP_OP_SET_PKT_HEADER = 0x08,
    BT_MP_OP_SET_WHITENCOEFF = 0x09,
    BT_MP_OP_SET_BD_ADDR_L = 0x0a,
    BT_MP_OP_SET_BD_ADDR_H = 0x0b,
    BT_MP_OP_WRITE_REG_ADDR = 0x0c,
    BT_MP_OP_WRITE_REG_VALUE = 0x0d,
    BT_MP_OP_GET_BT_STATUS = 0x0e,
    BT_MP_OP_GET_BD_ADDR_L = 0x0f,
    BT_MP_OP_GET_BD_ADDR_H = 0x10,
    BT_MP_OP_READ_REG = 0x11,
    BT_MP_OP_SET_TARGET_BD_ADDR_L = 0x12,
    BT_MP_OP_SET_TARGET_BD_ADDR_H = 0x13,
    BT_MP_OP_SET_TX_POWER_CALIBRATION = 0x14,
    BT_MP_OP_GET_RX_PKT_CNT_L = 0x15,
    BT_MP_OP_GET_RX_PKT_CNT_H = 0x16,
    BT_MP_OP_GET_RX_ERROR_BITS_L = 0x17,
    BT_MP_OP_GET_RX_ERROR_BITS_H = 0x18,
    BT_MP_OP_GET_RSSI = 0x19,
    BT_MP_OP_GET_CFO_HDR_QUALITY_L = 0x1a,
    BT_MP_OP_GET_CFO_HDR_QUALITY_H = 0x1b,
    BT_MP_OP_GET_TARGET_BD_ADDR_L = 0x1c,
    BT_MP_OP_GET_TARGET_BD_ADDR_H = 0x1d,
    BT_MP_OP_GET_AFH_MAP_L = 0x1e,
    BT_MP_OP_GET_AFH_MAP_M = 0x1f,
    BT_MP_OP_GET_AFH_MAP_H = 0x20,
    BT_MP_OP_GET_AFH_STATUS = 0x21,
    BT_MP_OP_SET_TRACKING_INTERVAL = 0x22,
    BT_MP_OP_SET_THERMAL_METER = 0x23,
    BT_MP_OP_ENABLE_CFO_TRACKING = 0x24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl8xxxu_bw_mode {
    RTL8XXXU_CHANNEL_WIDTH_20 = 0,
    RTL8XXXU_CHANNEL_WIDTH_40 = 1,
    RTL8XXXU_CHANNEL_WIDTH_80 = 2,
    RTL8XXXU_CHANNEL_WIDTH_160 = 3,
    RTL8XXXU_CHANNEL_WIDTH_80_80 = 4,
    RTL8XXXU_CHANNEL_WIDTH_MAX = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8723bu_c2h {
    pub id: u8,
    pub seq: u8,
    pub payload: [u8; 0],
    pub raw: } __packed,
    pub ext_id: u8,
    pub status:4: u8,
    pub retlen:4: u8,
    pub opcode_ver:4: u8,
    pub req_num:4: u8,
    pub payload: [u8; 2],
    pub bt_mp_info: } __packed,
    pub response_source:4: u8,
    pub dummy0_0:4: u8,
    pub bt_info: u8,
    pub retry_count:4: u8,
    pub dummy2_0:1: u8,
    pub bt_page:1: u8,
    pub tx_rx_mask:1: u8,
    pub dummy2_2:1: u8,
    pub rssi: u8,
    pub basic_rate:1: u8,
    pub bt_has_reset:1: u8,
    pub dummy4_1:1: u8,
    pub ignore_wlan:1: u8,
    pub auto_report:1: u8,
    pub dummy4_2:3: u8,
    pub a4: u8,
    pub a5: u8,
    pub bt_info: } __packed,
    pub rate:7: u8,
    pub sgi:1: u8,
    pub macid: u8,
    pub ldpc:1: u8,
    pub txbf:1: u8,
    pub noisy_state:1: u8,
    pub dummy2_0:5: u8,
    pub dummy3_0: u8,
    pub dummy4_0: u8,
    pub dummy5_0: u8,
    pub bw: u8,
    pub ra_report: } __packed,
}

// mlme related.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wireless_mode {
    WIRELESS_MODE_UNKNOWN = 0,
// Sub-Element
    WIRELESS_MODE_B = BIT(0),
    WIRELESS_MODE_G = BIT(1),
    WIRELESS_MODE_A = BIT(2),
    WIRELESS_MODE_N_24G = BIT(3),
    WIRELESS_MODE_N_5G = BIT(4),
    WIRELESS_AUTO = BIT(5),
    WIRELESS_MODE_AC = BIT(6),
    WIRELESS_MODE_MAX = 0x7F,
}

// from rtlwifi/wifi.h
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ratr_table_mode_new {
    RATEID_IDX_BGN_40M_2SS = 0,
    RATEID_IDX_BGN_40M_1SS = 1,
    RATEID_IDX_BGN_20M_2SS_BN = 2,
    RATEID_IDX_BGN_20M_1SS_BN = 3,
    RATEID_IDX_GN_N2SS = 4,
    RATEID_IDX_GN_N1SS = 5,
    RATEID_IDX_BG = 6,
    RATEID_IDX_G = 7,
    RATEID_IDX_B = 8,
    RATEID_IDX_VHT_2SS = 9,
    RATEID_IDX_VHT_1SS = 10,
    RATEID_IDX_MIX1 = 11,
    RATEID_IDX_MIX2 = 12,
    RATEID_IDX_VHT_3SS = 13,
    RATEID_IDX_BGN_3SS = 14,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _BT_8723B_1ANT_STATUS {
    BT_8723B_1ANT_STATUS_NON_CONNECTED_IDLE      = 0x0,
    BT_8723B_1ANT_STATUS_CONNECTED_IDLE          = 0x1,
    BT_8723B_1ANT_STATUS_INQ_PAGE                = 0x2,
    BT_8723B_1ANT_STATUS_ACL_BUSY                = 0x3,
    BT_8723B_1ANT_STATUS_SCO_BUSY                = 0x4,
    BT_8723B_1ANT_STATUS_ACL_SCO_BUSY            = 0x5,
    BT_8723B_1ANT_STATUS_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_btcoex {
    pub bt_status: u8,
    pub bt_busy: bool,
    pub has_sco: bool,
    pub has_a2dp: bool,
    pub has_hid: bool,
    pub has_pan: bool,
    pub hid_only: bool,
    pub a2dp_only: bool,
    pub c2h_bt_inquiry: bool,
}

pub const RTL8XXXU_RATR_STA_INIT: c_int = 0;
pub const RTL8XXXU_RATR_STA_HIGH: c_int = 1;
pub const RTL8XXXU_RATR_STA_MID: c_int = 2;
pub const RTL8XXXU_RATR_STA_LOW: c_int = 3;

pub const RTL8XXXU_SNR_THRESH_HIGH: c_int = 50;
pub const RTL8XXXU_SNR_THRESH_LOW: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_ra_report {
    pub txrate: rate_info,
    pub bit_rate: u32,
    pub desc_rate: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_ra_info {
    pub rate_id: u8,
    pub rate_mask: u32,
    pub ra_use_rate: u32,
    pub rate_sgi: u8,
    pub /: *mut *mut u8 rssi_sta_ra; / Percentage,
    pub pre_rssi_sta_ra: u8,
    pub sgi_enable: u8,
    pub decision_rate: u8,
    pub pre_rate: u8,
    pub highest_rate: u8,
    pub lowest_rate: u8,
    pub nsc_up: u32,
    pub nsc_down: u32,
    pub total: u32,
    pub retry: [u16; 5],
    pub drop: u16,
    pub rpt_time: u16,
    pub pre_min_rpt_time: u16,
    pub dynamic_tx_rpt_timing_counter: u8,
    pub ra_waiting_counter: u8,
    pub ra_pending_counter: u8,
    pub ra_drop_after_down: u8,
    pub /: *mut *mut u8 pt_try_state; / 0 trying state, 1 for decision state,
    pub /: *mut *mut u8 pt_stage; / 0~6,
    pub /: *mut *mut u8 pt_stop_count; / Stop PT counter,
    pub /: *mut *mut u8 pt_pre_rate; / if rate change do PT,
    pub /: *mut *mut u8 pt_pre_rssi; / if RSSI change 5% do PT,
    pub /: *mut *mut u8 pt_mode_ss; / decide which rate should do PT,
    pub /: *mut *mut u8 ra_stage; / StageRA, decide how many times RA will be done between PT,
    pub pt_smooth_factor: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_cfo_tracking {
    pub adjust: bool,
    pub atc_status: bool,
    pub cfo_tail: [c_int; 2],
    pub crystal_cap: u8,
    pub packet_count: u32,
    pub packet_count_pre: u32,
}

pub const RTL8XXXU_HW_LED_CONTROL: c_int = 2;
pub const RTL8XXXU_MAX_MAC_ID_NUM: c_int = 128;
pub const RTL8XXXU_BC_MC_MACID: c_int = 0;
pub const RTL8XXXU_BC_MC_MACID1: c_int = 1;
pub const RTL8XXXU_MAX_SEC_CAM_NUM: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_hw_feature {
    pub max_bw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_priv {
    pub hw: *mut ieee80211_hw,
    pub udev: *mut usb_device,
    pub fops: *mut rtl8xxxu_fileops,
    pub hw_feature: rtl8xxxu_hw_feature,
    pub tx_urb_lock: spinlock_t,
    pub tx_urb_free_list: list_head,
    pub tx_urb_free_count: c_int,
    pub tx_stopped: bool,
    pub rx_urb_lock: spinlock_t,
    pub rx_urb_pending_list: list_head,
    pub rx_urb_pending_count: c_int,
    pub shutdown: bool,
    pub rx_urb_wq: work_struct,
    pub mac_addr: [u8; ETH_ALEN],
    pub chip_name: [c_char; 8],
    pub chip_vendor: [c_char; 8],
    pub cck_tx_power_index_A: [u8; RTL8XXXU_MAX_CHANNEL_GROUPS],
    pub cck_tx_power_index_B: [u8; RTL8XXXU_MAX_CHANNEL_GROUPS],
    pub ht40_1s_tx_power_index_A: [u8; RTL8XXXU_MAX_CHANNEL_GROUPS],
    pub ht40_1s_tx_power_index_B: [u8; RTL8XXXU_MAX_CHANNEL_GROUPS],
//
// The following entries are half-bytes split as:
// bits 0-3: path A, bits 4-7: path B, all values 4 bits signed
//
    pub ht20_tx_power_index_diff: [rtl8723au_idx; RTL8723A_CHANNEL_GROUPS],
    pub ofdm_tx_power_index_diff: [rtl8723au_idx; RTL8723A_CHANNEL_GROUPS],
    pub ht40_max_power_offset: [rtl8723au_idx; RTL8723A_CHANNEL_GROUPS],
    pub ht20_max_power_offset: [rtl8723au_idx; RTL8723A_CHANNEL_GROUPS],
//
// Newer generation chips only keep power diffs per TX count,
// not per channel group.
//
    pub ofdm_tx_power_diff: [rtl8723au_idx; RTL8723B_TX_COUNT],
    pub ht20_tx_power_diff: [rtl8723au_idx; RTL8723B_TX_COUNT],
    pub ht40_tx_power_diff: [rtl8723au_idx; RTL8723B_TX_COUNT],
    pub power_base: *mut rtl8xxxu_power_base,
    pub package_type: u8,
    pub chip_cut:4: u32,
    pub rom_rev:4: u32,
    pub is_multi_func:1: u32,
    pub has_wifi:1: u32,
    pub has_bluetooth:1: u32,
    pub enable_bluetooth:1: u32,
    pub has_gps:1: u32,
    pub hi_pa:1: u32,
    pub vendor_umc:1: u32,
    pub vendor_smic:1: u32,
    pub has_polarity_ctrl:1: u32,
    pub has_eeprom:1: u32,
    pub boot_eeprom:1: u32,
    pub usb_interrupts:1: u32,
    pub ep_tx_high_queue:1: u32,
    pub ep_tx_normal_queue:1: u32,
    pub ep_tx_low_queue:1: u32,
    pub rx_buf_aggregation:1: u32,
    pub cck_agc_report_type:1: u32,
    pub cck_new_agc:1: u32,
    pub default_crystal_cap: u8,
    pub rfe_type: u8,
    pub pipe_interrupt: c_uint,
    pub pipe_in: c_uint,
    pub pipe_out: [c_uint; TXDESC_QUEUE_MAX],
    pub out_ep: [u8; RTL8XXXU_OUT_ENDPOINTS],
    pub ep_tx_count: u8,
    pub rf_paths: u8,
    pub rx_paths: u8,
    pub tx_paths: u8,
    pub rege94: u32,
    pub rege9c: u32,
    pub regeb4: u32,
    pub regebc: u32,
    pub regrcr: u32,
    pub next_mbox: c_int,
    pub nr_out_eps: c_int,
// Ensure no added or deleted stas while iterating
    pub sta_mutex: mutex,
    pub h2c_mutex: mutex,
// Protect the indirect register accesses of RTL8710BU.
    pub syson_indirect_access_mutex: mutex,
    pub rx_anchor: usb_anchor,
    pub tx_anchor: usb_anchor,
    pub int_anchor: usb_anchor,
    pub fw_data: *mut rtl8xxxu_firmware_header,
    pub fw_size: usize,
    pub usb_buf_mutex: mutex,
    pub val32: __le32,
    pub val16: __le16,
    pub val8: u8,
    pub usb_buf: },
    pub raw: [u8; EFUSE_MAP_LEN],
    pub efuse8723: rtl8723au_efuse,
    pub efuse8723bu: rtl8723bu_efuse,
    pub efuse8192: rtl8192cu_efuse,
    pub efuse8192eu: rtl8192eu_efuse,
    pub efuse8188fu: rtl8188fu_efuse,
    pub efuse8188eu: rtl8188eu_efuse,
    pub efuse8710bu: rtl8710bu_efuse,
    pub efuse8192fu: rtl8192fu_efuse,
    pub efuse_wifi: },
    pub adda_backup: [u32; RTL8XXXU_ADDA_REGS],
    pub mac_backup: [u32; RTL8XXXU_MAC_REGS],
    pub bb_backup: [u32; RTL8XXXU_BB_REGS],
    pub bb_recovery_backup: [u32; RTL8XXXU_BB_REGS],
    pub rtl_chip: rtl8xxxu_rtl_chip,
    pub pi_enabled:1: u8,
    pub no_pape:1: u8,
    pub int_buf: [u8; USB_INTR_CONTENT_LENGTH],
    pub IEEE80211_NUM_TIDS): DECLARE_BITMAP(tx_aggr_started,,
    pub IEEE80211_NUM_TIDS): DECLARE_BITMAP(tid_tx_operational,,
    pub vifs: [*mut ieee80211_vif; 2],
    pub ra_watchdog: delayed_work,
    pub c2hcmd_work: work_struct,
    pub c2hcmd_queue: sk_buff_head,
    pub update_beacon_work: delayed_work,
    pub bt_coex: rtl8xxxu_btcoex,
    pub ra_report: rtl8xxxu_ra_report,
    pub cfo_tracking: rtl8xxxu_cfo_tracking,
    pub ra_info: rtl8xxxu_ra_info,
    pub led_registered: bool,
    pub led_name: [c_char; 32],
    pub led_cdev: led_classdev,
    pub RTL8XXXU_MAX_MAC_ID_NUM): DECLARE_BITMAP(mac_id_map,,
    pub RTL8XXXU_MAX_SEC_CAM_NUM): DECLARE_BITMAP(cam_map,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_sta_info {
    pub sta: *mut ieee80211_sta,
    pub vif: *mut ieee80211_vif,
    pub macid: u8,
    pub avg_rssi: ewma_rssi,
    pub rssi_level: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_vif {
    pub port_num: c_int,
    pub hw_key_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_rx_urb {
    pub urb: urb,
    pub hw: *mut ieee80211_hw,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_tx_urb {
    pub urb: urb,
    pub hw: *mut ieee80211_hw,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8xxxu_fileops {
    pub priv): *mut *mut int (identify_chip) (struct rtl8xxxu_priv,
    pub priv): *mut *mut int (read_efuse) (struct rtl8xxxu_priv,
    pub priv): *mut *mut int (parse_efuse) (struct rtl8xxxu_priv,
    pub priv): *mut *mut int (load_firmware) (struct rtl8xxxu_priv,
    pub priv): *mut *mut int (power_on) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (power_off) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (reset_8051) (struct rtl8xxxu_priv,
    pub priv): *mut *mut int (llt_init) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (init_phy_bb) (struct rtl8xxxu_priv,
    pub priv): *mut *mut int (init_phy_rf) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (phy_init_antenna_selection) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (phy_lc_calibrate) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (phy_iq_calibrate) (struct rtl8xxxu_priv,
    pub hw): *mut *mut void (config_channel) (struct ieee80211_hw,
    pub skb): *mut *mut *mut int (parse_rx_desc) (struct rtl8xxxu_priv priv, struct sk_buff,
    pub crc_icv_err): bool,
    pub priv): *mut *mut void (init_aggregation) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (init_statistics) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (init_burst) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (enable_rf) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (disable_rf) (struct rtl8xxxu_priv,
    pub priv): *mut *mut void (usb_quirks) (struct rtl8xxxu_priv,
    pub ht40): bool,
    pub macid): u8,
    pub connect): u8 macid, u8 role, bool,
    pub rssi): *mut *mut *mut void (report_rssi) (struct rtl8xxxu_priv priv, u8 macid, u8,
    pub macid): u32 rts_rate, u8,
    pub crystal_cap): *mut *mut *mut void (set_crystal_cap) (struct rtl8xxxu_priv priv, u8,
    pub phy_stats): *mut *mut *mut s8 (cck_rssi) (struct rtl8xxxu_priv priv, struct rtl8723au_phy_stats,
    pub brightness): led_brightness,
    pub writeN_block_size: c_int,
    pub rx_agg_buf_size: c_int,
    pub tx_desc_size: c_char,
    pub rx_desc_size: c_char,
    pub has_s0s1:1: u8,
    pub has_tx_report:1: u8,
    pub gen2_thermal_meter:1: u8,
    pub needs_full_init:1: u8,
    pub init_reg_rxfltmap:1: u8,
    pub init_reg_pkt_life_time:1: u8,
    pub init_reg_hmtfr:1: u8,
    pub supports_concurrent:1: u8,
    pub hw_feature_report:1: u8,
    pub ampdu_max_time: u8,
    pub ustime_tsf_edca: u8,
    pub max_aggr_num: u16,
    pub supports_ap:1: u8,
    pub max_macid_num: u16,
    pub max_sec_cam_num: u16,
    pub adda_1t_init: u32,
    pub adda_1t_path_on: u32,
    pub adda_2t_path_on_a: u32,
    pub adda_2t_path_on_b: u32,
    pub trxff_boundary: u16,
    pub pbp_rx: u8,
    pub pbp_tx: u8,
    pub mactable: *const rtl8xxxu_reg8val,
    pub total_page_num: u8,
    pub page_num_hi: u8,
    pub page_num_lo: u8,
    pub page_num_norm: u8,
    pub last_llt_entry: u8,
}

extern "C" {
    pub fn rtl8xxxu_read8(priv: *mut rtl8xxxu_priv, addr: u16) -> u8;
}
extern "C" {
    pub fn rtl8xxxu_read16(priv: *mut rtl8xxxu_priv, addr: u16) -> u16;
}
extern "C" {
    pub fn rtl8xxxu_read32(priv: *mut rtl8xxxu_priv, addr: u16) -> u32;
}
extern "C" {
    pub fn rtl8xxxu_write8(priv: *mut rtl8xxxu_priv, addr: u16, val: u8) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_write16(priv: *mut rtl8xxxu_priv, addr: u16, val: u16) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_write32(priv: *mut rtl8xxxu_priv, addr: u16, val: u32) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_write8_set(priv: *mut rtl8xxxu_priv, addr: u16, bits: u8) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_write8_clear(priv: *mut rtl8xxxu_priv, addr: u16, bits: u8) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_write16_set(priv: *mut rtl8xxxu_priv, addr: u16, bits: u16) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_write16_clear(priv: *mut rtl8xxxu_priv, addr: u16, bits: u16) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_write32_set(priv: *mut rtl8xxxu_priv, addr: u16, bits: u32) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_write32_clear(priv: *mut rtl8xxxu_priv, addr: u16, bits: u32) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_load_firmware(priv: *mut rtl8xxxu_priv, fw_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_firmware_self_reset(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_identify_vendor_1bit(priv: *mut rtl8xxxu_priv, vendor: u32);
}
extern "C" {
    pub fn rtl8xxxu_identify_vendor_2bits(priv: *mut rtl8xxxu_priv, vendor: u32);
}
extern "C" {
    pub fn rtl8xxxu_config_endpoints_sie(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_config_endpoints_no_sie(priv: *mut rtl8xxxu_priv) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_read_efuse8(priv: *mut rtl8xxxu_priv, offset: u16, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_read_efuse(priv: *mut rtl8xxxu_priv) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_reset_8051(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_auto_llt_table(priv: *mut rtl8xxxu_priv) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_gen2_prepare_calibrate(priv: *mut rtl8xxxu_priv, start: u8);
}
extern "C" {
    pub fn rtl8723a_phy_lc_calibrate(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8188f_phy_lc_calibrate(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_flush_fifo(priv: *mut rtl8xxxu_priv) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_active_to_lps(priv: *mut rtl8xxxu_priv) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_disabled_to_emu(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_init_llt_table(priv: *mut rtl8xxxu_priv) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_gen1_phy_iq_calibrate(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_gen1_init_phy_bb(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8188f_channel_to_group(channel: c_int, group: *mut c_int, cck_group: *mut c_int);
}
extern "C" {
    pub fn rtl8xxxu_gen1_config_channel(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8xxxu_gen2_config_channel(hw: *mut ieee80211_hw);
}
extern "C" {
    pub fn rtl8xxxu_gen1_usb_quirks(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_gen2_usb_quirks(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_gen1_report_rssi(priv: *mut rtl8xxxu_priv, macid: u8, rssi: u8);
}
extern "C" {
    pub fn rtl8xxxu_gen2_report_rssi(priv: *mut rtl8xxxu_priv, macid: u8, rssi: u8);
}
extern "C" {
    pub fn rtl8xxxu_gen1_init_aggregation(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_gen1_enable_rf(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_gen1_disable_rf(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_gen2_disable_rf(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_init_burst(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8xxxu_parse_rxdesc16(priv: *mut rtl8xxxu_priv, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_parse_rxdesc24(priv: *mut rtl8xxxu_priv, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn rtl8xxxu_gen2_channel_to_group(channel: c_int) -> c_int;
}
extern "C" {
    pub fn rtl8723bu_phy_init_antenna_selection(priv: *mut rtl8xxxu_priv);
}
extern "C" {
    pub fn rtl8723a_set_crystal_cap(priv: *mut rtl8xxxu_priv, crystal_cap: u8);
}
extern "C" {
    pub fn rtl8188f_set_crystal_cap(priv: *mut rtl8xxxu_priv, crystal_cap: u8);
}
extern "C" {
    pub fn rtl8723a_cck_rssi(priv: *mut rtl8xxxu_priv, phy_stats: *mut rtl8723au_phy_stats) -> i8;
}
extern "C" {
    pub fn rtl8188e_ra_info_init_all(ra: *mut rtl8xxxu_ra_info);
}
extern "C" {
    pub fn rtl8188e_handle_ra_tx_report2(priv: *mut rtl8xxxu_priv, skb: *mut sk_buff);
}
