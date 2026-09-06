//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/cmd.h
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
pub const WIRE_PORT: c_uint = 0xFFFF;
pub const ACCESS_KEY_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_cmd_ext_dest_flags {
    MLX5HWS_CMD_EXT_DEST_REFORMAT = 1 << 0,
    MLX5HWS_CMD_EXT_DEST_ESW_OWNER_VHCA_ID = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_set_fte_dest {
    pub destination_type: u8,
    pub destination_id: u32,
    pub ext_flags: mlx5hws_cmd_ext_dest_flags,
    pub ext_reformat_id: u32,
    pub esw_owner_vhca_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_set_fte_attr {
    pub action_flags: u32,
    pub ignore_flow_level: bool,
    pub flow_source: u8,
    pub extended_dest: u8,
    pub encrypt_decrypt_type: u8,
    pub encrypt_decrypt_obj_id: u32,
    pub packet_reformat_id: u32,
    pub dests_num: u32,
    pub dests: *mut mlx5hws_cmd_set_fte_dest,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_ft_create_attr {
    pub type: u8,
    pub level: u8,
    pub uid: u16,
    pub rtc_valid: bool,
    pub decap_en: bool,
    pub reformat_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_ft_modify_attr {
    pub type: u8,
    pub rtc_id_0: u32,
    pub rtc_id_1: u32,
    pub table_miss_id: u32,
    pub table_miss_action: u8,
    pub modify_fs: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_ft_query_attr {
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_fg_attr {
    pub table_id: u32,
    pub table_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_forward_tbl {
    pub type: u8,
    pub ft_id: u32,
    pub fg_id: u32,
    pub /: *mut *mut u32 refcount; / protected by context ctrl lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_rtc_create_attr {
    pub pd: u32,
    pub stc_base: u32,
    pub ste_base: u32,
    pub miss_ft_id: u32,
    pub fw_gen_wqe: bool,
    pub update_index_mode: u8,
    pub access_index_mode: u8,
    pub num_hash_definer: u8,
    pub log_depth: u8,
    pub log_size: u8,
    pub table_type: u8,
    pub match_definer_0: u8,
    pub match_definer_1: u8,
    pub reparse_mode: u8,
    pub is_frst_jumbo: bool,
    pub is_scnd_range: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_alias_obj_create_attr {
    pub obj_id: u32,
    pub vhca_id: u16,
    pub obj_type: u16,
    pub access_key: [u8; ACCESS_KEY_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_stc_create_attr {
    pub log_obj_range: u8,
    pub table_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_stc_modify_attr {
    pub stc_offset: u32,
    pub action_offset: u8,
    pub reparse_mode: u8,
    pub action_type: mlx5_ifc_stc_action_type,
    pub /: *mut *mut u32 id; / TIRN, TAG, FT ID, STE ID, CRYPTO,
    pub decap: u8,
    pub start_anchor: u16,
    pub end_anchor: u16,
    pub remove_header: },
    pub arg_id: u32,
    pub pattern_id: u32,
    pub modify_header: },
    pub data: __be64,
    pub modify_action: },
    pub arg_id: u32,
    pub header_size: u32,
    pub is_inline: u8,
    pub encap: u8,
    pub insert_anchor: u16,
    pub insert_offset: u16,
    pub insert_header: },
    pub aso_type: u8,
    pub devx_obj_id: u32,
    pub return_reg_id: u8,
    pub aso: },
    pub vport_num: u16,
    pub esw_owner_vhca_id: u16,
    pub eswitch_owner_vhca_id_valid: u8,
    pub vport: },
    pub ste: mlx5hws_pool_chunk,
    pub ste_pool: *mut mlx5hws_pool,
    pub /: *mut *mut u32 ste_obj_id; / Internal,
    pub match_definer_id: u32,
    pub log_hash_size: u8,
    pub ignore_tx: bool,
    pub ste_table: },
    pub start_anchor: u16,
    pub num_of_words: u16,
    pub remove_words: },
    pub type: u8,
    pub op: u8,
    pub size: u8,
    pub reformat_trailer: },
    pub dest_table_id: u32,
    pub dest_tir_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_ste_create_attr {
    pub log_obj_range: u8,
    pub table_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_definer_create_attr {
    pub dw_selector: *mut u8,
    pub byte_selector: *mut u8,
    pub match_mask: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_allow_other_vhca_access_attr {
    pub obj_type: u16,
    pub obj_id: u32,
    pub access_key: [u8; ACCESS_KEY_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_packet_reformat_create_attr {
    pub type: u8,
    pub data_sz: usize,
    pub data: *mut c_void,
    pub reformat_param_0: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_query_ft_caps {
    pub max_level: u8,
    pub reparse: u8,
    pub ignore_flow_level_rtc_valid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_generate_wqe_attr {
    pub wqe_ctrl: *mut u8,
    pub gta_ctrl: *mut u8,
    pub gta_data_0: *mut u8,
    pub gta_data_1: *mut u8,
    pub pdn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_cmd_query_caps {
    pub flex_protocols: u32,
    pub wqe_based_update: u8,
    pub rtc_reparse_mode: u8,
    pub ste_format: u16,
    pub rtc_index_mode: u8,
    pub ste_alloc_log_max: u8,
    pub ste_alloc_log_gran: u8,
    pub stc_alloc_log_max: u8,
    pub stc_alloc_log_gran: u8,
    pub rtc_log_depth_max: u8,
    pub format_select_gtpu_dw_0: u8,
    pub format_select_gtpu_dw_1: u8,
    pub flow_table_hash_type: u8,
    pub format_select_gtpu_dw_2: u8,
    pub format_select_gtpu_ext_dw_0: u8,
    pub access_index_mode: u8,
    pub linear_match_definer: u32,
    pub full_dw_jumbo_support: bool,
    pub rtc_hash_split_table: bool,
    pub rtc_linear_lookup_table: bool,
    pub supp_type_gen_wqe: u32,
    pub rtc_max_hash_def_gen_wqe: u8,
    pub supp_ste_format_gen_wqe: u16,
    pub nic_ft: mlx5hws_cmd_query_ft_caps,
    pub fdb_ft: mlx5hws_cmd_query_ft_caps,
    pub eswitch_manager: bool,
    pub merged_eswitch: bool,
    pub eswitch_manager_vport_number: u32,
    pub log_header_modify_argument_granularity: u8,
    pub log_header_modify_argument_max_alloc: u8,
    pub sq_ts_format: u8,
    pub fdb_tir_stc: u8,
    pub definer_format_sup: u64,
    pub trivial_match_definer: u32,
    pub vhca_id: u32,
    pub shared_vhca_id: u32,
    pub fw_ver: [c_char; 64],
    pub ipsec_offload: bool,
    pub is_ecpf: bool,
    pub flex_parser_ok_bits_supp: u8,
    pub flex_parser_id_geneve_tlv_option_0: u8,
    pub flex_parser_id_mpls_over_gre: u8,
    pub flex_parser_id_mpls_over_udp: u8,
}

extern "C" {
    pub fn mlx5hws_cmd_rtc_destroy(mdev: *mut mlx5_core_dev, rtc_id: u32);
}
extern "C" {
    pub fn mlx5hws_cmd_stc_destroy(mdev: *mut mlx5_core_dev, stc_id: u32);
}
extern "C" {
    pub fn mlx5hws_cmd_ste_destroy(mdev: *mut mlx5_core_dev, ste_id: u32);
}
extern "C" {
    pub fn mlx5hws_cmd_sq_modify_rdy(mdev: *mut mlx5_core_dev, sqn: u32) -> c_int;
}
