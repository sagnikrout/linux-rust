//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/mwifiex/11ac.h
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
// NXP Wireless LAN device driver: 802.11ac
//
// Copyright 2011-2020 NXP
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vht_cfg_misc_config {
    VHT_CAP_TX_OPERATION = 1,
    VHT_CAP_ASSOCIATION,
    VHT_CAP_UAP_ONLY
}

pub const DEFAULT_VHT_MCS_SET: c_uint = 0xfffa;
pub const DISABLE_VHT_MCS_SET: c_uint = 0xffff;

