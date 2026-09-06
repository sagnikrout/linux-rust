//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_sched.h
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
// Copyright (c) 2018, Intel Corporation.

//
// DOC: ice_sched.h
//
// This header file stores everything that is needed for broadly understood
// scheduler. It consists of defines related to layers, structures related to
// aggregator, functions declarations and others.
//
pub const ICE_SCHED_5_LAYERS: c_int = 5;
pub const ICE_SCHED_9_LAYERS: c_int = 9;
pub const SCHED_NODE_NAME_MAX_LEN: c_int = 32;
pub const ICE_QGRP_LAYER_OFFSET: c_int = 2;
pub const ICE_VSI_LAYER_OFFSET: c_int = 4;
pub const ICE_AGG_LAYER_OFFSET: c_int = 6;
pub const ICE_SCHED_INVAL_LAYER_NUM: c_uint = 0xFF;
// Burst size is a 12 bits register that is configured while creating the RL
// profile(s). MSB is a granularity bit and tells the granularity type
// 0 - LSB bits are in 64 bytes granularity
// 1 - LSB bits are in 1K bytes granularity
//
pub const ICE_64_BYTE_GRANULARITY: c_int = 0;

pub const ICE_RL_PROF_ACCURACY_BYTES: c_int = 128;
pub const ICE_RL_PROF_MULTIPLIER: c_int = 10000;
pub const ICE_RL_PROF_TS_MULTIPLIER: c_int = 32;
pub const ICE_RL_PROF_FRACTION: c_int = 512;
pub const ICE_PSM_CLK_367MHZ_IN_HZ: c_int = 367647059;
pub const ICE_PSM_CLK_416MHZ_IN_HZ: c_int = 416666667;
pub const ICE_PSM_CLK_446MHZ_IN_HZ: c_int = 446428571;
pub const ICE_PSM_CLK_390MHZ_IN_HZ: c_int = 390625000;
// BW rate limit profile parameters list entry along
// with bandwidth maintained per layer in port info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aqc_rl_profile_info {
    pub profile: ice_aqc_rl_profile_elem,
    pub list_entry: list_head,
    pub /: *mut *mut u32 bw; / requested,
    pub /: *mut *mut u16 prof_id_ref; / profile ID to node association ref count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sched_agg_vsi_info {
    pub list_entry: list_head,
    pub ICE_MAX_TRAFFIC_CLASS): DECLARE_BITMAP(tc_bitmap,,
    pub vsi_handle: u16,
// save aggregator VSI TC bitmap
    pub ICE_MAX_TRAFFIC_CLASS): DECLARE_BITMAP(replay_tc_bitmap,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sched_agg_info {
    pub agg_vsi_list: list_head,
    pub list_entry: list_head,
    pub ICE_MAX_TRAFFIC_CLASS): DECLARE_BITMAP(tc_bitmap,,
    pub agg_id: u32,
    pub agg_type: ice_agg_type,
// bw_t_info saves aggregator BW information
    pub bw_t_info: [ice_bw_type_info; ICE_MAX_TRAFFIC_CLASS],
// save aggregator TC bitmap
    pub ICE_MAX_TRAFFIC_CLASS): DECLARE_BITMAP(replay_tc_bitmap,,
}

// FW AQ command calls
extern "C" {
    pub fn ice_sched_set_node_weight(pi: *mut ice_port_info, node: *mut ice_sched_node, weight: u16) -> c_int;
}
extern "C" {
    pub fn ice_sched_init_port(pi: *mut ice_port_info) -> c_int;
}
extern "C" {
    pub fn ice_sched_query_res_alloc(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_sched_get_psm_clk_freq(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_sched_clear_port(pi: *mut ice_port_info);
}
extern "C" {
    pub fn ice_sched_cleanup_all(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_sched_clear_agg(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_free_sched_node(pi: *mut ice_port_info, node: *mut ice_sched_node);
}
extern "C" {
    pub fn ice_rm_vsi_lan_cfg(pi: *mut ice_port_info, vsi_handle: u16) -> c_int;
}
extern "C" {
    pub fn ice_rm_vsi_rdma_cfg(pi: *mut ice_port_info, vsi_handle: u16) -> c_int;
}
// Tx scheduler rate limiter functions
extern "C" {
    pub fn ice_cfg_rl_burst_size(hw: *mut ice_hw, bytes: u32) -> c_int;
}
extern "C" {
    pub fn ice_sched_get_agg_layer(hw: *mut ice_hw) -> u8;
}
extern "C" {
    pub fn ice_sched_get_vsi_layer(hw: *mut ice_hw) -> u8;
}
extern "C" {
    pub fn ice_sched_replay_agg_vsi_preinit(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_sched_replay_agg(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_replay_vsi_agg(hw: *mut ice_hw, vsi_handle: u16) -> c_int;
}
extern "C" {
    pub fn ice_sched_replay_q_bw(pi: *mut ice_port_info, q_ctx: *mut ice_q_ctx) -> c_int;
}
