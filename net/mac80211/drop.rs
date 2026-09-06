//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac80211/drop.h
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
// mac80211 drop reason list
//
// Copyright (C) 2023-2024, 2026 Intel Corporation
//

pub type ieee80211_rx_result = u32;

// 0x00 == ___RX_DROP_UNUSABLE */	\
// 0x10 */				\
// 0x20 */				\
// 0x30 */				\
// 0x40 */				\
// 0x50 */				\
// this line for the trailing \ - add before this
// having two enums allows for checking ieee80211_rx_result use with sparse
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ___mac80211_drop_reason {
// if we get to the end of handlers with RX_CONTINUE this will be the reason
    ___RX_CONTINUE	= SKB_CONSUMED,

// this never gets used as an argument to kfree_skb_reason()
    ___RX_QUEUED	= SKB_NOT_DROPPED_YET,

    ___RX_DROP_UNUSABLE = SKB_DROP_REASON_SUBSYS_MAC80211_UNUSABLE <<
    SKB_DROP_REASON_SUBSYS_SHIFT,
    MAC80211_DROP_REASONS_UNUSABLE(ENUM)

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac80211_drop_reason {
    RX_CONTINUE	= ( ieee80211_rx_result)___RX_CONTINUE,
    RX_QUEUED	= ( ieee80211_rx_result)___RX_QUEUED,

    MAC80211_DROP_REASONS_UNUSABLE(DEF)

}

