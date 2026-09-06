//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/resources.h
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
// Copyright (c) 2016-2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_res_id {
    MLXSW_RES_ID_KVD_SIZE,
    MLXSW_RES_ID_KVD_SINGLE_MIN_SIZE,
    MLXSW_RES_ID_KVD_DOUBLE_MIN_SIZE,
    MLXSW_RES_ID_PGT_SIZE,
    MLXSW_RES_ID_MAX_KVD_LINEAR_RANGE,
    MLXSW_RES_ID_MAX_KVD_ACTION_SETS,
    MLXSW_RES_ID_MAX_TRAP_GROUPS,
    MLXSW_RES_ID_CQE_V0,
    MLXSW_RES_ID_CQE_V1,
    MLXSW_RES_ID_CQE_V2,
    MLXSW_RES_ID_COUNTER_POOL_SIZE,
    MLXSW_RES_ID_COUNTER_BANK_SIZE,
    MLXSW_RES_ID_MAX_SPAN,
    MLXSW_RES_ID_COUNTER_SIZE_PACKETS_BYTES,
    MLXSW_RES_ID_COUNTER_SIZE_ROUTER_BASIC,
    MLXSW_RES_ID_MAX_SYSTEM_PORT,
    MLXSW_RES_ID_FID,
    MLXSW_RES_ID_MAX_LAG,
    MLXSW_RES_ID_MAX_LAG_MEMBERS,
    MLXSW_RES_ID_MAX_NVE_FLOOD_PRF,
    MLXSW_RES_ID_GUARANTEED_SHARED_BUFFER,
    MLXSW_RES_ID_CELL_SIZE,
    MLXSW_RES_ID_MAX_HEADROOM_SIZE,
    MLXSW_RES_ID_ACL_MAX_TCAM_REGIONS,
    MLXSW_RES_ID_ACL_MAX_TCAM_RULES,
    MLXSW_RES_ID_ACL_MAX_REGIONS,
    MLXSW_RES_ID_ACL_MAX_GROUPS,
    MLXSW_RES_ID_ACL_MAX_GROUP_SIZE,
    MLXSW_RES_ID_ACL_MAX_DEFAULT_ACTIONS,
    MLXSW_RES_ID_ACL_FLEX_KEYS,
    MLXSW_RES_ID_ACL_MAX_ACTION_PER_RULE,
    MLXSW_RES_ID_ACL_ACTIONS_PER_SET,
    MLXSW_RES_ID_ACL_MAX_L4_PORT_RANGE,
    MLXSW_RES_ID_ACL_MAX_ERPT_BANKS,
    MLXSW_RES_ID_ACL_MAX_ERPT_BANK_SIZE,
    MLXSW_RES_ID_ACL_MAX_LARGE_KEY_ID,
    MLXSW_RES_ID_ACL_ERPT_ENTRIES_2KB,
    MLXSW_RES_ID_ACL_ERPT_ENTRIES_4KB,
    MLXSW_RES_ID_ACL_ERPT_ENTRIES_8KB,
    MLXSW_RES_ID_ACL_ERPT_ENTRIES_12KB,
    MLXSW_RES_ID_ACL_MAX_BF_LOG,
    MLXSW_RES_ID_MAX_GLOBAL_POLICERS,
    MLXSW_RES_ID_MAX_CPU_POLICERS,
    MLXSW_RES_ID_MAX_VRS,
    MLXSW_RES_ID_MAX_RIFS,
    MLXSW_RES_ID_MC_ERIF_LIST_ENTRIES,
    MLXSW_RES_ID_MAX_RIF_MAC_PROFILES,
    MLXSW_RES_ID_MAX_LPM_TREES,
    MLXSW_RES_ID_MAX_NVE_MC_ENTRIES_IPV4,
    MLXSW_RES_ID_MAX_NVE_MC_ENTRIES_IPV6,

// Internal resources.
// Determined by the SW, not queried from the HW.
//
    MLXSW_RES_ID_KVD_SINGLE_SIZE,
    MLXSW_RES_ID_KVD_DOUBLE_SIZE,
    MLXSW_RES_ID_KVD_LINEAR_SIZE,

    __MLXSW_RES_ID_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_res {
    pub valid: [bool; __MLXSW_RES_ID_MAX],
    pub values: [u64; __MLXSW_RES_ID_MAX],
}

