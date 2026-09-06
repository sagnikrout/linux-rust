//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee80211-nan.h
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
// WFA NAN definitions
//
// Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
// <jkmaline@cc.hut.fi>
// Copyright (c) 2002-2003, Jouni Malinen <jkmaline@cc.hut.fi>
// Copyright (c) 2005, Devicescape Software, Inc.
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright (c) 2013 - 2014 Intel Mobile Communications GmbH
// Copyright (c) 2016 - 2017 Intel Deutschland GmbH
// Copyright (c) 2018 - 2026 Intel Corporation
//
// NAN operation mode, as defined in Wi-Fi Aware (TM) specification Table 81
pub const NAN_OP_MODE_PHY_MODE_VHT: c_uint = 0x01;
pub const NAN_OP_MODE_PHY_MODE_HE: c_uint = 0x10;
pub const NAN_OP_MODE_PHY_MODE_MASK: c_uint = 0x11;
pub const NAN_OP_MODE_80P80MHZ: c_uint = 0x02;
pub const NAN_OP_MODE_160MHZ: c_uint = 0x04;
pub const NAN_OP_MODE_PNDL_SUPPRTED: c_uint = 0x08;
pub const NAN_DEV_CAPA_NUM_TX_ANT_POS: c_int = 0;
pub const NAN_DEV_CAPA_NUM_TX_ANT_MASK: c_uint = 0x0f;
pub const NAN_DEV_CAPA_NUM_RX_ANT_POS: c_int = 4;
pub const NAN_DEV_CAPA_NUM_RX_ANT_MASK: c_uint = 0xf0;
// NAN Device capabilities, as defined in Wi-Fi Aware (TM) specification
// Table 79
//
pub const NAN_DEV_CAPA_DFS_OWNER: c_uint = 0x01;
pub const NAN_DEV_CAPA_EXT_KEY_ID_SUPPORTED: c_uint = 0x02;
pub const NAN_DEV_CAPA_SIM_NDP_RX_SUPPORTED: c_uint = 0x04;
pub const NAN_DEV_CAPA_NDPE_SUPPORTED: c_uint = 0x08;
pub const NAN_DEV_CAPA_S3_SUPPORTED: c_uint = 0x10;
// NAN attributes, as defined in Wi-Fi Aware (TM) specification 4.0 Table 42
pub const NAN_ATTR_MASTER_INDICATION: c_uint = 0x00;
pub const NAN_ATTR_CLUSTER_INFO: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_nan_attr {
    pub attr: u8,
    pub length: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_nan_master_indication {
    pub master_pref: u8,
    pub random_factor: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_nan_anchor_master_info {
    pub master_rank: __le64,
    pub master_addr: [u8; ETH_ALEN],
    pub random_factor: u8,
    pub master_pref: u8,
    pub __packed: },
    pub __packed: },
    pub hop_count: u8,
    pub ambtt: __le32,
    pub __packed: },

    pub \: *const *const for (_attr = (struct ieee80211_nan_attr )(_data);,
    pub \: *mut *mut (int)sizeof(_attr) + le16_to_cpu(_attr->length);,
