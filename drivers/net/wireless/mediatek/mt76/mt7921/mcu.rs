//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7921/mcu.h
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
// Copyright (C) 2020 MediaTek Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_mcu_tx_done_event {
    pub pid: u8,
    pub status: u8,
    pub seq: __le16,
    pub wlan_idx: u8,
    pub tx_cnt: u8,
    pub tx_rate: __le16,
    pub flag: u8,
    pub tid: u8,
    pub rsp_rate: u8,
    pub mcs: u8,
    pub bw: u8,
    pub tx_pwr: u8,
    pub reason: u8,
    pub rsv0: [u8; 1],
    pub delay: __le32,
    pub timestamp: __le32,
    pub applied_flag: __le32,
    pub txs: [u8; 28],
    pub rsv1: [u8; 32],
    pub __packed: },
// ext event table
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_mcu_eeprom_info {
    pub addr: __le32,
    pub valid: __le32,
    pub data: [u8; MT7921_EEPROM_BLOCK_SIZE],
    pub __packed: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_mcu_ant_id_config {
    pub ant_id: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_txpwr_req {
    pub ver: u8,
    pub action: u8,
    pub len: __le16,
    pub dbdc_idx: u8,
    pub rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_txpwr_event {
    pub ver: u8,
    pub action: u8,
    pub len: __le16,
    pub txpwr: mt7921_txpwr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_wf_rf_pin_ctrl_event {
    pub result: u8,
    pub value: u8,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_rftest_cmd {
    pub action: u8,
    pub rsv: [u8; 3],
    pub param0: __le32,
    pub param1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_rftest_evt {
    pub param0: __le32,
    pub param1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7921_clc_info_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub UNII-4: *mut *mut u8 chan_conf; / BIT(0) : Enable,
// BIT(1) : Enable UNII-5
// BIT(2) : Enable UNII-6
// BIT(3) : Enable UNII-7
// BIT(4) : Enable UNII-8
//
    pub rsv: [u8; 63],
    pub __packed: },
