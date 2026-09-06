//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/mlx5_ifc_dr_ste_v1.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_ste_v1_modify_hdr_offset {
    MLX5_MODIFY_HEADER_V1_QW_OFFSET = 0x20,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_single_action_flow_tag_v1_bits {
    pub action_id: [u8; 0x8],
    pub flow_tag: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_single_action_modify_list_v1_bits {
    pub action_id: [u8; 0x8],
    pub num_of_modify_actions: [u8; 0x8],
    pub modify_actions_ptr: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_single_action_remove_header_v1_bits {
    pub action_id: [u8; 0x8],
    pub reserved_at_8: [u8; 0x2],
    pub start_anchor: [u8; 0x6],
    pub reserved_at_10: [u8; 0x2],
    pub end_anchor: [u8; 0x6],
    pub reserved_at_18: [u8; 0x4],
    pub decap: [u8; 0x1],
    pub vni_to_cqe: [u8; 0x1],
    pub qos_profile: [u8; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_single_action_remove_header_size_v1_bits {
    pub action_id: [u8; 0x8],
    pub reserved_at_8: [u8; 0x2],
    pub start_anchor: [u8; 0x6],
    pub outer_l4_remove: [u8; 0x1],
    pub reserved_at_11: [u8; 0x1],
    pub start_offset: [u8; 0x7],
    pub reserved_at_18: [u8; 0x1],
    pub remove_size: [u8; 0x6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_double_action_copy_v1_bits {
    pub action_id: [u8; 0x8],
    pub destination_dw_offset: [u8; 0x8],
    pub reserved_at_10: [u8; 0x2],
    pub destination_left_shifter: [u8; 0x6],
    pub reserved_at_17: [u8; 0x2],
    pub destination_length: [u8; 0x6],
    pub reserved_at_20: [u8; 0x8],
    pub source_dw_offset: [u8; 0x8],
    pub reserved_at_30: [u8; 0x2],
    pub source_right_shifter: [u8; 0x6],
    pub reserved_at_38: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_double_action_set_v1_bits {
    pub action_id: [u8; 0x8],
    pub destination_dw_offset: [u8; 0x8],
    pub reserved_at_10: [u8; 0x2],
    pub destination_left_shifter: [u8; 0x6],
    pub reserved_at_18: [u8; 0x2],
    pub destination_length: [u8; 0x6],
    pub inline_data: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_double_action_add_v1_bits {
    pub action_id: [u8; 0x8],
    pub destination_dw_offset: [u8; 0x8],
    pub reserved_at_10: [u8; 0x2],
    pub destination_left_shifter: [u8; 0x6],
    pub reserved_at_18: [u8; 0x2],
    pub destination_length: [u8; 0x6],
    pub add_value: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_double_action_insert_with_inline_v1_bits {
    pub action_id: [u8; 0x8],
    pub reserved_at_8: [u8; 0x2],
    pub start_anchor: [u8; 0x6],
    pub start_offset: [u8; 0x7],
    pub reserved_at_17: [u8; 0x9],
    pub inline_data: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_double_action_insert_with_ptr_v1_bits {
    pub action_id: [u8; 0x8],
    pub reserved_at_8: [u8; 0x2],
    pub start_anchor: [u8; 0x6],
    pub start_offset: [u8; 0x7],
    pub size: [u8; 0x6],
    pub attributes: [u8; 0x3],
    pub pointer: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_double_action_accelerated_modify_action_list_v1_bits {
    pub action_id: [u8; 0x8],
    pub modify_actions_pattern_pointer: [u8; 0x18],
    pub number_of_modify_actions: [u8; 0x8],
    pub modify_actions_argument_pointer: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_match_bwc_v1_bits {
    pub entry_format: [u8; 0x8],
    pub counter_id: [u8; 0x18],
    pub miss_address_63_48: [u8; 0x10],
    pub match_definer_ctx_idx: [u8; 0x8],
    pub miss_address_39_32: [u8; 0x8],
    pub miss_address_31_6: [u8; 0x1a],
    pub reserved_at_5a: [u8; 0x1],
    pub match_polarity: [u8; 0x1],
    pub reparse: [u8; 0x1],
    pub reserved_at_5d: [u8; 0x3],
    pub next_table_base_63_48: [u8; 0x10],
    pub hash_definer_ctx_idx: [u8; 0x8],
    pub next_table_base_39_32_size: [u8; 0x8],
    pub next_table_base_31_5_size: [u8; 0x1b],
    pub hash_type: [u8; 0x2],
    pub hash_after_actions: [u8; 0x1],
    pub reserved_at_9e: [u8; 0x2],
    pub byte_mask: [u8; 0x10],
    pub next_entry_format: [u8; 0x1],
    pub mask_mode: [u8; 0x1],
    pub gvmi: [u8; 0xe],
    pub action: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_mask_and_match_v1_bits {
    pub entry_format: [u8; 0x8],
    pub counter_id: [u8; 0x18],
    pub miss_address_63_48: [u8; 0x10],
    pub match_definer_ctx_idx: [u8; 0x8],
    pub miss_address_39_32: [u8; 0x8],
    pub miss_address_31_6: [u8; 0x1a],
    pub reserved_at_5a: [u8; 0x1],
    pub match_polarity: [u8; 0x1],
    pub reparse: [u8; 0x1],
    pub reserved_at_5d: [u8; 0x3],
    pub next_table_base_63_48: [u8; 0x10],
    pub hash_definer_ctx_idx: [u8; 0x8],
    pub next_table_base_39_32_size: [u8; 0x8],
    pub next_table_base_31_5_size: [u8; 0x1b],
    pub hash_type: [u8; 0x2],
    pub hash_after_actions: [u8; 0x1],
    pub reserved_at_9e: [u8; 0x2],
    pub action: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_match_ranges_v1_bits {
    pub entry_format: [u8; 0x8],
    pub counter_id: [u8; 0x18],
    pub miss_address_63_48: [u8; 0x10],
    pub match_definer_ctx_idx: [u8; 0x8],
    pub miss_address_39_32: [u8; 0x8],
    pub miss_address_31_6: [u8; 0x1a],
    pub reserved_at_5a: [u8; 0x1],
    pub match_polarity: [u8; 0x1],
    pub reparse: [u8; 0x1],
    pub reserved_at_5d: [u8; 0x3],
    pub next_table_base_63_48: [u8; 0x10],
    pub hash_definer_ctx_idx: [u8; 0x8],
    pub next_table_base_39_32_size: [u8; 0x8],
    pub next_table_base_31_5_size: [u8; 0x1b],
    pub hash_type: [u8; 0x2],
    pub hash_after_actions: [u8; 0x1],
    pub reserved_at_9e: [u8; 0x2],
    pub action: [u8; 0x60],
    pub max_value_0: [u8; 0x20],
    pub min_value_0: [u8; 0x20],
    pub max_value_1: [u8; 0x20],
    pub min_value_1: [u8; 0x20],
    pub max_value_2: [u8; 0x20],
    pub min_value_2: [u8; 0x20],
    pub max_value_3: [u8; 0x20],
    pub min_value_3: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l2_src_v1_bits {
    pub reserved_at_0: [u8; 0x1],
    pub sx_sniffer: [u8; 0x1],
    pub functional_loopback: [u8; 0x1],
    pub ip_fragmented: [u8; 0x1],
    pub qp_type: [u8; 0x2],
    pub encapsulation_type: [u8; 0x2],
    pub port: [u8; 0x2],
    pub l3_type: [u8; 0x2],
    pub l4_type: [u8; 0x2],
    pub first_vlan_qualifier: [u8; 0x2],
    pub first_priority: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub first_vlan_id: [u8; 0xc],
    pub smac_47_16: [u8; 0x20],
    pub smac_15_0: [u8; 0x10],
    pub l3_ethertype: [u8; 0x10],
    pub reserved_at_60: [u8; 0x6],
    pub tcp_syn: [u8; 0x1],
    pub reserved_at_67: [u8; 0x3],
    pub force_loopback: [u8; 0x1],
    pub l2_ok: [u8; 0x1],
    pub l3_ok: [u8; 0x1],
    pub l4_ok: [u8; 0x1],
    pub second_vlan_qualifier: [u8; 0x2],
    pub second_priority: [u8; 0x3],
    pub second_cfi: [u8; 0x1],
    pub second_vlan_id: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l2_dst_v1_bits {
    pub reserved_at_0: [u8; 0x1],
    pub sx_sniffer: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub ip_fragmented: [u8; 0x1],
    pub qp_type: [u8; 0x2],
    pub encapsulation_type: [u8; 0x2],
    pub port: [u8; 0x2],
    pub l3_type: [u8; 0x2],
    pub l4_type: [u8; 0x2],
    pub first_vlan_qualifier: [u8; 0x2],
    pub first_priority: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub first_vlan_id: [u8; 0xc],
    pub dmac_47_16: [u8; 0x20],
    pub dmac_15_0: [u8; 0x10],
    pub l3_ethertype: [u8; 0x10],
    pub reserved_at_60: [u8; 0x6],
    pub tcp_syn: [u8; 0x1],
    pub reserved_at_67: [u8; 0x3],
    pub force_lb: [u8; 0x1],
    pub l2_ok: [u8; 0x1],
    pub l3_ok: [u8; 0x1],
    pub l4_ok: [u8; 0x1],
    pub second_vlan_qualifier: [u8; 0x2],
    pub second_priority: [u8; 0x3],
    pub second_cfi: [u8; 0x1],
    pub second_vlan_id: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l2_src_dst_v1_bits {
    pub dmac_47_16: [u8; 0x20],
    pub smac_47_16: [u8; 0x20],
    pub dmac_15_0: [u8; 0x10],
    pub reserved_at_50: [u8; 0x2],
    pub functional_lb: [u8; 0x1],
    pub reserved_at_53: [u8; 0x5],
    pub port: [u8; 0x2],
    pub l3_type: [u8; 0x2],
    pub reserved_at_5c: [u8; 0x2],
    pub first_vlan_qualifier: [u8; 0x2],
    pub first_priority: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub first_vlan_id: [u8; 0xc],
    pub smac_15_0: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l3_ipv4_5_tuple_v1_bits {
    pub source_address: [u8; 0x20],
    pub destination_address: [u8; 0x20],
    pub source_port: [u8; 0x10],
    pub destination_port: [u8; 0x10],
    pub reserved_at_60: [u8; 0x4],
    pub l4_ok: [u8; 0x1],
    pub l3_ok: [u8; 0x1],
    pub fragmented: [u8; 0x1],
    pub tcp_ns: [u8; 0x1],
    pub tcp_cwr: [u8; 0x1],
    pub tcp_ece: [u8; 0x1],
    pub tcp_urg: [u8; 0x1],
    pub tcp_ack: [u8; 0x1],
    pub tcp_psh: [u8; 0x1],
    pub tcp_rst: [u8; 0x1],
    pub tcp_syn: [u8; 0x1],
    pub tcp_fin: [u8; 0x1],
    pub dscp: [u8; 0x6],
    pub ecn: [u8; 0x2],
    pub protocol: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l2_tnl_v1_bits {
    pub l2_tunneling_network_id: [u8; 0x20],
    pub dmac_47_16: [u8; 0x20],
    pub dmac_15_0: [u8; 0x10],
    pub l3_ethertype: [u8; 0x10],
    pub reserved_at_60: [u8; 0x3],
    pub ip_fragmented: [u8; 0x1],
    pub reserved_at_64: [u8; 0x2],
    pub encp_type: [u8; 0x2],
    pub reserved_at_68: [u8; 0x2],
    pub l3_type: [u8; 0x2],
    pub l4_type: [u8; 0x2],
    pub first_vlan_qualifier: [u8; 0x2],
    pub first_priority: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub first_vlan_id: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l3_ipv4_misc_v1_bits {
    pub identification: [u8; 0x10],
    pub flags: [u8; 0x3],
    pub fragment_offset: [u8; 0xd],
    pub total_length: [u8; 0x10],
    pub checksum: [u8; 0x10],
    pub version: [u8; 0x4],
    pub ihl: [u8; 0x4],
    pub time_to_live: [u8; 0x8],
    pub reserved_at_50: [u8; 0x10],
    pub reserved_at_60: [u8; 0x1c],
    pub voq_internal_prio: [u8; 0x4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l4_v1_bits {
    pub ipv6_version: [u8; 0x4],
    pub reserved_at_4: [u8; 0x4],
    pub dscp: [u8; 0x6],
    pub ecn: [u8; 0x2],
    pub ipv6_hop_limit: [u8; 0x8],
    pub protocol: [u8; 0x8],
    pub src_port: [u8; 0x10],
    pub dst_port: [u8; 0x10],
    pub first_fragment: [u8; 0x1],
    pub reserved_at_41: [u8; 0xb],
    pub flow_label: [u8; 0x14],
    pub tcp_data_offset: [u8; 0x4],
    pub l4_ok: [u8; 0x1],
    pub l3_ok: [u8; 0x1],
    pub fragmented: [u8; 0x1],
    pub tcp_ns: [u8; 0x1],
    pub tcp_cwr: [u8; 0x1],
    pub tcp_ece: [u8; 0x1],
    pub tcp_urg: [u8; 0x1],
    pub tcp_ack: [u8; 0x1],
    pub tcp_psh: [u8; 0x1],
    pub tcp_rst: [u8; 0x1],
    pub tcp_syn: [u8; 0x1],
    pub tcp_fin: [u8; 0x1],
    pub ipv6_paylen: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l4_misc_v1_bits {
    pub window_size: [u8; 0x10],
    pub urgent_pointer: [u8; 0x10],
    pub ack_num: [u8; 0x20],
    pub seq_num: [u8; 0x20],
    pub length: [u8; 0x10],
    pub checksum: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_mpls_v1_bits {
    pub reserved_at_0: [u8; 0x15],
    pub mpls_ok: [u8; 0x1],
    pub mpls4_s_bit: [u8; 0x1],
    pub mpls4_qualifier: [u8; 0x1],
    pub mpls3_s_bit: [u8; 0x1],
    pub mpls3_qualifier: [u8; 0x1],
    pub mpls2_s_bit: [u8; 0x1],
    pub mpls2_qualifier: [u8; 0x1],
    pub mpls1_s_bit: [u8; 0x1],
    pub mpls1_qualifier: [u8; 0x1],
    pub mpls0_s_bit: [u8; 0x1],
    pub mpls0_qualifier: [u8; 0x1],
    pub mpls0_label: [u8; 0x14],
    pub mpls0_exp: [u8; 0x3],
    pub mpls0_s_bos: [u8; 0x1],
    pub mpls0_ttl: [u8; 0x8],
    pub mpls1_label: [u8; 0x20],
    pub mpls2_label: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_gre_v1_bits {
    pub gre_c_present: [u8; 0x1],
    pub reserved_at_1: [u8; 0x1],
    pub gre_k_present: [u8; 0x1],
    pub gre_s_present: [u8; 0x1],
    pub strict_src_route: [u8; 0x1],
    pub recur: [u8; 0x3],
    pub flags: [u8; 0x5],
    pub version: [u8; 0x3],
    pub gre_protocol: [u8; 0x10],
    pub reserved_at_20: [u8; 0x20],
    pub gre_key_h: [u8; 0x18],
    pub gre_key_l: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_src_gvmi_qp_v1_bits {
    pub loopback_synd: [u8; 0x8],
    pub reserved_at_8: [u8; 0x7],
    pub functional_lb: [u8; 0x1],
    pub source_gvmi: [u8; 0x10],
    pub force_lb: [u8; 0x1],
    pub reserved_at_21: [u8; 0x1],
    pub source_is_requestor: [u8; 0x1],
    pub reserved_at_23: [u8; 0x5],
    pub source_qp: [u8; 0x18],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_icmp_v1_bits {
    pub icmp_payload_data: [u8; 0x20],
    pub icmp_header_data: [u8; 0x20],
    pub icmp_type: [u8; 0x8],
    pub icmp_code: [u8; 0x8],
    pub reserved_at_50: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}
