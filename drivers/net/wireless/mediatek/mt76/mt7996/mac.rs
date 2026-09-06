//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt7996/mac.h
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
//
// Copyright (C) 2022 MediaTek Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt7996_dfs_pulse {
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
pub struct mt7996_dfs_pattern {
    pub enb: u8,
    pub stgr: u8,
    pub min_crpn: u8,
    pub max_crpn: u8,
    pub min_crpr: u8,
    pub min_pw: u8,
    pub min_pri: u32,
    pub max_pri: u32,
    pub max_pw: u8,
    pub min_crbn: u8,
    pub max_crbn: u8,
    pub min_stgpn: u8,
    pub max_stgpn: u8,
    pub min_stgpr: u8,
    pub rsv: [u8; 2],
    pub min_stgpr_diff: u32,
    pub __packed: },
