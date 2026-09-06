//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns/hns_dsaf_ppe.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2014-2015 Hisilicon Limited.
//

pub const HNS_PPE_DEBUG_NW_ENGINE_NUM: c_int = 1;

pub const PPE_COMMON_REG_OFFSET: c_uint = 0x70000;
pub const PPE_REG_OFFSET: c_uint = 0x10000;
pub const ETH_PPE_DUMP_NUM: c_int = 576;
pub const ETH_PPE_STATIC_NUM: c_int = 12;
pub const HNS_PPEV2_RSS_IND_TBL_SIZE: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ppe_qid_mode {
    PPE_QID_MODE0 = 0, /* fixed queue id mode */
    PPE_QID_MODE1,	   /* switch:128VM non switch:6Port/4VM/4TC */
    PPE_QID_MODE2,	   /* switch:32VM/4TC non switch:6Port/16VM */
    PPE_QID_MODE3,	   /* switch:4TC/8RSS non switch:2Port/64VM */
    PPE_QID_MODE4,	   /* switch:8VM/16RSS non switch:2Port/16VM/4TC */
    PPE_QID_MODE5,	   /* switch:16VM/8TC non switch:6Port/16RSS */
    PPE_QID_MODE6,	   /* switch:32VM/4RSS non switch:6Port/2VM/8TC */
    PPE_QID_MODE7,	   /* switch:32RSS non switch:2Port/8VM/8TC */
    PPE_QID_MODE8,	   /* switch:6VM/4TC/4RSS non switch:2Port/16VM/4RSS */
    PPE_QID_MODE9,	   /* non switch:2Port/32VM/2RSS */
    PPE_QID_MODE10,	   /* non switch:2Port/32RSS */
    PPE_QID_MODE11,	   /* non switch:2Port/4TC/16RSS */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ppe_port_mode {
    PPE_MODE_GE = 0,
    PPE_MODE_XGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ppe_common_mode {
    PPE_COMMON_MODE_DEBUG = 0,
    PPE_COMMON_MODE_SERVICE,
    PPE_COMMON_MODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_ppe_hw_stats {
    pub rx_pkts_from_sw: u64,
    pub rx_pkts: u64,
    pub rx_drop_no_bd: u64,
    pub rx_alloc_buf_fail: u64,
    pub rx_alloc_buf_wait: u64,
    pub rx_drop_no_buf: u64,
    pub rx_err_fifo_full: u64,
    pub tx_bd_form_rcb: u64,
    pub tx_pkts_from_rcb: u64,
    pub tx_pkts: u64,
    pub tx_err_fifo_empty: u64,
    pub tx_err_checksum: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_ppe_cb {
    pub dev: *mut device,
    pub /: *mut *mut *mut hns_ppe_cb next; / pointer to next ppe device,
    pub /: *mut *mut *mut ppe_common_cb ppe_common_cb; / belong to,
    pub hw_stats: hns_ppe_hw_stats,
    pub /: *mut *mut u8 index; / index in a ppe common device,
    pub io_base: *mut u8 __iomem,
    pub virq: c_int,
    pub /: *mut *mut u32 rss_indir_table[HNS_PPEV2_RSS_IND_TBL_SIZE]; /shadow indir tab,
    pub /: *mut *mut u32 rss_key[HNS_PPEV2_RSS_KEY_NUM]; / rss hash key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppe_common_cb {
    pub dev: *mut device,
    pub dsaf_dev: *mut dsaf_device,
    pub io_base: *mut u8 __iomem,
    pub ppe_mode: ppe_common_mode,
    pub index*/: *mut *mut u8 comm_index; /ppe_common,
    pub ppe_num: u32,
    pub __counted_by(ppe_num): hns_ppe_cb ppe_cb[],
}

extern "C" {
    pub fn hns_ppe_wait_tx_fifo_clean(ppe_cb: *mut hns_ppe_cb) -> c_int;
}
extern "C" {
    pub fn hns_ppe_init(dsaf_dev: *mut dsaf_device) -> c_int;
}
extern "C" {
    pub fn hns_ppe_uninit(dsaf_dev: *mut dsaf_device);
}
extern "C" {
    pub fn hns_ppe_reset_common(dsaf_dev: *mut dsaf_device, ppe_common_index: u8);
}
extern "C" {
    pub fn hns_ppe_update_stats(ppe_cb: *mut hns_ppe_cb);
}
extern "C" {
    pub fn hns_ppe_get_sset_count(stringset: c_int) -> c_int;
}
extern "C" {
    pub fn hns_ppe_get_regs_count() -> c_int;
}
extern "C" {
    pub fn hns_ppe_get_regs(ppe_cb: *mut hns_ppe_cb, data: *mut c_void);
}
extern "C" {
    pub fn hns_ppe_get_strings(ppe_cb: *mut hns_ppe_cb, stringset: c_int, data: *mut u8);
}
extern "C" {
    pub fn hns_ppe_get_stats(ppe_cb: *mut hns_ppe_cb, data: *mut u64);
}
extern "C" {
    pub fn hns_ppe_set_tso_enable(ppe_cb: *mut hns_ppe_cb, value: u32);
}
