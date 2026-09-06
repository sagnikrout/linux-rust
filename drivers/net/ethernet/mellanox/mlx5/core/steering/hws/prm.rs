//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/prm.h
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
pub const MLX5_MAX_ACTIONS_DATA_IN_HEADER_MODIFY: c_int = 512;
// Action type of header modification.
// The field of packet to be modified.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_modification_field {
    MLX5_MODI_OUT_NONE = -1,
    MLX5_MODI_OUT_SMAC_47_16 = 1,
    MLX5_MODI_OUT_SMAC_15_0,
    MLX5_MODI_OUT_ETHERTYPE,
    MLX5_MODI_OUT_DMAC_47_16,
    MLX5_MODI_OUT_DMAC_15_0,
    MLX5_MODI_OUT_IP_DSCP,
    MLX5_MODI_OUT_TCP_FLAGS,
    MLX5_MODI_OUT_TCP_SPORT,
    MLX5_MODI_OUT_TCP_DPORT,
    MLX5_MODI_OUT_IPV4_TTL,
    MLX5_MODI_OUT_UDP_SPORT,
    MLX5_MODI_OUT_UDP_DPORT,
    MLX5_MODI_OUT_SIPV6_127_96,
    MLX5_MODI_OUT_SIPV6_95_64,
    MLX5_MODI_OUT_SIPV6_63_32,
    MLX5_MODI_OUT_SIPV6_31_0,
    MLX5_MODI_OUT_DIPV6_127_96,
    MLX5_MODI_OUT_DIPV6_95_64,
    MLX5_MODI_OUT_DIPV6_63_32,
    MLX5_MODI_OUT_DIPV6_31_0,
    MLX5_MODI_OUT_SIPV4,
    MLX5_MODI_OUT_DIPV4,
    MLX5_MODI_OUT_FIRST_VID,
    MLX5_MODI_IN_SMAC_47_16 = 0x31,
    MLX5_MODI_IN_SMAC_15_0,
    MLX5_MODI_IN_ETHERTYPE,
    MLX5_MODI_IN_DMAC_47_16,
    MLX5_MODI_IN_DMAC_15_0,
    MLX5_MODI_IN_IP_DSCP,
    MLX5_MODI_IN_TCP_FLAGS,
    MLX5_MODI_IN_TCP_SPORT,
    MLX5_MODI_IN_TCP_DPORT,
    MLX5_MODI_IN_IPV4_TTL,
    MLX5_MODI_IN_UDP_SPORT,
    MLX5_MODI_IN_UDP_DPORT,
    MLX5_MODI_IN_SIPV6_127_96,
    MLX5_MODI_IN_SIPV6_95_64,
    MLX5_MODI_IN_SIPV6_63_32,
    MLX5_MODI_IN_SIPV6_31_0,
    MLX5_MODI_IN_DIPV6_127_96,
    MLX5_MODI_IN_DIPV6_95_64,
    MLX5_MODI_IN_DIPV6_63_32,
    MLX5_MODI_IN_DIPV6_31_0,
    MLX5_MODI_IN_SIPV4,
    MLX5_MODI_IN_DIPV4,
    MLX5_MODI_OUT_IPV6_HOPLIMIT,
    MLX5_MODI_IN_IPV6_HOPLIMIT,
    MLX5_MODI_META_DATA_REG_A,
    MLX5_MODI_META_DATA_REG_B = 0x50,
    MLX5_MODI_META_REG_C_0,
    MLX5_MODI_META_REG_C_1,
    MLX5_MODI_META_REG_C_2,
    MLX5_MODI_META_REG_C_3,
    MLX5_MODI_META_REG_C_4,
    MLX5_MODI_META_REG_C_5,
    MLX5_MODI_META_REG_C_6,
    MLX5_MODI_META_REG_C_7,
    MLX5_MODI_OUT_TCP_SEQ_NUM,
    MLX5_MODI_IN_TCP_SEQ_NUM,
    MLX5_MODI_OUT_TCP_ACK_NUM,
    MLX5_MODI_IN_TCP_ACK_NUM = 0x5C,
    MLX5_MODI_GTP_TEID = 0x6E,
    MLX5_MODI_OUT_IP_ECN = 0x73,
    MLX5_MODI_TUNNEL_HDR_DW_1 = 0x75,
    MLX5_MODI_GTPU_FIRST_EXT_DW_0 = 0x76,
    MLX5_MODI_HASH_RESULT = 0x81,
    MLX5_MODI_IN_MPLS_LABEL_0 = 0x8a,
    MLX5_MODI_IN_MPLS_LABEL_1,
    MLX5_MODI_IN_MPLS_LABEL_2,
    MLX5_MODI_IN_MPLS_LABEL_3,
    MLX5_MODI_IN_MPLS_LABEL_4,
    MLX5_MODI_OUT_IP_PROTOCOL = 0x4A,
    MLX5_MODI_OUT_IPV6_NEXT_HDR = 0x4A,
    MLX5_MODI_META_REG_C_8 = 0x8F,
    MLX5_MODI_META_REG_C_9 = 0x90,
    MLX5_MODI_META_REG_C_10 = 0x91,
    MLX5_MODI_META_REG_C_11 = 0x92,
    MLX5_MODI_META_REG_C_12 = 0x93,
    MLX5_MODI_META_REG_C_13 = 0x94,
    MLX5_MODI_META_REG_C_14 = 0x95,
    MLX5_MODI_META_REG_C_15 = 0x96,
    MLX5_MODI_OUT_IPV4_TOTAL_LEN = 0x11D,
    MLX5_MODI_OUT_IPV6_PAYLOAD_LEN = 0x11E,
    MLX5_MODI_OUT_IPV4_IHL = 0x11F,
    MLX5_MODI_OUT_TCP_DATA_OFFSET = 0x120,
    MLX5_MODI_OUT_ESP_SPI = 0x5E,
    MLX5_MODI_OUT_ESP_SEQ_NUM = 0x82,
    MLX5_MODI_OUT_IPSEC_NEXT_HDR = 0x126,
    MLX5_MODI_INVALID = INT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_rtc_update_mode {
    MLX5_IFC_RTC_STE_UPDATE_MODE_BY_HASH = 0x0,
    MLX5_IFC_RTC_STE_UPDATE_MODE_BY_OFFSET = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_rtc_access_mode {
    MLX5_IFC_RTC_STE_ACCESS_MODE_BY_HASH = 0x0,
    MLX5_IFC_RTC_STE_ACCESS_MODE_LINEAR = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_rtc_ste_format {
    MLX5_IFC_RTC_STE_FORMAT_8DW = 0x4,
    MLX5_IFC_RTC_STE_FORMAT_11DW = 0x5,
    MLX5_IFC_RTC_STE_FORMAT_RANGE = 0x7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_rtc_reparse_mode {
    MLX5_IFC_RTC_REPARSE_NEVER = 0x0,
    MLX5_IFC_RTC_REPARSE_ALWAYS = 0x1,
    MLX5_IFC_RTC_REPARSE_BY_STC = 0x2,
}

pub const MLX5_IFC_RTC_LINEAR_LOOKUP_TBL_LOG_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rtc_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x40],
    pub update_index_mode: [u8; 0x2],
    pub reparse_mode: [u8; 0x2],
    pub num_match_ste: [u8; 0x4],
    pub pd: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x9],
    pub access_index_mode: [u8; 0x3],
    pub num_hash_definer: [u8; 0x4],
    pub update_method: [u8; 0x1],
    pub reserved_at_b1: [u8; 0x2],
    pub log_depth: [u8; 0x5],
    pub log_hash_size: [u8; 0x8],
    pub ste_format_0: [u8; 0x8],
    pub table_type: [u8; 0x8],
    pub ste_format_1: [u8; 0x8],
    pub reserved_at_d8: [u8; 0x8],
    pub match_definer_0: [u8; 0x20],
    pub stc_id: [u8; 0x20],
    pub ste_table_base_id: [u8; 0x20],
    pub ste_table_offset: [u8; 0x20],
    pub reserved_at_160: [u8; 0x8],
    pub miss_flow_table_id: [u8; 0x18],
    pub match_definer_1: [u8; 0x20],
    pub reserved_at_1a0: [u8; 0x260],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_stc_action_type {
    MLX5_IFC_STC_ACTION_TYPE_NOP = 0x00,
    MLX5_IFC_STC_ACTION_TYPE_COPY = 0x05,
    MLX5_IFC_STC_ACTION_TYPE_SET = 0x06,
    MLX5_IFC_STC_ACTION_TYPE_ADD = 0x07,
    MLX5_IFC_STC_ACTION_TYPE_REMOVE_WORDS = 0x08,
    MLX5_IFC_STC_ACTION_TYPE_HEADER_REMOVE = 0x09,
    MLX5_IFC_STC_ACTION_TYPE_HEADER_INSERT = 0x0b,
    MLX5_IFC_STC_ACTION_TYPE_TAG = 0x0c,
    MLX5_IFC_STC_ACTION_TYPE_ACC_MODIFY_LIST = 0x0e,
    MLX5_IFC_STC_ACTION_TYPE_CRYPTO_IPSEC_ENCRYPTION = 0x10,
    MLX5_IFC_STC_ACTION_TYPE_CRYPTO_IPSEC_DECRYPTION = 0x11,
    MLX5_IFC_STC_ACTION_TYPE_ASO = 0x12,
    MLX5_IFC_STC_ACTION_TYPE_TRAILER = 0x13,
    MLX5_IFC_STC_ACTION_TYPE_COUNTER = 0x14,
    MLX5_IFC_STC_ACTION_TYPE_ADD_FIELD = 0x1b,
    MLX5_IFC_STC_ACTION_TYPE_JUMP_TO_STE_TABLE = 0x80,
    MLX5_IFC_STC_ACTION_TYPE_JUMP_TO_TIR = 0x81,
    MLX5_IFC_STC_ACTION_TYPE_JUMP_TO_FT = 0x82,
    MLX5_IFC_STC_ACTION_TYPE_DROP = 0x83,
    MLX5_IFC_STC_ACTION_TYPE_ALLOW = 0x84,
    MLX5_IFC_STC_ACTION_TYPE_JUMP_TO_VPORT = 0x85,
    MLX5_IFC_STC_ACTION_TYPE_JUMP_TO_UPLINK = 0x86,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_stc_reparse_mode {
    MLX5_IFC_STC_REPARSE_IGNORE = 0x0,
    MLX5_IFC_STC_REPARSE_NEVER = 0x1,
    MLX5_IFC_STC_REPARSE_ALWAYS = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_ste_table_bits {
    pub ste_obj_id: [u8; 0x20],
    pub match_definer_id: [u8; 0x20],
    pub reserved_at_40: [u8; 0x3],
    pub log_hash_size: [u8; 0x5],
    pub reserved_at_48: [u8; 0x38],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_tir_bits {
    pub reserved_at_0: [u8; 0x8],
    pub tirn: [u8; 0x18],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_table_bits {
    pub reserved_at_0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_flow_counter_bits {
    pub flow_counter_id: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_execute_aso_bits {
    pub aso_object_id: [u8; 0x20],
    pub return_reg_id: [u8; 0x4],
    pub aso_type: [u8; 0x4],
    pub reserved_at_28: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_ipsec_encrypt_bits {
    pub ipsec_object_id: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_ipsec_decrypt_bits {
    pub ipsec_object_id: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_trailer_bits {
    pub reserved_at_0: [u8; 0x8],
    pub command: [u8; 0x4],
    pub reserved_at_c: [u8; 0x2],
    pub type: [u8; 0x2],
    pub reserved_at_10: [u8; 0xa],
    pub length: [u8; 0x6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_header_modify_list_bits {
    pub header_modify_pattern_id: [u8; 0x20],
    pub header_modify_argument_id: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_header_anchors {
    MLX5_HEADER_ANCHOR_PACKET_START = 0x0,
    MLX5_HEADER_ANCHOR_MAC = 0x1,
    MLX5_HEADER_ANCHOR_FIRST_VLAN_START = 0x2,
    MLX5_HEADER_ANCHOR_IPV6_IPV4 = 0x07,
    MLX5_HEADER_ANCHOR_ESP = 0x08,
    MLX5_HEADER_ANCHOR_TCP_UDP = 0x09,
    MLX5_HEADER_ANCHOR_TUNNEL_HEADER = 0x0a,
    MLX5_HEADER_ANCHOR_INNER_MAC = 0x13,
    MLX5_HEADER_ANCHOR_INNER_IPV6_IPV4 = 0x19,
    MLX5_HEADER_ANCHOR_INNER_TCP_UDP = 0x1a,
    MLX5_HEADER_ANCHOR_L4_PAYLOAD = 0x1b,
    MLX5_HEADER_ANCHOR_INNER_L4_PAYLOAD = 0x1c
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_remove_bits {
    pub action_type: [u8; 0x4],
    pub decap: [u8; 0x1],
    pub reserved_at_5: [u8; 0x5],
    pub remove_start_anchor: [u8; 0x6],
    pub reserved_at_10: [u8; 0x2],
    pub remove_end_anchor: [u8; 0x6],
    pub reserved_at_18: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_remove_words_bits {
    pub action_type: [u8; 0x4],
    pub reserved_at_4: [u8; 0x6],
    pub remove_start_anchor: [u8; 0x6],
    pub reserved_at_10: [u8; 0x1],
    pub remove_offset: [u8; 0x7],
    pub reserved_at_18: [u8; 0x2],
    pub remove_size: [u8; 0x6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_insert_bits {
    pub action_type: [u8; 0x4],
    pub encap: [u8; 0x1],
    pub inline_data: [u8; 0x1],
    pub reserved_at_6: [u8; 0x4],
    pub insert_anchor: [u8; 0x6],
    pub reserved_at_10: [u8; 0x1],
    pub insert_offset: [u8; 0x7],
    pub reserved_at_18: [u8; 0x1],
    pub insert_size: [u8; 0x7],
    pub insert_argument: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_ste_param_vport_bits {
    pub eswitch_owner_vhca_id: [u8; 0x10],
    pub vport_number: [u8; 0x10],
    pub eswitch_owner_vhca_id_valid: [u8; 0x1],
    pub reserved_at_21: [u8; 0x5f],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_stc_param_bits {
    pub ste_table: mlx5_ifc_stc_ste_param_ste_table_bits,
    pub tir: mlx5_ifc_stc_ste_param_tir_bits,
    pub table: mlx5_ifc_stc_ste_param_table_bits,
    pub counter: mlx5_ifc_stc_ste_param_flow_counter_bits,
    pub modify_header: mlx5_ifc_stc_ste_param_header_modify_list_bits,
    pub aso: mlx5_ifc_stc_ste_param_execute_aso_bits,
    pub remove_header: mlx5_ifc_stc_ste_param_remove_bits,
    pub insert_header: mlx5_ifc_stc_ste_param_insert_bits,
    pub add: mlx5_ifc_set_action_in_bits,
    pub set: mlx5_ifc_set_action_in_bits,
    pub copy: mlx5_ifc_copy_action_in_bits,
    pub vport: mlx5_ifc_stc_ste_param_vport_bits,
    pub ipsec_encrypt: mlx5_ifc_stc_ste_param_ipsec_encrypt_bits,
    pub ipsec_decrypt: mlx5_ifc_stc_ste_param_ipsec_decrypt_bits,
    pub trailer: mlx5_ifc_stc_ste_param_trailer_bits,
    pub reserved_at_0: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stc_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x46],
    pub reparse_mode: [u8; 0x2],
    pub table_type: [u8; 0x8],
    pub ste_action_offset: [u8; 0x8],
    pub action_type: [u8; 0x8],
    pub reserved_at_a0: [u8; 0x60],
    pub stc_param: mlx5_ifc_stc_param_bits,
    pub reserved_at_180: [u8; 0x280],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x48],
    pub table_type: [u8; 0x8],
    pub reserved_at_90: [u8; 0x370],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x50],
    pub format_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x60],
    pub format_select_dw3: [u8; 0x8],
    pub format_select_dw2: [u8; 0x8],
    pub format_select_dw1: [u8; 0x8],
    pub format_select_dw0: [u8; 0x8],
    pub format_select_dw7: [u8; 0x8],
    pub format_select_dw6: [u8; 0x8],
    pub format_select_dw5: [u8; 0x8],
    pub format_select_dw4: [u8; 0x8],
    pub reserved_at_100: [u8; 0x18],
    pub format_select_dw8: [u8; 0x8],
    pub reserved_at_120: [u8; 0x20],
    pub format_select_byte3: [u8; 0x8],
    pub format_select_byte2: [u8; 0x8],
    pub format_select_byte1: [u8; 0x8],
    pub format_select_byte0: [u8; 0x8],
    pub format_select_byte7: [u8; 0x8],
    pub format_select_byte6: [u8; 0x8],
    pub format_select_byte5: [u8; 0x8],
    pub format_select_byte4: [u8; 0x8],
    pub reserved_at_180: [u8; 0x40],
    pub ctrl: [u8; 0xa0],
    pub match_mask: [u8; 0x160],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_modify_pattern_in_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x40],
    pub pattern_length: [u8; 0x8],
    pub reserved_at_88: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x60],
    pub 8]: *mut *mut u8 pattern_data[MLX5_MAX_ACTIONS_DATA_IN_HEADER_MODIFY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_rtc_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub rtc: mlx5_ifc_rtc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_stc_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub stc: mlx5_ifc_stc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_ste_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub ste: mlx5_ifc_ste_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_definer_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub definer: mlx5_ifc_definer_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_header_modify_pattern_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub pattern: mlx5_ifc_header_modify_pattern_in_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_generate_wqe_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mode: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub reserved_at_80: [u8; 0x8],
    pub pdn: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x160],
    pub wqe_ctrl: [u8; 0x80],
    pub wqe_gta_ctrl: [u8; 0x180],
    pub wqe_gta_data_0: [u8; 0x200],
    pub wqe_gta_data_1: [u8; 0x200],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_generate_wqe_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x1c0],
    pub cqe_data: [u8; 0x200],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_access_aso_opc_mod {
    ASO_OPC_MOD_IPSEC = 0x0,
    ASO_OPC_MOD_CONNECTION_TRACKING = 0x1,
    ASO_OPC_MOD_POLICER = 0x2,
    ASO_OPC_MOD_RACE_AVOIDANCE = 0x3,
    ASO_OPC_MOD_FLOW_HIT = 0x4,
}
