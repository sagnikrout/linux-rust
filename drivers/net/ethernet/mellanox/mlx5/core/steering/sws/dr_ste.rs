//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/dr_ste.h
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
// Copyright (c) 2020 NVIDIA CORPORATION. All rights reserved.

pub const STE_IPV4: c_uint = 0x1;
pub const STE_IPV6: c_uint = 0x2;
pub const STE_TCP: c_uint = 0x1;
pub const STE_UDP: c_uint = 0x2;
pub const STE_SPI: c_uint = 0x3;
pub const IP_VERSION_IPV4: c_uint = 0x4;
pub const IP_VERSION_IPV6: c_uint = 0x6;
pub const STE_SVLAN: c_uint = 0x1;
pub const STE_CVLAN: c_uint = 0x2;
pub const HDR_LEN_L2_MACS: c_uint = 0xC;
pub const HDR_LEN_L2_VLAN: c_uint = 0x4;
pub const HDR_LEN_L2_ETHER: c_uint = 0x2;

// Set to STE a specific value using DR_STE_SET

// Set to STE spec->s_fname to tag->t_fname set spec->s_fname as used

// Set to STE -1 to tag->t_fname and set spec->s_fname as used

// (__be32 *)parser_ptr = cpu_to_be32((spec)->fname);\

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dr_ste_action_modify_type_l3 {
    DR_STE_ACTION_MDFY_TYPE_L3_NONE	= 0x0,
    DR_STE_ACTION_MDFY_TYPE_L3_IPV4	= 0x1,
    DR_STE_ACTION_MDFY_TYPE_L3_IPV6	= 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dr_ste_action_modify_type_l4 {
    DR_STE_ACTION_MDFY_TYPE_L4_NONE	= 0x0,
    DR_STE_ACTION_MDFY_TYPE_L4_TCP	= 0x1,
    DR_STE_ACTION_MDFY_TYPE_L4_UDP	= 0x2,
}

extern "C" {
    pub fn mlx5dr_ste_conv_bit_to_byte_mask(bit_mask: *mut u8) -> u16;
}
// Calculate tag byte offset based on flex parser id

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_ste_ctx {
// Builders
    pub DR_STE_CTX_BUILDER(eth_l2_src_dst): c_void,
    pub DR_STE_CTX_BUILDER(eth_l3_ipv6_src): c_void,
    pub DR_STE_CTX_BUILDER(eth_l3_ipv6_dst): c_void,
    pub DR_STE_CTX_BUILDER(eth_l3_ipv4_5_tuple): c_void,
    pub DR_STE_CTX_BUILDER(eth_l2_src): c_void,
    pub DR_STE_CTX_BUILDER(eth_l2_dst): c_void,
    pub DR_STE_CTX_BUILDER(eth_l2_tnl): c_void,
    pub DR_STE_CTX_BUILDER(eth_l3_ipv4_misc): c_void,
    pub DR_STE_CTX_BUILDER(eth_ipv6_l3_l4): c_void,
    pub DR_STE_CTX_BUILDER(mpls): c_void,
    pub DR_STE_CTX_BUILDER(tnl_gre): c_void,
    pub DR_STE_CTX_BUILDER(tnl_mpls): c_void,
    pub DR_STE_CTX_BUILDER(tnl_mpls_over_gre): c_void,
    pub DR_STE_CTX_BUILDER(tnl_mpls_over_udp): c_void,
    pub DR_STE_CTX_BUILDER(icmp): c_void,
    pub DR_STE_CTX_BUILDER(general_purpose): c_void,
    pub DR_STE_CTX_BUILDER(eth_l4_misc): c_void,
    pub DR_STE_CTX_BUILDER(tnl_vxlan_gpe): c_void,
    pub DR_STE_CTX_BUILDER(tnl_geneve): c_void,
    pub DR_STE_CTX_BUILDER(tnl_geneve_tlv_opt): c_void,
    pub DR_STE_CTX_BUILDER(tnl_geneve_tlv_opt_exist): c_void,
    pub DR_STE_CTX_BUILDER(register_0): c_void,
    pub DR_STE_CTX_BUILDER(register_1): c_void,
    pub DR_STE_CTX_BUILDER(src_gvmi_qpn): c_void,
    pub DR_STE_CTX_BUILDER(flex_parser_0): c_void,
    pub DR_STE_CTX_BUILDER(flex_parser_1): c_void,
    pub DR_STE_CTX_BUILDER(tnl_gtpu): c_void,
    pub DR_STE_CTX_BUILDER(tnl_header_0_1): c_void,
    pub DR_STE_CTX_BUILDER(tnl_gtpu_flex_parser_0): c_void,
    pub DR_STE_CTX_BUILDER(tnl_gtpu_flex_parser_1): c_void,
// Getters and Setters
    pub gvmi): bool is_rx, u16,
    pub lu_type): *mut *mut *mut void (set_next_lu_type)(u8 hw_ste_p, u16,
    pub hw_ste_p): *mut *mut u16 (get_next_lu_type)(u8,
    pub hw_ste_p): *mut *mut bool (is_miss_addr_set)(u8,
    pub miss_addr): *mut *mut *mut void (set_miss_addr)(u8 hw_ste_p, u64,
    pub hw_ste_p): *mut *mut u64 (get_miss_addr)(u8,
    pub ht_size): *mut *mut *mut void (set_hit_addr)(u8 hw_ste_p, u64 icm_addr, u32,
    pub byte_mask): *mut *mut *mut void (set_byte_mask)(u8 hw_ste_p, u16,
    pub hw_ste_p): *mut *mut u16 (get_byte_mask)(u8,
// Actions
    pub actions_caps: u32,
    pub added_stes): *mut u32,
    pub added_stes): *mut u32,
    pub modify_field_arr_sz: u32,
    pub modify_field_arr: *const mlx5dr_ste_action_modify_field,
    pub data): u32,
    pub data): u32,
    pub src_shifter): u8,
    pub used_hw_action_num): *mut u16,
    pub action): *mut *mut int (alloc_modify_hdr_chunk)(struct mlx5dr_action,
    pub action): *mut *mut void (dealloc_modify_hdr_chunk)(struct mlx5dr_action,
// Actions bit set
    pub size): u32 reformat_id, int,
    pub vlan_hdr): u32,
    pub vlans_num): u8,
    pub s_action): *mut *mut *mut void (set_rx_decap)(u8 hw_ste_p, u8,
    pub size): c_int,
    pub size): u8 anchor, u8 offset, int,
    pub size): u8 offset, int,
// Send
    pub ste_size): *mut *mut *mut void (prepare_for_postsend)(u8 hw_ste_p, u32,
}
