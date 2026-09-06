//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76_connac2_mac.h
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
// Copyright (C) 2022 MediaTek Inc.
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

// 0: success, others: dropped

// will support this field in further revision

// VHT/HE only use bits 0-3

// PPDU based TXS

// RXD DW0

// RXD DW1

// RXD DW2

// RXD DW4

// RXD DW3

// RXD GROUP4

// P-RXV DW0

// P-RXV DW1

// C-RXV

pub const MT_CRXV_FOE_SHIFT: c_int = 13;
pub const MT_CT_PARSE_LEN: c_int = 72;
pub const MT_CT_DMA_BUF_NUM: c_int = 2;

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
pub enum tx_port_idx {
    MT_TX_PORT_IDX_LMAC,
    MT_TX_PORT_IDX_MCU
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_frag_idx {
    MT_TX_FRAG_NONE,
    MT_TX_FRAG_FIRST,
    MT_TX_FRAG_MID,
    MT_TX_FRAG_LAST
}
