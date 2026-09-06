//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3_ethtool.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2021 Hisilicon Limited.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_stats {
    pub stats_string: [c_char; ETH_GSTRING_LEN],
    pub stats_offset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_sfp_type {
    pub type: u8,
    pub ext_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_pflag_desc {
    pub name: [c_char; ETH_GSTRING_LEN],
    pub enable): *mut *mut *mut void (handler)(struct net_device netdev, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_ethtool_link_ext_state_mapping {
    pub status_code: u32,
    pub link_ext_state: ethtool_link_ext_state,
    pub link_ext_substate: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_ring_param {
    pub tx_desc_num: u32,
    pub rx_desc_num: u32,
    pub rx_buf_len: u32,
}
