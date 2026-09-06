//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtw89/txrx.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright(c) 2020  Realtek Corporation
//

pub const DATA_RATE_MODE_NON_HT: c_uint = 0x0;

pub const DATA_RATE_MODE_HT: c_uint = 0x1;

pub const DATA_RATE_MODE_VHT: c_uint = 0x2;
pub const DATA_RATE_MODE_HE: c_uint = 0x3;
pub const DATA_RATE_MODE_EHT: c_uint = 0x4;
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_MODE_CTRL_MASK_V1) -> return;
}
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_MODE_CTRL_MASK) -> return;
}
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_NOT_HT_IDX_MASK) -> return;
}
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_HT_IDX_MASK_V1) -> return;
}
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_HT_IDX_MASK) -> return;
}
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_MCS_MASK_V1) -> return;
}
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_VHT_HE_IDX_MASK) -> return;
}
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_HT_NSS_MASK) -> return;
}
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_NSS_MASK_V1) -> return;
}
extern "C" {
    pub fn u16_get_bits(_arg: hw_rate, _arg: DATA_RATE_VHT_HE_NSS_MASK) -> return;
}
// TX WD BODY DWORD 0

// TX WD BODY DWORD 1

// TX WD BODY DWORD 2

// TX WD BODY DWORD 3

// TX WD BODY DWORD 4

// TX WD BODY DWORD 5

// TX WD BODY DWORD 6 (V1)
// TX WD BODY DWORD 7 (V1)

// TX WD INFO DWORD 0

// TX WD INFO DWORD 1

// TX WD INFO DWORD 2

// TX WD INFO DWORD 3

// TX WD INFO DWORD 4

// TX WD INFO DWORD 5
// TX WD BODY DWORD 0

// TX WD BODY DWORD 1

// TX WD BODY DWORD 2

// TX WD BODY DWORD 3

// TX WD BODY DWORD 4

// TX WD BODY DWORD 5

// TX WD BODY DWORD 6

// TX WD BODY DWORD 7

// TX WD INFO DWORD 0

// TX WD INFO DWORD 1

// TX WD INFO DWORD 2

// TX WD INFO DWORD 3

// TX WD INFO DWORD 4

// TX WD INFO DWORD 5

// TX WD INFO DWORD 6

// TX WD INFO DWORD 7

// RX WD dword0

// RX WD dword1

// RX WD dword2

// RX WD dword3

// RX WD dword4

// RX WD dword5

// RX WD dword6

// RX WD dword7

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rxinfo_user {
    pub w0: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_rxinfo {
    pub w0: __le32,
    pub w1: __le32,
    pub user: [rtw89_rxinfo_user; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_sts_hdr {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_sts_hdr_v2 {
    pub w0: __le32,
    pub w1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_sts_iehdr {
    pub w0: __le32,
}

// BE RXD dword0

// BE RXD dword1

// BE RXD dword2

// BE RXD dword3

// BE RXD dword4

// BE RXD dword5

// BE RXD dword6

// BE RXD dword7

// BE RXD dword8

// BE RXD dword9

// BE RXD - PHY RPT dword0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_sts_ie00 {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_sts_ie00_v2 {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_sts_ie01 {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_sts_ie01_v2 {
    pub w0: __le32,
    pub w1: __le32,
    pub w2: __le32,
    pub w3: __le32,
    pub w4: __le32,
    pub w5: __le32,
    pub w6: __le32,
    pub w7: __le32,
    pub w8: __le32,
    pub w9: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub union rtw89_phy_sts_ie09 {
    pub qw0: __le64,
    pub gen0: },
    pub qw0: __le64,
    pub qw1: __le64,
    pub gen2: },
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtw89_phy_sts_ie10 {
    pub qw0: __le64,
    pub sigb: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tx_channel {
    RTW89_TXCH_ACH0	= 0,
    RTW89_TXCH_ACH1	= 1,
    RTW89_TXCH_ACH2	= 2,
    RTW89_TXCH_ACH3	= 3,
    RTW89_TXCH_ACH4	= 4,
    RTW89_TXCH_ACH5	= 5,
    RTW89_TXCH_ACH6	= 6,
    RTW89_TXCH_ACH7	= 7,
    RTW89_TXCH_CH8	= 8,  /* MGMT Band 0 */
    RTW89_TXCH_CH9	= 9,  /* HI Band 0 */
    RTW89_TXCH_CH10	= 10, /* MGMT Band 1 */
    RTW89_TXCH_CH11	= 11, /* HI Band 1 */
    RTW89_TXCH_CH12	= 12, /* FW CMD */

// keep last
    RTW89_TXCH_NUM,
    RTW89_TXCH_MAX = RTW89_TXCH_NUM - 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_rx_channel {
    RTW89_RXCH_RXQ	= 0,
    RTW89_RXCH_RPQ	= 1,

// keep last
    RTW89_RXCH_NUM,
    RTW89_RXCH_MAX = RTW89_RXCH_NUM - 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtw89_tx_qsel {
    RTW89_TX_QSEL_BE_0		= 0x00,
    RTW89_TX_QSEL_BK_0		= 0x01,
    RTW89_TX_QSEL_VI_0		= 0x02,
    RTW89_TX_QSEL_VO_0		= 0x03,
    RTW89_TX_QSEL_BE_1		= 0x04,
    RTW89_TX_QSEL_BK_1		= 0x05,
    RTW89_TX_QSEL_VI_1		= 0x06,
    RTW89_TX_QSEL_VO_1		= 0x07,
    RTW89_TX_QSEL_BE_2		= 0x08,
    RTW89_TX_QSEL_BK_2		= 0x09,
    RTW89_TX_QSEL_VI_2		= 0x0a,
    RTW89_TX_QSEL_VO_2		= 0x0b,
    RTW89_TX_QSEL_BE_3		= 0x0c,
    RTW89_TX_QSEL_BK_3		= 0x0d,
    RTW89_TX_QSEL_VI_3		= 0x0e,
    RTW89_TX_QSEL_VO_3		= 0x0f,
    RTW89_TX_QSEL_B0_BCN		= 0x10,
    RTW89_TX_QSEL_B0_HI		= 0x11,
    RTW89_TX_QSEL_B0_MGMT		= 0x12,
    RTW89_TX_QSEL_B0_NOPS		= 0x13,
    RTW89_TX_QSEL_B0_MGMT_FAST	= 0x14,
// reserved
    RTW89_TX_QSEL_B1_BCN		= 0x18,
    RTW89_TX_QSEL_B1_HI		= 0x19,
    RTW89_TX_QSEL_B1_MGMT		= 0x1a,
    RTW89_TX_QSEL_B1_NOPS		= 0x1b,
    RTW89_TX_QSEL_B1_MGMT_FAST	= 0x1c,
// reserved
}

    pub tid): rtw89_warn(rtwdev, "Should use tag 1d: %d\n",,
    pub RTW89_TX_QSEL_BE_0: return,
    pub RTW89_TX_QSEL_BK_0: return,
    pub RTW89_TX_QSEL_VI_0: return,
    pub RTW89_TX_QSEL_VO_0: return,
    pub &tx_req->desc_info: *mut *mut rtw89_tx_desc_info desc_info =,
    pub tx_req->rtwvif_link: *mut *mut rtw89_vif_link rtwvif_link =,
    pub RTW89_TX_QSEL_B1_HI: return,
    pub RTW89_TX_QSEL_B0_HI: return,
    pub RTW89_TX_QSEL_B1_MGMT: return,
    pub RTW89_TX_QSEL_B0_MGMT: return,
    pub 1: return,
    pub tid): rtw89_warn(rtwdev, "Should use tag 1d: %d\n",,
    pub 0: return,
