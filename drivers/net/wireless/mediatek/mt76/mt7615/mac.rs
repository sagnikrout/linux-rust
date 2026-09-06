//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7615/mac.h
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
// Copyright (C) 2019 MediaTek Inc.
pub const MT_CT_PARSE_LEN: c_int = 72;
pub const MT_CT_DMA_BUF_NUM: c_int = 2;

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
    MT_TX_MCU_PORT_RX_Q0 = 0,
    MT_TX_MCU_PORT_RX_Q1,
    MT_TX_MCU_PORT_RX_Q2,
    MT_TX_MCU_PORT_RX_Q3,
    MT_TX_MCU_PORT_RX_FWDL = 0x1e
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_phy_bandwidth {
    MT_PHY_BW_20,
    MT_PHY_BW_40,
    MT_PHY_BW_80,
    MT_PHY_BW_160,
}

// MT7663 DW7 HW-AMSDU

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_dfs_pulse {
    pub /: *mut *mut u32 max_width; / us,
    pub /: *mut *mut int max_pwr; / dbm,
    pub /: *mut *mut int min_pwr; / dbm,
    pub /: *mut *mut u32 min_stgr_pri; / us,
    pub /: *mut *mut u32 max_stgr_pri; / us,
    pub /: *mut *mut u32 min_cr_pri; / us,
    pub /: *mut *mut u32 max_cr_pri; / us,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_dfs_pattern {
    pub enb: u8,
    pub stgr: u8,
    pub min_crpn: u8,
    pub max_crpn: u8,
    pub min_crpr: u8,
    pub min_pw: u8,
    pub max_pw: u8,
    pub min_pri: u32,
    pub max_pri: u32,
    pub min_crbn: u8,
    pub max_crbn: u8,
    pub min_stgpn: u8,
    pub max_stgpn: u8,
    pub min_stgpr: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7615_dfs_radar_spec {
    pub pulse_th: mt7615_dfs_pulse,
    pub radar_pattern: [mt7615_dfs_pattern; 16],
}
