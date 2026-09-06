//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/mlx5_ifc_dr.h
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
// Copyright (c) 2019, Mellanox Technologies
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_general_bits {
    pub entry_type: [u8; 0x4],
    pub reserved_at_4: [u8; 0x4],
    pub entry_sub_type: [u8; 0x8],
    pub byte_mask: [u8; 0x10],
    pub next_table_base_63_48: [u8; 0x10],
    pub next_lu_type: [u8; 0x8],
    pub next_table_base_39_32_size: [u8; 0x8],
    pub next_table_base_31_5_size: [u8; 0x1b],
    pub linear_hash_enable: [u8; 0x1],
    pub reserved_at_5c: [u8; 0x2],
    pub next_table_rank: [u8; 0x2],
    pub reserved_at_60: [u8; 0xa0],
    pub tag_value: [u8; 0x60],
    pub bit_mask: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_sx_transmit_bits {
    pub entry_type: [u8; 0x4],
    pub reserved_at_4: [u8; 0x4],
    pub entry_sub_type: [u8; 0x8],
    pub byte_mask: [u8; 0x10],
    pub next_table_base_63_48: [u8; 0x10],
    pub next_lu_type: [u8; 0x8],
    pub next_table_base_39_32_size: [u8; 0x8],
    pub next_table_base_31_5_size: [u8; 0x1b],
    pub linear_hash_enable: [u8; 0x1],
    pub reserved_at_5c: [u8; 0x2],
    pub next_table_rank: [u8; 0x2],
    pub sx_wire: [u8; 0x1],
    pub sx_func_lb: [u8; 0x1],
    pub sx_sniffer: [u8; 0x1],
    pub sx_wire_enable: [u8; 0x1],
    pub sx_func_lb_enable: [u8; 0x1],
    pub sx_sniffer_enable: [u8; 0x1],
    pub action_type: [u8; 0x3],
    pub reserved_at_69: [u8; 0x1],
    pub action_description: [u8; 0x6],
    pub gvmi: [u8; 0x10],
    pub encap_pointer_vlan_data: [u8; 0x20],
    pub loopback_syndome_en: [u8; 0x8],
    pub loopback_syndome: [u8; 0x8],
    pub counter_trigger: [u8; 0x10],
    pub miss_address_63_48: [u8; 0x10],
    pub counter_trigger_23_16: [u8; 0x8],
    pub miss_address_39_32: [u8; 0x8],
    pub miss_address_31_6: [u8; 0x1a],
    pub learning_point: [u8; 0x1],
    pub go_back: [u8; 0x1],
    pub match_polarity: [u8; 0x1],
    pub mask_mode: [u8; 0x1],
    pub miss_rank: [u8; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_rx_steering_mult_bits {
    pub entry_type: [u8; 0x4],
    pub reserved_at_4: [u8; 0x4],
    pub entry_sub_type: [u8; 0x8],
    pub byte_mask: [u8; 0x10],
    pub next_table_base_63_48: [u8; 0x10],
    pub next_lu_type: [u8; 0x8],
    pub next_table_base_39_32_size: [u8; 0x8],
    pub next_table_base_31_5_size: [u8; 0x1b],
    pub linear_hash_enable: [u8; 0x1],
    pub reserved_at_: [u8; 0x2],
    pub next_table_rank: [u8; 0x2],
    pub member_count: [u8; 0x10],
    pub gvmi: [u8; 0x10],
    pub qp_list_pointer: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x1],
    pub tunneling_action: [u8; 0x3],
    pub action_description: [u8; 0x4],
    pub reserved_at_a8: [u8; 0x8],
    pub counter_trigger_15_0: [u8; 0x10],
    pub miss_address_63_48: [u8; 0x10],
    pub counter_trigger_23_16: [u8; 0x08],
    pub miss_address_39_32: [u8; 0x8],
    pub miss_address_31_6: [u8; 0x1a],
    pub learning_point: [u8; 0x1],
    pub fail_on_error: [u8; 0x1],
    pub match_polarity: [u8; 0x1],
    pub mask_mode: [u8; 0x1],
    pub miss_rank: [u8; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_modify_packet_bits {
    pub entry_type: [u8; 0x4],
    pub reserved_at_4: [u8; 0x4],
    pub entry_sub_type: [u8; 0x8],
    pub byte_mask: [u8; 0x10],
    pub next_table_base_63_48: [u8; 0x10],
    pub next_lu_type: [u8; 0x8],
    pub next_table_base_39_32_size: [u8; 0x8],
    pub next_table_base_31_5_size: [u8; 0x1b],
    pub linear_hash_enable: [u8; 0x1],
    pub reserved_at_: [u8; 0x2],
    pub next_table_rank: [u8; 0x2],
    pub number_of_re_write_actions: [u8; 0x10],
    pub gvmi: [u8; 0x10],
    pub header_re_write_actions_pointer: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x1],
    pub tunneling_action: [u8; 0x3],
    pub action_description: [u8; 0x4],
    pub reserved_at_a8: [u8; 0x8],
    pub counter_trigger_15_0: [u8; 0x10],
    pub miss_address_63_48: [u8; 0x10],
    pub counter_trigger_23_16: [u8; 0x08],
    pub miss_address_39_32: [u8; 0x8],
    pub miss_address_31_6: [u8; 0x1a],
    pub learning_point: [u8; 0x1],
    pub fail_on_error: [u8; 0x1],
    pub match_polarity: [u8; 0x1],
    pub mask_mode: [u8; 0x1],
    pub miss_rank: [u8; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l2_src_bits {
    pub smac_47_16: [u8; 0x20],
    pub smac_15_0: [u8; 0x10],
    pub l3_ethertype: [u8; 0x10],
    pub qp_type: [u8; 0x2],
    pub ethertype_filter: [u8; 0x1],
    pub reserved_at_43: [u8; 0x1],
    pub sx_sniffer: [u8; 0x1],
    pub force_lb: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub port: [u8; 0x1],
    pub reserved_at_48: [u8; 0x4],
    pub first_priority: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub first_vlan_qualifier: [u8; 0x2],
    pub reserved_at_52: [u8; 0x2],
    pub first_vlan_id: [u8; 0xc],
    pub ip_fragmented: [u8; 0x1],
    pub tcp_syn: [u8; 0x1],
    pub encp_type: [u8; 0x2],
    pub l3_type: [u8; 0x2],
    pub l4_type: [u8; 0x2],
    pub reserved_at_68: [u8; 0x4],
    pub second_priority: [u8; 0x3],
    pub second_cfi: [u8; 0x1],
    pub second_vlan_qualifier: [u8; 0x2],
    pub reserved_at_72: [u8; 0x2],
    pub second_vlan_id: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l2_dst_bits {
    pub dmac_47_16: [u8; 0x20],
    pub dmac_15_0: [u8; 0x10],
    pub l3_ethertype: [u8; 0x10],
    pub qp_type: [u8; 0x2],
    pub ethertype_filter: [u8; 0x1],
    pub reserved_at_43: [u8; 0x1],
    pub sx_sniffer: [u8; 0x1],
    pub force_lb: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub port: [u8; 0x1],
    pub reserved_at_48: [u8; 0x4],
    pub first_priority: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub first_vlan_qualifier: [u8; 0x2],
    pub reserved_at_52: [u8; 0x2],
    pub first_vlan_id: [u8; 0xc],
    pub ip_fragmented: [u8; 0x1],
    pub tcp_syn: [u8; 0x1],
    pub encp_type: [u8; 0x2],
    pub l3_type: [u8; 0x2],
    pub l4_type: [u8; 0x2],
    pub reserved_at_68: [u8; 0x4],
    pub second_priority: [u8; 0x3],
    pub second_cfi: [u8; 0x1],
    pub second_vlan_qualifier: [u8; 0x2],
    pub reserved_at_72: [u8; 0x2],
    pub second_vlan_id: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l2_src_dst_bits {
    pub dmac_47_16: [u8; 0x20],
    pub dmac_15_0: [u8; 0x10],
    pub smac_47_32: [u8; 0x10],
    pub smac_31_0: [u8; 0x20],
    pub sx_sniffer: [u8; 0x1],
    pub force_lb: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub port: [u8; 0x1],
    pub l3_type: [u8; 0x2],
    pub reserved_at_66: [u8; 0x6],
    pub first_priority: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub first_vlan_qualifier: [u8; 0x2],
    pub reserved_at_72: [u8; 0x2],
    pub first_vlan_id: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l3_ipv4_5_tuple_bits {
    pub destination_address: [u8; 0x20],
    pub source_address: [u8; 0x20],
    pub source_port: [u8; 0x10],
    pub destination_port: [u8; 0x10],
    pub fragmented: [u8; 0x1],
    pub first_fragment: [u8; 0x1],
    pub reserved_at_62: [u8; 0x2],
    pub reserved_at_64: [u8; 0x1],
    pub ecn: [u8; 0x2],
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
    pub reserved_at_76: [u8; 0x2],
    pub protocol: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l3_ipv6_dst_bits {
    pub dst_ip_127_96: [u8; 0x20],
    pub dst_ip_95_64: [u8; 0x20],
    pub dst_ip_63_32: [u8; 0x20],
    pub dst_ip_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l2_tnl_bits {
    pub dmac_47_16: [u8; 0x20],
    pub dmac_15_0: [u8; 0x10],
    pub l3_ethertype: [u8; 0x10],
    pub l2_tunneling_network_id: [u8; 0x20],
    pub ip_fragmented: [u8; 0x1],
    pub tcp_syn: [u8; 0x1],
    pub encp_type: [u8; 0x2],
    pub l3_type: [u8; 0x2],
    pub l4_type: [u8; 0x2],
    pub first_priority: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub reserved_at_6c: [u8; 0x3],
    pub gre_key_flag: [u8; 0x1],
    pub first_vlan_qualifier: [u8; 0x2],
    pub reserved_at_72: [u8; 0x2],
    pub first_vlan_id: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l3_ipv6_src_bits {
    pub src_ip_127_96: [u8; 0x20],
    pub src_ip_95_64: [u8; 0x20],
    pub src_ip_63_32: [u8; 0x20],
    pub src_ip_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l3_ipv4_misc_bits {
    pub version: [u8; 0x4],
    pub ihl: [u8; 0x4],
    pub reserved_at_8: [u8; 0x8],
    pub total_length: [u8; 0x10],
    pub identification: [u8; 0x10],
    pub flags: [u8; 0x3],
    pub fragment_offset: [u8; 0xd],
    pub time_to_live: [u8; 0x8],
    pub reserved_at_48: [u8; 0x8],
    pub checksum: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l4_bits {
    pub fragmented: [u8; 0x1],
    pub first_fragment: [u8; 0x1],
    pub reserved_at_2: [u8; 0x6],
    pub protocol: [u8; 0x8],
    pub dst_port: [u8; 0x10],
    pub ipv6_version: [u8; 0x4],
    pub reserved_at_24: [u8; 0x1],
    pub ecn: [u8; 0x2],
    pub tcp_ns: [u8; 0x1],
    pub tcp_cwr: [u8; 0x1],
    pub tcp_ece: [u8; 0x1],
    pub tcp_urg: [u8; 0x1],
    pub tcp_ack: [u8; 0x1],
    pub tcp_psh: [u8; 0x1],
    pub tcp_rst: [u8; 0x1],
    pub tcp_syn: [u8; 0x1],
    pub tcp_fin: [u8; 0x1],
    pub src_port: [u8; 0x10],
    pub ipv6_payload_length: [u8; 0x10],
    pub ipv6_hop_limit: [u8; 0x8],
    pub dscp: [u8; 0x6],
    pub reserved_at_5e: [u8; 0x2],
    pub tcp_data_offset: [u8; 0x4],
    pub reserved_at_64: [u8; 0x8],
    pub flow_label: [u8; 0x14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_eth_l4_misc_bits {
    pub checksum: [u8; 0x10],
    pub length: [u8; 0x10],
    pub seq_num: [u8; 0x20],
    pub ack_num: [u8; 0x20],
    pub urgent_pointer: [u8; 0x10],
    pub window_size: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_mpls_bits {
    pub mpls0_label: [u8; 0x14],
    pub mpls0_exp: [u8; 0x3],
    pub mpls0_s_bos: [u8; 0x1],
    pub mpls0_ttl: [u8; 0x8],
    pub mpls1_label: [u8; 0x20],
    pub mpls2_label: [u8; 0x20],
    pub reserved_at_60: [u8; 0x16],
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
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_register_0_bits {
    pub register_0_h: [u8; 0x20],
    pub register_0_l: [u8; 0x20],
    pub register_1_h: [u8; 0x20],
    pub register_1_l: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_register_1_bits {
    pub register_2_h: [u8; 0x20],
    pub register_2_l: [u8; 0x20],
    pub register_3_h: [u8; 0x20],
    pub register_3_l: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_gre_bits {
    pub gre_c_present: [u8; 0x1],
    pub reserved_at_30: [u8; 0x1],
    pub gre_k_present: [u8; 0x1],
    pub gre_s_present: [u8; 0x1],
    pub strict_src_route: [u8; 0x1],
    pub recur: [u8; 0x3],
    pub flags: [u8; 0x5],
    pub version: [u8; 0x3],
    pub gre_protocol: [u8; 0x10],
    pub checksum: [u8; 0x10],
    pub offset: [u8; 0x10],
    pub gre_key_h: [u8; 0x18],
    pub gre_key_l: [u8; 0x8],
    pub seq_num: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_flex_parser_0_bits {
    pub flex_parser_3: [u8; 0x20],
    pub flex_parser_2: [u8; 0x20],
    pub flex_parser_1: [u8; 0x20],
    pub flex_parser_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_flex_parser_1_bits {
    pub flex_parser_7: [u8; 0x20],
    pub flex_parser_6: [u8; 0x20],
    pub flex_parser_5: [u8; 0x20],
    pub flex_parser_4: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_flex_parser_ok_bits {
    pub flex_parser_3: [u8; 0x20],
    pub flex_parser_2: [u8; 0x20],
    pub flex_parsers_ok: [u8; 0x8],
    pub reserved_at_48: [u8; 0x18],
    pub flex_parser_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_flex_parser_tnl_bits {
    pub flex_parser_tunneling_header_63_32: [u8; 0x20],
    pub flex_parser_tunneling_header_31_0: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_flex_parser_tnl_vxlan_gpe_bits {
    pub outer_vxlan_gpe_flags: [u8; 0x8],
    pub reserved_at_8: [u8; 0x10],
    pub outer_vxlan_gpe_next_protocol: [u8; 0x8],
    pub outer_vxlan_gpe_vni: [u8; 0x18],
    pub reserved_at_38: [u8; 0x8],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_flex_parser_tnl_geneve_bits {
    pub reserved_at_0: [u8; 0x2],
    pub geneve_opt_len: [u8; 0x6],
    pub geneve_oam: [u8; 0x1],
    pub reserved_at_9: [u8; 0x7],
    pub geneve_protocol_type: [u8; 0x10],
    pub geneve_vni: [u8; 0x18],
    pub reserved_at_38: [u8; 0x8],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_flex_parser_tnl_gtpu_bits {
    pub reserved_at_0: [u8; 0x5],
    pub gtpu_msg_flags: [u8; 0x3],
    pub gtpu_msg_type: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub gtpu_teid: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_tunnel_header_bits {
    pub tunnel_header_0: [u8; 0x20],
    pub tunnel_header_1: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_general_purpose_bits {
    pub general_purpose_lookup_field: [u8; 0x20],
    pub reserved_at_20: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_src_gvmi_qp_bits {
    pub loopback_syndrome: [u8; 0x8],
    pub reserved_at_8: [u8; 0x8],
    pub source_gvmi: [u8; 0x10],
    pub reserved_at_20: [u8; 0x5],
    pub force_lb: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub source_is_requestor: [u8; 0x1],
    pub source_qp: [u8; 0x18],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_l2_hdr_bits {
    pub dmac_47_16: [u8; 0x20],
    pub dmac_15_0: [u8; 0x10],
    pub smac_47_32: [u8; 0x10],
    pub smac_31_0: [u8; 0x20],
    pub ethertype: [u8; 0x10],
    pub vlan_type: [u8; 0x10],
    pub vlan: [u8; 0x10],
    pub reserved_at_90: [u8; 0x10],
}

// Both HW set and HW add share the same HW format with different opcodes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dr_action_hw_set_bits {
    pub opcode: [u8; 0x8],
    pub destination_field_code: [u8; 0x8],
    pub reserved_at_10: [u8; 0x2],
    pub destination_left_shifter: [u8; 0x6],
    pub reserved_at_18: [u8; 0x3],
    pub destination_length: [u8; 0x5],
    pub inline_data: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dr_action_hw_copy_bits {
    pub opcode: [u8; 0x8],
    pub destination_field_code: [u8; 0x8],
    pub reserved_at_10: [u8; 0x2],
    pub destination_left_shifter: [u8; 0x6],
    pub reserved_at_18: [u8; 0x2],
    pub destination_length: [u8; 0x6],
    pub reserved_at_20: [u8; 0x8],
    pub source_field_code: [u8; 0x8],
    pub reserved_at_30: [u8; 0x2],
    pub source_left_shifter: [u8; 0x6],
    pub reserved_at_38: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_aso_flow_meter_action_bits {
    pub reserved_at_0: [u8; 0xc],
    pub action: [u8; 0x1],
    pub initial_color: [u8; 0x2],
    pub line_id: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_double_action_aso_v1_bits {
    pub action_id: [u8; 0x8],
    pub aso_context_number: [u8; 0x18],
    pub dest_reg_id: [u8; 0x2],
    pub change_ordering_tag: [u8; 0x1],
    pub aso_check_ordering: [u8; 0x1],
    pub aso_context_type: [u8; 0x4],
    pub reserved_at_28: [u8; 0x8],
    pub aso_fields: [u8; 0x10],
    pub flow_meter: mlx5_ifc_ste_aso_flow_meter_action_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_single_action_remove_header_v3_bits {
    pub action_id: [u8; 0x8],
    pub start_anchor: [u8; 0x7],
    pub end_anchor: [u8; 0x7],
    pub reserved_at_16: [u8; 0x1],
    pub outer_l4_remove: [u8; 0x1],
    pub reserved_at_18: [u8; 0x4],
    pub decap: [u8; 0x1],
    pub vni_to_cqe: [u8; 0x1],
    pub qos_profile: [u8; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_single_action_remove_header_size_v3_bits {
    pub action_id: [u8; 0x8],
    pub start_anchor: [u8; 0x7],
    pub start_offset: [u8; 0x8],
    pub outer_l4_remove: [u8; 0x1],
    pub reserved_at_18: [u8; 0x2],
    pub remove_size: [u8; 0x6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_double_action_insert_with_inline_v3_bits {
    pub action_id: [u8; 0x8],
    pub start_anchor: [u8; 0x7],
    pub start_offset: [u8; 0x8],
    pub reserved_at_17: [u8; 0x9],
    pub inline_data: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ste_double_action_insert_with_ptr_v3_bits {
    pub action_id: [u8; 0x8],
    pub start_anchor: [u8; 0x7],
    pub start_offset: [u8; 0x8],
    pub size: [u8; 0x6],
    pub attributes: [u8; 0x3],
    pub pointer: [u8; 0x20],
}
