//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3_common/hclge_comm_rss.h
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
// Copyright (c) 2021-2021 Hisilicon Limited.

pub const HCLGE_COMM_RSS_HASH_ALGO_TOEPLITZ: c_int = 0;
pub const HCLGE_COMM_RSS_HASH_ALGO_SIMPLE: c_int = 1;
pub const HCLGE_COMM_RSS_HASH_ALGO_SYMMETRIC: c_int = 2;

pub const HCLGE_COMM_MAX_TC_NUM: c_int = 8;
pub const HCLGE_COMM_RSS_TC_OFFSET_S: c_int = 0;

pub const HCLGE_COMM_RSS_TC_SIZE_MSB_B: c_int = 11;
pub const HCLGE_COMM_RSS_TC_SIZE_S: c_int = 12;

pub const HCLGE_COMM_RSS_TC_VALID_B: c_int = 15;
pub const HCLGE_COMM_RSS_TC_SIZE_MSB_OFFSET: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_comm_rss_tuple_cfg {
    pub ipv4_tcp_en: u8,
    pub ipv4_udp_en: u8,
    pub ipv4_sctp_en: u8,
    pub ipv4_fragment_en: u8,
    pub ipv6_tcp_en: u8,
    pub ipv6_udp_en: u8,
    pub ipv6_sctp_en: u8,
    pub ipv6_fragment_en: u8,
}

pub const HCLGE_COMM_RSS_KEY_SIZE: c_int = 40;
pub const HCLGE_COMM_RSS_CFG_TBL_SIZE: c_int = 16;

pub const HCLGE_COMM_RSS_CFG_TBL_SIZE_H: c_int = 4;

pub const HCLGE_COMM_RSS_HASH_KEY_OFFSET_B: c_int = 4;
pub const HCLGE_COMM_RSS_HASH_KEY_NUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_comm_rss_config_cmd {
    pub hash_config: u8,
    pub rsv: [u8; 7],
    pub hash_key: [u8; HCLGE_COMM_RSS_HASH_KEY_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_comm_rss_cfg {
    pub /: *mut *mut u8 rss_hash_key[HCLGE_COMM_RSS_KEY_SIZE]; / user configured hash keys,
// shadow table
    pub rss_indirection_tbl: *mut u16,
    pub rss_algo: u32,
    pub rss_tuple_sets: hclge_comm_rss_tuple_cfg,
    pub rss_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_comm_rss_input_tuple_cmd {
    pub ipv4_tcp_en: u8,
    pub ipv4_udp_en: u8,
    pub ipv4_sctp_en: u8,
    pub ipv4_fragment_en: u8,
    pub ipv6_tcp_en: u8,
    pub ipv6_udp_en: u8,
    pub ipv6_sctp_en: u8,
    pub ipv6_fragment_en: u8,
    pub rsv: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_comm_rss_ind_tbl_cmd {
    pub start_table_index: __le16,
    pub rss_set_bitmap: __le16,
    pub rss_qid_h: [u8; HCLGE_COMM_RSS_CFG_TBL_SIZE_H],
    pub rss_qid_l: [u8; HCLGE_COMM_RSS_CFG_TBL_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_comm_rss_tc_mode_cmd {
    pub rss_tc_mode: [__le16; HCLGE_COMM_MAX_TC_NUM],
    pub rsv: [u8; 8],
}

extern "C" {
    pub fn hclge_comm_get_rss_key_size(handle: *mut hnae3_handle) -> u32;
}
extern "C" {
    pub fn hclge_comm_convert_rss_tuple(tuple_sets: u8) -> u64;
}
