//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/matcher.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2024 NVIDIA Corporation & Affiliates
// We calculated that concatenating a collision table to the main table with
// 3% of the main table rows will be enough resources for high insertion
// success probability.
//
// The calculation: log2(2^x * 3 / 100) = log2(2^x) + log2(3/100) = x - 5.05 ~ 5
//
pub const MLX5HWS_MATCHER_ASSURED_ROW_RATIO: c_int = 5;
// Threshold to determine if amount of rules require a collision table
pub const MLX5HWS_MATCHER_ASSURED_RULES_TH: c_int = 10;
// Required depth of an assured collision table
pub const MLX5HWS_MATCHER_ASSURED_COL_TBL_DEPTH: c_int = 4;
// Required depth of the main large table
pub const MLX5HWS_MATCHER_ASSURED_MAIN_TBL_DEPTH: c_int = 2;
// Action RTC size multiplier that is required in order
// to support rule update for rules with action STEs.
//
pub const MLX5HWS_MATCHER_ACTION_RTC_UPDATE_MULT: c_int = 1;
// Maximum number of action templates that can be attached to a matcher.
pub const MLX5HWS_MATCHER_MAX_AT: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_matcher_offset {
    MLX5HWS_MATCHER_OFFSET_TAG_DW1 = 12,
    MLX5HWS_MATCHER_OFFSET_TAG_DW0 = 13,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_matcher_flags {
    MLX5HWS_MATCHER_FLAGS_COLLISION = 1 << 2,
    MLX5HWS_MATCHER_FLAGS_RESIZABLE	= 1 << 3,
    MLX5HWS_MATCHER_FLAGS_ISOLATED	= 1 << 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_match_template {
    pub definer: *mut mlx5hws_definer,
    pub fc: *mut mlx5hws_definer_fc,
    pub match_param: *mut u32,
    pub match_criteria_enable: u8,
    pub fc_sz: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_matcher_match_ste {
    pub rtc_0_id: u32,
    pub rtc_1_id: u32,
    pub ste_0_base: u32,
    pub ste_1_base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_matcher {
    pub tbl: *mut mlx5hws_table,
    pub attr: mlx5hws_matcher_attr,
    pub mt: *mut mlx5hws_match_template,
    pub at: *mut mlx5hws_action_template,
    pub num_of_at: u8,
    pub size_of_at_array: u8,
    pub num_of_mt: u8,
    pub num_of_action_stes: u8,
// enum mlx5hws_matcher_flags
    pub flags: u8,
    pub matches_outer_ethertype:1: u8,
    pub matches_outer_ip_version:1: u8,
    pub matches_inner_ethertype:1: u8,
    pub matches_inner_ip_version:1: u8,
    pub outer_ip_version:2: u8,
    pub inner_ip_version:2: u8,
    pub end_ft_id: u32,
    pub col_matcher: *mut mlx5hws_matcher,
    pub resize_dst: *mut mlx5hws_matcher,
    pub match_ste: mlx5hws_matcher_match_ste,
    pub list_node: list_head,
}

extern "C" {
    pub fn mlx5hws_definer_is_jumbo(_arg: mt->definer) -> return;
}
