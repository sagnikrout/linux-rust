//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dsa/sja1105.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019, Vladimir Oltean <olteanv@gmail.com>
//
// Included by drivers/net/dsa/sja1105/sja1105.h and net/dsa/tag_sja1105.c

pub const ETH_P_SJA1105_META: c_uint = 0x0008;
pub const ETH_P_SJA1110: c_uint = 0xdadc;

// IEEE 802.3 Annex 57A: Slow Protocols PDUs (01:80:C2:xx:xx:xx)
pub const SJA1105_LINKLOCAL_FILTER_A: c_uint = 0x0180C2000000ull;
pub const SJA1105_LINKLOCAL_FILTER_A_MASK: c_uint = 0xFFFFFF000000ull;
// IEEE 1588 Annex F: Transport of PTP over Ethernet (01:1B:19:xx:xx:xx)
pub const SJA1105_LINKLOCAL_FILTER_B: c_uint = 0x011B19000000ull;
pub const SJA1105_LINKLOCAL_FILTER_B_MASK: c_uint = 0xFFFFFF000000ull;
// Source and Destination MAC of follow-up meta frames.
// Whereas the choice of SMAC only affects the unique identification of the
// switch as sender of meta frames, the DMAC must be an address that is present
// in the DSA conduit port's multicast MAC filter.
// 01-80-C2-00-00-0E is a good choice for this, as all profiles of IEEE 1588
// over L2 use this address for some purpose already.
//
pub const SJA1105_META_SMAC: c_uint = 0x222222222222ull;
pub const SJA1105_META_DMAC: c_uint = 0x0180C200000Eull;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1110_meta_tstamp {
    SJA1110_META_TSTAMP_TX = 0,
    SJA1110_META_TSTAMP_RX = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_deferred_xmit_work {
    pub dp: *mut dsa_port,
    pub skb: *mut sk_buff,
    pub work: kthread_work,
}

// Global tagger data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_tagger_data {
    pub work): *mut *mut void (xmit_work_fn)(struct kthread_work,
    pub tstamp): sja1110_meta_tstamp dir, u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_skb_cb {
    pub clone: *mut sk_buff,
    pub tstamp: u64,
// Only valid for packets cloned for 2-step TX timestamping
    pub ts_id: u8,
}

