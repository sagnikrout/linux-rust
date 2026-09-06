//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_acl_tcam.h
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
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_tcam {
    pub used_regions: ida,
    pub max_regions: c_uint,
    pub used_groups: ida,
    pub max_groups: c_uint,
    pub max_group_size: c_uint,
    pub /: *mut *mut mutex lock; / guards vregion list,
    pub vregion_list: list_head,
    pub /: *mut *mut u32 vregion_rehash_intrvl; / ms,
    pub priv: [c_ulong; ],
// priv has to be always the last item
}

extern "C" {
    pub fn mlxsw_sp_acl_tcam_priv_size(mlxsw_sp: *mut mlxsw_sp) -> usize;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_profile_ops {
    pub ruleset_priv_size: usize,
    pub p_max_prio): *mut *mut unsigned int p_min_prio, unsigned int,
    pub ruleset_priv): *mut *mut *mut void (ruleset_del)(struct mlxsw_sp mlxsw_sp, void,
    pub ingress): bool,
    pub ingress): bool,
    pub ruleset_priv): *mut *mut u16 (ruleset_group_id)(void,
    pub rule_priv_size: usize,
    pub rulei): *mut mlxsw_sp_acl_rule_info,
    pub rule_priv): *mut *mut *mut void (rule_del)(struct mlxsw_sp mlxsw_sp, void,
    pub rulei): *mut mlxsw_sp_acl_rule_info,
    pub activity): *mut bool,
}

pub const MLXSW_SP_ACL_TCAM_REGION_BASE_COUNT: c_int = 16;
pub const MLXSW_SP_ACL_TCAM_REGION_RESIZE_STEP: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_tcam_region {
    pub vregion: *mut mlxsw_sp_acl_tcam_vregion,
    pub group: *mut mlxsw_sp_acl_tcam_group,
    pub /: *mut *mut list_head list; / Member of a TCAM group,
    pub key_type: mlxsw_reg_ptar_key_type,
    pub /: *mut *mut u16 id; / ACL ID and region ID - they are same,
    pub tcam_region_info: [c_char; MLXSW_REG_PXXX_TCAM_REGION_INFO_LEN],
    pub key_info: *mut mlxsw_afk_key_info,
    pub mlxsw_sp: *mut mlxsw_sp,
    pub priv: [c_ulong; ],
// priv has to be always the last item
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_ctcam_region {
    pub parman: *mut parman,
    pub ops: *const mlxsw_sp_acl_ctcam_region_ops,
    pub region: *mut mlxsw_sp_acl_tcam_region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_ctcam_chunk {
    pub parman_prio: parman_prio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_ctcam_entry {
    pub parman_item: parman_item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_ctcam_region_ops {
    pub mask): *const c_char,
    pub centry): *mut mlxsw_sp_acl_ctcam_entry,
}

extern "C" {
    pub fn mlxsw_sp_acl_ctcam_region_fini(cregion: *mut mlxsw_sp_acl_ctcam_region);
}
extern "C" {
    pub fn mlxsw_sp_acl_ctcam_chunk_fini(cchunk: *mut mlxsw_sp_acl_ctcam_chunk);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_acl_atcam_region_type {
    MLXSW_SP_ACL_ATCAM_REGION_TYPE_2KB,
    MLXSW_SP_ACL_ATCAM_REGION_TYPE_4KB,
    MLXSW_SP_ACL_ATCAM_REGION_TYPE_8KB,
    MLXSW_SP_ACL_ATCAM_REGION_TYPE_12KB,
    __MLXSW_SP_ACL_ATCAM_REGION_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_atcam {
    pub erp_core: *mut mlxsw_sp_acl_erp_core,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_atcam_region {
    pub /: *mut *mut rhashtable entries_ht; / A-TCAM only,
    pub /: *mut *mut list_head entries_list; / A-TCAM only,
    pub cregion: mlxsw_sp_acl_ctcam_region,
    pub ops: *const mlxsw_sp_acl_atcam_region_ops,
    pub region: *mut mlxsw_sp_acl_tcam_region,
    pub atcam: *mut mlxsw_sp_acl_atcam,
    pub type: mlxsw_sp_acl_atcam_region_type,
    pub erp_table: *mut mlxsw_sp_acl_erp_table,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_atcam_entry_ht_key {
    pub minus: *mut *mut char enc_key[MLXSW_REG_PTCEX_FLEX_KEY_BLOCKS_LEN]; / Encoded key,,
// delta bits.
//
    pub erp_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_atcam_chunk {
    pub cchunk: mlxsw_sp_acl_ctcam_chunk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_atcam_entry {
    pub ht_node: rhash_head,
    pub /: *mut *mut list_head list; / Member in entries_list,
    pub ht_key: mlxsw_sp_acl_atcam_entry_ht_key,
    pub start: u16,
    pub mask: u8,
    pub value: u8,
    pub delta_info: },
    pub centry: mlxsw_sp_acl_ctcam_entry,
    pub lkey_id: *mut mlxsw_sp_acl_atcam_lkey_id,
    pub erp_mask: *mut mlxsw_sp_acl_erp_mask,
}

extern "C" {
    pub fn container_of(_arg: cregion, mlxsw_sp_acl_atcam_region: struct, _arg: cregion) -> return;
}
extern "C" {
    pub fn container_of(_arg: centry, mlxsw_sp_acl_atcam_entry: struct, _arg: centry) -> return;
}
extern "C" {
    pub fn mlxsw_sp_acl_atcam_region_fini(aregion: *mut mlxsw_sp_acl_atcam_region);
}
extern "C" {
    pub fn mlxsw_sp_acl_atcam_chunk_fini(achunk: *mut mlxsw_sp_acl_atcam_chunk);
}
extern "C" {
    pub fn mlxsw_sp_acl_atcam_rehash_hints_put(hints_priv: *mut c_void);
}
extern "C" {
    pub fn mlxsw_sp_acl_erp_delta_start(delta: *const mlxsw_sp_acl_erp_delta) -> u16;
}
extern "C" {
    pub fn mlxsw_sp_acl_erp_delta_mask(delta: *const mlxsw_sp_acl_erp_delta) -> u8;
}
extern "C" {
    pub fn mlxsw_sp_acl_erp_mask_erp_id(erp_mask: *const mlxsw_sp_acl_erp_mask) -> u8;
}
extern "C" {
    pub fn mlxsw_sp_acl_erp_rehash_hints_put(hints_priv: *mut c_void);
}
extern "C" {
    pub fn mlxsw_sp_acl_erp_region_fini(aregion: *mut mlxsw_sp_acl_atcam_region);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_acl_bf_ops {
    pub aentry): *mut mlxsw_sp_acl_atcam_entry,
}

extern "C" {
    pub fn mlxsw_sp_acl_bf_fini(bf: *mut mlxsw_sp_acl_bf);
}
