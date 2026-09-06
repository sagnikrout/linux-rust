//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/rule.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_rule_status {
    MLX5HWS_RULE_STATUS_UNKNOWN,
    MLX5HWS_RULE_STATUS_CREATING,
    MLX5HWS_RULE_STATUS_CREATED,
    MLX5HWS_RULE_STATUS_UPDATING,
    MLX5HWS_RULE_STATUS_UPDATED,
    MLX5HWS_RULE_STATUS_DELETING,
    MLX5HWS_RULE_STATUS_DELETED,
    MLX5HWS_RULE_STATUS_FAILING,
    MLX5HWS_RULE_STATUS_FAILED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_rule_move_state {
    MLX5HWS_RULE_RESIZE_STATE_IDLE,
    MLX5HWS_RULE_RESIZE_STATE_WRITING,
    MLX5HWS_RULE_RESIZE_STATE_DELETING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_rule_jumbo_match_tag_offset {
    MLX5HWS_RULE_JUMBO_MATCH_TAG_OFFSET_DW0 = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_rule_match_tag {
    pub jumbo: [u8; MLX5HWS_JUMBO_TAG_SZ],
    pub reserved: [u8; MLX5HWS_ACTIONS_SZ],
    pub match: [u8; MLX5HWS_MATCH_TAG_SZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_rule_resize_info {
    pub rtc_0: u32,
    pub rtc_1: u32,
    pub rule_idx: u32,
    pub state: u8,
    pub /: *mut *mut u8 ctrl_seg[MLX5HWS_WQE_SZ_GTA_CTRL]; / Ctrl segment of STE: 48 bytes,
    pub /: *mut *mut u8 data_seg[MLX5HWS_WQE_SZ_GTA_DATA]; / Data segment of STE: 64 bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_rule {
    pub matcher: *mut mlx5hws_matcher,
    pub tag: mlx5hws_rule_match_tag,
    pub resize_info: *mut mlx5hws_rule_resize_info,
}

// still exists, so don't actually delete this rule.
//
extern "C" {
    pub fn mlx5hws_rule_free_action_ste(action_ste: *mut mlx5hws_action_ste_chunk);
}
extern "C" {
    pub fn mlx5hws_rule_move_in_progress(rule: *mut mlx5hws_rule) -> bool;
}
extern "C" {
    pub fn mlx5hws_rule_clear_resize_info(rule: *mut mlx5hws_rule);
}
