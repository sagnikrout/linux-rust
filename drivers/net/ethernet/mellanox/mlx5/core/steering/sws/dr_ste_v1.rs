//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/dr_ste_v1.h
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
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

pub const DR_STE_DECAP_L3_ACTION_NUM: c_int = 8;
pub const DR_STE_L2_HDR_MAX_SZ: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dr_ste_v1_entry_format {
    DR_STE_V1_TYPE_BWC_BYTE	= 0x0,
    DR_STE_V1_TYPE_BWC_DW	= 0x1,
    DR_STE_V1_TYPE_MATCH	= 0x2,
    DR_STE_V1_TYPE_MATCH_RANGES = 0x7,
}

// Lookup type is built from 2B: [ Definer mode 1B ][ Definer index 1B ]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dr_ste_v1_header_anchors {
    DR_STE_HEADER_ANCHOR_START_OUTER		= 0x00,
    DR_STE_HEADER_ANCHOR_1ST_VLAN			= 0x02,
    DR_STE_HEADER_ANCHOR_IPV6_IPV4			= 0x07,
    DR_STE_HEADER_ANCHOR_INNER_MAC			= 0x13,
    DR_STE_HEADER_ANCHOR_INNER_IPV6_IPV4		= 0x19,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dr_ste_v1_action_size {
    DR_STE_ACTION_SINGLE_SZ = 4,
    DR_STE_ACTION_DOUBLE_SZ = 8,
    DR_STE_ACTION_TRIPLE_SZ = 12,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dr_ste_v1_action_insert_ptr_attr {
    DR_STE_V1_ACTION_INSERT_PTR_ATTR_NONE = 0,  /* Regular push header (e.g. push vlan) */
    DR_STE_V1_ACTION_INSERT_PTR_ATTR_ENCAP = 1, /* Encapsulation / Tunneling */
    DR_STE_V1_ACTION_INSERT_PTR_ATTR_ESP = 2,   /* IPsec */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dr_ste_v1_action_id {
    DR_STE_V1_ACTION_ID_NOP				= 0x00,
    DR_STE_V1_ACTION_ID_COPY			= 0x05,
    DR_STE_V1_ACTION_ID_SET				= 0x06,
    DR_STE_V1_ACTION_ID_ADD				= 0x07,
    DR_STE_V1_ACTION_ID_REMOVE_BY_SIZE		= 0x08,
    DR_STE_V1_ACTION_ID_REMOVE_HEADER_TO_HEADER	= 0x09,
    DR_STE_V1_ACTION_ID_INSERT_INLINE		= 0x0a,
    DR_STE_V1_ACTION_ID_INSERT_POINTER		= 0x0b,
    DR_STE_V1_ACTION_ID_FLOW_TAG			= 0x0c,
    DR_STE_V1_ACTION_ID_QUEUE_ID_SEL		= 0x0d,
    DR_STE_V1_ACTION_ID_ACCELERATED_LIST		= 0x0e,
    DR_STE_V1_ACTION_ID_MODIFY_LIST			= 0x0f,
    DR_STE_V1_ACTION_ID_ASO				= 0x12,
    DR_STE_V1_ACTION_ID_TRAILER			= 0x13,
    DR_STE_V1_ACTION_ID_COUNTER_ID			= 0x14,
    DR_STE_V1_ACTION_ID_MAX				= 0x21,
// use for special cases
    DR_STE_V1_ACTION_ID_SPECIAL_ENCAP_L3		= 0x22,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dr_ste_v1_aso_ctx_type {
    DR_STE_V1_ASO_CTX_TYPE_POLICERS = 0x2,
}

extern "C" {
    pub fn dr_ste_v1_is_miss_addr_set(hw_ste_p: *mut u8) -> bool;
}
extern "C" {
    pub fn dr_ste_v1_set_miss_addr(hw_ste_p: *mut u8, miss_addr: u64);
}
extern "C" {
    pub fn dr_ste_v1_get_miss_addr(hw_ste_p: *mut u8) -> u64;
}
extern "C" {
    pub fn dr_ste_v1_set_byte_mask(hw_ste_p: *mut u8, byte_mask: u16);
}
extern "C" {
    pub fn dr_ste_v1_get_byte_mask(hw_ste_p: *mut u8) -> u16;
}
extern "C" {
    pub fn dr_ste_v1_set_next_lu_type(hw_ste_p: *mut u8, lu_type: u16);
}
extern "C" {
    pub fn dr_ste_v1_get_next_lu_type(hw_ste_p: *mut u8) -> u16;
}
extern "C" {
    pub fn dr_ste_v1_set_hit_addr(hw_ste_p: *mut u8, icm_addr: u64, ht_size: u32);
}
extern "C" {
    pub fn dr_ste_v1_init(hw_ste_p: *mut u8, lu_type: u16, is_rx: bool, gvmi: u16);
}
extern "C" {
    pub fn dr_ste_v1_prepare_for_postsend(hw_ste_p: *mut u8, ste_size: u32);
}
extern "C" {
    pub fn dr_ste_v1_set_reparse(hw_ste_p: *mut u8);
}
extern "C" {
    pub fn dr_ste_v1_set_encap(hw_ste_p: *mut u8, d_action: *mut u8, reformat_id: u32, size: c_int);
}
extern "C" {
    pub fn dr_ste_v1_set_push_vlan(hw_ste_p: *mut u8, d_action: *mut u8, vlan_hdr: u32);
}
extern "C" {
    pub fn dr_ste_v1_set_pop_vlan(hw_ste_p: *mut u8, s_action: *mut u8, vlans_num: u8);
}
extern "C" {
    pub fn dr_ste_v1_set_rx_decap(hw_ste_p: *mut u8, s_action: *mut u8);
}
extern "C" {
    pub fn dr_ste_v1_alloc_modify_hdr_ptrn_arg(action: *mut mlx5dr_action) -> c_int;
}
extern "C" {
    pub fn dr_ste_v1_free_modify_hdr_ptrn_arg(action: *mut mlx5dr_action);
}
