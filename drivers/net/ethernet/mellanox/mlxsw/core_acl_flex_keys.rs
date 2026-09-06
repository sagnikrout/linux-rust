//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/core_acl_flex_keys.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2017-2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_afk_element {
    MLXSW_AFK_ELEMENT_SRC_SYS_PORT,
    MLXSW_AFK_ELEMENT_DMAC_32_47,
    MLXSW_AFK_ELEMENT_DMAC_0_31,
    MLXSW_AFK_ELEMENT_SMAC_32_47,
    MLXSW_AFK_ELEMENT_SMAC_0_31,
    MLXSW_AFK_ELEMENT_ETHERTYPE,
    MLXSW_AFK_ELEMENT_IP_PROTO,
    MLXSW_AFK_ELEMENT_SRC_IP_96_127,
    MLXSW_AFK_ELEMENT_SRC_IP_64_95,
    MLXSW_AFK_ELEMENT_SRC_IP_32_63,
    MLXSW_AFK_ELEMENT_SRC_IP_0_31,
    MLXSW_AFK_ELEMENT_DST_IP_96_127,
    MLXSW_AFK_ELEMENT_DST_IP_64_95,
    MLXSW_AFK_ELEMENT_DST_IP_32_63,
    MLXSW_AFK_ELEMENT_DST_IP_0_31,
    MLXSW_AFK_ELEMENT_DST_L4_PORT,
    MLXSW_AFK_ELEMENT_SRC_L4_PORT,
    MLXSW_AFK_ELEMENT_VID,
    MLXSW_AFK_ELEMENT_PCP,
    MLXSW_AFK_ELEMENT_TCP_FLAGS,
    MLXSW_AFK_ELEMENT_IP_TTL_,
    MLXSW_AFK_ELEMENT_IP_ECN,
    MLXSW_AFK_ELEMENT_IP_DSCP,
    MLXSW_AFK_ELEMENT_VIRT_ROUTER,
    MLXSW_AFK_ELEMENT_FDB_MISS,
    MLXSW_AFK_ELEMENT_L4_PORT_RANGE,
    MLXSW_AFK_ELEMENT_VIRT_ROUTER_0_3,
    MLXSW_AFK_ELEMENT_VIRT_ROUTER_4_7,
    MLXSW_AFK_ELEMENT_VIRT_ROUTER_MSB,
    MLXSW_AFK_ELEMENT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_afk_element_type {
    MLXSW_AFK_ELEMENT_TYPE_U32,
    MLXSW_AFK_ELEMENT_TYPE_BUF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_afk_element_info {
    pub /: *mut *mut mlxsw_afk_element element; / element ID,
    pub type: mlxsw_afk_element_type,
    pub /: *mut *mut mlxsw_item item; / element geometry in internal storage,
}

pub const MLXSW_AFK_ELEMENT_STORAGE_SIZE: c_uint = 0x44;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_afk_element_inst {
    pub element: mlxsw_afk_element,
    pub type: mlxsw_afk_element_type,
    pub /: *mut *mut mlxsw_item item; / element geometry in block,
    pub write: *mut *mut int u32_key_diff; / in case value needs to be adjusted before,
// this diff is here to handle that
//
    pub avoid_size_check: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_afk_block {
    pub /: *mut *mut u16 encoding; / block ID,
    pub instances: *const mlxsw_afk_element_inst,
    pub instances_count: c_uint,
    pub high_entropy: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_afk_element_usage {
    pub MLXSW_AFK_ELEMENT_MAX): DECLARE_BITMAP(usage,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_afk_ops {
    pub blocks: *const mlxsw_afk_block,
    pub blocks_count: c_uint,
    pub block): *mut *mut *mut void (encode_block)(char output, int block_index, char,
    pub block_index): *mut *mut *mut void (clear_block)(char output, int,
}

extern "C" {
    pub fn mlxsw_afk_destroy(mlxsw_afk: *mut mlxsw_afk);
}
extern "C" {
    pub fn mlxsw_afk_key_info_put(key_info: *mut mlxsw_afk_key_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_afk_element_values {
    pub elusage: mlxsw_afk_element_usage,
    pub key: [c_char; MLXSW_AFK_ELEMENT_STORAGE_SIZE],
    pub mask: [c_char; MLXSW_AFK_ELEMENT_STORAGE_SIZE],
    pub storage: },
}
