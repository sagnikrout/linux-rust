//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76_connac3_mac.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2023 MediaTek Inc.
pub const MT_CT_PARSE_LEN: c_int = 72;
pub const MT_CT_DMA_BUF_NUM: c_int = 2;

pub const MT_RXD0_SW_PKT_TYPE_MAP: c_uint = 0x380F;
pub const MT_RXD0_SW_PKT_TYPE_FRAME: c_uint = 0x3801;
// RXD DW1

// RXD DW2

// RXD DW3

// RXD DW4

// RXD GROUP4

// P-RXV

// C-RXV

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_header_format {
    MT_HDR_FORMAT_802_3,
    MT_HDR_FORMAT_CMD,
    MT_HDR_FORMAT_802_11,
    MT_HDR_FORMAT_802_11_EXT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_pkt_type {
    MT_TX_TYPE_CT,
    MT_TX_TYPE_SF,
    MT_TX_TYPE_CMD,
    MT_TX_TYPE_FW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_port_idx {
    MT_TX_PORT_IDX_LMAC,
    MT_TX_PORT_IDX_MCU
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_mcu_port_q_idx {
    MT_TX_MCU_PORT_RX_Q0 = 0x20,
    MT_TX_MCU_PORT_RX_Q1,
    MT_TX_MCU_PORT_RX_Q2,
    MT_TX_MCU_PORT_RX_Q3,
    MT_TX_MCU_PORT_RX_FWDL = 0x3e
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_mgnt_type {
    MT_TX_NORMAL,
    MT_TX_TIMING,
    MT_TX_ADDBA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_frag_idx {
    MT_TX_FRAG_NONE,
    MT_TX_FRAG_FIRST,
    MT_TX_FRAG_MID,
    MT_TX_FRAG_LAST
}

// VHT/HE only use bits 0-3

// MPDU based TXS

// PPDU based TXS

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7928_uni_txdone_event {
    pub tag: __le16,
    pub len: __le16,
    pub /: *mut *mut u8 pid; / HW packet ID,
    pub /: *mut *mut u8 status; / TX_RESULT_xx,
    pub /: *mut *mut __le16 seq; / packet sequence number,
    pub /: *mut *mut u8 wcid; / WLAN index (WTBL),
    pub /: *mut *mut u8 tx_count; / TX attempts including retries,
    pub tx_rate: __le16,
    pub /: *mut *mut u8 flag; / TXS_WITH_ADVANCED_INFO or TXS_IS_EXIST,
    pub tid: u8,
    pub rsp_rate: u8,
    pub /: *mut *mut u8 rate_tbl_idx; / last TX rate index from WLAN table,
    pub /: *mut *mut u8 bw; / bandwidth used for this PPDU,
    pub /: *mut *mut u8 tx_pwr; / dBm,
    pub flush_reason: u8,
    pub rsv: [u8; 1],
    pub /: *mut *mut __le32 tx_delay; / unit: 32us, UMAC TX to TX status,
    pub /: *mut *mut __le32 timestamp; / local TSF at first bit of MAC header,
    pub applied_flags: __le32,
    pub __packed: },
