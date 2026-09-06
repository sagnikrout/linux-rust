//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/action.h
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
// Max number of STEs needed for a rule (including match)
pub const MLX5HWS_ACTION_MAX_STE: c_int = 20;
// Max number of internal subactions of ipv6_ext
pub const MLX5HWS_ACTION_IPV6_EXT_MAX_SA: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_action_stc_idx {
    MLX5HWS_ACTION_STC_IDX_CTRL = 0,
    MLX5HWS_ACTION_STC_IDX_HIT = 1,
    MLX5HWS_ACTION_STC_IDX_DW5 = 2,
    MLX5HWS_ACTION_STC_IDX_DW6 = 3,
    MLX5HWS_ACTION_STC_IDX_DW7 = 4,
    MLX5HWS_ACTION_STC_IDX_MAX = 5,
// STC Jumvo STE combo: CTR, Hit
    MLX5HWS_ACTION_STC_IDX_LAST_JUMBO_STE = 1,
// STC combo1: CTR, SINGLE, DOUBLE, Hit
    MLX5HWS_ACTION_STC_IDX_LAST_COMBO1 = 3,
// STC combo2: CTR, 3 x SINGLE, Hit
    MLX5HWS_ACTION_STC_IDX_LAST_COMBO2 = 4,
// STC combo2: CTR, TRIPLE, Hit
    MLX5HWS_ACTION_STC_IDX_LAST_COMBO3 = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_action_offset {
    MLX5HWS_ACTION_OFFSET_DW0 = 0,
    MLX5HWS_ACTION_OFFSET_DW5 = 5,
    MLX5HWS_ACTION_OFFSET_DW6 = 6,
    MLX5HWS_ACTION_OFFSET_DW7 = 7,
    MLX5HWS_ACTION_OFFSET_HIT = 3,
    MLX5HWS_ACTION_OFFSET_HIT_LSB = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_action_setter_flag {
    ASF_SINGLE1 = 1 << 0,
    ASF_SINGLE2 = 1 << 1,
    ASF_SINGLE3 = 1 << 2,
    ASF_DOUBLE = ASF_SINGLE2 | ASF_SINGLE3,
    ASF_TRIPLE = ASF_SINGLE1 | ASF_DOUBLE,
    ASF_INSERT = 1 << 3,
    ASF_REMOVE = 1 << 4,
    ASF_MODIFY = 1 << 5,
    ASF_CTR = 1 << 6,
    ASF_HIT = 1 << 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_default_stc {
    pub nop_ctr: mlx5hws_pool_chunk,
    pub nop_dw5: mlx5hws_pool_chunk,
    pub nop_dw6: mlx5hws_pool_chunk,
    pub nop_dw7: mlx5hws_pool_chunk,
    pub default_hit: mlx5hws_pool_chunk,
    pub /: *mut *mut u32 refcount; / protected by context ctrl lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_shared_stc {
    pub stc_chunk: mlx5hws_pool_chunk,
    pub /: *mut *mut u32 refcount; / protected by context ctrl lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_actions_apply_data {
    pub queue: *mut mlx5hws_send_engine,
    pub rule_action: *mut mlx5hws_rule_action,
    pub wqe_data: *mut __be32,
    pub wqe_ctrl: *mut mlx5hws_wqe_gta_ctrl_seg,
    pub jump_to_action_stc: u32,
    pub common_res: *mut mlx5hws_context_common_res,
    pub tbl_type: mlx5hws_table_type,
    pub next_direct_idx: u32,
    pub require_dep: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_actions_wqe_setter {
    pub set_single: mlx5hws_action_setter_fp,
    pub set_double: mlx5hws_action_setter_fp,
    pub set_triple: mlx5hws_action_setter_fp,
    pub set_hit: mlx5hws_action_setter_fp,
    pub set_ctr: mlx5hws_action_setter_fp,
    pub idx_single: u8,
    pub idx_double: u8,
    pub idx_triple: u8,
    pub idx_ctr: u8,
    pub idx_hit: u8,
    pub stage_idx: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action_template {
    pub setters: [mlx5hws_actions_wqe_setter; MLX5HWS_ACTION_MAX_STE],
    pub action_type_arr: *mut mlx5hws_action_type,
    pub num_of_action_stes: u8,
    pub num_actions: u8,
    pub only_term: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_range_action_table {
    pub pool: *mut mlx5hws_pool,
    pub rtc_0_id: u32,
    pub rtc_1_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_action {
    pub type: u8,
    pub flags: u8,
    pub ctx: *mut mlx5hws_context,
    pub stc: mlx5hws_pool_chunk,
    pub pat_id: u32,
    pub arg_id: u32,
    pub single_action: __be64,
    pub nop_locations: u32,
    pub num_of_patterns: u8,
    pub single_action_type: u8,
    pub num_of_actions: u8,
    pub max_num_of_actions: u8,
    pub require_reparse: u8,
    pub modify_header: },
    pub arg_id: u32,
    pub header_size: u32,
    pub max_hdr_sz: u16,
    pub num_of_hdrs: u8,
    pub anchor: u8,
    pub e_anchor: u8,
    pub offset: u8,
    pub encap: bool,
    pub require_reparse: u8,
    pub reformat: },
    pub obj_id: u32,
    pub return_reg_id: u8,
    pub aso: },
    pub vport_num: u16,
    pub esw_owner_vhca_id: u16,
    pub esw_owner_vhca_id_valid: bool,
    pub vport: },
    pub obj_id: u32,
    pub dest_obj: },
    pub fw_island: *mut mlx5hws_cmd_forward_tbl,
    pub num_dest: usize,
    pub dest_list: *mut mlx5hws_cmd_set_fte_dest,
    pub dest_array: },
    pub fw_island: *mut mlx5hws_cmd_forward_tbl,
    pub flow_sampler: },
    pub type: u8,
    pub start_anchor: u8,
    pub end_anchor: u8,
    pub num_of_words: u8,
    pub decap: bool,
    pub insert_hdr: },
// PRM start anchor from which header will be removed
    pub anchor: u8,
// Header remove offset in bytes, from the start
// anchor to the location where remove header starts.
//
    pub offset: u8,
// Indicates the removed header size in bytes
    pub size: usize,
    pub remove_header: },
    pub table_ste: *mut mlx5hws_range_action_table,
    pub hit_ft_action: *mut mlx5hws_action,
    pub definer: *mut mlx5hws_definer,
    pub range: },
}

extern "C" {
    pub fn mlx5hws_action_template_process(at: *mut mlx5hws_action_template) -> c_int;
}
// Set control counter
// Set triple on match
// Set single and double on match
// Set next/final hit action
// Set number of actions
