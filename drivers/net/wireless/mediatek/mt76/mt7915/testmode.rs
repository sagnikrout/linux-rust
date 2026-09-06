//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7915/testmode.h
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
pub struct mt7915_tm_trx {
    pub type: u8,
    pub enable: u8,
    pub band: u8,
    pub rsv: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_tm_freq_offset {
    pub band: u8,
    pub freq_offset: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_tm_slot_time {
    pub slot_time: u8,
    pub sifs: u8,
    pub rifs: u8,
    pub _rsv: u8,
    pub eifs: __le16,
    pub band: u8,
    pub _rsv1: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_tm_clean_txq {
    pub sta_pause: bool,
    pub /: *mut *mut u8 wcid; / 256 sta,
    pub band: u8,
    pub rsv: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_tm_cmd {
    pub testmode_en: u8,
    pub param_idx: u8,
    pub _rsv: [u8; 2],
    pub data: __le32,
    pub trx: mt7915_tm_trx,
    pub freq: mt7915_tm_freq_offset,
    pub slot: mt7915_tm_slot_time,
    pub clean: mt7915_tm_clean_txq,
    pub test: [u8; 72],
    pub param: },
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tm_tx_cont {
    pub control_ch: u8,
    pub center_ch: u8,
    pub bw: u8,
    pub tx_ant: u8,
    pub rateval: __le16,
    pub band: u8,
    pub txfd_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7915_tm_rf_test {
    pub action: u8,
    pub icap_len: u8,
    pub _rsv: [u8; 2],
    pub op_mode: __le32,
    pub freq: __le32,
    pub func_idx: __le32,
    pub func_data: __le32,
    pub cal_dump: __le32,
    pub tx_cont: tm_tx_cont,
    pub _pad: [u8; 80],
    pub param: },
    pub rf: },
    pub op: },
    pub __packed: },
}
