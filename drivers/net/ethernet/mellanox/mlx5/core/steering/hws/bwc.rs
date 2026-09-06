//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/bwc.h
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
pub const MLX5HWS_BWC_MATCHER_INIT_SIZE_LOG: c_int = 1;
pub const MLX5HWS_BWC_MATCHER_SIZE_LOG_STEP: c_int = 1;
pub const MLX5HWS_BWC_MATCHER_REHASH_PERCENT_TH: c_int = 70;
pub const MLX5HWS_BWC_MATCHER_REHASH_BURST_TH: c_int = 32;
// Max number of AT attach operations for the same matcher.
// When the limit is reached, a larger buffer is allocated for the ATs.
//
pub const MLX5HWS_BWC_MATCHER_ATTACH_AT_NUM: c_int = 8;
pub const MLX5HWS_BWC_MAX_ACTS: c_int = 16;
pub const MLX5HWS_BWC_POLLING_TIMEOUT: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_bwc_matcher_type {
// Standalone bwc matcher.
    MLX5HWS_BWC_MATCHER_SIMPLE,
// The first matcher of a complex matcher. When rules are inserted into
// a matcher of this type, they are split into subrules and inserted
// into their corresponding submatchers.
//
    MLX5HWS_BWC_MATCHER_COMPLEX_FIRST,
// A submatcher that is part of a complex matcher. For most purposes
// these are treated as simple matchers, except when it comes to moving
// rules during resize.
//
    MLX5HWS_BWC_MATCHER_COMPLEX_SUBMATCHER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_bwc_matcher_size {
    pub size_log: u8,
    pub num_of_rules: core::sync::atomic::AtomicI32,
    pub rehash_required: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_bwc_matcher {
    pub matcher: *mut mlx5hws_matcher,
    pub mt: *mut mlx5hws_match_template,
    pub at: *mut mlx5hws_action_template,
    pub complex: *mut mlx5hws_bwc_matcher_complex_data,
    pub num_of_at: u8,
    pub size_of_at_array: u8,
    pub matcher_type: mlx5hws_bwc_matcher_type,
    pub priority: u32,
    pub rx_size: mlx5hws_bwc_matcher_size,
    pub tx_size: mlx5hws_bwc_matcher_size,
    pub rules: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_bwc_rule {
    pub bwc_matcher: *mut mlx5hws_bwc_matcher,
    pub rule: *mut mlx5hws_rule,
    pub next_subrule: *mut mlx5hws_bwc_rule,
    pub subrule_data: *mut mlx5hws_bwc_complex_subrule_data,
    pub flow_source: u32,
    pub bwc_queue_idx: u16,
    pub skip_rx: bool,
    pub skip_tx: bool,
    pub list_node: list_head,
}

extern "C" {
    pub fn mlx5hws_bwc_matcher_destroy_simple(bwc_matcher: *mut mlx5hws_bwc_matcher) -> c_int;
}
extern "C" {
    pub fn mlx5hws_bwc_rule_free(bwc_rule: *mut mlx5hws_bwc_rule);
}
extern "C" {
    pub fn mlx5hws_bwc_rule_destroy_simple(bwc_rule: *mut mlx5hws_bwc_rule) -> c_int;
}
// Besides the control queue, half of the queues are
// regular HWS queues, and the other half are BWC queues.
//
