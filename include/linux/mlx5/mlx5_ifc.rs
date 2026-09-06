//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/mlx5_ifc.h
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


//
// Copyright (c) 2013-2015, Mellanox Technologies, Ltd.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// Valid range for general commands that don't work over an object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_table_fields_supported_bits {
    pub outer_dmac: [u8; 0x1],
    pub outer_smac: [u8; 0x1],
    pub outer_ether_type: [u8; 0x1],
    pub outer_ip_version: [u8; 0x1],
    pub outer_first_prio: [u8; 0x1],
    pub outer_first_cfi: [u8; 0x1],
    pub outer_first_vid: [u8; 0x1],
    pub outer_ipv4_ttl: [u8; 0x1],
    pub outer_second_prio: [u8; 0x1],
    pub outer_second_cfi: [u8; 0x1],
    pub outer_second_vid: [u8; 0x1],
    pub reserved_at_b: [u8; 0x1],
    pub outer_sip: [u8; 0x1],
    pub outer_dip: [u8; 0x1],
    pub outer_frag: [u8; 0x1],
    pub outer_ip_protocol: [u8; 0x1],
    pub outer_ip_ecn: [u8; 0x1],
    pub outer_ip_dscp: [u8; 0x1],
    pub outer_udp_sport: [u8; 0x1],
    pub outer_udp_dport: [u8; 0x1],
    pub outer_tcp_sport: [u8; 0x1],
    pub outer_tcp_dport: [u8; 0x1],
    pub outer_tcp_flags: [u8; 0x1],
    pub outer_gre_protocol: [u8; 0x1],
    pub outer_gre_key: [u8; 0x1],
    pub outer_vxlan_vni: [u8; 0x1],
    pub outer_geneve_vni: [u8; 0x1],
    pub outer_geneve_oam: [u8; 0x1],
    pub outer_geneve_protocol_type: [u8; 0x1],
    pub outer_geneve_opt_len: [u8; 0x1],
    pub source_vhca_port: [u8; 0x1],
    pub source_eswitch_port: [u8; 0x1],
    pub inner_dmac: [u8; 0x1],
    pub inner_smac: [u8; 0x1],
    pub inner_ether_type: [u8; 0x1],
    pub inner_ip_version: [u8; 0x1],
    pub inner_first_prio: [u8; 0x1],
    pub inner_first_cfi: [u8; 0x1],
    pub inner_first_vid: [u8; 0x1],
    pub reserved_at_27: [u8; 0x1],
    pub inner_second_prio: [u8; 0x1],
    pub inner_second_cfi: [u8; 0x1],
    pub inner_second_vid: [u8; 0x1],
    pub reserved_at_2b: [u8; 0x1],
    pub inner_sip: [u8; 0x1],
    pub inner_dip: [u8; 0x1],
    pub inner_frag: [u8; 0x1],
    pub inner_ip_protocol: [u8; 0x1],
    pub inner_ip_ecn: [u8; 0x1],
    pub inner_ip_dscp: [u8; 0x1],
    pub inner_udp_sport: [u8; 0x1],
    pub inner_udp_dport: [u8; 0x1],
    pub inner_tcp_sport: [u8; 0x1],
    pub inner_tcp_dport: [u8; 0x1],
    pub inner_tcp_flags: [u8; 0x1],
    pub reserved_at_37: [u8; 0x9],
    pub geneve_tlv_option_0_data: [u8; 0x1],
    pub geneve_tlv_option_0_exist: [u8; 0x1],
    pub reserved_at_42: [u8; 0x3],
    pub outer_first_mpls_over_udp: [u8; 0x4],
    pub outer_first_mpls_over_gre: [u8; 0x4],
    pub inner_first_mpls: [u8; 0x4],
    pub outer_first_mpls: [u8; 0x4],
    pub reserved_at_55: [u8; 0x2],
    pub outer_esp_spi: [u8; 0x1],
    pub reserved_at_58: [u8; 0x2],
    pub bth_dst_qp: [u8; 0x1],
    pub reserved_at_5b: [u8; 0x5],
    pub reserved_at_60: [u8; 0x18],
    pub metadata_reg_c_7: [u8; 0x1],
    pub metadata_reg_c_6: [u8; 0x1],
    pub metadata_reg_c_5: [u8; 0x1],
    pub metadata_reg_c_4: [u8; 0x1],
    pub metadata_reg_c_3: [u8; 0x1],
    pub metadata_reg_c_2: [u8; 0x1],
    pub metadata_reg_c_1: [u8; 0x1],
    pub metadata_reg_c_0: [u8; 0x1],
}

// Table 2170 - Flow Table Fields Supported 2 Format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_table_fields_supported_2_bits {
    pub inner_l4_type_ext: [u8; 0x1],
    pub outer_l4_type_ext: [u8; 0x1],
    pub inner_l4_type: [u8; 0x1],
    pub outer_l4_type: [u8; 0x1],
    pub reserved_at_4: [u8; 0xa],
    pub bth_opcode: [u8; 0x1],
    pub reserved_at_f: [u8; 0x1],
    pub tunnel_header_0_1: [u8; 0x1],
    pub reserved_at_11: [u8; 0xf],
    pub reserved_at_20: [u8; 0xf],
    pub ipsec_next_header: [u8; 0x1],
    pub reserved_at_30: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_table_prop_layout_bits {
    pub ft_support: [u8; 0x1],
    pub reserved_at_1: [u8; 0x1],
    pub flow_counter: [u8; 0x1],
    pub flow_modify_en: [u8; 0x1],
    pub modify_root: [u8; 0x1],
    pub identified_miss_table_mode: [u8; 0x1],
    pub flow_table_modify: [u8; 0x1],
    pub reformat: [u8; 0x1],
    pub decap: [u8; 0x1],
    pub reset_root_to_default: [u8; 0x1],
    pub pop_vlan: [u8; 0x1],
    pub push_vlan: [u8; 0x1],
    pub reserved_at_c: [u8; 0x1],
    pub pop_vlan_2: [u8; 0x1],
    pub push_vlan_2: [u8; 0x1],
    pub reformat_and_vlan_action: [u8; 0x1],
    pub reserved_at_10: [u8; 0x1],
    pub sw_owner: [u8; 0x1],
    pub reformat_l3_tunnel_to_l2: [u8; 0x1],
    pub reformat_l2_to_l3_tunnel: [u8; 0x1],
    pub reformat_and_modify_action: [u8; 0x1],
    pub ignore_flow_level: [u8; 0x1],
    pub reserved_at_16: [u8; 0x1],
    pub table_miss_action_domain: [u8; 0x1],
    pub termination_table: [u8; 0x1],
    pub reformat_and_fwd_to_table: [u8; 0x1],
    pub forward_vhca_rx: [u8; 0x1],
    pub reserved_at_1b: [u8; 0x1],
    pub ipsec_encrypt: [u8; 0x1],
    pub ipsec_decrypt: [u8; 0x1],
    pub sw_owner_v2: [u8; 0x1],
    pub reserved_at_1f: [u8; 0x1],
    pub termination_table_raw_traffic: [u8; 0x1],
    pub reserved_at_21: [u8; 0x1],
    pub log_max_ft_size: [u8; 0x6],
    pub log_max_modify_header_context: [u8; 0x8],
    pub max_modify_header_actions: [u8; 0x8],
    pub max_ft_level: [u8; 0x8],
    pub reformat_add_esp_trasport: [u8; 0x1],
    pub reformat_l2_to_l3_esp_tunnel: [u8; 0x1],
    pub reformat_add_esp_transport_over_udp: [u8; 0x1],
    pub reformat_del_esp_trasport: [u8; 0x1],
    pub reformat_l3_esp_tunnel_to_l2: [u8; 0x1],
    pub reformat_del_esp_transport_over_udp: [u8; 0x1],
    pub execute_aso: [u8; 0x1],
    pub reserved_at_47: [u8; 0x19],
    pub reformat_l2_to_l3_psp_tunnel: [u8; 0x1],
    pub reformat_l3_psp_tunnel_to_l2: [u8; 0x1],
    pub reformat_insert: [u8; 0x1],
    pub reformat_remove: [u8; 0x1],
    pub macsec_encrypt: [u8; 0x1],
    pub macsec_decrypt: [u8; 0x1],
    pub psp_encrypt: [u8; 0x1],
    pub psp_decrypt: [u8; 0x1],
    pub reformat_add_macsec: [u8; 0x1],
    pub reformat_remove_macsec: [u8; 0x1],
    pub reparse: [u8; 0x1],
    pub reserved_at_6b: [u8; 0x1],
    pub cross_vhca_object: [u8; 0x1],
    pub reformat_l2_to_l3_audp_tunnel: [u8; 0x1],
    pub reformat_l3_audp_tunnel_to_l2: [u8; 0x1],
    pub ignore_flow_level_rtc_valid: [u8; 0x1],
    pub reserved_at_70: [u8; 0x7],
    pub reformat_del_psp_transport: [u8; 0x1],
    pub log_max_ft_num: [u8; 0x8],
    pub reserved_at_80: [u8; 0x10],
    pub log_max_flow_counter: [u8; 0x8],
    pub log_max_destination: [u8; 0x8],
    pub reserved_at_a0: [u8; 0x18],
    pub log_max_flow: [u8; 0x8],
    pub reserved_at_c0: [u8; 0x40],
    pub ft_field_support: mlx5_ifc_flow_table_fields_supported_bits,
    pub ft_field_bitmask_support: mlx5_ifc_flow_table_fields_supported_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_odp_per_transport_service_cap_bits {
    pub send: [u8; 0x1],
    pub receive: [u8; 0x1],
    pub write: [u8; 0x1],
    pub read: [u8; 0x1],
    pub atomic: [u8; 0x1],
    pub srq_receive: [u8; 0x1],
    pub reserved_at_6: [u8; 0x1a],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ipv4_layout_bits {
    pub reserved_at_0: [u8; 0x60],
    pub ipv4: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ipv6_layout_bits {
    pub ipv6: [u8; 16][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ipv6_simple_layout_bits {
    pub ipv6_127_96: [u8; 0x20],
    pub ipv6_95_64: [u8; 0x20],
    pub ipv6_63_32: [u8; 0x20],
    pub ipv6_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_ipv6_layout_ipv4_layout_auto_bits {
    pub ipv6_simple_layout: mlx5_ifc_ipv6_simple_layout_bits,
    pub ipv6_layout: mlx5_ifc_ipv6_layout_bits,
    pub ipv4_layout: mlx5_ifc_ipv4_layout_bits,
    pub reserved_at_0: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fte_match_set_lyr_2_4_bits {
    pub smac_47_16: [u8; 0x20],
    pub smac_15_0: [u8; 0x10],
    pub ethertype: [u8; 0x10],
    pub dmac_47_16: [u8; 0x20],
    pub dmac_15_0: [u8; 0x10],
    pub first_prio: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub first_vid: [u8; 0xc],
    pub ip_protocol: [u8; 0x8],
    pub ip_dscp: [u8; 0x6],
    pub ip_ecn: [u8; 0x2],
    pub cvlan_tag: [u8; 0x1],
    pub svlan_tag: [u8; 0x1],
    pub frag: [u8; 0x1],
    pub ip_version: [u8; 0x4],
    pub tcp_flags: [u8; 0x9],
    pub tcp_sport: [u8; 0x10],
    pub tcp_dport: [u8; 0x10],
    pub l4_type: [u8; 0x2],
    pub l4_type_ext: [u8; 0x4],
    pub reserved_at_c6: [u8; 0xa],
    pub ipv4_ihl: [u8; 0x4],
    pub reserved_at_d4: [u8; 0x4],
    pub ttl_hoplimit: [u8; 0x8],
    pub udp_sport: [u8; 0x10],
    pub udp_dport: [u8; 0x10],
    pub src_ipv4_src_ipv6: mlx5_ifc_ipv6_layout_ipv4_layout_auto_bits,
    pub dst_ipv4_dst_ipv6: mlx5_ifc_ipv6_layout_ipv4_layout_auto_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_nvgre_key_bits {
    pub hi: [u8; 0x18],
    pub lo: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_gre_key_bits {
    pub nvgre: mlx5_ifc_nvgre_key_bits,
    pub key: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fte_match_set_misc_bits {
    pub gre_c_present: [u8; 0x1],
    pub reserved_at_1: [u8; 0x1],
    pub gre_k_present: [u8; 0x1],
    pub gre_s_present: [u8; 0x1],
    pub source_vhca_port: [u8; 0x4],
    pub source_sqn: [u8; 0x18],
    pub source_eswitch_owner_vhca_id: [u8; 0x10],
    pub source_port: [u8; 0x10],
    pub outer_second_prio: [u8; 0x3],
    pub outer_second_cfi: [u8; 0x1],
    pub outer_second_vid: [u8; 0xc],
    pub inner_second_prio: [u8; 0x3],
    pub inner_second_cfi: [u8; 0x1],
    pub inner_second_vid: [u8; 0xc],
    pub outer_second_cvlan_tag: [u8; 0x1],
    pub inner_second_cvlan_tag: [u8; 0x1],
    pub outer_second_svlan_tag: [u8; 0x1],
    pub inner_second_svlan_tag: [u8; 0x1],
    pub reserved_at_64: [u8; 0xc],
    pub gre_protocol: [u8; 0x10],
    pub gre_key: mlx5_ifc_gre_key_bits,
    pub vxlan_vni: [u8; 0x18],
    pub bth_opcode: [u8; 0x8],
    pub geneve_vni: [u8; 0x18],
    pub reserved_at_d8: [u8; 0x6],
    pub geneve_tlv_option_0_exist: [u8; 0x1],
    pub geneve_oam: [u8; 0x1],
    pub reserved_at_e0: [u8; 0xc],
    pub outer_ipv6_flow_label: [u8; 0x14],
    pub reserved_at_100: [u8; 0xc],
    pub inner_ipv6_flow_label: [u8; 0x14],
    pub reserved_at_120: [u8; 0xa],
    pub geneve_opt_len: [u8; 0x6],
    pub geneve_protocol_type: [u8; 0x10],
    pub reserved_at_140: [u8; 0x8],
    pub bth_dst_qp: [u8; 0x18],
    pub inner_esp_spi: [u8; 0x20],
    pub outer_esp_spi: [u8; 0x20],
    pub reserved_at_1a0: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fte_match_mpls_bits {
    pub mpls_label: [u8; 0x14],
    pub mpls_exp: [u8; 0x3],
    pub mpls_s_bos: [u8; 0x1],
    pub mpls_ttl: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fte_match_set_misc2_bits {
    pub outer_first_mpls: mlx5_ifc_fte_match_mpls_bits,
    pub inner_first_mpls: mlx5_ifc_fte_match_mpls_bits,
    pub outer_first_mpls_over_gre: mlx5_ifc_fte_match_mpls_bits,
    pub outer_first_mpls_over_udp: mlx5_ifc_fte_match_mpls_bits,
    pub metadata_reg_c_7: [u8; 0x20],
    pub metadata_reg_c_6: [u8; 0x20],
    pub metadata_reg_c_5: [u8; 0x20],
    pub metadata_reg_c_4: [u8; 0x20],
    pub metadata_reg_c_3: [u8; 0x20],
    pub metadata_reg_c_2: [u8; 0x20],
    pub metadata_reg_c_1: [u8; 0x20],
    pub metadata_reg_c_0: [u8; 0x20],
    pub metadata_reg_a: [u8; 0x20],
    pub psp_syndrome: [u8; 0x8],
    pub macsec_syndrome: [u8; 0x8],
    pub ipsec_syndrome: [u8; 0x8],
    pub ipsec_next_header: [u8; 0x8],
    pub reserved_at_1c0: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fte_match_set_misc3_bits {
    pub inner_tcp_seq_num: [u8; 0x20],
    pub outer_tcp_seq_num: [u8; 0x20],
    pub inner_tcp_ack_num: [u8; 0x20],
    pub outer_tcp_ack_num: [u8; 0x20],
    pub reserved_at_80: [u8; 0x8],
    pub outer_vxlan_gpe_vni: [u8; 0x18],
    pub outer_vxlan_gpe_next_protocol: [u8; 0x8],
    pub outer_vxlan_gpe_flags: [u8; 0x8],
    pub reserved_at_b0: [u8; 0x10],
    pub icmp_header_data: [u8; 0x20],
    pub icmpv6_header_data: [u8; 0x20],
    pub icmp_type: [u8; 0x8],
    pub icmp_code: [u8; 0x8],
    pub icmpv6_type: [u8; 0x8],
    pub icmpv6_code: [u8; 0x8],
    pub geneve_tlv_option_0_data: [u8; 0x20],
    pub gtpu_teid: [u8; 0x20],
    pub gtpu_msg_type: [u8; 0x8],
    pub gtpu_msg_flags: [u8; 0x8],
    pub reserved_at_170: [u8; 0x10],
    pub gtpu_dw_2: [u8; 0x20],
    pub gtpu_first_ext_dw_0: [u8; 0x20],
    pub gtpu_dw_0: [u8; 0x20],
    pub reserved_at_1e0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fte_match_set_misc4_bits {
    pub prog_sample_field_value_0: [u8; 0x20],
    pub prog_sample_field_id_0: [u8; 0x20],
    pub prog_sample_field_value_1: [u8; 0x20],
    pub prog_sample_field_id_1: [u8; 0x20],
    pub prog_sample_field_value_2: [u8; 0x20],
    pub prog_sample_field_id_2: [u8; 0x20],
    pub prog_sample_field_value_3: [u8; 0x20],
    pub prog_sample_field_id_3: [u8; 0x20],
    pub reserved_at_100: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fte_match_set_misc5_bits {
    pub macsec_tag_0: [u8; 0x20],
    pub macsec_tag_1: [u8; 0x20],
    pub macsec_tag_2: [u8; 0x20],
    pub macsec_tag_3: [u8; 0x20],
    pub tunnel_header_0: [u8; 0x20],
    pub tunnel_header_1: [u8; 0x20],
    pub tunnel_header_2: [u8; 0x20],
    pub tunnel_header_3: [u8; 0x20],
    pub reserved_at_100: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fte_match_set_misc6_bits {
    pub reserved_at_0: [u8; 0x1a],
    pub psp_version: [u8; 0x4],
    pub reserved_at_1e: [u8; 0x2],
    pub reserved_at_20: [u8; 0x1e0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cmd_pas_bits {
    pub pa_h: [u8; 0x20],
    pub pa_l: [u8; 0x14],
    pub reserved_at_34: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_uint64_bits {
    pub hi: [u8; 0x20],
    pub lo: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ads_bits {
    pub fl: [u8; 0x1],
    pub free_ar: [u8; 0x1],
    pub reserved_at_2: [u8; 0xe],
    pub pkey_index: [u8; 0x10],
    pub plane_index: [u8; 0x8],
    pub grh: [u8; 0x1],
    pub mlid: [u8; 0x7],
    pub rlid: [u8; 0x10],
    pub ack_timeout: [u8; 0x5],
    pub reserved_at_45: [u8; 0x3],
    pub src_addr_index: [u8; 0x8],
    pub reserved_at_50: [u8; 0x4],
    pub stat_rate: [u8; 0x4],
    pub hop_limit: [u8; 0x8],
    pub reserved_at_60: [u8; 0x4],
    pub tclass: [u8; 0x8],
    pub flow_label: [u8; 0x14],
    pub rgid_rip: [u8; 16][0x8],
    pub reserved_at_100: [u8; 0x4],
    pub f_dscp: [u8; 0x1],
    pub f_ecn: [u8; 0x1],
    pub reserved_at_106: [u8; 0x1],
    pub f_eth_prio: [u8; 0x1],
    pub ecn: [u8; 0x2],
    pub dscp: [u8; 0x6],
    pub udp_sport: [u8; 0x10],
    pub dei_cfi: [u8; 0x1],
    pub eth_prio: [u8; 0x3],
    pub sl: [u8; 0x4],
    pub vhca_port_num: [u8; 0x8],
    pub rmac_47_32: [u8; 0x10],
    pub rmac_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_table_nic_cap_bits {
    pub nic_rx_multi_path_tirs: [u8; 0x1],
    pub nic_rx_multi_path_tirs_fts: [u8; 0x1],
    pub allow_sniffer_and_nic_rx_shared_tir: [u8; 0x1],
    pub reserved_at_3: [u8; 0x4],
    pub sw_owner_reformat_supported: [u8; 0x1],
    pub reserved_at_8: [u8; 0x18],
    pub encap_general_header: [u8; 0x1],
    pub reserved_at_21: [u8; 0xa],
    pub log_max_packet_reformat_context: [u8; 0x5],
    pub reserved_at_30: [u8; 0x6],
    pub max_encap_header_size: [u8; 0xa],
    pub reserved_at_40: [u8; 0x1c0],
    pub flow_table_properties_nic_receive: mlx5_ifc_flow_table_prop_layout_bits,
    pub flow_table_properties_nic_receive_rdma: mlx5_ifc_flow_table_prop_layout_bits,
    pub flow_table_properties_nic_receive_sniffer: mlx5_ifc_flow_table_prop_layout_bits,
    pub flow_table_properties_nic_transmit: mlx5_ifc_flow_table_prop_layout_bits,
    pub flow_table_properties_nic_transmit_rdma: mlx5_ifc_flow_table_prop_layout_bits,
    pub flow_table_properties_nic_transmit_sniffer: mlx5_ifc_flow_table_prop_layout_bits,
    pub reserved_at_e00: [u8; 0x600],
    pub ft_field_support_2_nic_receive: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub reserved_at_1480: [u8; 0x80],
    pub ft_field_support_2_nic_receive_rdma: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub reserved_at_1580: [u8; 0x280],
    pub ft_field_support_2_nic_transmit_rdma: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub reserved_at_1880: [u8; 0x780],
    pub sw_steering_nic_rx_action_drop_icm_address: [u8; 0x40],
    pub sw_steering_nic_tx_action_drop_icm_address: [u8; 0x40],
    pub sw_steering_nic_tx_action_allow_icm_address: [u8; 0x40],
    pub reserved_at_20c0: [u8; 0x5f40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_port_selection_cap_bits {
    pub reserved_at_0: [u8; 0x10],
    pub port_select_flow_table: [u8; 0x1],
    pub reserved_at_11: [u8; 0x1],
    pub port_select_flow_table_bypass: [u8; 0x1],
    pub reserved_at_13: [u8; 0xd],
    pub reserved_at_20: [u8; 0x1e0],
    pub flow_table_properties_port_selection: mlx5_ifc_flow_table_prop_layout_bits,
    pub ft_field_support_2_port_selection: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub reserved_at_480: [u8; 0x7b80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_table_eswitch_cap_bits {
    pub fdb_to_vport_reg_c_id: [u8; 0x8],
    pub reserved_at_8: [u8; 0x5],
    pub fdb_uplink_hairpin: [u8; 0x1],
    pub fdb_multi_path_any_table_limit_regc: [u8; 0x1],
    pub reserved_at_f: [u8; 0x1],
    pub fdb_dynamic_tunnel: [u8; 0x1],
    pub reserved_at_11: [u8; 0x1],
    pub fdb_multi_path_any_table: [u8; 0x1],
    pub reserved_at_13: [u8; 0x2],
    pub fdb_modify_header_fwd_to_table: [u8; 0x1],
    pub fdb_ipv4_ttl_modify: [u8; 0x1],
    pub flow_source: [u8; 0x1],
    pub reserved_at_18: [u8; 0x2],
    pub multi_fdb_encap: [u8; 0x1],
    pub egress_acl_forward_to_vport: [u8; 0x1],
    pub fdb_multi_path_to_table: [u8; 0x1],
    pub reserved_at_1d: [u8; 0x3],
    pub reserved_at_20: [u8; 0x1e0],
    pub flow_table_properties_nic_esw_fdb: mlx5_ifc_flow_table_prop_layout_bits,
    pub flow_table_properties_esw_acl_ingress: mlx5_ifc_flow_table_prop_layout_bits,
    pub flow_table_properties_esw_acl_egress: mlx5_ifc_flow_table_prop_layout_bits,
    pub reserved_at_800: [u8; 0xC00],
    pub ft_field_support_2_esw_fdb: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub ft_field_bitmask_support_2_esw_fdb: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub reserved_at_1500: [u8; 0x300],
    pub sw_steering_fdb_action_drop_icm_address_rx: [u8; 0x40],
    pub sw_steering_fdb_action_drop_icm_address_tx: [u8; 0x40],
    pub sw_steering_uplink_icm_address_rx: [u8; 0x40],
    pub sw_steering_uplink_icm_address_tx: [u8; 0x40],
    pub reserved_at_1900: [u8; 0x6700],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_wqe_based_flow_table_cap_bits {
    pub reserved_at_0: [u8; 0x3],
    pub log_max_num_ste: [u8; 0x5],
    pub reserved_at_8: [u8; 0x3],
    pub log_max_num_stc: [u8; 0x5],
    pub reserved_at_10: [u8; 0x3],
    pub log_max_num_rtc: [u8; 0x5],
    pub reserved_at_18: [u8; 0x3],
    pub log_max_num_header_modify_pattern: [u8; 0x5],
    pub rtc_hash_split_table: [u8; 0x1],
    pub rtc_linear_lookup_table: [u8; 0x1],
    pub reserved_at_22: [u8; 0x1],
    pub stc_alloc_log_granularity: [u8; 0x5],
    pub reserved_at_28: [u8; 0x3],
    pub stc_alloc_log_max: [u8; 0x5],
    pub reserved_at_30: [u8; 0x3],
    pub ste_alloc_log_granularity: [u8; 0x5],
    pub reserved_at_38: [u8; 0x3],
    pub ste_alloc_log_max: [u8; 0x5],
    pub reserved_at_40: [u8; 0xb],
    pub rtc_reparse_mode: [u8; 0x5],
    pub reserved_at_50: [u8; 0x3],
    pub rtc_index_mode: [u8; 0x5],
    pub reserved_at_58: [u8; 0x3],
    pub rtc_log_depth_max: [u8; 0x5],
    pub reserved_at_60: [u8; 0x10],
    pub ste_format: [u8; 0x10],
    pub stc_action_type: [u8; 0x80],
    pub header_insert_type: [u8; 0x10],
    pub header_remove_type: [u8; 0x10],
    pub trivial_match_definer: [u8; 0x20],
    pub reserved_at_140: [u8; 0x1b],
    pub rtc_max_num_hash_definer_gen_wqe: [u8; 0x5],
    pub reserved_at_160: [u8; 0x18],
    pub access_index_mode: [u8; 0x8],
    pub reserved_at_180: [u8; 0x10],
    pub ste_format_gen_wqe: [u8; 0x10],
    pub linear_match_definer_reg_c3: [u8; 0x20],
    pub fdb_jump_to_tir_stc: [u8; 0x1],
    pub reserved_at_1c1: [u8; 0x1f],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_e_switch_cap_bits {
    pub vport_svlan_strip: [u8; 0x1],
    pub vport_cvlan_strip: [u8; 0x1],
    pub vport_svlan_insert: [u8; 0x1],
    pub vport_cvlan_insert_if_not_exist: [u8; 0x1],
    pub vport_cvlan_insert_overwrite: [u8; 0x1],
    pub reserved_at_5: [u8; 0x1],
    pub vport_cvlan_insert_always: [u8; 0x1],
    pub esw_shared_ingress_acl: [u8; 0x1],
    pub esw_uplink_ingress_acl: [u8; 0x1],
    pub root_ft_on_other_esw: [u8; 0x1],
    pub reserved_at_a: [u8; 0x1],
    pub esw_vport_state_max_tx_speed: [u8; 0x1],
    pub reserved_at_c: [u8; 0xd],
    pub esw_functions_changed: [u8; 0x1],
    pub reserved_at_1a: [u8; 0x1],
    pub ecpf_vport_exists: [u8; 0x1],
    pub counter_eswitch_affinity: [u8; 0x1],
    pub merged_eswitch: [u8; 0x1],
    pub nic_vport_node_guid_modify: [u8; 0x1],
    pub nic_vport_port_guid_modify: [u8; 0x1],
    pub vxlan_encap_decap: [u8; 0x1],
    pub nvgre_encap_decap: [u8; 0x1],
    pub reserved_at_22: [u8; 0x1],
    pub log_max_fdb_encap_uplink: [u8; 0x5],
    pub reserved_at_21: [u8; 0x3],
    pub log_max_packet_reformat_context: [u8; 0x5],
    pub reserved_2b: [u8; 0x6],
    pub max_encap_header_size: [u8; 0xa],
    pub reserved_at_40: [u8; 0xb],
    pub log_max_esw_sf: [u8; 0x5],
    pub esw_sf_base_id: [u8; 0x10],
    pub esw_manager_vport_number_valid: [u8; 0x1],
    pub reserved_at_61: [u8; 0xf],
    pub esw_manager_vport_number: [u8; 0x10],
    pub reserved_at_80: [u8; 0x780],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qos_cap_bits {
    pub packet_pacing: [u8; 0x1],
    pub esw_scheduling: [u8; 0x1],
    pub esw_bw_share: [u8; 0x1],
    pub esw_rate_limit: [u8; 0x1],
    pub reserved_at_4: [u8; 0x1],
    pub packet_pacing_burst_bound: [u8; 0x1],
    pub packet_pacing_typical_size: [u8; 0x1],
    pub reserved_at_7: [u8; 0x1],
    pub nic_sq_scheduling: [u8; 0x1],
    pub nic_bw_share: [u8; 0x1],
    pub nic_rate_limit: [u8; 0x1],
    pub packet_pacing_uid: [u8; 0x1],
    pub log_esw_max_sched_depth: [u8; 0x4],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x2],
    pub packet_pacing_req_ud: [u8; 0x1],
    pub packet_pacing_req_uc: [u8; 0x1],
    pub reserved_at_24: [u8; 0x5],
    pub esw_cross_esw_sched: [u8; 0x1],
    pub reserved_at_2a: [u8; 0x1],
    pub log_max_qos_nic_queue_group: [u8; 0x5],
    pub reserved_at_30: [u8; 0x10],
    pub packet_pacing_max_rate: [u8; 0x20],
    pub packet_pacing_min_rate: [u8; 0x20],
    pub reserved_at_80: [u8; 0xb],
    pub log_esw_max_rate_limit: [u8; 0x5],
    pub packet_pacing_rate_table_size: [u8; 0x10],
    pub esw_element_type: [u8; 0x10],
    pub esw_tsar_type: [u8; 0x10],
    pub reserved_at_c0: [u8; 0x10],
    pub max_qos_para_vport: [u8; 0x10],
    pub max_tsar_bw_share: [u8; 0x20],
    pub nic_element_type: [u8; 0x10],
    pub nic_tsar_type: [u8; 0x10],
    pub reserved_at_120: [u8; 0x3],
    pub log_meter_aso_granularity: [u8; 0x5],
    pub reserved_at_128: [u8; 0x3],
    pub log_meter_aso_max_alloc: [u8; 0x5],
    pub reserved_at_130: [u8; 0x3],
    pub log_max_num_meter_aso: [u8; 0x5],
    pub reserved_at_138: [u8; 0x8],
    pub reserved_at_140: [u8; 0x6c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_debug_cap_bits {
    pub core_dump_general: [u8; 0x1],
    pub core_dump_qp: [u8; 0x1],
    pub reserved_at_2: [u8; 0x7],
    pub resource_dump: [u8; 0x1],
    pub reserved_at_a: [u8; 0x16],
    pub reserved_at_20: [u8; 0x2],
    pub stall_detect: [u8; 0x1],
    pub reserved_at_23: [u8; 0x1d],
    pub reserved_at_40: [u8; 0x7c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_per_protocol_networking_offload_caps_bits {
    pub csum_cap: [u8; 0x1],
    pub vlan_cap: [u8; 0x1],
    pub lro_cap: [u8; 0x1],
    pub lro_psh_flag: [u8; 0x1],
    pub lro_time_stamp: [u8; 0x1],
    pub reserved_at_5: [u8; 0x2],
    pub wqe_vlan_insert: [u8; 0x1],
    pub self_lb_en_modifiable: [u8; 0x1],
    pub reserved_at_9: [u8; 0x2],
    pub max_lso_cap: [u8; 0x5],
    pub multi_pkt_send_wqe: [u8; 0x2],
    pub wqe_inline_mode: [u8; 0x2],
    pub rss_ind_tbl_cap: [u8; 0x4],
    pub reg_umr_sq: [u8; 0x1],
    pub scatter_fcs: [u8; 0x1],
    pub enhanced_multi_pkt_send_wqe: [u8; 0x1],
    pub tunnel_lso_const_out_ip_id: [u8; 0x1],
    pub tunnel_lro_gre: [u8; 0x1],
    pub tunnel_lro_vxlan: [u8; 0x1],
    pub tunnel_stateless_gre: [u8; 0x1],
    pub tunnel_stateless_vxlan: [u8; 0x1],
    pub swp: [u8; 0x1],
    pub swp_csum: [u8; 0x1],
    pub swp_lso: [u8; 0x1],
    pub cqe_checksum_full: [u8; 0x1],
    pub tunnel_stateless_geneve_tx: [u8; 0x1],
    pub tunnel_stateless_mpls_over_udp: [u8; 0x1],
    pub tunnel_stateless_mpls_over_gre: [u8; 0x1],
    pub tunnel_stateless_vxlan_gpe: [u8; 0x1],
    pub tunnel_stateless_ipv4_over_vxlan: [u8; 0x1],
    pub tunnel_stateless_ip_over_ip: [u8; 0x1],
    pub insert_trailer: [u8; 0x1],
    pub reserved_at_2b: [u8; 0x1],
    pub tunnel_stateless_ip_over_ip_rx: [u8; 0x1],
    pub tunnel_stateless_ip_over_ip_tx: [u8; 0x1],
    pub reserved_at_2e: [u8; 0x2],
    pub max_vxlan_udp_ports: [u8; 0x8],
    pub swp_csum_l4_partial: [u8; 0x1],
    pub reserved_at_39: [u8; 0x5],
    pub max_geneve_opt_len: [u8; 0x1],
    pub tunnel_stateless_geneve_rx: [u8; 0x1],
    pub reserved_at_40: [u8; 0x10],
    pub lro_min_mss_size: [u8; 0x10],
    pub reserved_at_60: [u8; 0x120],
    pub lro_timer_supported_periods: [u8; 4][0x20],
    pub reserved_at_200: [u8; 0x600],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_roce_cap_bits {
    pub roce_apm: [u8; 0x1],
    pub reserved_at_1: [u8; 0x3],
    pub sw_r_roce_src_udp_port: [u8; 0x1],
    pub fl_rc_qp_when_roce_disabled: [u8; 0x1],
    pub fl_rc_qp_when_roce_enabled: [u8; 0x1],
    pub roce_cc_general: [u8; 0x1],
    pub qp_ooo_transmit_default: [u8; 0x1],
    pub reserved_at_9: [u8; 0x15],
    pub qp_ts_format: [u8; 0x2],
    pub reserved_at_20: [u8; 0x60],
    pub reserved_at_80: [u8; 0xc],
    pub l3_type: [u8; 0x4],
    pub reserved_at_90: [u8; 0x8],
    pub roce_version: [u8; 0x8],
    pub reserved_at_a0: [u8; 0x10],
    pub r_roce_dest_udp_port: [u8; 0x10],
    pub r_roce_max_src_udp_port: [u8; 0x10],
    pub r_roce_min_src_udp_port: [u8; 0x10],
    pub reserved_at_e0: [u8; 0x10],
    pub roce_address_table_size: [u8; 0x10],
    pub reserved_at_100: [u8; 0x700],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sync_steering_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0xc0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sync_steering_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sync_crypto_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x10],
    pub crypto_type: [u8; 0x10],
    pub reserved_at_80: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sync_crypto_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_device_mem_cap_bits {
    pub memic: [u8; 0x1],
    pub reserved_at_1: [u8; 0x1f],
    pub reserved_at_20: [u8; 0xb],
    pub log_min_memic_alloc_size: [u8; 0x5],
    pub reserved_at_30: [u8; 0x8],
    pub log_max_memic_addr_alignment: [u8; 0x8],
    pub memic_bar_start_addr: [u8; 0x40],
    pub memic_bar_size: [u8; 0x20],
    pub max_memic_size: [u8; 0x20],
    pub steering_sw_icm_start_address: [u8; 0x40],
    pub reserved_at_100: [u8; 0x8],
    pub log_header_modify_sw_icm_size: [u8; 0x8],
    pub reserved_at_110: [u8; 0x2],
    pub log_sw_icm_alloc_granularity: [u8; 0x6],
    pub log_steering_sw_icm_size: [u8; 0x8],
    pub log_indirect_encap_sw_icm_size: [u8; 0x8],
    pub reserved_at_128: [u8; 0x10],
    pub log_header_modify_pattern_sw_icm_size: [u8; 0x8],
    pub header_modify_sw_icm_start_address: [u8; 0x40],
    pub reserved_at_180: [u8; 0x40],
    pub header_modify_pattern_sw_icm_start_address: [u8; 0x40],
    pub memic_operations: [u8; 0x20],
    pub reserved_at_220: [u8; 0x20],
    pub indirect_encap_sw_icm_start_address: [u8; 0x40],
    pub reserved_at_280: [u8; 0x580],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_device_event_cap_bits {
    pub user_affiliated_events: [u8; 4][0x40],
    pub user_unaffiliated_events: [u8; 4][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_virtio_emulation_cap_bits {
    pub desc_tunnel_offload_type: [u8; 0x1],
    pub eth_frame_offload_type: [u8; 0x1],
    pub virtio_version_1_0: [u8; 0x1],
    pub device_features_bits_mask: [u8; 0xd],
    pub event_mode: [u8; 0x8],
    pub virtio_queue_type: [u8; 0x8],
    pub max_tunnel_desc: [u8; 0x10],
    pub reserved_at_30: [u8; 0x3],
    pub log_doorbell_stride: [u8; 0x5],
    pub reserved_at_38: [u8; 0x3],
    pub log_doorbell_bar_size: [u8; 0x5],
    pub doorbell_bar_offset: [u8; 0x40],
    pub max_emulated_devices: [u8; 0x8],
    pub max_num_virtio_queues: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x20],
    pub reserved_at_c0: [u8; 0x13],
    pub desc_group_mkey_supported: [u8; 0x1],
    pub freeze_to_rdy_supported: [u8; 0x1],
    pub reserved_at_d5: [u8; 0xb],
    pub reserved_at_e0: [u8; 0x20],
    pub umem_1_buffer_param_a: [u8; 0x20],
    pub umem_1_buffer_param_b: [u8; 0x20],
    pub umem_2_buffer_param_a: [u8; 0x20],
    pub umem_2_buffer_param_b: [u8; 0x20],
    pub umem_3_buffer_param_a: [u8; 0x20],
    pub umem_3_buffer_param_b: [u8; 0x20],
    pub reserved_at_1c0: [u8; 0x640],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tlp_dev_emu_capabilities_bits {
    pub reserved_at_0: [u8; 0x20],
    pub reserved_at_20: [u8; 0x13],
    pub log_tlp_rsp_gw_page_stride: [u8; 0x5],
    pub reserved_at_38: [u8; 0x8],
    pub reserved_at_40: [u8; 0xc0],
    pub reserved_at_100: [u8; 0xc],
    pub tlp_rsp_gw_num_pages: [u8; 0x4],
    pub reserved_at_110: [u8; 0x10],
    pub reserved_at_120: [u8; 0xa0],
    pub tlp_rsp_gw_pages_bar_offset: [u8; 0x40],
    pub reserved_at_200: [u8; 0x600],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_atomic_caps_bits {
    pub reserved_at_0: [u8; 0x40],
    pub atomic_req_8B_endianness_mode: [u8; 0x2],
    pub reserved_at_42: [u8; 0x4],
    pub supported_atomic_req_8B_endianness_mode_1: [u8; 0x1],
    pub reserved_at_47: [u8; 0x19],
    pub reserved_at_60: [u8; 0x20],
    pub reserved_at_80: [u8; 0x10],
    pub atomic_operations: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x10],
    pub atomic_size_qp: [u8; 0x10],
    pub reserved_at_c0: [u8; 0x10],
    pub atomic_size_dc: [u8; 0x10],
    pub reserved_at_e0: [u8; 0x720],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_odp_scheme_cap_bits {
    pub reserved_at_0: [u8; 0x40],
    pub sig: [u8; 0x1],
    pub reserved_at_41: [u8; 0x4],
    pub page_prefetch: [u8; 0x1],
    pub reserved_at_46: [u8; 0x1a],
    pub reserved_at_60: [u8; 0x20],
    pub rc_odp_caps: mlx5_ifc_odp_per_transport_service_cap_bits,
    pub uc_odp_caps: mlx5_ifc_odp_per_transport_service_cap_bits,
    pub ud_odp_caps: mlx5_ifc_odp_per_transport_service_cap_bits,
    pub xrc_odp_caps: mlx5_ifc_odp_per_transport_service_cap_bits,
    pub dc_odp_caps: mlx5_ifc_odp_per_transport_service_cap_bits,
    pub reserved_at_120: [u8; 0xe0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_odp_cap_bits {
    pub transport_page_fault_scheme_cap: mlx5_ifc_odp_scheme_cap_bits,
    pub memory_page_fault_scheme_cap: mlx5_ifc_odp_scheme_cap_bits,
    pub reserved_at_400: [u8; 0x200],
    pub mem_page_fault: [u8; 0x1],
    pub reserved_at_601: [u8; 0x1f],
    pub reserved_at_620: [u8; 0x1e0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tls_cap_bits {
    pub tls_1_2_aes_gcm_128: [u8; 0x1],
    pub tls_1_3_aes_gcm_128: [u8; 0x1],
    pub tls_1_2_aes_gcm_256: [u8; 0x1],
    pub tls_1_3_aes_gcm_256: [u8; 0x1],
    pub reserved_at_4: [u8; 0x1c],
    pub reserved_at_20: [u8; 0x7e0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ipsec_cap_bits {
    pub ipsec_full_offload: [u8; 0x1],
    pub ipsec_crypto_offload: [u8; 0x1],
    pub ipsec_esn: [u8; 0x1],
    pub ipsec_crypto_esp_aes_gcm_256_encrypt: [u8; 0x1],
    pub ipsec_crypto_esp_aes_gcm_128_encrypt: [u8; 0x1],
    pub ipsec_crypto_esp_aes_gcm_256_decrypt: [u8; 0x1],
    pub ipsec_crypto_esp_aes_gcm_128_decrypt: [u8; 0x1],
    pub reserved_at_7: [u8; 0x4],
    pub log_max_ipsec_offload: [u8; 0x5],
    pub reserved_at_10: [u8; 0x10],
    pub min_log_ipsec_full_replay_window: [u8; 0x8],
    pub max_log_ipsec_full_replay_window: [u8; 0x8],
    pub reserved_at_30: [u8; 0x7d0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_macsec_cap_bits {
    pub macsec_epn: [u8; 0x1],
    pub reserved_at_1: [u8; 0x2],
    pub macsec_crypto_esp_aes_gcm_256_encrypt: [u8; 0x1],
    pub macsec_crypto_esp_aes_gcm_128_encrypt: [u8; 0x1],
    pub macsec_crypto_esp_aes_gcm_256_decrypt: [u8; 0x1],
    pub macsec_crypto_esp_aes_gcm_128_decrypt: [u8; 0x1],
    pub reserved_at_7: [u8; 0x4],
    pub log_max_macsec_offload: [u8; 0x5],
    pub reserved_at_10: [u8; 0x10],
    pub min_log_macsec_full_replay_window: [u8; 0x8],
    pub max_log_macsec_full_replay_window: [u8; 0x8],
    pub reserved_at_30: [u8; 0x10],
    pub reserved_at_40: [u8; 0x7c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_psp_cap_bits {
    pub reserved_at_0: [u8; 0x1],
    pub psp_crypto_offload: [u8; 0x1],
    pub reserved_at_2: [u8; 0x1],
    pub psp_crypto_esp_aes_gcm_256_encrypt: [u8; 0x1],
    pub psp_crypto_esp_aes_gcm_128_encrypt: [u8; 0x1],
    pub psp_crypto_esp_aes_gcm_256_decrypt: [u8; 0x1],
    pub psp_crypto_esp_aes_gcm_128_decrypt: [u8; 0x1],
    pub reserved_at_7: [u8; 0x4],
    pub log_max_num_of_psp_spi: [u8; 0x5],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x7e0],
}

pub const MLX5_FC_BULK_SIZE_FACTOR: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_fc_bulk_alloc_bitmask {
    MLX5_FC_BULK_128   = (1 << 0),
    MLX5_FC_BULK_256   = (1 << 1),
    MLX5_FC_BULK_512   = (1 << 2),
    MLX5_FC_BULK_1024  = (1 << 3),
    MLX5_FC_BULK_2048  = (1 << 4),
    MLX5_FC_BULK_4096  = (1 << 5),
    MLX5_FC_BULK_8192  = (1 << 6),
    MLX5_FC_BULK_16384 = (1 << 7),
}

pub const MLX5_FT_MAX_MULTIPATH_LEVEL: c_int = 63;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cmd_hca_cap_bits {
    pub reserved_at_0: [u8; 0x6],
    pub page_request_disable: [u8; 0x1],
    pub abs_native_port_num: [u8; 0x1],
    pub reserved_at_8: [u8; 0x8],
    pub shared_object_to_user_object_allowed: [u8; 0x1],
    pub reserved_at_13: [u8; 0xe],
    pub vhca_resource_manager: [u8; 0x1],
    pub hca_cap_2: [u8; 0x1],
    pub create_lag_when_not_master_up: [u8; 0x1],
    pub dtor: [u8; 0x1],
    pub event_on_vhca_state_teardown_request: [u8; 0x1],
    pub event_on_vhca_state_in_use: [u8; 0x1],
    pub event_on_vhca_state_active: [u8; 0x1],
    pub event_on_vhca_state_allocated: [u8; 0x1],
    pub event_on_vhca_state_invalid: [u8; 0x1],
    pub reserved_at_28: [u8; 0x8],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub log_max_srq_sz: [u8; 0x8],
    pub log_max_qp_sz: [u8; 0x8],
    pub event_cap: [u8; 0x1],
    pub reserved_at_91: [u8; 0x2],
    pub isolate_vl_tc_new: [u8; 0x1],
    pub reserved_at_94: [u8; 0x4],
    pub prio_tag_required: [u8; 0x1],
    pub reserved_at_99: [u8; 0x2],
    pub log_max_qp: [u8; 0x5],
    pub reserved_at_a0: [u8; 0x3],
    pub ece_support: [u8; 0x1],
    pub reserved_at_a4: [u8; 0x5],
    pub reg_c_preserve: [u8; 0x1],
    pub reserved_at_aa: [u8; 0x1],
    pub log_max_srq: [u8; 0x5],
    pub reserved_at_b0: [u8; 0x1],
    pub uplink_follow: [u8; 0x1],
    pub ts_cqe_to_dest_cqn: [u8; 0x1],
    pub reserved_at_b3: [u8; 0x6],
    pub go_back_n: [u8; 0x1],
    pub reserved_at_ba: [u8; 0x6],
    pub max_sgl_for_optimized_performance: [u8; 0x8],
    pub log_max_cq_sz: [u8; 0x8],
    pub relaxed_ordering_write_umr: [u8; 0x1],
    pub relaxed_ordering_read_umr: [u8; 0x1],
    pub reserved_at_d2: [u8; 0x7],
    pub virtio_net_device_emualtion_manager: [u8; 0x1],
    pub virtio_blk_device_emualtion_manager: [u8; 0x1],
    pub log_max_cq: [u8; 0x5],
    pub log_max_eq_sz: [u8; 0x8],
    pub relaxed_ordering_write: [u8; 0x1],
    pub relaxed_ordering_read_pci_enabled: [u8; 0x1],
    pub log_max_mkey: [u8; 0x6],
    pub reserved_at_f0: [u8; 0x6],
    pub terminate_scatter_list_mkey: [u8; 0x1],
    pub repeated_mkey: [u8; 0x1],
    pub dump_fill_mkey: [u8; 0x1],
    pub reserved_at_f9: [u8; 0x2],
    pub fast_teardown: [u8; 0x1],
    pub log_max_eq: [u8; 0x4],
    pub max_indirection: [u8; 0x8],
    pub fixed_buffer_size: [u8; 0x1],
    pub log_max_mrw_sz: [u8; 0x7],
    pub force_teardown: [u8; 0x1],
    pub reserved_at_111: [u8; 0x1],
    pub log_max_bsf_list_size: [u8; 0x6],
    pub umr_extended_translation_offset: [u8; 0x1],
    pub null_mkey: [u8; 0x1],
    pub log_max_klm_list_size: [u8; 0x6],
    pub reserved_at_120: [u8; 0x2],
    pub qpc_extension: [u8; 0x1],
    pub reserved_at_123: [u8; 0x7],
    pub log_max_ra_req_dc: [u8; 0x6],
    pub reserved_at_130: [u8; 0x2],
    pub eth_wqe_too_small: [u8; 0x1],
    pub reserved_at_133: [u8; 0x6],
    pub vnic_env_cq_overrun: [u8; 0x1],
    pub log_max_ra_res_dc: [u8; 0x6],
    pub reserved_at_140: [u8; 0x5],
    pub release_all_pages: [u8; 0x1],
    pub must_not_use: [u8; 0x1],
    pub reserved_at_147: [u8; 0x2],
    pub roce_accl: [u8; 0x1],
    pub log_max_ra_req_qp: [u8; 0x6],
    pub reserved_at_150: [u8; 0xa],
    pub log_max_ra_res_qp: [u8; 0x6],
    pub end_pad: [u8; 0x1],
    pub cc_query_allowed: [u8; 0x1],
    pub cc_modify_allowed: [u8; 0x1],
    pub start_pad: [u8; 0x1],
    pub cache_line_128byte: [u8; 0x1],
    pub reserved_at_165: [u8; 0x4],
    pub rts2rts_qp_counters_set_id: [u8; 0x1],
    pub reserved_at_16a: [u8; 0x2],
    pub vnic_env_int_rq_oob: [u8; 0x1],
    pub sbcam_reg: [u8; 0x1],
    pub reserved_at_16e: [u8; 0x1],
    pub qcam_reg: [u8; 0x1],
    pub gid_table_size: [u8; 0x10],
    pub out_of_seq_cnt: [u8; 0x1],
    pub vport_counters: [u8; 0x1],
    pub retransmission_q_counters: [u8; 0x1],
    pub debug: [u8; 0x1],
    pub modify_rq_counter_set_id: [u8; 0x1],
    pub rq_delay_drop: [u8; 0x1],
    pub max_qp_cnt: [u8; 0xa],
    pub pkey_table_size: [u8; 0x10],
    pub vport_group_manager: [u8; 0x1],
    pub vhca_group_manager: [u8; 0x1],
    pub ib_virt: [u8; 0x1],
    pub eth_virt: [u8; 0x1],
    pub vnic_env_queue_counters: [u8; 0x1],
    pub ets: [u8; 0x1],
    pub nic_flow_table: [u8; 0x1],
    pub eswitch_manager: [u8; 0x1],
    pub device_memory: [u8; 0x1],
    pub mcam_reg: [u8; 0x1],
    pub pcam_reg: [u8; 0x1],
    pub local_ca_ack_delay: [u8; 0x5],
    pub port_module_event: [u8; 0x1],
    pub enhanced_error_q_counters: [u8; 0x1],
    pub ports_check: [u8; 0x1],
    pub reserved_at_1b3: [u8; 0x1],
    pub disable_link_up: [u8; 0x1],
    pub beacon_led: [u8; 0x1],
    pub port_type: [u8; 0x2],
    pub num_ports: [u8; 0x8],
    pub reserved_at_1c0: [u8; 0x1],
    pub pps: [u8; 0x1],
    pub pps_modify: [u8; 0x1],
    pub log_max_msg: [u8; 0x5],
    pub reserved_at_1c8: [u8; 0x4],
    pub max_tc: [u8; 0x4],
    pub temp_warn_event: [u8; 0x1],
    pub dcbx: [u8; 0x1],
    pub general_notification_event: [u8; 0x1],
    pub reserved_at_1d3: [u8; 0x2],
    pub fpga: [u8; 0x1],
    pub rol_s: [u8; 0x1],
    pub rol_g: [u8; 0x1],
    pub reserved_at_1d8: [u8; 0x1],
    pub wol_s: [u8; 0x1],
    pub wol_g: [u8; 0x1],
    pub wol_a: [u8; 0x1],
    pub wol_b: [u8; 0x1],
    pub wol_m: [u8; 0x1],
    pub wol_u: [u8; 0x1],
    pub wol_p: [u8; 0x1],
    pub stat_rate_support: [u8; 0x10],
    pub reserved_at_1f0: [u8; 0x1],
    pub pci_sync_for_fw_update_event: [u8; 0x1],
    pub reserved_at_1f2: [u8; 0x6],
    pub init2_lag_tx_port_affinity: [u8; 0x1],
    pub reserved_at_1fa: [u8; 0x2],
    pub wqe_based_flow_table_update_cap: [u8; 0x1],
    pub cqe_version: [u8; 0x4],
    pub compact_address_vector: [u8; 0x1],
    pub striding_rq: [u8; 0x1],
    pub reserved_at_202: [u8; 0x1],
    pub ipoib_enhanced_offloads: [u8; 0x1],
    pub ipoib_basic_offloads: [u8; 0x1],
    pub reserved_at_205: [u8; 0x1],
    pub repeated_block_disabled: [u8; 0x1],
    pub umr_modify_entity_size_disabled: [u8; 0x1],
    pub umr_modify_atomic_disabled: [u8; 0x1],
    pub umr_indirect_mkey_disabled: [u8; 0x1],
    pub umr_fence: [u8; 0x2],
    pub dc_req_scat_data_cqe: [u8; 0x1],
    pub reserved_at_20d: [u8; 0x2],
    pub drain_sigerr: [u8; 0x1],
    pub cmdif_checksum: [u8; 0x2],
    pub sigerr_cqe: [u8; 0x1],
    pub reserved_at_213: [u8; 0x1],
    pub wq_signature: [u8; 0x1],
    pub sctr_data_cqe: [u8; 0x1],
    pub reserved_at_216: [u8; 0x1],
    pub sho: [u8; 0x1],
    pub tph: [u8; 0x1],
    pub rf: [u8; 0x1],
    pub dct: [u8; 0x1],
    pub qos: [u8; 0x1],
    pub eth_net_offloads: [u8; 0x1],
    pub roce: [u8; 0x1],
    pub atomic: [u8; 0x1],
    pub reserved_at_21f: [u8; 0x1],
    pub cq_oi: [u8; 0x1],
    pub cq_resize: [u8; 0x1],
    pub cq_moderation: [u8; 0x1],
    pub cq_period_mode_modify: [u8; 0x1],
    pub reserved_at_224: [u8; 0x2],
    pub cq_eq_remap: [u8; 0x1],
    pub pg: [u8; 0x1],
    pub block_lb_mc: [u8; 0x1],
    pub reserved_at_229: [u8; 0x1],
    pub scqe_break_moderation: [u8; 0x1],
    pub cq_period_start_from_cqe: [u8; 0x1],
    pub cd: [u8; 0x1],
    pub reserved_at_22d: [u8; 0x1],
    pub apm: [u8; 0x1],
    pub vector_calc: [u8; 0x1],
    pub umr_ptr_rlky: [u8; 0x1],
    pub imaicl: [u8; 0x1],
    pub qp_packet_based: [u8; 0x1],
    pub reserved_at_233: [u8; 0x3],
    pub qkv: [u8; 0x1],
    pub pkv: [u8; 0x1],
    pub set_deth_sqpn: [u8; 0x1],
    pub reserved_at_239: [u8; 0x3],
    pub xrc: [u8; 0x1],
    pub ud: [u8; 0x1],
    pub uc: [u8; 0x1],
    pub rc: [u8; 0x1],
    pub uar_4k: [u8; 0x1],
    pub reserved_at_241: [u8; 0x7],
    pub fl_rc_qp_when_roce_disabled: [u8; 0x1],
    pub regexp_params: [u8; 0x1],
    pub uar_sz: [u8; 0x6],
    pub port_selection_cap: [u8; 0x1],
    pub nic_cap_reg: [u8; 0x1],
    pub umem_uid_0: [u8; 0x1],
    pub reserved_at_253: [u8; 0x5],
    pub log_pg_sz: [u8; 0x8],
    pub bf: [u8; 0x1],
    pub driver_version: [u8; 0x1],
    pub pad_tx_eth_packet: [u8; 0x1],
    pub reserved_at_263: [u8; 0x3],
    pub mkey_by_name: [u8; 0x1],
    pub reserved_at_267: [u8; 0x4],
    pub log_bf_reg_size: [u8; 0x5],
    pub disciplined_fr_counter: [u8; 0x1],
    pub reserved_at_271: [u8; 0x2],
    pub qp_error_syndrome: [u8; 0x1],
    pub reserved_at_274: [u8; 0x2],
    pub lag_dct: [u8; 0x2],
    pub lag_tx_port_affinity: [u8; 0x1],
    pub lag_native_fdb_selection: [u8; 0x1],
    pub reserved_at_27a: [u8; 0x1],
    pub lag_master: [u8; 0x1],
    pub num_lag_ports: [u8; 0x4],
    pub reserved_at_280: [u8; 0x10],
    pub max_wqe_sz_sq: [u8; 0x10],
    pub icm_mng_function_id_mode: [u8; 0x1],
    pub reserved_at_2a1: [u8; 0x6],
    pub mkey_pcie_tph: [u8; 0x1],
    pub reserved_at_2a8: [u8; 0x1],
    pub tis_tir_td_order: [u8; 0x1],
    pub psp: [u8; 0x1],
    pub shampo: [u8; 0x1],
    pub reserved_at_2ac: [u8; 0x4],
    pub max_wqe_sz_rq: [u8; 0x10],
    pub max_flow_counter_31_16: [u8; 0x10],
    pub max_wqe_sz_sq_dc: [u8; 0x10],
    pub query_host_net_function_num_max: [u8; 0x5],
    pub reserved_at_2e5: [u8; 0x2],
    pub max_qp_mcg: [u8; 0x19],
    pub reserved_at_300: [u8; 0x10],
    pub flow_counter_bulk_alloc: [u8; 0x8],
    pub log_max_mcg: [u8; 0x8],
    pub reserved_at_320: [u8; 0x3],
    pub log_max_transport_domain: [u8; 0x5],
    pub reserved_at_328: [u8; 0x2],
    pub relaxed_ordering_read: [u8; 0x1],
    pub log_max_pd: [u8; 0x5],
    pub dp_ordering_ooo_all_ud: [u8; 0x1],
    pub dp_ordering_ooo_all_uc: [u8; 0x1],
    pub dp_ordering_ooo_all_xrc: [u8; 0x1],
    pub dp_ordering_ooo_all_dc: [u8; 0x1],
    pub dp_ordering_ooo_all_rc: [u8; 0x1],
    pub pcie_reset_using_hotreset_method: [u8; 0x1],
    pub pci_sync_for_fw_update_with_driver_unload: [u8; 0x1],
    pub vnic_env_cnt_steering_fail: [u8; 0x1],
    pub vport_counter_local_loopback: [u8; 0x1],
    pub q_counter_aggregation: [u8; 0x1],
    pub q_counter_other_vport: [u8; 0x1],
    pub log_max_xrcd: [u8; 0x5],
    pub nic_receive_steering_discard: [u8; 0x1],
    pub receive_discard_vport_down: [u8; 0x1],
    pub transmit_discard_vport_down: [u8; 0x1],
    pub eq_overrun_count: [u8; 0x1],
    pub reserved_at_344: [u8; 0x1],
    pub invalid_command_count: [u8; 0x1],
    pub quota_exceeded_count: [u8; 0x1],
    pub reserved_at_347: [u8; 0x1],
    pub log_max_flow_counter_bulk: [u8; 0x8],
    pub max_flow_counter_15_0: [u8; 0x10],
    pub reserved_at_360: [u8; 0x3],
    pub log_max_rq: [u8; 0x5],
    pub ft_alias_sw_vhca_id: [u8; 0x1],
    pub reserved_at_369: [u8; 0x2],
    pub log_max_sq: [u8; 0x5],
    pub reserved_at_370: [u8; 0x3],
    pub log_max_tir: [u8; 0x5],
    pub reserved_at_378: [u8; 0x3],
    pub log_max_tis: [u8; 0x5],
    pub basic_cyclic_rcv_wqe: [u8; 0x1],
    pub reserved_at_381: [u8; 0x2],
    pub log_max_rmp: [u8; 0x5],
    pub sd_group_size: [u8; 0x1],
    pub reserved_at_389: [u8; 0x2],
    pub log_max_rqt: [u8; 0x5],
    pub reserved_at_390: [u8; 0x3],
    pub log_max_rqt_size: [u8; 0x5],
    pub tlp_device_emulation_manager: [u8; 0x1],
    pub vnic_env_cnt_bar_uar_access: [u8; 0x1],
    pub vnic_env_cnt_odp_page_fault: [u8; 0x1],
    pub log_max_tis_per_sq: [u8; 0x5],
    pub ext_stride_num_range: [u8; 0x1],
    pub roce_rw_supported: [u8; 0x1],
    pub log_max_current_uc_list_wr_supported: [u8; 0x1],
    pub log_max_stride_sz_rq: [u8; 0x5],
    pub reserved_at_3a8: [u8; 0x3],
    pub log_min_stride_sz_rq: [u8; 0x5],
    pub reserved_at_3b0: [u8; 0x2],
    pub qp_latency_sensitive_disable: [u8; 0x1],
    pub log_max_stride_sz_sq: [u8; 0x5],
    pub reserved_at_3b8: [u8; 0x3],
    pub log_min_stride_sz_sq: [u8; 0x5],
    pub hairpin: [u8; 0x1],
    pub reserved_at_3c1: [u8; 0x2],
    pub log_max_hairpin_queues: [u8; 0x5],
    pub reserved_at_3c8: [u8; 0x3],
    pub log_max_hairpin_wq_data_sz: [u8; 0x5],
    pub reserved_at_3d0: [u8; 0x3],
    pub log_max_hairpin_num_packets: [u8; 0x5],
    pub reserved_at_3d8: [u8; 0x3],
    pub log_max_wq_sz: [u8; 0x5],
    pub nic_vport_change_event: [u8; 0x1],
    pub disable_local_lb_uc: [u8; 0x1],
    pub disable_local_lb_mc: [u8; 0x1],
    pub log_min_hairpin_wq_data_sz: [u8; 0x5],
    pub reserved_at_3e8: [u8; 0x1],
    pub silent_mode_set: [u8; 0x1],
    pub vhca_state: [u8; 0x1],
    pub log_max_vlan_list: [u8; 0x5],
    pub reserved_at_3f0: [u8; 0x3],
    pub log_max_current_mc_list: [u8; 0x5],
    pub reserved_at_3f8: [u8; 0x1],
    pub silent_mode_query: [u8; 0x1],
    pub query_host_net_function_v1: [u8; 0x1],
    pub log_max_current_uc_list: [u8; 0x5],
    pub general_obj_types: [u8; 0x40],
    pub sq_ts_format: [u8; 0x2],
    pub rq_ts_format: [u8; 0x2],
    pub steering_format_version: [u8; 0x4],
    pub create_qp_start_hint: [u8; 0x18],
    pub reserved_at_460: [u8; 0x1],
    pub ats: [u8; 0x1],
    pub cross_vhca_rqt: [u8; 0x1],
    pub log_max_uctx: [u8; 0x5],
    pub reserved_at_468: [u8; 0x1],
    pub crypto: [u8; 0x1],
    pub ipsec_offload: [u8; 0x1],
    pub log_max_umem: [u8; 0x5],
    pub max_num_eqs: [u8; 0x10],
    pub reserved_at_480: [u8; 0x1],
    pub tls_tx: [u8; 0x1],
    pub tls_rx: [u8; 0x1],
    pub log_max_l2_table: [u8; 0x5],
    pub reserved_at_488: [u8; 0x8],
    pub log_uar_page_sz: [u8; 0x10],
    pub reserved_at_4a0: [u8; 0x20],
    pub device_frequency_mhz: [u8; 0x20],
    pub device_frequency_khz: [u8; 0x20],
    pub reserved_at_500: [u8; 0x20],
    pub num_of_uars_per_page: [u8; 0x20],
    pub flex_parser_protocols: [u8; 0x20],
    pub max_geneve_tlv_options: [u8; 0x8],
    pub reserved_at_568: [u8; 0x3],
    pub max_geneve_tlv_option_data_len: [u8; 0x5],
    pub reserved_at_570: [u8; 0x1],
    pub adv_rdma: [u8; 0x1],
    pub reserved_at_572: [u8; 0x7],
    pub adv_virtualization: [u8; 0x1],
    pub reserved_at_57a: [u8; 0x6],
    pub reserved_at_580: [u8; 0xb],
    pub log_max_dci_stream_channels: [u8; 0x5],
    pub reserved_at_590: [u8; 0x3],
    pub log_max_dci_errored_streams: [u8; 0x5],
    pub reserved_at_598: [u8; 0x8],
    pub reserved_at_5a0: [u8; 0x10],
    pub enhanced_cqe_compression: [u8; 0x1],
    pub reserved_at_5b1: [u8; 0x1],
    pub crossing_vhca_mkey: [u8; 0x1],
    pub log_max_dek: [u8; 0x5],
    pub reserved_at_5b8: [u8; 0x4],
    pub mini_cqe_resp_stride_index: [u8; 0x1],
    pub cqe_128_always: [u8; 0x1],
    pub cqe_compression_128: [u8; 0x1],
    pub cqe_compression: [u8; 0x1],
    pub cqe_compression_timeout: [u8; 0x10],
    pub cqe_compression_max_num: [u8; 0x10],
    pub reserved_at_5e0: [u8; 0x8],
    pub flex_parser_id_gtpu_dw_0: [u8; 0x4],
    pub reserved_at_5ec: [u8; 0x4],
    pub tag_matching: [u8; 0x1],
    pub rndv_offload_rc: [u8; 0x1],
    pub rndv_offload_dc: [u8; 0x1],
    pub log_tag_matching_list_sz: [u8; 0x5],
    pub reserved_at_5f8: [u8; 0x3],
    pub log_max_xrq: [u8; 0x5],
    pub affiliate_nic_vport_criteria: [u8; 0x8],
    pub native_port_num: [u8; 0x8],
    pub num_vhca_ports: [u8; 0x8],
    pub flex_parser_id_gtpu_teid: [u8; 0x4],
    pub reserved_at_61c: [u8; 0x2],
    pub sw_owner_id: [u8; 0x1],
    pub reserved_at_61f: [u8; 0x1],
    pub max_num_of_monitor_counters: [u8; 0x10],
    pub num_ppcnt_monitor_counters: [u8; 0x10],
    pub max_num_sf: [u8; 0x10],
    pub num_q_monitor_counters: [u8; 0x10],
    pub reserved_at_660: [u8; 0x20],
    pub sf: [u8; 0x1],
    pub sf_set_partition: [u8; 0x1],
    pub reserved_at_682: [u8; 0x1],
    pub log_max_sf: [u8; 0x5],
    pub apu: [u8; 0x1],
    pub reserved_at_689: [u8; 0x4],
    pub migration: [u8; 0x1],
    pub reserved_at_68e: [u8; 0x2],
    pub log_min_sf_size: [u8; 0x8],
    pub max_num_sf_partitions: [u8; 0x8],
    pub uctx_cap: [u8; 0x20],
    pub reserved_at_6c0: [u8; 0x4],
    pub flex_parser_id_geneve_tlv_option_0: [u8; 0x4],
    pub flex_parser_id_icmp_dw1: [u8; 0x4],
    pub flex_parser_id_icmp_dw0: [u8; 0x4],
    pub flex_parser_id_icmpv6_dw1: [u8; 0x4],
    pub flex_parser_id_icmpv6_dw0: [u8; 0x4],
    pub flex_parser_id_outer_first_mpls_over_gre: [u8; 0x4],
    pub flex_parser_id_outer_first_mpls_over_udp_label: [u8; 0x4],
    pub max_num_match_definer: [u8; 0x10],
    pub sf_base_id: [u8; 0x10],
    pub flex_parser_id_gtpu_dw_2: [u8; 0x4],
    pub flex_parser_id_gtpu_first_ext_dw_0: [u8; 0x4],
    pub num_total_dynamic_vf_msix: [u8; 0x18],
    pub reserved_at_720: [u8; 0x14],
    pub dynamic_msix_table_size: [u8; 0xc],
    pub reserved_at_740: [u8; 0xc],
    pub min_dynamic_vf_msix_table_size: [u8; 0x4],
    pub reserved_at_750: [u8; 0x2],
    pub data_direct: [u8; 0x1],
    pub reserved_at_753: [u8; 0x1],
    pub max_dynamic_vf_msix_table_size: [u8; 0xc],
    pub reserved_at_760: [u8; 0x3],
    pub log_max_num_header_modify_argument: [u8; 0x5],
    pub log_header_modify_argument_granularity_offset: [u8; 0x4],
    pub log_header_modify_argument_granularity: [u8; 0x4],
    pub reserved_at_770: [u8; 0x3],
    pub log_header_modify_argument_max_alloc: [u8; 0x5],
    pub reserved_at_778: [u8; 0x8],
    pub vhca_tunnel_commands: [u8; 0x40],
    pub match_definer_format_supported: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cmd_hca_cap_2_bits {
    pub reserved_at_0: [u8; 0x80],
    pub migratable: [u8; 0x1],
    pub reserved_at_81: [u8; 0x7],
    pub dp_ordering_force: [u8; 0x1],
    pub reserved_at_89: [u8; 0x9],
    pub query_vuid: [u8; 0x1],
    pub reserved_at_93: [u8; 0x5],
    pub umr_log_entity_size_5: [u8; 0x1],
    pub reserved_at_99: [u8; 0x7],
    pub max_reformat_insert_size: [u8; 0x8],
    pub max_reformat_insert_offset: [u8; 0x8],
    pub max_reformat_remove_size: [u8; 0x8],
    pub max_reformat_remove_offset: [u8; 0x8],
    pub reserved_at_c0: [u8; 0x8],
    pub migration_multi_load: [u8; 0x1],
    pub migration_tracking_state: [u8; 0x1],
    pub multiplane_qp_ud: [u8; 0x1],
    pub reserved_at_cb: [u8; 0x5],
    pub migration_in_chunks: [u8; 0x1],
    pub reserved_at_d1: [u8; 0x1],
    pub sf_eq_usage: [u8; 0x1],
    pub reserved_at_d3: [u8; 0x5],
    pub multiplane: [u8; 0x1],
    pub migration_state: [u8; 0x1],
    pub reserved_at_da: [u8; 0x6],
    pub cross_vhca_object_to_object_supported: [u8; 0x20],
    pub allowed_object_for_other_vhca_access: [u8; 0x40],
    pub reserved_at_140: [u8; 0x60],
    pub flow_table_type_2_type: [u8; 0x8],
    pub reserved_at_1a8: [u8; 0x2],
    pub format_select_dw_8_6_ext: [u8; 0x1],
    pub log_min_mkey_entity_size: [u8; 0x5],
    pub reserved_at_1b0: [u8; 0x10],
    pub general_obj_types_127_64: [u8; 0x40],
    pub reserved_at_200: [u8; 0x20],
    pub reserved_at_220: [u8; 0x1],
    pub sw_vhca_id_valid: [u8; 0x1],
    pub sw_vhca_id: [u8; 0xe],
    pub reserved_at_230: [u8; 0x10],
    pub reserved_at_240: [u8; 0xb],
    pub ts_cqe_metadata_size2wqe_counter: [u8; 0x5],
    pub reserved_at_250: [u8; 0x10],
    pub reserved_at_260: [u8; 0x20],
    pub format_select_dw_gtpu_dw_0: [u8; 0x8],
    pub format_select_dw_gtpu_dw_1: [u8; 0x8],
    pub format_select_dw_gtpu_dw_2: [u8; 0x8],
    pub format_select_dw_gtpu_first_ext_dw_0: [u8; 0x8],
    pub generate_wqe_type: [u8; 0x20],
    pub reserved_at_2c0: [u8; 0xc0],
    pub reserved_at_380: [u8; 0xb],
    pub min_mkey_log_entity_size_fixed_buffer: [u8; 0x5],
    pub ec_vf_vport_base: [u8; 0x10],
    pub reserved_at_3a0: [u8; 0x2],
    pub max_mkey_log_entity_size_fixed_buffer: [u8; 0x6],
    pub reserved_at_3a8: [u8; 0x2],
    pub max_mkey_log_entity_size_mtt: [u8; 0x6],
    pub max_rqt_vhca_id: [u8; 0x10],
    pub reserved_at_3c0: [u8; 0x20],
    pub reserved_at_3e0: [u8; 0x10],
    pub pcc_ifa2: [u8; 0x1],
    pub reserved_at_3f1: [u8; 0xf],
    pub reserved_at_400: [u8; 0x1],
    pub min_mkey_log_entity_size_fixed_buffer_valid: [u8; 0x1],
    pub reserved_at_402: [u8; 0xe],
    pub return_reg_id: [u8; 0x10],
    pub reserved_at_420: [u8; 0x1c],
    pub flow_table_hash_type: [u8; 0x4],
    pub reserved_at_440: [u8; 0x8],
    pub max_num_eqs_24b: [u8; 0x18],
    pub reserved_at_460: [u8; 0x144],
    pub load_balance_id: [u8; 0x4],
    pub reserved_at_5a8: [u8; 0x18],
    pub query_adjacent_functions_id: [u8; 0x1],
    pub ingress_egress_esw_vport_connect: [u8; 0x1],
    pub function_id_type_vhca_id: [u8; 0x1],
    pub reserved_at_5c3: [u8; 0x1],
    pub lag_per_mp_group: [u8; 0x1],
    pub reserved_at_5c5: [u8; 0xb],
    pub delegate_vhca_management_profiles: [u8; 0x10],
    pub delegated_vhca_max: [u8; 0x10],
    pub delegate_vhca_max: [u8; 0x10],
    pub reserved_at_600: [u8; 0x200],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_flow_destination_type {
    MLX5_IFC_FLOW_DESTINATION_TYPE_VPORT        = 0x0,
    MLX5_IFC_FLOW_DESTINATION_TYPE_FLOW_TABLE   = 0x1,
    MLX5_IFC_FLOW_DESTINATION_TYPE_TIR          = 0x2,
    MLX5_IFC_FLOW_DESTINATION_TYPE_VHCA_RX	    = 0x4,
    MLX5_IFC_FLOW_DESTINATION_TYPE_FLOW_SAMPLER = 0x6,
    MLX5_IFC_FLOW_DESTINATION_TYPE_UPLINK       = 0x8,
    MLX5_IFC_FLOW_DESTINATION_TYPE_TABLE_TYPE   = 0xA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_flow_table_miss_action {
    MLX5_FLOW_TABLE_MISS_ACTION_DEF,
    MLX5_FLOW_TABLE_MISS_ACTION_FWD,
    MLX5_FLOW_TABLE_MISS_ACTION_SWITCH_DOMAIN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dest_format_struct_bits {
    pub destination_type: [u8; 0x8],
    pub destination_id: [u8; 0x18],
    pub destination_eswitch_owner_vhca_id_valid: [u8; 0x1],
    pub packet_reformat: [u8; 0x1],
    pub reserved_at_22: [u8; 0x6],
    pub destination_table_type: [u8; 0x8],
    pub destination_eswitch_owner_vhca_id: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_counter_list_bits {
    pub flow_counter_id: [u8; 0x20],
    pub reserved_at_20: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_extended_dest_format_bits {
    pub destination_entry: mlx5_ifc_dest_format_struct_bits,
    pub packet_reformat_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_dest_format_flow_counter_list_auto_bits {
    pub extended_dest_format: mlx5_ifc_extended_dest_format_bits,
    pub flow_counter_list: mlx5_ifc_flow_counter_list_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_fte_match_param_bits {
    pub outer_headers: mlx5_ifc_fte_match_set_lyr_2_4_bits,
    pub misc_parameters: mlx5_ifc_fte_match_set_misc_bits,
    pub inner_headers: mlx5_ifc_fte_match_set_lyr_2_4_bits,
    pub misc_parameters_2: mlx5_ifc_fte_match_set_misc2_bits,
    pub misc_parameters_3: mlx5_ifc_fte_match_set_misc3_bits,
    pub misc_parameters_4: mlx5_ifc_fte_match_set_misc4_bits,
    pub misc_parameters_5: mlx5_ifc_fte_match_set_misc5_bits,
    pub misc_parameters_6: mlx5_ifc_fte_match_set_misc6_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rx_hash_field_select_bits {
    pub l3_prot_type: [u8; 0x1],
    pub l4_prot_type: [u8; 0x1],
    pub selected_fields: [u8; 0x1e],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_wq_bits {
    pub wq_type: [u8; 0x4],
    pub wq_signature: [u8; 0x1],
    pub end_padding_mode: [u8; 0x2],
    pub cd_slave: [u8; 0x1],
    pub reserved_at_8: [u8; 0x18],
    pub hds_skip_first_sge: [u8; 0x1],
    pub log2_hds_buf_size: [u8; 0x3],
    pub reserved_at_24: [u8; 0x7],
    pub page_offset: [u8; 0x5],
    pub lwm: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub reserved_at_60: [u8; 0x8],
    pub uar_page: [u8; 0x18],
    pub dbr_addr: [u8; 0x40],
    pub hw_counter: [u8; 0x20],
    pub sw_counter: [u8; 0x20],
    pub reserved_at_100: [u8; 0xc],
    pub log_wq_stride: [u8; 0x4],
    pub reserved_at_110: [u8; 0x3],
    pub log_wq_pg_sz: [u8; 0x5],
    pub reserved_at_118: [u8; 0x3],
    pub log_wq_sz: [u8; 0x5],
    pub dbr_umem_valid: [u8; 0x1],
    pub wq_umem_valid: [u8; 0x1],
    pub reserved_at_122: [u8; 0x1],
    pub log_hairpin_num_packets: [u8; 0x5],
    pub reserved_at_128: [u8; 0x3],
    pub log_hairpin_data_sz: [u8; 0x5],
    pub reserved_at_130: [u8; 0x4],
    pub log_wqe_num_of_strides: [u8; 0x4],
    pub two_byte_shift_en: [u8; 0x1],
    pub reserved_at_139: [u8; 0x4],
    pub log_wqe_stride_size: [u8; 0x3],
    pub dbr_umem_id: [u8; 0x20],
    pub wq_umem_id: [u8; 0x20],
    pub wq_umem_offset: [u8; 0x40],
    pub headers_mkey: [u8; 0x20],
    pub shampo_enable: [u8; 0x1],
    pub reserved_at_1e1: [u8; 0x1],
    pub shampo_mode: [u8; 0x2],
    pub reserved_at_1e4: [u8; 0x1],
    pub log_reservation_size: [u8; 0x3],
    pub reserved_at_1e8: [u8; 0x5],
    pub log_max_num_of_packets_per_reservation: [u8; 0x3],
    pub reserved_at_1f0: [u8; 0x6],
    pub log_headers_entry_size: [u8; 0x2],
    pub reserved_at_1f8: [u8; 0x4],
    pub log_headers_buffer_entry_num: [u8; 0x4],
    pub reserved_at_200: [u8; 0x400],
    pub pas: [mlx5_ifc_cmd_pas_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rq_num_bits {
    pub reserved_at_0: [u8; 0x8],
    pub rq_num: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rq_vhca_bits {
    pub reserved_at_0: [u8; 0x8],
    pub rq_num: [u8; 0x18],
    pub reserved_at_20: [u8; 0x10],
    pub rq_vhca_id: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mac_address_layout_bits {
    pub reserved_at_0: [u8; 0x10],
    pub mac_addr_47_32: [u8; 0x10],
    pub mac_addr_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_vlan_layout_bits {
    pub reserved_at_0: [u8; 0x14],
    pub vlan: [u8; 0x0c],
    pub reserved_at_20: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cong_control_r_roce_ecn_np_bits {
    pub reserved_at_0: [u8; 0xa0],
    pub min_time_between_cnps: [u8; 0x20],
    pub reserved_at_c0: [u8; 0x12],
    pub cnp_dscp: [u8; 0x6],
    pub reserved_at_d8: [u8; 0x4],
    pub cnp_prio_mode: [u8; 0x1],
    pub cnp_802p_prio: [u8; 0x3],
    pub reserved_at_e0: [u8; 0x720],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cong_control_r_roce_ecn_rp_bits {
    pub reserved_at_0: [u8; 0x60],
    pub reserved_at_60: [u8; 0x4],
    pub clamp_tgt_rate: [u8; 0x1],
    pub reserved_at_65: [u8; 0x3],
    pub clamp_tgt_rate_after_time_inc: [u8; 0x1],
    pub reserved_at_69: [u8; 0x17],
    pub reserved_at_80: [u8; 0x20],
    pub rpg_time_reset: [u8; 0x20],
    pub rpg_byte_reset: [u8; 0x20],
    pub rpg_threshold: [u8; 0x20],
    pub rpg_max_rate: [u8; 0x20],
    pub rpg_ai_rate: [u8; 0x20],
    pub rpg_hai_rate: [u8; 0x20],
    pub rpg_gd: [u8; 0x20],
    pub rpg_min_dec_fac: [u8; 0x20],
    pub rpg_min_rate: [u8; 0x20],
    pub reserved_at_1c0: [u8; 0xe0],
    pub rate_to_set_on_first_cnp: [u8; 0x20],
    pub dce_tcp_g: [u8; 0x20],
    pub dce_tcp_rtt: [u8; 0x20],
    pub rate_reduce_monitor_period: [u8; 0x20],
    pub reserved_at_320: [u8; 0x20],
    pub initial_alpha_value: [u8; 0x20],
    pub reserved_at_360: [u8; 0x4a0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cong_control_r_roce_general_bits {
    pub reserved_at_0: [u8; 0x80],
    pub reserved_at_80: [u8; 0x10],
    pub rtt_resp_dscp_valid: [u8; 0x1],
    pub reserved_at_91: [u8; 0x9],
    pub rtt_resp_dscp: [u8; 0x6],
    pub reserved_at_a0: [u8; 0x760],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cong_control_802_1qau_rp_bits {
    pub reserved_at_0: [u8; 0x80],
    pub rppp_max_rps: [u8; 0x20],
    pub rpg_time_reset: [u8; 0x20],
    pub rpg_byte_reset: [u8; 0x20],
    pub rpg_threshold: [u8; 0x20],
    pub rpg_max_rate: [u8; 0x20],
    pub rpg_ai_rate: [u8; 0x20],
    pub rpg_hai_rate: [u8; 0x20],
    pub rpg_gd: [u8; 0x20],
    pub rpg_min_dec_fac: [u8; 0x20],
    pub rpg_min_rate: [u8; 0x20],
    pub reserved_at_1c0: [u8; 0x640],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resize_field_select_bits {
    pub resize_field_select: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resource_dump_bits {
    pub more_dump: [u8; 0x1],
    pub inline_dump: [u8; 0x1],
    pub reserved_at_2: [u8; 0xa],
    pub seq_num: [u8; 0x4],
    pub segment_type: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub vhca_id: [u8; 0x10],
    pub index1: [u8; 0x20],
    pub index2: [u8; 0x20],
    pub num_of_obj1: [u8; 0x10],
    pub num_of_obj2: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x20],
    pub device_opaque: [u8; 0x40],
    pub mkey: [u8; 0x20],
    pub size: [u8; 0x20],
    pub address: [u8; 0x40],
    pub inline_data: [u8; 52][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resource_dump_menu_record_bits {
    pub reserved_at_0: [u8; 0x4],
    pub num_of_obj2_supports_active: [u8; 0x1],
    pub num_of_obj2_supports_all: [u8; 0x1],
    pub must_have_num_of_obj2: [u8; 0x1],
    pub support_num_of_obj2: [u8; 0x1],
    pub num_of_obj1_supports_active: [u8; 0x1],
    pub num_of_obj1_supports_all: [u8; 0x1],
    pub must_have_num_of_obj1: [u8; 0x1],
    pub support_num_of_obj1: [u8; 0x1],
    pub must_have_index2: [u8; 0x1],
    pub support_index2: [u8; 0x1],
    pub must_have_index1: [u8; 0x1],
    pub support_index1: [u8; 0x1],
    pub segment_type: [u8; 0x10],
    pub segment_name: [u8; 4][0x20],
    pub index1_name: [u8; 4][0x20],
    pub index2_name: [u8; 4][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resource_dump_segment_header_bits {
    pub length_dw: [u8; 0x10],
    pub segment_type: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resource_dump_command_segment_bits {
    pub segment_header: mlx5_ifc_resource_dump_segment_header_bits,
    pub segment_called: [u8; 0x10],
    pub vhca_id: [u8; 0x10],
    pub index1: [u8; 0x20],
    pub index2: [u8; 0x20],
    pub num_of_obj1: [u8; 0x10],
    pub num_of_obj2: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resource_dump_error_segment_bits {
    pub segment_header: mlx5_ifc_resource_dump_segment_header_bits,
    pub reserved_at_20: [u8; 0x10],
    pub syndrome_id: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub error: [u8; 8][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resource_dump_info_segment_bits {
    pub segment_header: mlx5_ifc_resource_dump_segment_header_bits,
    pub reserved_at_20: [u8; 0x18],
    pub dump_version: [u8; 0x8],
    pub hw_version: [u8; 0x20],
    pub fw_version: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resource_dump_menu_segment_bits {
    pub segment_header: mlx5_ifc_resource_dump_segment_header_bits,
    pub reserved_at_20: [u8; 0x10],
    pub num_of_records: [u8; 0x10],
    pub record: [mlx5_ifc_resource_dump_menu_record_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resource_dump_resource_segment_bits {
    pub segment_header: mlx5_ifc_resource_dump_segment_header_bits,
    pub reserved_at_20: [u8; 0x20],
    pub index1: [u8; 0x20],
    pub index2: [u8; 0x20],
    pub payload: [u8; ][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resource_dump_terminate_segment_bits {
    pub segment_header: mlx5_ifc_resource_dump_segment_header_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_menu_resource_dump_response_bits {
    pub info: mlx5_ifc_resource_dump_info_segment_bits,
    pub cmd: mlx5_ifc_resource_dump_command_segment_bits,
    pub menu: mlx5_ifc_resource_dump_menu_segment_bits,
    pub terminate: mlx5_ifc_resource_dump_terminate_segment_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_field_select_bits {
    pub modify_field_select: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_field_select_r_roce_np_bits {
    pub field_select_r_roce_np: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_field_select_r_roce_rp_bits {
    pub field_select_r_roce_rp: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_field_select_802_1qau_rp_bits {
    pub field_select_8021qaurp: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_phys_layer_recovery_cntrs_bits {
    pub total_successful_recovery_events: [u8; 0x20],
    pub reserved_at_20: [u8; 0x7a0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_phys_layer_cntrs_bits {
    pub time_since_last_clear_high: [u8; 0x20],
    pub time_since_last_clear_low: [u8; 0x20],
    pub symbol_errors_high: [u8; 0x20],
    pub symbol_errors_low: [u8; 0x20],
    pub sync_headers_errors_high: [u8; 0x20],
    pub sync_headers_errors_low: [u8; 0x20],
    pub edpl_bip_errors_lane0_high: [u8; 0x20],
    pub edpl_bip_errors_lane0_low: [u8; 0x20],
    pub edpl_bip_errors_lane1_high: [u8; 0x20],
    pub edpl_bip_errors_lane1_low: [u8; 0x20],
    pub edpl_bip_errors_lane2_high: [u8; 0x20],
    pub edpl_bip_errors_lane2_low: [u8; 0x20],
    pub edpl_bip_errors_lane3_high: [u8; 0x20],
    pub edpl_bip_errors_lane3_low: [u8; 0x20],
    pub fc_fec_corrected_blocks_lane0_high: [u8; 0x20],
    pub fc_fec_corrected_blocks_lane0_low: [u8; 0x20],
    pub fc_fec_corrected_blocks_lane1_high: [u8; 0x20],
    pub fc_fec_corrected_blocks_lane1_low: [u8; 0x20],
    pub fc_fec_corrected_blocks_lane2_high: [u8; 0x20],
    pub fc_fec_corrected_blocks_lane2_low: [u8; 0x20],
    pub fc_fec_corrected_blocks_lane3_high: [u8; 0x20],
    pub fc_fec_corrected_blocks_lane3_low: [u8; 0x20],
    pub fc_fec_uncorrectable_blocks_lane0_high: [u8; 0x20],
    pub fc_fec_uncorrectable_blocks_lane0_low: [u8; 0x20],
    pub fc_fec_uncorrectable_blocks_lane1_high: [u8; 0x20],
    pub fc_fec_uncorrectable_blocks_lane1_low: [u8; 0x20],
    pub fc_fec_uncorrectable_blocks_lane2_high: [u8; 0x20],
    pub fc_fec_uncorrectable_blocks_lane2_low: [u8; 0x20],
    pub fc_fec_uncorrectable_blocks_lane3_high: [u8; 0x20],
    pub fc_fec_uncorrectable_blocks_lane3_low: [u8; 0x20],
    pub rs_fec_corrected_blocks_high: [u8; 0x20],
    pub rs_fec_corrected_blocks_low: [u8; 0x20],
    pub rs_fec_uncorrectable_blocks_high: [u8; 0x20],
    pub rs_fec_uncorrectable_blocks_low: [u8; 0x20],
    pub rs_fec_no_errors_blocks_high: [u8; 0x20],
    pub rs_fec_no_errors_blocks_low: [u8; 0x20],
    pub rs_fec_single_error_blocks_high: [u8; 0x20],
    pub rs_fec_single_error_blocks_low: [u8; 0x20],
    pub rs_fec_corrected_symbols_total_high: [u8; 0x20],
    pub rs_fec_corrected_symbols_total_low: [u8; 0x20],
    pub rs_fec_corrected_symbols_lane0_high: [u8; 0x20],
    pub rs_fec_corrected_symbols_lane0_low: [u8; 0x20],
    pub rs_fec_corrected_symbols_lane1_high: [u8; 0x20],
    pub rs_fec_corrected_symbols_lane1_low: [u8; 0x20],
    pub rs_fec_corrected_symbols_lane2_high: [u8; 0x20],
    pub rs_fec_corrected_symbols_lane2_low: [u8; 0x20],
    pub rs_fec_corrected_symbols_lane3_high: [u8; 0x20],
    pub rs_fec_corrected_symbols_lane3_low: [u8; 0x20],
    pub link_down_events: [u8; 0x20],
    pub successful_recovery_events: [u8; 0x20],
    pub reserved_at_640: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_phys_layer_statistical_cntrs_bits {
    pub time_since_last_clear_high: [u8; 0x20],
    pub time_since_last_clear_low: [u8; 0x20],
    pub phy_received_bits_high: [u8; 0x20],
    pub phy_received_bits_low: [u8; 0x20],
    pub phy_symbol_errors_high: [u8; 0x20],
    pub phy_symbol_errors_low: [u8; 0x20],
    pub phy_corrected_bits_high: [u8; 0x20],
    pub phy_corrected_bits_low: [u8; 0x20],
    pub phy_corrected_bits_lane0_high: [u8; 0x20],
    pub phy_corrected_bits_lane0_low: [u8; 0x20],
    pub phy_corrected_bits_lane1_high: [u8; 0x20],
    pub phy_corrected_bits_lane1_low: [u8; 0x20],
    pub phy_corrected_bits_lane2_high: [u8; 0x20],
    pub phy_corrected_bits_lane2_low: [u8; 0x20],
    pub phy_corrected_bits_lane3_high: [u8; 0x20],
    pub phy_corrected_bits_lane3_low: [u8; 0x20],
    pub reserved_at_200: [u8; 0x5c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ib_port_cntrs_grp_data_layout_bits {
    pub symbol_error_counter: [u8; 0x10],
    pub link_error_recovery_counter: [u8; 0x8],
    pub link_downed_counter: [u8; 0x8],
    pub port_rcv_errors: [u8; 0x10],
    pub port_rcv_remote_physical_errors: [u8; 0x10],
    pub port_rcv_switch_relay_errors: [u8; 0x10],
    pub port_xmit_discards: [u8; 0x10],
    pub port_xmit_constraint_errors: [u8; 0x8],
    pub port_rcv_constraint_errors: [u8; 0x8],
    pub reserved_at_70: [u8; 0x8],
    pub link_overrun_errors: [u8; 0x8],
    pub reserved_at_80: [u8; 0x10],
    pub vl_15_dropped: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x80],
    pub port_xmit_wait: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ib_ext_port_cntrs_grp_data_layout_bits {
    pub reserved_at_0: [u8; 0x300],
    pub port_xmit_data_high: [u8; 0x20],
    pub port_xmit_data_low: [u8; 0x20],
    pub port_rcv_data_high: [u8; 0x20],
    pub port_rcv_data_low: [u8; 0x20],
    pub port_xmit_pkts_high: [u8; 0x20],
    pub port_xmit_pkts_low: [u8; 0x20],
    pub port_rcv_pkts_high: [u8; 0x20],
    pub port_rcv_pkts_low: [u8; 0x20],
    pub reserved_at_400: [u8; 0x80],
    pub port_unicast_xmit_pkts_high: [u8; 0x20],
    pub port_unicast_xmit_pkts_low: [u8; 0x20],
    pub port_multicast_xmit_pkts_high: [u8; 0x20],
    pub port_multicast_xmit_pkts_low: [u8; 0x20],
    pub port_unicast_rcv_pkts_high: [u8; 0x20],
    pub port_unicast_rcv_pkts_low: [u8; 0x20],
    pub port_multicast_rcv_pkts_high: [u8; 0x20],
    pub port_multicast_rcv_pkts_low: [u8; 0x20],
    pub reserved_at_580: [u8; 0x240],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eth_per_tc_prio_grp_data_layout_bits {
    pub transmit_queue_high: [u8; 0x20],
    pub transmit_queue_low: [u8; 0x20],
    pub no_buffer_discard_uc_high: [u8; 0x20],
    pub no_buffer_discard_uc_low: [u8; 0x20],
    pub reserved_at_80: [u8; 0x740],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eth_per_tc_congest_prio_grp_data_layout_bits {
    pub wred_discard_high: [u8; 0x20],
    pub wred_discard_low: [u8; 0x20],
    pub ecn_marked_tc_high: [u8; 0x20],
    pub ecn_marked_tc_low: [u8; 0x20],
    pub reserved_at_80: [u8; 0x740],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eth_per_prio_grp_data_layout_bits {
    pub rx_octets_high: [u8; 0x20],
    pub rx_octets_low: [u8; 0x20],
    pub reserved_at_40: [u8; 0xc0],
    pub rx_frames_high: [u8; 0x20],
    pub rx_frames_low: [u8; 0x20],
    pub tx_octets_high: [u8; 0x20],
    pub tx_octets_low: [u8; 0x20],
    pub reserved_at_180: [u8; 0xc0],
    pub tx_frames_high: [u8; 0x20],
    pub tx_frames_low: [u8; 0x20],
    pub rx_pause_high: [u8; 0x20],
    pub rx_pause_low: [u8; 0x20],
    pub rx_pause_duration_high: [u8; 0x20],
    pub rx_pause_duration_low: [u8; 0x20],
    pub tx_pause_high: [u8; 0x20],
    pub tx_pause_low: [u8; 0x20],
    pub tx_pause_duration_high: [u8; 0x20],
    pub tx_pause_duration_low: [u8; 0x20],
    pub rx_pause_transition_high: [u8; 0x20],
    pub rx_pause_transition_low: [u8; 0x20],
    pub rx_discards_high: [u8; 0x20],
    pub rx_discards_low: [u8; 0x20],
    pub device_stall_minor_watermark_cnt_high: [u8; 0x20],
    pub device_stall_minor_watermark_cnt_low: [u8; 0x20],
    pub device_stall_critical_watermark_cnt_high: [u8; 0x20],
    pub device_stall_critical_watermark_cnt_low: [u8; 0x20],
    pub reserved_at_480: [u8; 0x340],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eth_extended_cntrs_grp_data_layout_bits {
    pub port_transmit_wait_high: [u8; 0x20],
    pub port_transmit_wait_low: [u8; 0x20],
    pub reserved_at_40: [u8; 0x100],
    pub rx_buffer_almost_full_high: [u8; 0x20],
    pub rx_buffer_almost_full_low: [u8; 0x20],
    pub rx_buffer_full_high: [u8; 0x20],
    pub rx_buffer_full_low: [u8; 0x20],
    pub rx_icrc_encapsulated_high: [u8; 0x20],
    pub rx_icrc_encapsulated_low: [u8; 0x20],
    pub reserved_at_200: [u8; 0x5c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eth_3635_cntrs_grp_data_layout_bits {
    pub dot3stats_alignment_errors_high: [u8; 0x20],
    pub dot3stats_alignment_errors_low: [u8; 0x20],
    pub dot3stats_fcs_errors_high: [u8; 0x20],
    pub dot3stats_fcs_errors_low: [u8; 0x20],
    pub dot3stats_single_collision_frames_high: [u8; 0x20],
    pub dot3stats_single_collision_frames_low: [u8; 0x20],
    pub dot3stats_multiple_collision_frames_high: [u8; 0x20],
    pub dot3stats_multiple_collision_frames_low: [u8; 0x20],
    pub dot3stats_sqe_test_errors_high: [u8; 0x20],
    pub dot3stats_sqe_test_errors_low: [u8; 0x20],
    pub dot3stats_deferred_transmissions_high: [u8; 0x20],
    pub dot3stats_deferred_transmissions_low: [u8; 0x20],
    pub dot3stats_late_collisions_high: [u8; 0x20],
    pub dot3stats_late_collisions_low: [u8; 0x20],
    pub dot3stats_excessive_collisions_high: [u8; 0x20],
    pub dot3stats_excessive_collisions_low: [u8; 0x20],
    pub dot3stats_internal_mac_transmit_errors_high: [u8; 0x20],
    pub dot3stats_internal_mac_transmit_errors_low: [u8; 0x20],
    pub dot3stats_carrier_sense_errors_high: [u8; 0x20],
    pub dot3stats_carrier_sense_errors_low: [u8; 0x20],
    pub dot3stats_frame_too_longs_high: [u8; 0x20],
    pub dot3stats_frame_too_longs_low: [u8; 0x20],
    pub dot3stats_internal_mac_receive_errors_high: [u8; 0x20],
    pub dot3stats_internal_mac_receive_errors_low: [u8; 0x20],
    pub dot3stats_symbol_errors_high: [u8; 0x20],
    pub dot3stats_symbol_errors_low: [u8; 0x20],
    pub dot3control_in_unknown_opcodes_high: [u8; 0x20],
    pub dot3control_in_unknown_opcodes_low: [u8; 0x20],
    pub dot3in_pause_frames_high: [u8; 0x20],
    pub dot3in_pause_frames_low: [u8; 0x20],
    pub dot3out_pause_frames_high: [u8; 0x20],
    pub dot3out_pause_frames_low: [u8; 0x20],
    pub reserved_at_400: [u8; 0x3c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eth_2819_cntrs_grp_data_layout_bits {
    pub ether_stats_drop_events_high: [u8; 0x20],
    pub ether_stats_drop_events_low: [u8; 0x20],
    pub ether_stats_octets_high: [u8; 0x20],
    pub ether_stats_octets_low: [u8; 0x20],
    pub ether_stats_pkts_high: [u8; 0x20],
    pub ether_stats_pkts_low: [u8; 0x20],
    pub ether_stats_broadcast_pkts_high: [u8; 0x20],
    pub ether_stats_broadcast_pkts_low: [u8; 0x20],
    pub ether_stats_multicast_pkts_high: [u8; 0x20],
    pub ether_stats_multicast_pkts_low: [u8; 0x20],
    pub ether_stats_crc_align_errors_high: [u8; 0x20],
    pub ether_stats_crc_align_errors_low: [u8; 0x20],
    pub ether_stats_undersize_pkts_high: [u8; 0x20],
    pub ether_stats_undersize_pkts_low: [u8; 0x20],
    pub ether_stats_oversize_pkts_high: [u8; 0x20],
    pub ether_stats_oversize_pkts_low: [u8; 0x20],
    pub ether_stats_fragments_high: [u8; 0x20],
    pub ether_stats_fragments_low: [u8; 0x20],
    pub ether_stats_jabbers_high: [u8; 0x20],
    pub ether_stats_jabbers_low: [u8; 0x20],
    pub ether_stats_collisions_high: [u8; 0x20],
    pub ether_stats_collisions_low: [u8; 0x20],
    pub ether_stats_pkts64octets_high: [u8; 0x20],
    pub ether_stats_pkts64octets_low: [u8; 0x20],
    pub ether_stats_pkts65to127octets_high: [u8; 0x20],
    pub ether_stats_pkts65to127octets_low: [u8; 0x20],
    pub ether_stats_pkts128to255octets_high: [u8; 0x20],
    pub ether_stats_pkts128to255octets_low: [u8; 0x20],
    pub ether_stats_pkts256to511octets_high: [u8; 0x20],
    pub ether_stats_pkts256to511octets_low: [u8; 0x20],
    pub ether_stats_pkts512to1023octets_high: [u8; 0x20],
    pub ether_stats_pkts512to1023octets_low: [u8; 0x20],
    pub ether_stats_pkts1024to1518octets_high: [u8; 0x20],
    pub ether_stats_pkts1024to1518octets_low: [u8; 0x20],
    pub ether_stats_pkts1519to2047octets_high: [u8; 0x20],
    pub ether_stats_pkts1519to2047octets_low: [u8; 0x20],
    pub ether_stats_pkts2048to4095octets_high: [u8; 0x20],
    pub ether_stats_pkts2048to4095octets_low: [u8; 0x20],
    pub ether_stats_pkts4096to8191octets_high: [u8; 0x20],
    pub ether_stats_pkts4096to8191octets_low: [u8; 0x20],
    pub ether_stats_pkts8192to10239octets_high: [u8; 0x20],
    pub ether_stats_pkts8192to10239octets_low: [u8; 0x20],
    pub reserved_at_540: [u8; 0x280],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eth_2863_cntrs_grp_data_layout_bits {
    pub if_in_octets_high: [u8; 0x20],
    pub if_in_octets_low: [u8; 0x20],
    pub if_in_ucast_pkts_high: [u8; 0x20],
    pub if_in_ucast_pkts_low: [u8; 0x20],
    pub if_in_discards_high: [u8; 0x20],
    pub if_in_discards_low: [u8; 0x20],
    pub if_in_errors_high: [u8; 0x20],
    pub if_in_errors_low: [u8; 0x20],
    pub if_in_unknown_protos_high: [u8; 0x20],
    pub if_in_unknown_protos_low: [u8; 0x20],
    pub if_out_octets_high: [u8; 0x20],
    pub if_out_octets_low: [u8; 0x20],
    pub if_out_ucast_pkts_high: [u8; 0x20],
    pub if_out_ucast_pkts_low: [u8; 0x20],
    pub if_out_discards_high: [u8; 0x20],
    pub if_out_discards_low: [u8; 0x20],
    pub if_out_errors_high: [u8; 0x20],
    pub if_out_errors_low: [u8; 0x20],
    pub if_in_multicast_pkts_high: [u8; 0x20],
    pub if_in_multicast_pkts_low: [u8; 0x20],
    pub if_in_broadcast_pkts_high: [u8; 0x20],
    pub if_in_broadcast_pkts_low: [u8; 0x20],
    pub if_out_multicast_pkts_high: [u8; 0x20],
    pub if_out_multicast_pkts_low: [u8; 0x20],
    pub if_out_broadcast_pkts_high: [u8; 0x20],
    pub if_out_broadcast_pkts_low: [u8; 0x20],
    pub reserved_at_340: [u8; 0x480],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eth_802_3_cntrs_grp_data_layout_bits {
    pub a_frames_transmitted_ok_high: [u8; 0x20],
    pub a_frames_transmitted_ok_low: [u8; 0x20],
    pub a_frames_received_ok_high: [u8; 0x20],
    pub a_frames_received_ok_low: [u8; 0x20],
    pub a_frame_check_sequence_errors_high: [u8; 0x20],
    pub a_frame_check_sequence_errors_low: [u8; 0x20],
    pub a_alignment_errors_high: [u8; 0x20],
    pub a_alignment_errors_low: [u8; 0x20],
    pub a_octets_transmitted_ok_high: [u8; 0x20],
    pub a_octets_transmitted_ok_low: [u8; 0x20],
    pub a_octets_received_ok_high: [u8; 0x20],
    pub a_octets_received_ok_low: [u8; 0x20],
    pub a_multicast_frames_xmitted_ok_high: [u8; 0x20],
    pub a_multicast_frames_xmitted_ok_low: [u8; 0x20],
    pub a_broadcast_frames_xmitted_ok_high: [u8; 0x20],
    pub a_broadcast_frames_xmitted_ok_low: [u8; 0x20],
    pub a_multicast_frames_received_ok_high: [u8; 0x20],
    pub a_multicast_frames_received_ok_low: [u8; 0x20],
    pub a_broadcast_frames_received_ok_high: [u8; 0x20],
    pub a_broadcast_frames_received_ok_low: [u8; 0x20],
    pub a_in_range_length_errors_high: [u8; 0x20],
    pub a_in_range_length_errors_low: [u8; 0x20],
    pub a_out_of_range_length_field_high: [u8; 0x20],
    pub a_out_of_range_length_field_low: [u8; 0x20],
    pub a_frame_too_long_errors_high: [u8; 0x20],
    pub a_frame_too_long_errors_low: [u8; 0x20],
    pub a_symbol_error_during_carrier_high: [u8; 0x20],
    pub a_symbol_error_during_carrier_low: [u8; 0x20],
    pub a_mac_control_frames_transmitted_high: [u8; 0x20],
    pub a_mac_control_frames_transmitted_low: [u8; 0x20],
    pub a_mac_control_frames_received_high: [u8; 0x20],
    pub a_mac_control_frames_received_low: [u8; 0x20],
    pub a_unsupported_opcodes_received_high: [u8; 0x20],
    pub a_unsupported_opcodes_received_low: [u8; 0x20],
    pub a_pause_mac_ctrl_frames_received_high: [u8; 0x20],
    pub a_pause_mac_ctrl_frames_received_low: [u8; 0x20],
    pub a_pause_mac_ctrl_frames_transmitted_high: [u8; 0x20],
    pub a_pause_mac_ctrl_frames_transmitted_low: [u8; 0x20],
    pub reserved_at_4c0: [u8; 0x300],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pcie_perf_cntrs_grp_data_layout_bits {
    pub life_time_counter_high: [u8; 0x20],
    pub life_time_counter_low: [u8; 0x20],
    pub rx_errors: [u8; 0x20],
    pub tx_errors: [u8; 0x20],
    pub l0_to_recovery_eieos: [u8; 0x20],
    pub l0_to_recovery_ts: [u8; 0x20],
    pub l0_to_recovery_framing: [u8; 0x20],
    pub l0_to_recovery_retrain: [u8; 0x20],
    pub crc_error_dllp: [u8; 0x20],
    pub crc_error_tlp: [u8; 0x20],
    pub tx_overflow_buffer_pkt_high: [u8; 0x20],
    pub tx_overflow_buffer_pkt_low: [u8; 0x20],
    pub outbound_stalled_reads: [u8; 0x20],
    pub outbound_stalled_writes: [u8; 0x20],
    pub outbound_stalled_reads_events: [u8; 0x20],
    pub outbound_stalled_writes_events: [u8; 0x20],
    pub reserved_at_200: [u8; 0x5c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cmd_inter_comp_event_bits {
    pub command_completion_vector: [u8; 0x20],
    pub reserved_at_20: [u8; 0xc0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_stall_vl_event_bits {
    pub reserved_at_0: [u8; 0x18],
    pub port_num: [u8; 0x1],
    pub reserved_at_19: [u8; 0x3],
    pub vl: [u8; 0x4],
    pub reserved_at_20: [u8; 0xa0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_db_bf_congestion_event_bits {
    pub event_subtype: [u8; 0x8],
    pub reserved_at_8: [u8; 0x8],
    pub congestion_level: [u8; 0x8],
    pub reserved_at_18: [u8; 0x8],
    pub reserved_at_20: [u8; 0xa0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_gpio_event_bits {
    pub reserved_at_0: [u8; 0x60],
    pub gpio_event_hi: [u8; 0x20],
    pub gpio_event_lo: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_port_state_change_event_bits {
    pub reserved_at_0: [u8; 0x40],
    pub port_num: [u8; 0x4],
    pub reserved_at_44: [u8; 0x1c],
    pub reserved_at_60: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dropped_packet_logged_bits {
    pub reserved_at_0: [u8; 0xe0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_nic_cap_reg_bits {
    pub reserved_at_0: [u8; 0x1a],
    pub vhca_icm_ctrl: [u8; 0x1],
    pub reserved_at_1b: [u8; 0x5],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_default_timeout_bits {
    pub to_multiplier: [u8; 0x3],
    pub reserved_at_3: [u8; 0x9],
    pub to_value: [u8; 0x14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dtor_reg_bits {
    pub reserved_at_0: [u8; 0x20],
    pub pcie_toggle_to: mlx5_ifc_default_timeout_bits,
    pub reserved_at_40: [u8; 0x60],
    pub health_poll_to: mlx5_ifc_default_timeout_bits,
    pub full_crdump_to: mlx5_ifc_default_timeout_bits,
    pub fw_reset_to: mlx5_ifc_default_timeout_bits,
    pub flush_on_err_to: mlx5_ifc_default_timeout_bits,
    pub pci_sync_update_to: mlx5_ifc_default_timeout_bits,
    pub tear_down_to: mlx5_ifc_default_timeout_bits,
    pub fsm_reactivate_to: mlx5_ifc_default_timeout_bits,
    pub reclaim_pages_to: mlx5_ifc_default_timeout_bits,
    pub reclaim_vfs_pages_to: mlx5_ifc_default_timeout_bits,
    pub reset_unload_to: mlx5_ifc_default_timeout_bits,
    pub reserved_at_1c0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_vhca_icm_ctrl_reg_bits {
    pub vhca_id_valid: [u8; 0x1],
    pub reserved_at_1: [u8; 0xf],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_20: [u8; 0xa0],
    pub cur_alloc_icm: [u8; 0x20],
    pub reserved_at_e0: [u8; 0x120],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cq_error_bits {
    pub reserved_at_0: [u8; 0x8],
    pub cqn: [u8; 0x18],
    pub reserved_at_20: [u8; 0x20],
    pub reserved_at_40: [u8; 0x18],
    pub syndrome: [u8; 0x8],
    pub reserved_at_60: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rdma_page_fault_event_bits {
    pub bytes_committed: [u8; 0x20],
    pub r_key: [u8; 0x20],
    pub reserved_at_40: [u8; 0x10],
    pub packet_len: [u8; 0x10],
    pub rdma_op_len: [u8; 0x20],
    pub rdma_va: [u8; 0x40],
    pub reserved_at_c0: [u8; 0x5],
    pub rdma: [u8; 0x1],
    pub write: [u8; 0x1],
    pub requestor: [u8; 0x1],
    pub qp_number: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_wqe_associated_page_fault_event_bits {
    pub bytes_committed: [u8; 0x20],
    pub reserved_at_20: [u8; 0x10],
    pub wqe_index: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub len: [u8; 0x10],
    pub reserved_at_60: [u8; 0x60],
    pub reserved_at_c0: [u8; 0x5],
    pub rdma: [u8; 0x1],
    pub write_read: [u8; 0x1],
    pub requestor: [u8; 0x1],
    pub qpn: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qp_events_bits {
    pub reserved_at_0: [u8; 0xa0],
    pub type: [u8; 0x8],
    pub reserved_at_a8: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x8],
    pub qpn_rqn_sqn: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dct_events_bits {
    pub reserved_at_0: [u8; 0xc0],
    pub reserved_at_c0: [u8; 0x8],
    pub dct_number: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_comp_event_bits {
    pub reserved_at_0: [u8; 0xc0],
    pub reserved_at_c0: [u8; 0x8],
    pub cq_number: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qpc_bits {
    pub state: [u8; 0x4],
    pub lag_tx_port_affinity: [u8; 0x4],
    pub st: [u8; 0x8],
    pub reserved_at_10: [u8; 0x2],
    pub isolate_vl_tc: [u8; 0x1],
    pub pm_state: [u8; 0x2],
    pub reserved_at_15: [u8; 0x1],
    pub req_e2e_credit_mode: [u8; 0x2],
    pub offload_type: [u8; 0x4],
    pub end_padding_mode: [u8; 0x2],
    pub reserved_at_1e: [u8; 0x2],
    pub wq_signature: [u8; 0x1],
    pub block_lb_mc: [u8; 0x1],
    pub atomic_like_write_en: [u8; 0x1],
    pub latency_sensitive: [u8; 0x1],
    pub reserved_at_24: [u8; 0x1],
    pub drain_sigerr: [u8; 0x1],
    pub reserved_at_26: [u8; 0x1],
    pub dp_ordering_force: [u8; 0x1],
    pub pd: [u8; 0x18],
    pub mtu: [u8; 0x3],
    pub log_msg_max: [u8; 0x5],
    pub reserved_at_48: [u8; 0x1],
    pub log_rq_size: [u8; 0x4],
    pub log_rq_stride: [u8; 0x3],
    pub no_sq: [u8; 0x1],
    pub log_sq_size: [u8; 0x4],
    pub reserved_at_55: [u8; 0x1],
    pub retry_mode: [u8; 0x2],
    pub ts_format: [u8; 0x2],
    pub reserved_at_5a: [u8; 0x1],
    pub rlky: [u8; 0x1],
    pub ulp_stateless_offload_mode: [u8; 0x4],
    pub counter_set_id: [u8; 0x8],
    pub uar_page: [u8; 0x18],
    pub reserved_at_80: [u8; 0x8],
    pub user_index: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x3],
    pub log_page_size: [u8; 0x5],
    pub remote_qpn: [u8; 0x18],
    pub primary_address_path: mlx5_ifc_ads_bits,
    pub secondary_address_path: mlx5_ifc_ads_bits,
    pub log_ack_req_freq: [u8; 0x4],
    pub reserved_at_384: [u8; 0x4],
    pub log_sra_max: [u8; 0x3],
    pub reserved_at_38b: [u8; 0x2],
    pub retry_count: [u8; 0x3],
    pub rnr_retry: [u8; 0x3],
    pub reserved_at_393: [u8; 0x1],
    pub fre: [u8; 0x1],
    pub cur_rnr_retry: [u8; 0x3],
    pub cur_retry_count: [u8; 0x3],
    pub reserved_at_39b: [u8; 0x5],
    pub reserved_at_3a0: [u8; 0x10],
    pub packet_pacing_rate_limit_index: [u8; 0x10],
    pub reserved_at_3c0: [u8; 0x8],
    pub next_send_psn: [u8; 0x18],
    pub reserved_at_3e0: [u8; 0x3],
    pub log_num_dci_stream_channels: [u8; 0x5],
    pub cqn_snd: [u8; 0x18],
    pub reserved_at_400: [u8; 0x3],
    pub log_num_dci_errored_streams: [u8; 0x5],
    pub deth_sqpn: [u8; 0x18],
    pub reserved_at_420: [u8; 0x20],
    pub reserved_at_440: [u8; 0x8],
    pub last_acked_psn: [u8; 0x18],
    pub reserved_at_460: [u8; 0x8],
    pub ssn: [u8; 0x18],
    pub reserved_at_480: [u8; 0x8],
    pub log_rra_max: [u8; 0x3],
    pub reserved_at_48b: [u8; 0x1],
    pub atomic_mode: [u8; 0x4],
    pub rre: [u8; 0x1],
    pub rwe: [u8; 0x1],
    pub rae: [u8; 0x1],
    pub reserved_at_493: [u8; 0x1],
    pub page_offset: [u8; 0x6],
    pub reserved_at_49a: [u8; 0x2],
    pub dp_ordering_1: [u8; 0x1],
    pub cd_slave_receive: [u8; 0x1],
    pub cd_slave_send: [u8; 0x1],
    pub cd_master: [u8; 0x1],
    pub reserved_at_4a0: [u8; 0x3],
    pub min_rnr_nak: [u8; 0x5],
    pub next_rcv_psn: [u8; 0x18],
    pub reserved_at_4c0: [u8; 0x8],
    pub xrcd: [u8; 0x18],
    pub reserved_at_4e0: [u8; 0x8],
    pub cqn_rcv: [u8; 0x18],
    pub dbr_addr: [u8; 0x40],
    pub q_key: [u8; 0x20],
    pub reserved_at_560: [u8; 0x5],
    pub rq_type: [u8; 0x3],
    pub srqn_rmpn_xrqn: [u8; 0x18],
    pub reserved_at_580: [u8; 0x8],
    pub rmsn: [u8; 0x18],
    pub hw_sq_wqebb_counter: [u8; 0x10],
    pub sw_sq_wqebb_counter: [u8; 0x10],
    pub hw_rq_counter: [u8; 0x20],
    pub sw_rq_counter: [u8; 0x20],
    pub reserved_at_600: [u8; 0x20],
    pub reserved_at_620: [u8; 0xf],
    pub cgs: [u8; 0x1],
    pub cs_req: [u8; 0x8],
    pub cs_res: [u8; 0x8],
    pub dc_access_key: [u8; 0x40],
    pub reserved_at_680: [u8; 0x3],
    pub dbr_umem_valid: [u8; 0x1],
    pub reserved_at_684: [u8; 0xbc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_roce_addr_layout_bits {
    pub source_l3_address: [u8; 16][0x8],
    pub reserved_at_80: [u8; 0x3],
    pub vlan_valid: [u8; 0x1],
    pub vlan_id: [u8; 0xc],
    pub source_mac_47_32: [u8; 0x10],
    pub source_mac_31_0: [u8; 0x20],
    pub reserved_at_c0: [u8; 0x14],
    pub roce_l3_type: [u8; 0x4],
    pub roce_version: [u8; 0x8],
    pub reserved_at_e0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_crypto_cap_bits {
    pub reserved_at_0: [u8; 0x3],
    pub synchronize_dek: [u8; 0x1],
    pub int_kek_manual: [u8; 0x1],
    pub int_kek_auto: [u8; 0x1],
    pub reserved_at_6: [u8; 0x1a],
    pub reserved_at_20: [u8; 0x3],
    pub log_dek_max_alloc: [u8; 0x5],
    pub reserved_at_28: [u8; 0x3],
    pub log_max_num_deks: [u8; 0x5],
    pub reserved_at_30: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x3],
    pub log_dek_granularity: [u8; 0x5],
    pub reserved_at_68: [u8; 0x3],
    pub log_max_num_int_kek: [u8; 0x5],
    pub sw_wrapped_dek: [u8; 0x10],
    pub reserved_at_80: [u8; 0x780],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_shampo_cap_bits {
    pub reserved_at_0: [u8; 0x3],
    pub shampo_log_max_reservation_size: [u8; 0x5],
    pub reserved_at_8: [u8; 0x3],
    pub shampo_log_min_reservation_size: [u8; 0x5],
    pub shampo_min_mss_size: [u8; 0x10],
    pub shampo_header_split: [u8; 0x1],
    pub shampo_header_split_data_merge: [u8; 0x1],
    pub reserved_at_22: [u8; 0x1],
    pub shampo_log_max_headers_entry_size: [u8; 0x5],
    pub reserved_at_28: [u8; 0x18],
    pub reserved_at_40: [u8; 0x7c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_hca_cap_union_bits {
    pub cmd_hca_cap: mlx5_ifc_cmd_hca_cap_bits,
    pub cmd_hca_cap_2: mlx5_ifc_cmd_hca_cap_2_bits,
    pub odp_cap: mlx5_ifc_odp_cap_bits,
    pub atomic_caps: mlx5_ifc_atomic_caps_bits,
    pub roce_cap: mlx5_ifc_roce_cap_bits,
    pub per_protocol_networking_offload_caps: mlx5_ifc_per_protocol_networking_offload_caps_bits,
    pub flow_table_nic_cap: mlx5_ifc_flow_table_nic_cap_bits,
    pub flow_table_eswitch_cap: mlx5_ifc_flow_table_eswitch_cap_bits,
    pub wqe_based_flow_table_cap: mlx5_ifc_wqe_based_flow_table_cap_bits,
    pub e_switch_cap: mlx5_ifc_e_switch_cap_bits,
    pub port_selection_cap: mlx5_ifc_port_selection_cap_bits,
    pub qos_cap: mlx5_ifc_qos_cap_bits,
    pub debug_cap: mlx5_ifc_debug_cap_bits,
    pub fpga_cap: mlx5_ifc_fpga_cap_bits,
    pub tls_cap: mlx5_ifc_tls_cap_bits,
    pub device_mem_cap: mlx5_ifc_device_mem_cap_bits,
    pub virtio_emulation_cap: mlx5_ifc_virtio_emulation_cap_bits,
    pub tlp_dev_emu_capabilities: mlx5_ifc_tlp_dev_emu_capabilities_bits,
    pub macsec_cap: mlx5_ifc_macsec_cap_bits,
    pub crypto_cap: mlx5_ifc_crypto_cap_bits,
    pub ipsec_cap: mlx5_ifc_ipsec_cap_bits,
    pub psp_cap: mlx5_ifc_psp_cap_bits,
    pub reserved_at_0: [u8; 0x8000],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_vlan_bits {
    pub ethtype: [u8; 0x10],
    pub prio: [u8; 0x3],
    pub cfi: [u8; 0x1],
    pub vid: [u8; 0xc],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_exe_aso_ctrl_flow_meter_bits {
    pub return_reg_id: [u8; 0x4],
    pub aso_type: [u8; 0x4],
    pub reserved_at_8: [u8; 0x14],
    pub action: [u8; 0x1],
    pub init_color: [u8; 0x2],
    pub meter_id: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_exe_aso_ctrl {
    pub exe_aso_ctrl_flow_meter: mlx5_ifc_exe_aso_ctrl_flow_meter_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_execute_aso_bits {
    pub valid: [u8; 0x1],
    pub reserved_at_1: [u8; 0x7],
    pub aso_object_id: [u8; 0x18],
    pub exe_aso_ctrl: mlx5_ifc_exe_aso_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_context_bits {
    pub push_vlan: mlx5_ifc_vlan_bits,
    pub group_id: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub flow_tag: [u8; 0x18],
    pub reserved_at_60: [u8; 0x10],
    pub action: [u8; 0x10],
    pub extended_destination: [u8; 0x1],
    pub uplink_hairpin_en: [u8; 0x1],
    pub flow_source: [u8; 0x2],
    pub encrypt_decrypt_type: [u8; 0x4],
    pub destination_list_size: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x8],
    pub flow_counter_list_size: [u8; 0x18],
    pub packet_reformat_id: [u8; 0x20],
    pub modify_header_id: [u8; 0x20],
    pub push_vlan_2: mlx5_ifc_vlan_bits,
    pub encrypt_decrypt_obj_id: [u8; 0x20],
    pub reserved_at_140: [u8; 0xc0],
    pub match_value: mlx5_ifc_fte_match_param_bits,
    pub execute_aso: [mlx5_ifc_execute_aso_bits; 4],
    pub reserved_at_1300: [u8; 0x500],
    pub destination: [mlx5_ifc_dest_format_flow_counter_list_auto_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_xrc_srqc_bits {
    pub state: [u8; 0x4],
    pub log_xrc_srq_size: [u8; 0x4],
    pub reserved_at_8: [u8; 0x18],
    pub wq_signature: [u8; 0x1],
    pub cont_srq: [u8; 0x1],
    pub reserved_at_22: [u8; 0x1],
    pub rlky: [u8; 0x1],
    pub basic_cyclic_rcv_wqe: [u8; 0x1],
    pub log_rq_stride: [u8; 0x3],
    pub xrcd: [u8; 0x18],
    pub page_offset: [u8; 0x6],
    pub reserved_at_46: [u8; 0x1],
    pub dbr_umem_valid: [u8; 0x1],
    pub cqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub user_index_equal_xrc_srqn: [u8; 0x1],
    pub reserved_at_81: [u8; 0x1],
    pub log_page_size: [u8; 0x6],
    pub user_index: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x20],
    pub reserved_at_c0: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub lwm: [u8; 0x10],
    pub wqe_cnt: [u8; 0x10],
    pub reserved_at_100: [u8; 0x40],
    pub db_record_addr_h: [u8; 0x20],
    pub db_record_addr_l: [u8; 0x1e],
    pub reserved_at_17e: [u8; 0x2],
    pub reserved_at_180: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_vnic_diagnostic_statistics_bits {
    pub counter_error_queues: [u8; 0x20],
    pub total_error_queues: [u8; 0x20],
    pub send_queue_priority_update_flow: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
    pub nic_receive_steering_discard: [u8; 0x40],
    pub receive_discard_vport_down: [u8; 0x40],
    pub transmit_discard_vport_down: [u8; 0x40],
    pub async_eq_overrun: [u8; 0x20],
    pub comp_eq_overrun: [u8; 0x20],
    pub reserved_at_180: [u8; 0x20],
    pub invalid_command: [u8; 0x20],
    pub quota_exceeded_command: [u8; 0x20],
    pub internal_rq_out_of_buffer: [u8; 0x20],
    pub cq_overrun: [u8; 0x20],
    pub eth_wqe_too_small: [u8; 0x20],
    pub reserved_at_220: [u8; 0xc0],
    pub generated_pkt_steering_fail: [u8; 0x40],
    pub handled_pkt_steering_fail: [u8; 0x40],
    pub bar_uar_access: [u8; 0x20],
    pub odp_local_triggered_page_fault: [u8; 0x20],
    pub odp_remote_triggered_page_fault: [u8; 0x20],
    pub reserved_at_3c0: [u8; 0xc20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_traffic_counter_bits {
    pub packets: [u8; 0x40],
    pub octets: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tisc_bits {
    pub strict_lag_tx_port_affinity: [u8; 0x1],
    pub tls_en: [u8; 0x1],
    pub reserved_at_2: [u8; 0x2],
    pub lag_tx_port_affinity: [u8; 0x04],
    pub reserved_at_8: [u8; 0x4],
    pub prio: [u8; 0x4],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x100],
    pub reserved_at_120: [u8; 0x8],
    pub transport_domain: [u8; 0x18],
    pub reserved_at_140: [u8; 0x8],
    pub underlay_qpn: [u8; 0x18],
    pub reserved_at_160: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub reserved_at_180: [u8; 0x380],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tirc_bits {
    pub reserved_at_0: [u8; 0x20],
    pub disp_type: [u8; 0x4],
    pub tls_en: [u8; 0x1],
    pub reserved_at_25: [u8; 0x1b],
    pub reserved_at_40: [u8; 0x40],
    pub reserved_at_80: [u8; 0x4],
    pub lro_timeout_period_usecs: [u8; 0x10],
    pub packet_merge_mask: [u8; 0x4],
    pub lro_max_ip_payload_size: [u8; 0x8],
    pub reserved_at_a0: [u8; 0x40],
    pub reserved_at_e0: [u8; 0x8],
    pub inline_rqn: [u8; 0x18],
    pub rx_hash_symmetric: [u8; 0x1],
    pub reserved_at_101: [u8; 0x1],
    pub tunneled_offload_en: [u8; 0x1],
    pub reserved_at_103: [u8; 0x5],
    pub indirect_table: [u8; 0x18],
    pub rx_hash_fn: [u8; 0x4],
    pub reserved_at_124: [u8; 0x2],
    pub self_lb_block: [u8; 0x2],
    pub transport_domain: [u8; 0x18],
    pub rx_hash_toeplitz_key: [u8; 10][0x20],
    pub rx_hash_field_selector_outer: mlx5_ifc_rx_hash_field_select_bits,
    pub rx_hash_field_selector_inner: mlx5_ifc_rx_hash_field_select_bits,
    pub reserved_at_2c0: [u8; 0x4c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_srqc_bits {
    pub state: [u8; 0x4],
    pub log_srq_size: [u8; 0x4],
    pub reserved_at_8: [u8; 0x18],
    pub wq_signature: [u8; 0x1],
    pub cont_srq: [u8; 0x1],
    pub reserved_at_22: [u8; 0x1],
    pub rlky: [u8; 0x1],
    pub reserved_at_24: [u8; 0x1],
    pub log_rq_stride: [u8; 0x3],
    pub xrcd: [u8; 0x18],
    pub page_offset: [u8; 0x6],
    pub reserved_at_46: [u8; 0x2],
    pub cqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub reserved_at_80: [u8; 0x2],
    pub log_page_size: [u8; 0x6],
    pub reserved_at_88: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x20],
    pub reserved_at_c0: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub lwm: [u8; 0x10],
    pub wqe_cnt: [u8; 0x10],
    pub reserved_at_100: [u8; 0x40],
    pub dbr_addr: [u8; 0x40],
    pub reserved_at_180: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sqc_bits {
    pub rlky: [u8; 0x1],
    pub cd_master: [u8; 0x1],
    pub fre: [u8; 0x1],
    pub flush_in_error_en: [u8; 0x1],
    pub allow_multi_pkt_send_wqe: [u8; 0x1],
    pub min_wqe_inline_mode: [u8; 0x3],
    pub state: [u8; 0x4],
    pub reg_umr: [u8; 0x1],
    pub allow_swp: [u8; 0x1],
    pub hairpin: [u8; 0x1],
    pub non_wire: [u8; 0x1],
    pub reserved_at_10: [u8; 0xa],
    pub ts_format: [u8; 0x2],
    pub reserved_at_1c: [u8; 0x4],
    pub reserved_at_20: [u8; 0x8],
    pub user_index: [u8; 0x18],
    pub reserved_at_40: [u8; 0x8],
    pub cqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x8],
    pub hairpin_peer_rq: [u8; 0x18],
    pub reserved_at_80: [u8; 0x10],
    pub hairpin_peer_vhca: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x20],
    pub reserved_at_c0: [u8; 0x8],
    pub ts_cqe_to_dest_cqn: [u8; 0x18],
    pub reserved_at_e0: [u8; 0x10],
    pub packet_pacing_rate_limit_index: [u8; 0x10],
    pub tis_lst_sz: [u8; 0x10],
    pub qos_queue_group_id: [u8; 0x10],
    pub reserved_at_120: [u8; 0x40],
    pub reserved_at_160: [u8; 0x8],
    pub tis_num_0: [u8; 0x18],
    pub wq: mlx5_ifc_wq_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tsar_element_bits {
    pub traffic_class: [u8; 0x4],
    pub reserved_at_4: [u8; 0x4],
    pub tsar_type: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_vport_element_bits {
    pub reserved_at_0: [u8; 0x4],
    pub eswitch_owner_vhca_id_valid: [u8; 0x1],
    pub eswitch_owner_vhca_id: [u8; 0xb],
    pub vport_number: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_vport_tc_element_bits {
    pub traffic_class: [u8; 0x4],
    pub eswitch_owner_vhca_id_valid: [u8; 0x1],
    pub eswitch_owner_vhca_id: [u8; 0xb],
    pub vport_number: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_element_attributes_bits {
    pub tsar: mlx5_ifc_tsar_element_bits,
    pub vport: mlx5_ifc_vport_element_bits,
    pub vport_tc: mlx5_ifc_vport_tc_element_bits,
    pub reserved_at_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_scheduling_context_bits {
    pub element_type: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub element_attributes: mlx5_ifc_element_attributes_bits,
    pub parent_element_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0x40],
    pub bw_share: [u8; 0x20],
    pub max_average_bw: [u8; 0x20],
    pub max_bw_obj_id: [u8; 0x20],
    pub reserved_at_100: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rqtc_bits {
    pub reserved_at_0: [u8; 0xa0],
    pub reserved_at_a0: [u8; 0x5],
    pub list_q_type: [u8; 0x3],
    pub reserved_at_a8: [u8; 0x8],
    pub rqt_max_size: [u8; 0x10],
    pub rq_vhca_id_format: [u8; 0x1],
    pub reserved_at_c1: [u8; 0xf],
    pub rqt_actual_size: [u8; 0x10],
    pub reserved_at_e0: [u8; 0x6a0],
    pub rq_num): DECLARE_FLEX_ARRAY(struct mlx5_ifc_rq_num_bits,,
    pub rq_vhca): DECLARE_FLEX_ARRAY(struct mlx5_ifc_rq_vhca_bits,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rqc_bits {
    pub rlky: [u8; 0x1],
    pub delay_drop_en: [u8; 0x1],
    pub scatter_fcs: [u8; 0x1],
    pub vsd: [u8; 0x1],
    pub mem_rq_type: [u8; 0x4],
    pub state: [u8; 0x4],
    pub reserved_at_c: [u8; 0x1],
    pub flush_in_error_en: [u8; 0x1],
    pub hairpin: [u8; 0x1],
    pub reserved_at_f: [u8; 0xb],
    pub ts_format: [u8; 0x2],
    pub reserved_at_1c: [u8; 0x4],
    pub reserved_at_20: [u8; 0x8],
    pub user_index: [u8; 0x18],
    pub reserved_at_40: [u8; 0x8],
    pub cqn: [u8; 0x18],
    pub counter_set_id: [u8; 0x8],
    pub reserved_at_68: [u8; 0x18],
    pub reserved_at_80: [u8; 0x8],
    pub rmpn: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x8],
    pub hairpin_peer_sq: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x10],
    pub hairpin_peer_vhca: [u8; 0x10],
    pub reserved_at_e0: [u8; 0x46],
    pub shampo_no_match_alignment_granularity: [u8; 0x2],
    pub reserved_at_128: [u8; 0x6],
    pub shampo_match_criteria_type: [u8; 0x2],
    pub reservation_timeout: [u8; 0x10],
    pub reserved_at_140: [u8; 0x40],
    pub wq: mlx5_ifc_wq_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rmpc_bits {
    pub reserved_at_0: [u8; 0x8],
    pub state: [u8; 0x4],
    pub reserved_at_c: [u8; 0x14],
    pub basic_cyclic_rcv_wqe: [u8; 0x1],
    pub reserved_at_21: [u8; 0x1f],
    pub reserved_at_40: [u8; 0x140],
    pub wq: mlx5_ifc_wq_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_nic_vport_context_bits {
    pub reserved_at_0: [u8; 0x5],
    pub min_wqe_inline_mode: [u8; 0x3],
    pub reserved_at_8: [u8; 0x15],
    pub disable_mc_local_lb: [u8; 0x1],
    pub disable_uc_local_lb: [u8; 0x1],
    pub roce_en: [u8; 0x1],
    pub arm_change_event: [u8; 0x1],
    pub reserved_at_21: [u8; 0x1a],
    pub event_on_mtu: [u8; 0x1],
    pub event_on_promisc_change: [u8; 0x1],
    pub event_on_vlan_change: [u8; 0x1],
    pub event_on_mc_address_change: [u8; 0x1],
    pub event_on_uc_address_change: [u8; 0x1],
    pub vhca_id_type: [u8; 0x1],
    pub reserved_at_41: [u8; 0xb],
    pub affiliation_criteria: [u8; 0x4],
    pub affiliated_vhca_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0xa0],
    pub reserved_at_100: [u8; 0x1],
    pub sd_group: [u8; 0x3],
    pub reserved_at_104: [u8; 0x4],
    pub sd_group_size: [u8; 0x8],
    pub reserved_at_110: [u8; 0x10],
    pub reserved_at_120: [u8; 0x10],
    pub mtu: [u8; 0x10],
    pub system_image_guid: [u8; 0x40],
    pub port_guid: [u8; 0x40],
    pub node_guid: [u8; 0x40],
    pub reserved_at_200: [u8; 0x140],
    pub qkey_violation_counter: [u8; 0x10],
    pub reserved_at_350: [u8; 0x430],
    pub promisc_uc: [u8; 0x1],
    pub promisc_mc: [u8; 0x1],
    pub promisc_all: [u8; 0x1],
    pub reserved_at_783: [u8; 0x2],
    pub allowed_list_type: [u8; 0x3],
    pub reserved_at_788: [u8; 0x8],
    pub allowed_list_size: [u8; 0x10],
    pub permanent_address: mlx5_ifc_mac_address_layout_bits,
    pub reserved_at_7e0: [u8; 0x20],
    pub current_uc_mac_address: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mkc_bits {
    pub reserved_at_0: [u8; 0x1],
    pub free: [u8; 0x1],
    pub reserved_at_2: [u8; 0x1],
    pub access_mode_4_2: [u8; 0x3],
    pub reserved_at_6: [u8; 0x7],
    pub relaxed_ordering_write: [u8; 0x1],
    pub reserved_at_e: [u8; 0x1],
    pub small_fence_on_rdma_read_response: [u8; 0x1],
    pub umr_en: [u8; 0x1],
    pub a: [u8; 0x1],
    pub rw: [u8; 0x1],
    pub rr: [u8; 0x1],
    pub lw: [u8; 0x1],
    pub lr: [u8; 0x1],
    pub access_mode_1_0: [u8; 0x2],
    pub reserved_at_18: [u8; 0x2],
    pub ma_translation_mode: [u8; 0x2],
    pub reserved_at_1c: [u8; 0x4],
    pub qpn: [u8; 0x18],
    pub mkey_7_0: [u8; 0x8],
    pub reserved_at_40: [u8; 0x20],
    pub length64: [u8; 0x1],
    pub bsf_en: [u8; 0x1],
    pub sync_umr: [u8; 0x1],
    pub reserved_at_63: [u8; 0x2],
    pub expected_sigerr_count: [u8; 0x1],
    pub reserved_at_66: [u8; 0x1],
    pub en_rinval: [u8; 0x1],
    pub pd: [u8; 0x18],
    pub start_addr: [u8; 0x40],
    pub len: [u8; 0x40],
    pub bsf_octword_size: [u8; 0x20],
    pub reserved_at_120: [u8; 0x60],
    pub crossing_target_vhca_id: [u8; 0x10],
    pub reserved_at_190: [u8; 0x10],
    pub translations_octword_size: [u8; 0x20],
    pub reserved_at_1c0: [u8; 0x19],
    pub relaxed_ordering_read: [u8; 0x1],
    pub log_page_size: [u8; 0x6],
    pub reserved_at_1e0: [u8; 0x5],
    pub pcie_tph_en: [u8; 0x1],
    pub pcie_tph_ph: [u8; 0x2],
    pub pcie_tph_steering_tag_index: [u8; 0x8],
    pub reserved_at_1f0: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pkey_bits {
    pub reserved_at_0: [u8; 0x10],
    pub pkey: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_array128_auto_bits {
    pub array128_auto: [u8; 16][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_hca_vport_context_bits {
    pub field_select: [u8; 0x20],
    pub reserved_at_20: [u8; 0xe0],
    pub sm_virt_aware: [u8; 0x1],
    pub has_smi: [u8; 0x1],
    pub has_raw: [u8; 0x1],
    pub grh_required: [u8; 0x1],
    pub reserved_at_104: [u8; 0x4],
    pub num_port_plane: [u8; 0x8],
    pub port_physical_state: [u8; 0x4],
    pub vport_state_policy: [u8; 0x4],
    pub port_state: [u8; 0x4],
    pub vport_state: [u8; 0x4],
    pub reserved_at_120: [u8; 0x20],
    pub system_image_guid: [u8; 0x40],
    pub port_guid: [u8; 0x40],
    pub node_guid: [u8; 0x40],
    pub cap_mask1: [u8; 0x20],
    pub cap_mask1_field_select: [u8; 0x20],
    pub cap_mask2: [u8; 0x20],
    pub cap_mask2_field_select: [u8; 0x20],
    pub reserved_at_280: [u8; 0x80],
    pub lid: [u8; 0x10],
    pub reserved_at_310: [u8; 0x4],
    pub init_type_reply: [u8; 0x4],
    pub lmc: [u8; 0x3],
    pub subnet_timeout: [u8; 0x5],
    pub sm_lid: [u8; 0x10],
    pub sm_sl: [u8; 0x4],
    pub reserved_at_334: [u8; 0xc],
    pub qkey_violation_counter: [u8; 0x10],
    pub pkey_violation_counter: [u8; 0x10],
    pub reserved_at_360: [u8; 0xca0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_esw_vport_context_bits {
    pub fdb_to_vport_reg_c: [u8; 0x1],
    pub reserved_at_1: [u8; 0x2],
    pub vport_svlan_strip: [u8; 0x1],
    pub vport_cvlan_strip: [u8; 0x1],
    pub vport_svlan_insert: [u8; 0x1],
    pub vport_cvlan_insert: [u8; 0x2],
    pub fdb_to_vport_reg_c_id: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x20],
    pub svlan_cfi: [u8; 0x1],
    pub svlan_pcp: [u8; 0x3],
    pub svlan_id: [u8; 0xc],
    pub cvlan_cfi: [u8; 0x1],
    pub cvlan_pcp: [u8; 0x3],
    pub cvlan_id: [u8; 0xc],
    pub reserved_at_60: [u8; 0x720],
    pub sw_steering_vport_icm_address_rx: [u8; 0x40],
    pub sw_steering_vport_icm_address_tx: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eqc_bits {
    pub status: [u8; 0x4],
    pub reserved_at_4: [u8; 0x9],
    pub ec: [u8; 0x1],
    pub oi: [u8; 0x1],
    pub reserved_at_f: [u8; 0x5],
    pub st: [u8; 0x4],
    pub reserved_at_18: [u8; 0x8],
    pub reserved_at_20: [u8; 0x20],
    pub reserved_at_40: [u8; 0x14],
    pub page_offset: [u8; 0x6],
    pub reserved_at_5a: [u8; 0x6],
    pub reserved_at_60: [u8; 0x3],
    pub log_eq_size: [u8; 0x5],
    pub uar_page: [u8; 0x18],
    pub reserved_at_80: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x14],
    pub intr: [u8; 0xc],
    pub reserved_at_c0: [u8; 0x3],
    pub log_page_size: [u8; 0x5],
    pub reserved_at_c8: [u8; 0x18],
    pub reserved_at_e0: [u8; 0x60],
    pub reserved_at_140: [u8; 0x8],
    pub consumer_counter: [u8; 0x18],
    pub reserved_at_160: [u8; 0x8],
    pub producer_counter: [u8; 0x18],
    pub reserved_at_180: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dctc_bits {
    pub reserved_at_0: [u8; 0x4],
    pub state: [u8; 0x4],
    pub reserved_at_8: [u8; 0x18],
    pub reserved_at_20: [u8; 0x7],
    pub dp_ordering_force: [u8; 0x1],
    pub user_index: [u8; 0x18],
    pub reserved_at_40: [u8; 0x8],
    pub cqn: [u8; 0x18],
    pub counter_set_id: [u8; 0x8],
    pub atomic_mode: [u8; 0x4],
    pub rre: [u8; 0x1],
    pub rwe: [u8; 0x1],
    pub rae: [u8; 0x1],
    pub atomic_like_write_en: [u8; 0x1],
    pub latency_sensitive: [u8; 0x1],
    pub rlky: [u8; 0x1],
    pub free_ar: [u8; 0x1],
    pub reserved_at_73: [u8; 0x1],
    pub dp_ordering_1: [u8; 0x1],
    pub reserved_at_75: [u8; 0xb],
    pub reserved_at_80: [u8; 0x8],
    pub cs_res: [u8; 0x8],
    pub reserved_at_90: [u8; 0x3],
    pub min_rnr_nak: [u8; 0x5],
    pub reserved_at_98: [u8; 0x8],
    pub reserved_at_a0: [u8; 0x8],
    pub srqn_xrqn: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub tclass: [u8; 0x8],
    pub reserved_at_e8: [u8; 0x4],
    pub flow_label: [u8; 0x14],
    pub dc_access_key: [u8; 0x40],
    pub reserved_at_140: [u8; 0x5],
    pub mtu: [u8; 0x3],
    pub port: [u8; 0x8],
    pub pkey_index: [u8; 0x10],
    pub reserved_at_160: [u8; 0x8],
    pub my_addr_index: [u8; 0x8],
    pub reserved_at_170: [u8; 0x8],
    pub hop_limit: [u8; 0x8],
    pub dc_access_key_violation_count: [u8; 0x20],
    pub reserved_at_1a0: [u8; 0x14],
    pub dei_cfi: [u8; 0x1],
    pub eth_prio: [u8; 0x3],
    pub ecn: [u8; 0x2],
    pub dscp: [u8; 0x6],
    pub reserved_at_1c0: [u8; 0x20],
    pub ece: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_cq_period_mode {
    MLX5_CQ_PERIOD_MODE_START_FROM_EQE = 0x0,
    MLX5_CQ_PERIOD_MODE_START_FROM_CQE = 0x1,
    MLX5_CQ_PERIOD_NUM_MODES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cqc_bits {
    pub status: [u8; 0x4],
    pub reserved_at_4: [u8; 0x2],
    pub dbr_umem_valid: [u8; 0x1],
    pub apu_cq: [u8; 0x1],
    pub cqe_sz: [u8; 0x3],
    pub cc: [u8; 0x1],
    pub reserved_at_c: [u8; 0x1],
    pub scqe_break_moderation_en: [u8; 0x1],
    pub oi: [u8; 0x1],
    pub cq_period_mode: [u8; 0x2],
    pub cqe_comp_en: [u8; 0x1],
    pub mini_cqe_res_format: [u8; 0x2],
    pub st: [u8; 0x4],
    pub reserved_at_18: [u8; 0x6],
    pub cqe_compression_layout: [u8; 0x2],
    pub reserved_at_20: [u8; 0x20],
    pub reserved_at_40: [u8; 0x14],
    pub page_offset: [u8; 0x6],
    pub reserved_at_5a: [u8; 0x6],
    pub reserved_at_60: [u8; 0x3],
    pub log_cq_size: [u8; 0x5],
    pub uar_page: [u8; 0x18],
    pub reserved_at_80: [u8; 0x4],
    pub cq_period: [u8; 0xc],
    pub cq_max_count: [u8; 0x10],
    pub c_eqn_or_apu_element: [u8; 0x20],
    pub reserved_at_c0: [u8; 0x3],
    pub log_page_size: [u8; 0x5],
    pub reserved_at_c8: [u8; 0x18],
    pub reserved_at_e0: [u8; 0x20],
    pub reserved_at_100: [u8; 0x8],
    pub last_notified_index: [u8; 0x18],
    pub reserved_at_120: [u8; 0x8],
    pub last_solicit_index: [u8; 0x18],
    pub reserved_at_140: [u8; 0x8],
    pub consumer_counter: [u8; 0x18],
    pub reserved_at_160: [u8; 0x8],
    pub producer_counter: [u8; 0x18],
    pub reserved_at_180: [u8; 0x40],
    pub dbr_addr: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_cong_control_roce_ecn_auto_bits {
    pub cong_control_802_1qau_rp: mlx5_ifc_cong_control_802_1qau_rp_bits,
    pub cong_control_r_roce_ecn_rp: mlx5_ifc_cong_control_r_roce_ecn_rp_bits,
    pub cong_control_r_roce_ecn_np: mlx5_ifc_cong_control_r_roce_ecn_np_bits,
    pub cong_control_r_roce_general: mlx5_ifc_cong_control_r_roce_general_bits,
    pub reserved_at_0: [u8; 0x800],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_adapter_param_block_bits {
    pub reserved_at_0: [u8; 0xc0],
    pub reserved_at_c0: [u8; 0x8],
    pub ieee_vendor_id: [u8; 0x18],
    pub reserved_at_e0: [u8; 0x10],
    pub vsd_vendor_id: [u8; 0x10],
    pub vsd: [u8; 208][0x8],
    pub vsd_contd_psid: [u8; 16][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tag_matching_topology_context_bits {
    pub log_matching_list_sz: [u8; 0x4],
    pub reserved_at_4: [u8; 0xc],
    pub append_next_index: [u8; 0x10],
    pub sw_phase_cnt: [u8; 0x10],
    pub hw_phase_cnt: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_xrqc_bits {
    pub state: [u8; 0x4],
    pub rlkey: [u8; 0x1],
    pub reserved_at_5: [u8; 0xf],
    pub topology: [u8; 0x4],
    pub reserved_at_18: [u8; 0x4],
    pub offload: [u8; 0x4],
    pub reserved_at_20: [u8; 0x8],
    pub user_index: [u8; 0x18],
    pub reserved_at_40: [u8; 0x8],
    pub cqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0xa0],
    pub tag_matching_topology_context: mlx5_ifc_tag_matching_topology_context_bits,
    pub reserved_at_180: [u8; 0x280],
    pub wq: mlx5_ifc_wq_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_modify_field_select_resize_field_select_auto_bits {
    pub modify_field_select: mlx5_ifc_modify_field_select_bits,
    pub resize_field_select: mlx5_ifc_resize_field_select_bits,
    pub reserved_at_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_field_select_802_1_r_roce_auto_bits {
    pub field_select_802_1qau_rp: mlx5_ifc_field_select_802_1qau_rp_bits,
    pub field_select_r_roce_rp: mlx5_ifc_field_select_r_roce_rp_bits,
    pub field_select_r_roce_np: mlx5_ifc_field_select_r_roce_np_bits,
    pub reserved_at_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rs_histogram_cntrs_bits {
    pub hist: [u8; 16][0x40],
    pub reserved_at_400: [u8; 0x2c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_eth_cntrs_grp_data_layout_auto_bits {
    pub eth_802_3_cntrs_grp_data_layout: mlx5_ifc_eth_802_3_cntrs_grp_data_layout_bits,
    pub eth_2863_cntrs_grp_data_layout: mlx5_ifc_eth_2863_cntrs_grp_data_layout_bits,
    pub eth_2819_cntrs_grp_data_layout: mlx5_ifc_eth_2819_cntrs_grp_data_layout_bits,
    pub eth_3635_cntrs_grp_data_layout: mlx5_ifc_eth_3635_cntrs_grp_data_layout_bits,
    pub eth_extended_cntrs_grp_data_layout: mlx5_ifc_eth_extended_cntrs_grp_data_layout_bits,
    pub eth_per_prio_grp_data_layout: mlx5_ifc_eth_per_prio_grp_data_layout_bits,
    pub eth_per_tc_prio_grp_data_layout: mlx5_ifc_eth_per_tc_prio_grp_data_layout_bits,
    pub eth_per_tc_congest_prio_grp_data_layout: mlx5_ifc_eth_per_tc_congest_prio_grp_data_layout_bits,
    pub ib_port_cntrs_grp_data_layout: mlx5_ifc_ib_port_cntrs_grp_data_layout_bits,
    pub ib_ext_port_cntrs_grp_data_layout: mlx5_ifc_ib_ext_port_cntrs_grp_data_layout_bits,
    pub phys_layer_cntrs: mlx5_ifc_phys_layer_cntrs_bits,
    pub phys_layer_statistical_cntrs: mlx5_ifc_phys_layer_statistical_cntrs_bits,
    pub phys_layer_recovery_cntrs: mlx5_ifc_phys_layer_recovery_cntrs_bits,
    pub rs_histogram_cntrs: mlx5_ifc_rs_histogram_cntrs_bits,
    pub reserved_at_0: [u8; 0x7c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_pcie_cntrs_grp_data_layout_auto_bits {
    pub pcie_perf_cntrs_grp_data_layout: mlx5_ifc_pcie_perf_cntrs_grp_data_layout_bits,
    pub reserved_at_0: [u8; 0x7c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_event_auto_bits {
    pub comp_event: mlx5_ifc_comp_event_bits,
    pub dct_events: mlx5_ifc_dct_events_bits,
    pub qp_events: mlx5_ifc_qp_events_bits,
    pub wqe_associated_page_fault_event: mlx5_ifc_wqe_associated_page_fault_event_bits,
    pub rdma_page_fault_event: mlx5_ifc_rdma_page_fault_event_bits,
    pub cq_error: mlx5_ifc_cq_error_bits,
    pub dropped_packet_logged: mlx5_ifc_dropped_packet_logged_bits,
    pub port_state_change_event: mlx5_ifc_port_state_change_event_bits,
    pub gpio_event: mlx5_ifc_gpio_event_bits,
    pub db_bf_congestion_event: mlx5_ifc_db_bf_congestion_event_bits,
    pub stall_vl_event: mlx5_ifc_stall_vl_event_bits,
    pub cmd_inter_comp_event: mlx5_ifc_cmd_inter_comp_event_bits,
    pub reserved_at_0: [u8; 0xe0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_health_buffer_bits {
    pub reserved_at_0: [u8; 0x100],
    pub assert_existptr: [u8; 0x20],
    pub assert_callra: [u8; 0x20],
    pub reserved_at_140: [u8; 0x20],
    pub time: [u8; 0x20],
    pub fw_version: [u8; 0x20],
    pub hw_id: [u8; 0x20],
    pub rfr: [u8; 0x1],
    pub reserved_at_1c1: [u8; 0x3],
    pub valid: [u8; 0x1],
    pub severity: [u8; 0x3],
    pub reserved_at_1c8: [u8; 0x18],
    pub irisc_index: [u8; 0x8],
    pub synd: [u8; 0x8],
    pub ext_synd: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_register_loopback_control_bits {
    pub no_lb: [u8; 0x1],
    pub reserved_at_1: [u8; 0x7],
    pub port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_teardown_hca_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x3f],
    pub state: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_teardown_hca_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub profile: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sqerr2rts_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sqerr2rts_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub opt_param_mask: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x20],
    pub qpc: mlx5_ifc_qpc_bits,
    pub reserved_at_800: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sqd2rts_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sqd2rts_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub opt_param_mask: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x20],
    pub qpc: mlx5_ifc_qpc_bits,
    pub reserved_at_800: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_roce_address_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_roce_address_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub roce_address_index: [u8; 0x10],
    pub reserved_at_50: [u8; 0xc],
    pub vhca_port_num: [u8; 0x4],
    pub reserved_at_60: [u8; 0x20],
    pub roce_address: mlx5_ifc_roce_addr_layout_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_mad_demux_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_mad_demux_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x6],
    pub demux_mode: [u8; 0x2],
    pub reserved_at_68: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_l2_table_entry_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_l2_table_entry_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x60],
    pub reserved_at_a0: [u8; 0x8],
    pub table_index: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x20],
    pub reserved_at_e0: [u8; 0x10],
    pub silent_mode_valid: [u8; 0x1],
    pub silent_mode: [u8; 0x1],
    pub reserved_at_f2: [u8; 0x1],
    pub vlan_valid: [u8; 0x1],
    pub vlan: [u8; 0xc],
    pub mac_address: mlx5_ifc_mac_address_layout_bits,
    pub reserved_at_140: [u8; 0xc0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_issi_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_issi_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub current_issi: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_hca_cap_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_hca_cap_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_function: [u8; 0x1],
    pub ec_vf_function: [u8; 0x1],
    pub reserved_at_42: [u8; 0x1],
    pub function_id_type: [u8; 0x1],
    pub reserved_at_44: [u8; 0xc],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub capability: mlx5_ifc_hca_cap_union_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_fte_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_fte_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub other_eswitch: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x8],
    pub eswitch_owner_vhca_id: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub ignore_flow_level: [u8; 0x1],
    pub reserved_at_c1: [u8; 0x17],
    pub modify_enable_mask: [u8; 0x8],
    pub reserved_at_e0: [u8; 0x20],
    pub flow_index: [u8; 0x20],
    pub reserved_at_120: [u8; 0xe0],
    pub flow_context: mlx5_ifc_flow_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dest_format_bits {
    pub destination_type: [u8; 0x8],
    pub destination_id: [u8; 0x18],
    pub destination_eswitch_owner_vhca_id_valid: [u8; 0x1],
    pub packet_reformat: [u8; 0x1],
    pub reserved_at_22: [u8; 0xe],
    pub destination_eswitch_owner_vhca_id: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rts2rts_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub ece: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rts2rts_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub opt_param_mask: [u8; 0x20],
    pub ece: [u8; 0x20],
    pub qpc: mlx5_ifc_qpc_bits,
    pub reserved_at_800: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rtr2rts_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub ece: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rtr2rts_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub opt_param_mask: [u8; 0x20],
    pub ece: [u8; 0x20],
    pub qpc: mlx5_ifc_qpc_bits,
    pub reserved_at_800: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rst2init_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub ece: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rst2init_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub opt_param_mask: [u8; 0x20],
    pub ece: [u8; 0x20],
    pub qpc: mlx5_ifc_qpc_bits,
    pub reserved_at_800: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_xrq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub xrq_context: mlx5_ifc_xrqc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_xrq_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub xrqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_xrc_srq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub xrc_srq_context_entry: mlx5_ifc_xrc_srqc_bits,
    pub reserved_at_280: [u8; 0x600],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_xrc_srq_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub xrc_srqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vport_state_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub max_tx_speed: [u8; 0x10],
    pub reserved_at_70: [u8; 0x8],
    pub admin_state: [u8; 0x4],
    pub state: [u8; 0x4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_array1024_auto_bits {
    pub array1024_auto: [u8; 32][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vuid_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x40],
    pub query_vfs_vuid: [u8; 0x1],
    pub data_direct: [u8; 0x1],
    pub reserved_at_62: [u8; 0xe],
    pub vhca_id: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vuid_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x1a0],
    pub reserved_at_1e0: [u8; 0x10],
    pub num_of_entries: [u8; 0x10],
    pub vuid: [mlx5_ifc_array1024_auto_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_monitor_counter_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_monitor_counter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_monitor_counter_ppcnt {
    MLX5_QUERY_MONITOR_PPCNT_IN_RANGE_LENGTH_ERRORS      = 0x0,
    MLX5_QUERY_MONITOR_PPCNT_OUT_OF_RANGE_LENGTH_FIELD   = 0x1,
    MLX5_QUERY_MONITOR_PPCNT_FRAME_TOO_LONG_ERRORS       = 0x2,
    MLX5_QUERY_MONITOR_PPCNT_FRAME_CHECK_SEQUENCE_ERRORS = 0x3,
    MLX5_QUERY_MONITOR_PPCNT_ALIGNMENT_ERRORS            = 0x4,
    MLX5_QUERY_MONITOR_PPCNT_IF_OUT_DISCARDS             = 0x5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_monitor_counter_output_bits {
    pub reserved_at_0: [u8; 0x4],
    pub type: [u8; 0x4],
    pub reserved_at_8: [u8; 0x8],
    pub counter: [u8; 0x10],
    pub counter_group_id: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_monitor_counter_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub num_of_counters: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub monitor_counter: [mlx5_ifc_monitor_counter_output_bits; MLX5_CMD_SET_MONITOR_NUM_COUNTER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_monitor_counter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vport_state_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vnic_env_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub vport_env: mlx5_ifc_vnic_diagnostic_statistics_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vnic_env_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vport_counter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub received_errors: mlx5_ifc_traffic_counter_bits,
    pub transmit_errors: mlx5_ifc_traffic_counter_bits,
    pub received_ib_unicast: mlx5_ifc_traffic_counter_bits,
    pub transmitted_ib_unicast: mlx5_ifc_traffic_counter_bits,
    pub received_ib_multicast: mlx5_ifc_traffic_counter_bits,
    pub transmitted_ib_multicast: mlx5_ifc_traffic_counter_bits,
    pub received_eth_broadcast: mlx5_ifc_traffic_counter_bits,
    pub transmitted_eth_broadcast: mlx5_ifc_traffic_counter_bits,
    pub received_eth_unicast: mlx5_ifc_traffic_counter_bits,
    pub transmitted_eth_unicast: mlx5_ifc_traffic_counter_bits,
    pub received_eth_multicast: mlx5_ifc_traffic_counter_bits,
    pub transmitted_eth_multicast: mlx5_ifc_traffic_counter_bits,
    pub local_loopback: mlx5_ifc_traffic_counter_bits,
    pub reserved_at_700: [u8; 0x980],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vport_counter_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xb],
    pub port_num: [u8; 0x4],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x60],
    pub clear: [u8; 0x1],
    pub reserved_at_c1: [u8; 0x1f],
    pub reserved_at_e0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_tis_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub tis_context: mlx5_ifc_tisc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_tis_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub tisn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_tir_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0xc0],
    pub tir_context: mlx5_ifc_tirc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_tir_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub tirn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_srq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub srq_context_entry: mlx5_ifc_srqc_bits,
    pub reserved_at_280: [u8; 0x600],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_srq_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub srqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_sq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0xc0],
    pub sq_context: mlx5_ifc_sqc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_sq_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub sqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_special_contexts_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub dump_fill_mkey: [u8; 0x20],
    pub resd_lkey: [u8; 0x20],
    pub null_mkey: [u8; 0x20],
    pub terminate_scatter_list_mkey: [u8; 0x20],
    pub repeated_mkey: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_special_contexts_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_scheduling_element_out_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0xc0],
    pub scheduling_context: mlx5_ifc_scheduling_context_bits,
    pub reserved_at_300: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_scheduling_element_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub scheduling_hierarchy: [u8; 0x8],
    pub reserved_at_48: [u8; 0x18],
    pub scheduling_element_id: [u8; 0x20],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_rqt_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0xc0],
    pub rqt_context: mlx5_ifc_rqtc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_rqt_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub rqtn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_rq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0xc0],
    pub rq_context: mlx5_ifc_rqc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_rq_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub rqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_roce_address_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub roce_address: mlx5_ifc_roce_addr_layout_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_roce_address_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub roce_address_index: [u8; 0x10],
    pub reserved_at_50: [u8; 0xc],
    pub vhca_port_num: [u8; 0x4],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_rmp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0xc0],
    pub rmp_context: mlx5_ifc_rmpc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_rmp_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub rmpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cqe_error_syndrome_bits {
    pub hw_error_syndrome: [u8; 0x8],
    pub hw_syndrome_type: [u8; 0x4],
    pub reserved_at_c: [u8; 0x4],
    pub vendor_error_syndrome: [u8; 0x8],
    pub syndrome: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qp_context_extension_bits {
    pub reserved_at_0: [u8; 0x60],
    pub error_syndrome: mlx5_ifc_cqe_error_syndrome_bits,
    pub reserved_at_80: [u8; 0x580],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qpc_extension_and_pas_list_in_bits {
    pub qpc_data_extension: mlx5_ifc_qp_context_extension_bits,
    pub pas: [u8; 0][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qp_pas_list_in_bits {
    pub pas: [mlx5_ifc_cmd_pas_bits; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_qp_pas_or_qpc_ext_and_pas_bits {
    pub qp_pas_list: mlx5_ifc_qp_pas_list_in_bits,
    pub qpc_ext_and_pas_list: mlx5_ifc_qpc_extension_and_pas_list_in_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub opt_param_mask: [u8; 0x20],
    pub ece: [u8; 0x20],
    pub qpc: mlx5_ifc_qpc_bits,
    pub reserved_at_800: [u8; 0x80],
    pub qp_pas_or_qpc_ext_and_pas: mlx5_ifc_qp_pas_or_qpc_ext_and_pas_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub qpc_ext: [u8; 0x1],
    pub reserved_at_41: [u8; 0x7],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_q_counter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub rx_write_requests: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x20],
    pub rx_read_requests: [u8; 0x20],
    pub reserved_at_e0: [u8; 0x20],
    pub rx_atomic_requests: [u8; 0x20],
    pub reserved_at_120: [u8; 0x20],
    pub rx_dct_connect: [u8; 0x20],
    pub reserved_at_160: [u8; 0x20],
    pub out_of_buffer: [u8; 0x20],
    pub reserved_at_1a0: [u8; 0x20],
    pub out_of_sequence: [u8; 0x20],
    pub reserved_at_1e0: [u8; 0x20],
    pub duplicate_request: [u8; 0x20],
    pub reserved_at_220: [u8; 0x20],
    pub rnr_nak_retry_err: [u8; 0x20],
    pub reserved_at_260: [u8; 0x20],
    pub packet_seq_err: [u8; 0x20],
    pub reserved_at_2a0: [u8; 0x20],
    pub implied_nak_seq_err: [u8; 0x20],
    pub reserved_at_2e0: [u8; 0x20],
    pub local_ack_timeout_err: [u8; 0x20],
    pub reserved_at_320: [u8; 0x60],
    pub req_rnr_retries_exceeded: [u8; 0x20],
    pub reserved_at_3a0: [u8; 0x20],
    pub resp_local_length_error: [u8; 0x20],
    pub req_local_length_error: [u8; 0x20],
    pub resp_local_qp_error: [u8; 0x20],
    pub local_operation_error: [u8; 0x20],
    pub resp_local_protection: [u8; 0x20],
    pub req_local_protection: [u8; 0x20],
    pub resp_cqe_error: [u8; 0x20],
    pub req_cqe_error: [u8; 0x20],
    pub req_mw_binding: [u8; 0x20],
    pub req_bad_response: [u8; 0x20],
    pub req_remote_invalid_request: [u8; 0x20],
    pub resp_remote_invalid_request: [u8; 0x20],
    pub req_remote_access_errors: [u8; 0x20],
    pub resp_remote_access_errors: [u8; 0x20],
    pub req_remote_operation_errors: [u8; 0x20],
    pub req_transport_retries_exceeded: [u8; 0x20],
    pub cq_overflow: [u8; 0x20],
    pub resp_cqe_flush_error: [u8; 0x20],
    pub req_cqe_flush_error: [u8; 0x20],
    pub reserved_at_620: [u8; 0x20],
    pub roce_adp_retrans: [u8; 0x20],
    pub roce_adp_retrans_to: [u8; 0x20],
    pub roce_slow_restart: [u8; 0x20],
    pub roce_slow_restart_cnps: [u8; 0x20],
    pub roce_slow_restart_trans: [u8; 0x20],
    pub reserved_at_6e0: [u8; 0x120],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_q_counter_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x60],
    pub clear: [u8; 0x1],
    pub aggregate: [u8; 0x1],
    pub reserved_at_c2: [u8; 0x1e],
    pub reserved_at_e0: [u8; 0x18],
    pub counter_set_id: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_pages_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub embedded_cpu_function: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub function_id: [u8; 0x10],
    pub num_pages: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_pages_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub embedded_cpu_function: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_nic_vport_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub nic_vport_context: mlx5_ifc_nic_vport_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_nic_vport_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x5],
    pub allowed_list_type: [u8; 0x3],
    pub reserved_at_68: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_mkey_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub memory_key_mkey_entry: mlx5_ifc_mkc_bits,
    pub reserved_at_280: [u8; 0x600],
    pub bsf0_klm0_pas_mtt0_1: [u8; 16][0x8],
    pub bsf1_klm1_pas_mtt2_3: [u8; 16][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_mkey_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub mkey_index: [u8; 0x18],
    pub pg_access: [u8; 0x1],
    pub reserved_at_61: [u8; 0x1f],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_mad_demux_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub mad_dumux_parameters_block: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_mad_demux_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_l2_table_entry_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0xa0],
    pub reserved_at_e0: [u8; 0x11],
    pub silent_mode: [u8; 0x1],
    pub reserved_at_f2: [u8; 0x1],
    pub vlan_valid: [u8; 0x1],
    pub vlan: [u8; 0xc],
    pub mac_address: mlx5_ifc_mac_address_layout_bits,
    pub reserved_at_140: [u8; 0xc0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_l2_table_entry_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub silent_mode_query: [u8; 0x1],
    pub reserved_at_81: [u8; 0x1f],
    pub reserved_at_a0: [u8; 0x8],
    pub table_index: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x140],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_issi_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x10],
    pub current_issi: [u8; 0x10],
    pub reserved_at_60: [u8; 0xa0],
    pub reserved_at_100: [u8; 76][0x8],
    pub supported_issi_dw0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_issi_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_driver_version_out_bits {
    pub status: [u8; 0x8],
    pub reserved_0: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_1: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_driver_version_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_0: [u8; 0x10],
    pub reserved_1: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_2: [u8; 0x40],
    pub driver_version: [u8; 64][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_hca_vport_pkey_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub pkey: [mlx5_ifc_pkey_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_hca_vport_pkey_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xb],
    pub port_num: [u8; 0x4],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x10],
    pub pkey_index: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_hca_vport_gid_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub gids_num: [u8; 0x10],
    pub reserved_at_70: [u8; 0x10],
    pub gid: [mlx5_ifc_array128_auto_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_hca_vport_gid_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xb],
    pub port_num: [u8; 0x4],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x10],
    pub gid_index: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_hca_vport_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub hca_vport_context: mlx5_ifc_hca_vport_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_hca_vport_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xb],
    pub port_num: [u8; 0x4],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_hca_cap_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub capability: mlx5_ifc_hca_cap_union_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_hca_cap_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_function: [u8; 0x1],
    pub ec_vf_function: [u8; 0x1],
    pub reserved_at_42: [u8; 0x1],
    pub function_id_type: [u8; 0x1],
    pub reserved_at_44: [u8; 0xc],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_other_hca_cap_bits {
    pub roce: [u8; 0x1],
    pub reserved_at_1: [u8; 0x27f],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_other_hca_cap_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub other_capability: mlx5_ifc_other_hca_cap_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_other_hca_cap_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_other_hca_cap_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_other_hca_cap_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub function_id: [u8; 0x10],
    pub field_select: [u8; 0x20],
    pub other_capability: mlx5_ifc_other_hca_cap_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sw_owner_icm_root_params_bits {
    pub sw_owner_icm_root_1: [u8; 0x40],
    pub sw_owner_icm_root_0: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rtc_params_bits {
    pub rtc_id_0: [u8; 0x20],
    pub rtc_id_1: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_table_context_bits {
    pub reformat_en: [u8; 0x1],
    pub decap_en: [u8; 0x1],
    pub sw_owner: [u8; 0x1],
    pub termination_table: [u8; 0x1],
    pub table_miss_action: [u8; 0x4],
    pub level: [u8; 0x8],
    pub rtc_valid: [u8; 0x1],
    pub reserved_at_11: [u8; 0x7],
    pub log_size: [u8; 0x8],
    pub reserved_at_20: [u8; 0x8],
    pub table_miss_id: [u8; 0x18],
    pub reserved_at_40: [u8; 0x8],
    pub lag_master_next_table_id: [u8; 0x18],
    pub reserved_at_60: [u8; 0x60],
    pub sws: mlx5_ifc_sw_owner_icm_root_params_bits,
    pub hws: mlx5_ifc_rtc_params_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_flow_table_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x80],
    pub flow_table_context: mlx5_ifc_flow_table_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_flow_table_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x140],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_fte_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x1c0],
    pub flow_context: mlx5_ifc_flow_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_fte_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x40],
    pub flow_index: [u8; 0x20],
    pub reserved_at_120: [u8; 0xe0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_match_definer_format_0_bits {
    pub reserved_at_0: [u8; 0x100],
    pub metadata_reg_c_0: [u8; 0x20],
    pub metadata_reg_c_1: [u8; 0x20],
    pub outer_dmac_47_16: [u8; 0x20],
    pub outer_dmac_15_0: [u8; 0x10],
    pub outer_ethertype: [u8; 0x10],
    pub reserved_at_180: [u8; 0x1],
    pub sx_sniffer: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub outer_ip_frag: [u8; 0x1],
    pub outer_qp_type: [u8; 0x2],
    pub outer_encap_type: [u8; 0x2],
    pub port_number: [u8; 0x2],
    pub outer_l3_type: [u8; 0x2],
    pub outer_l4_type: [u8; 0x2],
    pub outer_first_vlan_type: [u8; 0x2],
    pub outer_first_vlan_prio: [u8; 0x3],
    pub outer_first_vlan_cfi: [u8; 0x1],
    pub outer_first_vlan_vid: [u8; 0xc],
    pub outer_l4_type_ext: [u8; 0x4],
    pub reserved_at_1a4: [u8; 0x2],
    pub outer_ipsec_layer: [u8; 0x2],
    pub outer_l2_type: [u8; 0x2],
    pub force_lb: [u8; 0x1],
    pub outer_l2_ok: [u8; 0x1],
    pub outer_l3_ok: [u8; 0x1],
    pub outer_l4_ok: [u8; 0x1],
    pub outer_second_vlan_type: [u8; 0x2],
    pub outer_second_vlan_prio: [u8; 0x3],
    pub outer_second_vlan_cfi: [u8; 0x1],
    pub outer_second_vlan_vid: [u8; 0xc],
    pub outer_smac_47_16: [u8; 0x20],
    pub outer_smac_15_0: [u8; 0x10],
    pub inner_ipv4_checksum_ok: [u8; 0x1],
    pub inner_l4_checksum_ok: [u8; 0x1],
    pub outer_ipv4_checksum_ok: [u8; 0x1],
    pub outer_l4_checksum_ok: [u8; 0x1],
    pub inner_l3_ok: [u8; 0x1],
    pub inner_l4_ok: [u8; 0x1],
    pub outer_l3_ok_duplicate: [u8; 0x1],
    pub outer_l4_ok_duplicate: [u8; 0x1],
    pub outer_tcp_cwr: [u8; 0x1],
    pub outer_tcp_ece: [u8; 0x1],
    pub outer_tcp_urg: [u8; 0x1],
    pub outer_tcp_ack: [u8; 0x1],
    pub outer_tcp_psh: [u8; 0x1],
    pub outer_tcp_rst: [u8; 0x1],
    pub outer_tcp_syn: [u8; 0x1],
    pub outer_tcp_fin: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_match_definer_format_22_bits {
    pub reserved_at_0: [u8; 0x100],
    pub outer_ip_src_addr: [u8; 0x20],
    pub outer_ip_dest_addr: [u8; 0x20],
    pub outer_l4_sport: [u8; 0x10],
    pub outer_l4_dport: [u8; 0x10],
    pub reserved_at_160: [u8; 0x1],
    pub sx_sniffer: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub outer_ip_frag: [u8; 0x1],
    pub outer_qp_type: [u8; 0x2],
    pub outer_encap_type: [u8; 0x2],
    pub port_number: [u8; 0x2],
    pub outer_l3_type: [u8; 0x2],
    pub outer_l4_type: [u8; 0x2],
    pub outer_first_vlan_type: [u8; 0x2],
    pub outer_first_vlan_prio: [u8; 0x3],
    pub outer_first_vlan_cfi: [u8; 0x1],
    pub outer_first_vlan_vid: [u8; 0xc],
    pub metadata_reg_c_0: [u8; 0x20],
    pub outer_dmac_47_16: [u8; 0x20],
    pub outer_smac_47_16: [u8; 0x20],
    pub outer_smac_15_0: [u8; 0x10],
    pub outer_dmac_15_0: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_match_definer_format_23_bits {
    pub reserved_at_0: [u8; 0x100],
    pub inner_ip_src_addr: [u8; 0x20],
    pub inner_ip_dest_addr: [u8; 0x20],
    pub inner_l4_sport: [u8; 0x10],
    pub inner_l4_dport: [u8; 0x10],
    pub reserved_at_160: [u8; 0x1],
    pub sx_sniffer: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub inner_ip_frag: [u8; 0x1],
    pub inner_qp_type: [u8; 0x2],
    pub inner_encap_type: [u8; 0x2],
    pub port_number: [u8; 0x2],
    pub inner_l3_type: [u8; 0x2],
    pub inner_l4_type: [u8; 0x2],
    pub inner_first_vlan_type: [u8; 0x2],
    pub inner_first_vlan_prio: [u8; 0x3],
    pub inner_first_vlan_cfi: [u8; 0x1],
    pub inner_first_vlan_vid: [u8; 0xc],
    pub tunnel_header_0: [u8; 0x20],
    pub inner_dmac_47_16: [u8; 0x20],
    pub inner_smac_47_16: [u8; 0x20],
    pub inner_smac_15_0: [u8; 0x10],
    pub inner_dmac_15_0: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_match_definer_format_29_bits {
    pub reserved_at_0: [u8; 0xc0],
    pub outer_ip_dest_addr: [u8; 0x80],
    pub outer_ip_src_addr: [u8; 0x80],
    pub outer_l4_sport: [u8; 0x10],
    pub outer_l4_dport: [u8; 0x10],
    pub reserved_at_1e0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_match_definer_format_30_bits {
    pub reserved_at_0: [u8; 0xa0],
    pub outer_ip_dest_addr: [u8; 0x80],
    pub outer_ip_src_addr: [u8; 0x80],
    pub outer_dmac_47_16: [u8; 0x20],
    pub outer_smac_47_16: [u8; 0x20],
    pub outer_smac_15_0: [u8; 0x10],
    pub outer_dmac_15_0: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_match_definer_format_31_bits {
    pub reserved_at_0: [u8; 0xc0],
    pub inner_ip_dest_addr: [u8; 0x80],
    pub inner_ip_src_addr: [u8; 0x80],
    pub inner_l4_sport: [u8; 0x10],
    pub inner_l4_dport: [u8; 0x10],
    pub reserved_at_1e0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_match_definer_format_32_bits {
    pub reserved_at_0: [u8; 0xa0],
    pub inner_ip_dest_addr: [u8; 0x80],
    pub inner_ip_src_addr: [u8; 0x80],
    pub inner_dmac_47_16: [u8; 0x20],
    pub inner_smac_47_16: [u8; 0x20],
    pub inner_smac_15_0: [u8; 0x10],
    pub inner_dmac_15_0: [u8; 0x10],
}

pub const MLX5_IFC_DEFINER_FORMAT_OFFSET_UNUSED: c_uint = 0x0;
pub const MLX5_IFC_DEFINER_FORMAT_OFFSET_OUTER_ETH_PKT_LEN: c_uint = 0x48;
pub const MLX5_IFC_DEFINER_DW_SELECTORS_NUM: c_int = 9;
pub const MLX5_IFC_DEFINER_BYTE_SELECTORS_NUM: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_match_definer_match_mask_bits {
    pub reserved_at_1c0: [u8; 5][0x20],
    pub match_dw_8: [u8; 0x20],
    pub match_dw_7: [u8; 0x20],
    pub match_dw_6: [u8; 0x20],
    pub match_dw_5: [u8; 0x20],
    pub match_dw_4: [u8; 0x20],
    pub match_dw_3: [u8; 0x20],
    pub match_dw_2: [u8; 0x20],
    pub match_dw_1: [u8; 0x20],
    pub match_dw_0: [u8; 0x20],
    pub match_byte_7: [u8; 0x8],
    pub match_byte_6: [u8; 0x8],
    pub match_byte_5: [u8; 0x8],
    pub match_byte_4: [u8; 0x8],
    pub match_byte_3: [u8; 0x8],
    pub match_byte_2: [u8; 0x8],
    pub match_byte_1: [u8; 0x8],
    pub match_byte_0: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_match_definer_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x40],
    pub reserved_at_80: [u8; 0x10],
    pub format_id: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x60],
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
    pub match_mask: [u8; 16][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_general_obj_create_param_bits {
    pub alias_object: [u8; 0x1],
    pub reserved_at_1: [u8; 0x2],
    pub log_obj_range: [u8; 0x5],
    pub reserved_at_8: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_general_obj_query_param_bits {
    pub alias_object: [u8; 0x1],
    pub obj_offset: [u8; 0x1f],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_general_obj_in_cmd_hdr_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub vhca_tunnel_id: [u8; 0x10],
    pub obj_type: [u8; 0x10],
    pub obj_id: [u8; 0x20],
    pub create: mlx5_ifc_general_obj_create_param_bits,
    pub query: mlx5_ifc_general_obj_query_param_bits,
    pub op_param: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_general_obj_out_cmd_hdr_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub obj_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_allow_other_vhca_access_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x50],
    pub object_type_to_be_accessed: [u8; 0x10],
    pub object_id_to_be_accessed: [u8; 0x20],
    pub reserved_at_c0: [u8; 0x40],
    pub access_key_raw: [u8; 0x100],
    pub access_key: [u8; 8][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_allow_other_vhca_access_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_header_arg_bits {
    pub reserved_at_0: [u8; 0x80],
    pub reserved_at_80: [u8; 0x8],
    pub access_pd: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_modify_header_arg_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub arg: mlx5_ifc_modify_header_arg_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_match_definer_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_match_definer_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_match_definer_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alias_context_bits {
    pub vhca_id_to_be_accessed: [u8; 0x10],
    pub reserved_at_10: [u8; 0xb],
    pub vhca_id_type: [u8; 0x1],
    pub reserved_at_1c: [u8; 0x1],
    pub status: [u8; 0x3],
    pub object_id_to_be_accessed: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub access_key_raw: [u8; 0x100],
    pub access_key: [u8; 8][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_alias_obj_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub alias_ctx: mlx5_ifc_alias_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_flow_group_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0xa0],
    pub start_flow_index: [u8; 0x20],
    pub reserved_at_100: [u8; 0x20],
    pub end_flow_index: [u8; 0x20],
    pub reserved_at_140: [u8; 0xa0],
    pub reserved_at_1e0: [u8; 0x18],
    pub match_criteria_enable: [u8; 0x8],
    pub match_criteria: mlx5_ifc_fte_match_param_bits,
    pub reserved_at_1200: [u8; 0xe00],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_flow_group_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub group_id: [u8; 0x20],
    pub reserved_at_e0: [u8; 0x120],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_flow_counter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub flow_statistics: [mlx5_ifc_traffic_counter_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_flow_counter_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x80],
    pub clear: [u8; 0x1],
    pub reserved_at_c1: [u8; 0xf],
    pub num_of_counters: [u8; 0x10],
    pub flow_counter_id: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_esw_vport_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub esw_vport_context: mlx5_ifc_esw_vport_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_esw_vport_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_esw_vport_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_esw_vport_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub vport_num: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_esw_vport_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_esw_vport_context_fields_select_bits {
    pub reserved_at_0: [u8; 0x1b],
    pub fdb_to_vport_reg_c_id: [u8; 0x1],
    pub vport_cvlan_insert: [u8; 0x1],
    pub vport_svlan_insert: [u8; 0x1],
    pub vport_cvlan_strip: [u8; 0x1],
    pub vport_svlan_strip: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_esw_vport_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub vport_number: [u8; 0x10],
    pub field_select: mlx5_ifc_esw_vport_context_fields_select_bits,
    pub esw_vport_context: mlx5_ifc_esw_vport_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_eq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub eq_context_entry: mlx5_ifc_eqc_bits,
    pub reserved_at_280: [u8; 0x40],
    pub event_bitmask: [u8; 0x40],
    pub reserved_at_300: [u8; 0x580],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_eq_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x18],
    pub eq_number: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_packet_reformat_context_in_bits {
    pub reformat_type: [u8; 0x8],
    pub reserved_at_8: [u8; 0x4],
    pub reformat_param_0: [u8; 0x4],
    pub reserved_at_10: [u8; 0x6],
    pub reformat_data_size: [u8; 0xa],
    pub reformat_param_1: [u8; 0x8],
    pub reserved_at_28: [u8; 0x8],
    pub reformat_data: [u8; 2][0x8],
    pub more_reformat_data: [u8; ][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_packet_reformat_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0xa0],
    pub packet_reformat_context: [mlx5_ifc_packet_reformat_context_in_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_packet_reformat_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub packet_reformat_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0xa0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_packet_reformat_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub packet_reformat_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_reformat_ctx_type {
    MLX5_REFORMAT_TYPE_L2_TO_VXLAN = 0x0,
    MLX5_REFORMAT_TYPE_L2_TO_NVGRE = 0x1,
    MLX5_REFORMAT_TYPE_L2_TO_L2_TUNNEL = 0x2,
    MLX5_REFORMAT_TYPE_L3_TUNNEL_TO_L2 = 0x3,
    MLX5_REFORMAT_TYPE_L2_TO_L3_TUNNEL = 0x4,
    MLX5_REFORMAT_TYPE_ADD_ESP_TRANSPORT_OVER_IPV4 = 0x5,
    MLX5_REFORMAT_TYPE_L2_TO_L3_ESP_TUNNEL = 0x6,
    MLX5_REFORMAT_TYPE_ADD_ESP_TRANSPORT_OVER_UDPV4 = 0x7,
    MLX5_REFORMAT_TYPE_DEL_ESP_TRANSPORT = 0x8,
    MLX5_REFORMAT_TYPE_L3_ESP_TUNNEL_TO_L2 = 0x9,
    MLX5_REFORMAT_TYPE_DEL_ESP_TRANSPORT_OVER_UDP = 0xa,
    MLX5_REFORMAT_TYPE_ADD_ESP_TRANSPORT_OVER_IPV6 = 0xb,
    MLX5_REFORMAT_TYPE_ADD_ESP_TRANSPORT_OVER_UDPV6 = 0xc,
    MLX5_REFORMAT_TYPE_ADD_PSP_TUNNEL = 0xd,
    MLX5_REFORMAT_TYPE_DEL_PSP_TUNNEL = 0xe,
    MLX5_REFORMAT_TYPE_INSERT_HDR = 0xf,
    MLX5_REFORMAT_TYPE_REMOVE_HDR = 0x10,
    MLX5_REFORMAT_TYPE_ADD_MACSEC = 0x11,
    MLX5_REFORMAT_TYPE_DEL_MACSEC = 0x12,
    MLX5_REFORMAT_TYPE_REMOVE_PSP_TRANSPORT = 0x16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_packet_reformat_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0xa0],
    pub packet_reformat_context: mlx5_ifc_packet_reformat_context_in_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_packet_reformat_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_packet_reformat_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub packet_reformat_id: [u8; 0x20],
    pub reserved_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_action_in_bits {
    pub action_type: [u8; 0x4],
    pub field: [u8; 0xc],
    pub reserved_at_10: [u8; 0x3],
    pub offset: [u8; 0x5],
    pub reserved_at_18: [u8; 0x3],
    pub length: [u8; 0x5],
    pub data: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_add_action_in_bits {
    pub action_type: [u8; 0x4],
    pub field: [u8; 0xc],
    pub reserved_at_10: [u8; 0x10],
    pub data: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_copy_action_in_bits {
    pub action_type: [u8; 0x4],
    pub src_field: [u8; 0xc],
    pub reserved_at_10: [u8; 0x3],
    pub src_offset: [u8; 0x5],
    pub reserved_at_18: [u8; 0x3],
    pub length: [u8; 0x5],
    pub reserved_at_20: [u8; 0x4],
    pub dst_field: [u8; 0xc],
    pub reserved_at_30: [u8; 0x3],
    pub dst_offset: [u8; 0x5],
    pub reserved_at_38: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_set_add_copy_action_in_auto_bits {
    pub set_action_in: mlx5_ifc_set_action_in_bits,
    pub add_action_in: mlx5_ifc_add_action_in_bits,
    pub copy_action_in: mlx5_ifc_copy_action_in_bits,
    pub reserved_at_0: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_modify_header_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub modify_header_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_modify_header_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub table_type: [u8; 0x8],
    pub reserved_at_68: [u8; 0x10],
    pub num_of_actions: [u8; 0x8],
    pub actions: [mlx5_ifc_set_add_copy_action_in_auto_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_modify_header_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_modify_header_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub modify_header_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_modify_header_context_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub modify_header_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0xa0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_dct_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub dct_context_entry: mlx5_ifc_dctc_bits,
    pub reserved_at_280: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_dct_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub dctn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_cq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub cq_context: mlx5_ifc_cqc_bits,
    pub reserved_at_280: [u8; 0x600],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_cq_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub cqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_cong_status_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub enable: [u8; 0x1],
    pub tag_enable: [u8; 0x1],
    pub reserved_at_62: [u8; 0x1e],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_cong_status_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x18],
    pub priority: [u8; 0x4],
    pub cong_protocol: [u8; 0x4],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_cong_statistics_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub rp_cur_flows: [u8; 0x20],
    pub sum_flows: [u8; 0x20],
    pub rp_cnp_ignored_high: [u8; 0x20],
    pub rp_cnp_ignored_low: [u8; 0x20],
    pub rp_cnp_handled_high: [u8; 0x20],
    pub rp_cnp_handled_low: [u8; 0x20],
    pub reserved_at_140: [u8; 0x100],
    pub time_stamp_high: [u8; 0x20],
    pub time_stamp_low: [u8; 0x20],
    pub accumulators_period: [u8; 0x20],
    pub np_ecn_marked_roce_packets_high: [u8; 0x20],
    pub np_ecn_marked_roce_packets_low: [u8; 0x20],
    pub np_cnp_sent_high: [u8; 0x20],
    pub np_cnp_sent_low: [u8; 0x20],
    pub reserved_at_320: [u8; 0x560],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_cong_statistics_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub clear: [u8; 0x1],
    pub reserved_at_41: [u8; 0x1f],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_cong_params_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub congestion_parameters: mlx5_ifc_cong_control_roce_ecn_auto_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_cong_params_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x1c],
    pub cong_protocol: [u8; 0x4],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_adapter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub query_adapter_struct: mlx5_ifc_query_adapter_param_block_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_adapter_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_function_vhca_rid_info_reg_bits {
    pub host_number: [u8; 0x8],
    pub host_pci_device_function: [u8; 0x8],
    pub host_pci_bus: [u8; 0x8],
    pub reserved_at_18: [u8; 0x3],
    pub pci_bus_assigned: [u8; 0x1],
    pub function_type: [u8; 0x4],
    pub parent_pci_device_function: [u8; 0x8],
    pub parent_pci_bus: [u8; 0x8],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_delegated_function_vhca_rid_info_bits {
    pub function_vhca_rid_info: mlx5_ifc_function_vhca_rid_info_reg_bits,
    pub reserved_at_80: [u8; 0x18],
    pub manage_profile: [u8; 0x8],
    pub reserved_at_a0: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_delegated_vhca_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x10],
    pub functions_count: [u8; 0x10],
    pub reserved_at_80: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_delegated_vhca_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_esw_vport_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x10],
    pub vport_num: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_esw_vport_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub managed_vhca_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qp_2rst_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qp_2rst_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qp_2err_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qp_2err_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_trans_page_fault_info_bits {
    pub error: [u8; 0x1],
    pub reserved_at_1: [u8; 0x4],
    pub page_fault_type: [u8; 0x3],
    pub wq_number: [u8; 0x18],
    pub reserved_at_20: [u8; 0x8],
    pub fault_token: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mem_page_fault_info_bits {
    pub error: [u8; 0x1],
    pub reserved_at_1: [u8; 0xf],
    pub fault_token_47_32: [u8; 0x10],
    pub fault_token_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_page_fault_resume_in_page_fault_info_auto_bits {
    pub trans_page_fault_info: mlx5_ifc_trans_page_fault_info_bits,
    pub mem_page_fault_info: mlx5_ifc_mem_page_fault_info_bits,
    pub reserved_at_0: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_page_fault_resume_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_page_fault_resume_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_nop_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_nop_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_vport_state_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_vport_state_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub vport_number: [u8; 0x10],
    pub max_tx_speed: [u8; 0x10],
    pub ingress_connect: [u8; 0x1],
    pub egress_connect: [u8; 0x1],
    pub ingress_connect_valid: [u8; 0x1],
    pub egress_connect_valid: [u8; 0x1],
    pub reserved_at_74: [u8; 0x4],
    pub admin_state: [u8; 0x4],
    pub reserved_at_7c: [u8; 0x4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_tis_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_tis_bitmask_bits {
    pub reserved_at_0: [u8; 0x20],
    pub reserved_at_20: [u8; 0x1d],
    pub lag_tx_port_affinity: [u8; 0x1],
    pub strict_lag_tx_port_affinity: [u8; 0x1],
    pub prio: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_tis_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub tisn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub bitmask: mlx5_ifc_modify_tis_bitmask_bits,
    pub reserved_at_c0: [u8; 0x40],
    pub ctx: mlx5_ifc_tisc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_tir_bitmask_bits {
    pub reserved_at_0: [u8; 0x20],
    pub reserved_at_20: [u8; 0x1b],
    pub self_lb_en: [u8; 0x1],
    pub reserved_at_3c: [u8; 0x1],
    pub hash: [u8; 0x1],
    pub reserved_at_3e: [u8; 0x1],
    pub packet_merge: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_tir_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_tir_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub tirn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub bitmask: mlx5_ifc_modify_tir_bitmask_bits,
    pub reserved_at_c0: [u8; 0x40],
    pub ctx: mlx5_ifc_tirc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_sq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_sq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub sq_state: [u8; 0x4],
    pub reserved_at_44: [u8; 0x4],
    pub sqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub modify_bitmask: [u8; 0x40],
    pub reserved_at_c0: [u8; 0x40],
    pub ctx: mlx5_ifc_sqc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_scheduling_element_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x1c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_scheduling_element_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub scheduling_hierarchy: [u8; 0x8],
    pub reserved_at_48: [u8; 0x18],
    pub scheduling_element_id: [u8; 0x20],
    pub reserved_at_80: [u8; 0x20],
    pub modify_bitmask: [u8; 0x20],
    pub reserved_at_c0: [u8; 0x40],
    pub scheduling_context: mlx5_ifc_scheduling_context_bits,
    pub reserved_at_300: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_rqt_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rqt_bitmask_bits {
    pub reserved_at_0: [u8; 0x20],
    pub reserved_at_20: [u8; 0x1f],
    pub rqn_list: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_rqt_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub rqtn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub bitmask: mlx5_ifc_rqt_bitmask_bits,
    pub reserved_at_c0: [u8; 0x40],
    pub ctx: mlx5_ifc_rqtc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_rq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_rq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub rq_state: [u8; 0x4],
    pub reserved_at_44: [u8; 0x4],
    pub rqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub modify_bitmask: [u8; 0x40],
    pub reserved_at_c0: [u8; 0x40],
    pub ctx: mlx5_ifc_rqc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_rmp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_rmp_bitmask_bits {
    pub reserved_at_0: [u8; 0x20],
    pub reserved_at_20: [u8; 0x1f],
    pub lwm: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_rmp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub rmp_state: [u8; 0x4],
    pub reserved_at_44: [u8; 0x4],
    pub rmpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub bitmask: mlx5_ifc_rmp_bitmask_bits,
    pub reserved_at_c0: [u8; 0x40],
    pub ctx: mlx5_ifc_rmpc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_nic_vport_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_nic_vport_field_select_bits {
    pub reserved_at_0: [u8; 0x12],
    pub affiliation: [u8; 0x1],
    pub reserved_at_13: [u8; 0x1],
    pub disable_uc_local_lb: [u8; 0x1],
    pub disable_mc_local_lb: [u8; 0x1],
    pub node_guid: [u8; 0x1],
    pub port_guid: [u8; 0x1],
    pub min_inline: [u8; 0x1],
    pub mtu: [u8; 0x1],
    pub change_event: [u8; 0x1],
    pub promisc: [u8; 0x1],
    pub permanent_address: [u8; 0x1],
    pub addresses_list: [u8; 0x1],
    pub roce_en: [u8; 0x1],
    pub reserved_at_1f: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_nic_vport_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub vport_number: [u8; 0x10],
    pub field_select: mlx5_ifc_modify_nic_vport_field_select_bits,
    pub reserved_at_80: [u8; 0x780],
    pub nic_vport_context: mlx5_ifc_nic_vport_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_hca_vport_context_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_hca_vport_context_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub reserved_at_41: [u8; 0xb],
    pub port_num: [u8; 0x4],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub hca_vport_context: mlx5_ifc_hca_vport_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_cq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_cq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub cqn: [u8; 0x18],
    pub modify_field_select_resize_field_select: mlx5_ifc_modify_field_select_resize_field_select_auto_bits,
    pub cq_context: mlx5_ifc_cqc_bits,
    pub reserved_at_280: [u8; 0x60],
    pub cq_umem_valid: [u8; 0x1],
    pub reserved_at_2e1: [u8; 0x1f],
    pub reserved_at_300: [u8; 0x580],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_cong_status_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_cong_status_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x18],
    pub priority: [u8; 0x4],
    pub cong_protocol: [u8; 0x4],
    pub enable: [u8; 0x1],
    pub tag_enable: [u8; 0x1],
    pub reserved_at_62: [u8; 0x1e],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_cong_params_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_cong_params_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x1c],
    pub cong_protocol: [u8; 0x4],
    pub field_select: mlx5_ifc_field_select_802_1_r_roce_auto_bits,
    pub reserved_at_80: [u8; 0x80],
    pub congestion_parameters: mlx5_ifc_cong_control_roce_ecn_auto_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_manage_pages_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub output_num_entries: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_manage_pages_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub embedded_cpu_function: [u8; 0x1],
    pub reserved_at_41: [u8; 0xf],
    pub function_id: [u8; 0x10],
    pub input_num_entries: [u8; 0x20],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mad_ifc_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub response_mad_packet: [u8; 256][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mad_ifc_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub remote_lid: [u8; 0x10],
    pub plane_index: [u8; 0x8],
    pub port: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
    pub mad: [u8; 256][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_init_hca_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_init_hca_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x2],
    pub sw_vhca_id: [u8; 0xe],
    pub reserved_at_70: [u8; 0x10],
    pub sw_owner_id: [u8; 4][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_init2rtr_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub ece: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_init2rtr_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub opt_param_mask: [u8; 0x20],
    pub ece: [u8; 0x20],
    pub qpc: mlx5_ifc_qpc_bits,
    pub reserved_at_800: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_init2init_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub ece: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_init2init_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub opt_param_mask: [u8; 0x20],
    pub ece: [u8; 0x20],
    pub qpc: mlx5_ifc_qpc_bits,
    pub reserved_at_800: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_get_dropped_packet_log_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub packet_headers_log: [u8; 128][0x8],
    pub packet_syndrome: [u8; 64][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_get_dropped_packet_log_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_gen_eqe_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x18],
    pub eq_number: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
    pub eqe: [u8; 64][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_gen_eq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_enable_hca_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_enable_hca_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub embedded_cpu_function: [u8; 0x1],
    pub reserved_at_41: [u8; 0x2],
    pub function_id_type: [u8; 0x1],
    pub reserved_at_44: [u8; 0xc],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_drain_dct_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_drain_dct_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub dctn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_disable_hca_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_disable_hca_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub embedded_cpu_function: [u8; 0x1],
    pub reserved_at_41: [u8; 0x2],
    pub function_id_type: [u8; 0x1],
    pub reserved_at_44: [u8; 0xc],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_detach_from_mcg_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_detach_from_mcg_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub multicast_gid: [u8; 16][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_xrq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_xrq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub xrqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_xrc_srq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_xrc_srq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub xrc_srqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_tis_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_tis_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub tisn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_tir_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_tir_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub tirn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_srq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_srq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub srqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_sq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_sq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub sqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_scheduling_element_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x1c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_scheduling_element_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub scheduling_hierarchy: [u8; 0x8],
    pub reserved_at_48: [u8; 0x18],
    pub scheduling_element_id: [u8; 0x20],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_rqt_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_rqt_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub rqtn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_rq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_rq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub rqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_delay_drop_params_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x10],
    pub delay_drop_timeout: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_delay_drop_params_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_rmp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_rmp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub rmpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_psv_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_psv_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub psvn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_mkey_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_mkey_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub mkey_index: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_flow_table_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_flow_table_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub other_eswitch: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x8],
    pub eswitch_owner_vhca_id: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x140],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_flow_group_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_flow_group_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub other_eswitch: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x8],
    pub eswitch_owner_vhca_id: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub group_id: [u8; 0x20],
    pub reserved_at_e0: [u8; 0x120],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_eq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_eq_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x18],
    pub eq_number: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_dct_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_dct_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub dctn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_cq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_cq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub cqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_delete_vxlan_udp_dport_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_delete_vxlan_udp_dport_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x10],
    pub vxlan_udp_port: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_delete_l2_table_entry_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_delete_l2_table_entry_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x60],
    pub reserved_at_a0: [u8; 0x8],
    pub table_index: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x140],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_delete_fte_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_delete_fte_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub other_eswitch: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x8],
    pub eswitch_owner_vhca_id: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x40],
    pub flow_index: [u8; 0x20],
    pub reserved_at_120: [u8; 0xe0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_xrcd_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_xrcd_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub xrcd: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_uar_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_uar_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub uar: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_transport_domain_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_transport_domain_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub transport_domain: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_q_counter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_q_counter_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x18],
    pub counter_set_id: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_pd_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_pd_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_flow_counter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_flow_counter_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub flow_counter_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_xrq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub xrqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_xrq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub xrq_context: mlx5_ifc_xrqc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_xrc_srq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub xrc_srqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_xrc_srq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub xrc_srq_context_entry: mlx5_ifc_xrc_srqc_bits,
    pub reserved_at_280: [u8; 0x60],
    pub xrc_srq_umem_valid: [u8; 0x1],
    pub reserved_at_2e1: [u8; 0x1f],
    pub reserved_at_300: [u8; 0x580],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_tis_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub tisn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_tis_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0xc0],
    pub ctx: mlx5_ifc_tisc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_tir_out_bits {
    pub status: [u8; 0x8],
    pub icm_address_63_40: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub icm_address_39_32: [u8; 0x8],
    pub tirn: [u8; 0x18],
    pub icm_address_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_tir_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0xc0],
    pub ctx: mlx5_ifc_tirc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_srq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub srqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_srq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub srq_context_entry: mlx5_ifc_srqc_bits,
    pub reserved_at_280: [u8; 0x600],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_sq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub sqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_sq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0xc0],
    pub ctx: mlx5_ifc_sqc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_scheduling_element_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub scheduling_element_id: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x160],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_scheduling_element_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub scheduling_hierarchy: [u8; 0x8],
    pub reserved_at_48: [u8; 0x18],
    pub reserved_at_60: [u8; 0xa0],
    pub scheduling_context: mlx5_ifc_scheduling_context_bits,
    pub reserved_at_300: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_rqt_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub rqtn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_rqt_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0xc0],
    pub rqt_context: mlx5_ifc_rqtc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_rq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub rqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_rq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0xc0],
    pub ctx: mlx5_ifc_rqc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_rmp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub rmpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_rmp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0xc0],
    pub ctx: mlx5_ifc_rmpc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_qp_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub ece: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_qp_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub qpc_ext: [u8; 0x1],
    pub reserved_at_41: [u8; 0x7],
    pub input_qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub opt_param_mask: [u8; 0x20],
    pub ece: [u8; 0x20],
    pub qpc: mlx5_ifc_qpc_bits,
    pub wq_umem_offset: [u8; 0x40],
    pub wq_umem_id: [u8; 0x20],
    pub wq_umem_valid: [u8; 0x1],
    pub reserved_at_861: [u8; 0x1f],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_psv_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub reserved_at_80: [u8; 0x8],
    pub psv0_index: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x8],
    pub psv1_index: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x8],
    pub psv2_index: [u8; 0x18],
    pub reserved_at_e0: [u8; 0x8],
    pub psv3_index: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_psv_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub num_psv: [u8; 0x4],
    pub reserved_at_44: [u8; 0x4],
    pub pd: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_mkey_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub mkey_index: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_mkey_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub pg_access: [u8; 0x1],
    pub mkey_umem_valid: [u8; 0x1],
    pub data_direct: [u8; 0x1],
    pub reserved_at_63: [u8; 0x1d],
    pub memory_key_mkey_entry: mlx5_ifc_mkc_bits,
    pub reserved_at_280: [u8; 0x80],
    pub translations_octword_actual_size: [u8; 0x20],
    pub reserved_at_320: [u8; 0x560],
    pub klm_pas_mtt: [u8; ][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_flow_table_out_bits {
    pub status: [u8; 0x8],
    pub icm_address_63_40: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub icm_address_39_32: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub icm_address_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_flow_table_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub other_eswitch: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x8],
    pub eswitch_owner_vhca_id: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x20],
    pub flow_table_context: mlx5_ifc_flow_table_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_flow_group_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub group_id: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_flow_group_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub other_eswitch: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x4],
    pub group_type: [u8; 0x4],
    pub eswitch_owner_vhca_id: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub source_eswitch_owner_vhca_id_valid: [u8; 0x1],
    pub reserved_at_c1: [u8; 0x1f],
    pub start_flow_index: [u8; 0x20],
    pub reserved_at_100: [u8; 0x20],
    pub end_flow_index: [u8; 0x20],
    pub reserved_at_140: [u8; 0x10],
    pub match_definer_id: [u8; 0x10],
    pub reserved_at_160: [u8; 0x80],
    pub reserved_at_1e0: [u8; 0x18],
    pub match_criteria_enable: [u8; 0x8],
    pub match_criteria: mlx5_ifc_fte_match_param_bits,
    pub reserved_at_1200: [u8; 0xe00],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_eq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x18],
    pub eq_number: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_eq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub eq_context_entry: mlx5_ifc_eqc_bits,
    pub reserved_at_280: [u8; 0x40],
    pub event_bitmask: [u8; 4][0x40],
    pub reserved_at_3c0: [u8; 0x4c0],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_dct_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub dctn: [u8; 0x18],
    pub ece: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_dct_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub dct_context_entry: mlx5_ifc_dctc_bits,
    pub reserved_at_280: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_cq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub cqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_cq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub cq_context: mlx5_ifc_cqc_bits,
    pub reserved_at_280: [u8; 0x60],
    pub cq_umem_valid: [u8; 0x1],
    pub reserved_at_2e1: [u8; 0x59f],
    pub pas: [u8; ][0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_config_int_moderation_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x4],
    pub min_delay: [u8; 0xc],
    pub int_vector: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_config_int_moderation_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x4],
    pub min_delay: [u8; 0xc],
    pub int_vector: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_attach_to_mcg_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_attach_to_mcg_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
    pub multicast_gid: [u8; 16][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_xrq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_xrq_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub xrqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x10],
    pub lwm: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_xrc_srq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_xrc_srq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub xrc_srqn: [u8; 0x18],
    pub reserved_at_60: [u8; 0x10],
    pub lwm: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_rq_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_rq_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub srq_number: [u8; 0x18],
    pub reserved_at_60: [u8; 0x10],
    pub lwm: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_dct_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_arm_dct_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub dct_number: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_xrcd_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub xrcd: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_xrcd_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_uar_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub uar: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_uar_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_transport_domain_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub transport_domain: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_transport_domain_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_q_counter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x18],
    pub counter_set_id: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_q_counter_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_pd_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_pd_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_flow_counter_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub flow_counter_id: [u8; 0x20],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_flow_counter_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x33],
    pub flow_counter_bulk_log_size: [u8; 0x5],
    pub flow_counter_bulk: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_add_vxlan_udp_dport_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_add_vxlan_udp_dport_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x10],
    pub vxlan_udp_port: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_pp_rate_limit_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_pp_rate_limit_context_bits {
    pub rate_limit: [u8; 0x20],
    pub burst_upper_bound: [u8; 0x20],
    pub reserved_at_40: [u8; 0x10],
    pub typical_packet_size: [u8; 0x10],
    pub reserved_at_60: [u8; 0x120],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_pp_rate_limit_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub rate_limit_index: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub ctx: mlx5_ifc_set_pp_rate_limit_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_access_register_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub register_data: [u8; ][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_access_register_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub register_id: [u8; 0x10],
    pub argument: [u8; 0x20],
    pub register_data: [u8; ][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sltp_reg_bits {
    pub status: [u8; 0x4],
    pub version: [u8; 0x4],
    pub local_port: [u8; 0x8],
    pub pnat: [u8; 0x2],
    pub reserved_at_12: [u8; 0x2],
    pub lane: [u8; 0x4],
    pub reserved_at_18: [u8; 0x8],
    pub reserved_at_20: [u8; 0x20],
    pub reserved_at_40: [u8; 0x7],
    pub polarity: [u8; 0x1],
    pub ob_tap0: [u8; 0x8],
    pub ob_tap1: [u8; 0x8],
    pub ob_tap2: [u8; 0x8],
    pub reserved_at_60: [u8; 0xc],
    pub ob_preemp_mode: [u8; 0x4],
    pub ob_reg: [u8; 0x8],
    pub ob_bias: [u8; 0x8],
    pub reserved_at_80: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_slrg_reg_bits {
    pub status: [u8; 0x4],
    pub version: [u8; 0x4],
    pub local_port: [u8; 0x8],
    pub pnat: [u8; 0x2],
    pub reserved_at_12: [u8; 0x2],
    pub lane: [u8; 0x4],
    pub reserved_at_18: [u8; 0x8],
    pub time_to_link_up: [u8; 0x10],
    pub reserved_at_30: [u8; 0xc],
    pub grade_lane_speed: [u8; 0x4],
    pub grade_version: [u8; 0x8],
    pub grade: [u8; 0x18],
    pub reserved_at_60: [u8; 0x4],
    pub height_grade_type: [u8; 0x4],
    pub height_grade: [u8; 0x18],
    pub height_dz: [u8; 0x10],
    pub height_dv: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x10],
    pub height_sigma: [u8; 0x10],
    pub reserved_at_c0: [u8; 0x20],
    pub reserved_at_e0: [u8; 0x4],
    pub phase_grade_type: [u8; 0x4],
    pub phase_grade: [u8; 0x18],
    pub reserved_at_100: [u8; 0x8],
    pub phase_eo_pos: [u8; 0x8],
    pub reserved_at_110: [u8; 0x8],
    pub phase_eo_neg: [u8; 0x8],
    pub ffe_set_tested: [u8; 0x10],
    pub test_errors_per_lane: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pvlc_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x1c],
    pub vl_hw_cap: [u8; 0x4],
    pub reserved_at_40: [u8; 0x1c],
    pub vl_admin: [u8; 0x4],
    pub reserved_at_60: [u8; 0x1c],
    pub vl_operational: [u8; 0x4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pude_reg_bits {
    pub swid: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x4],
    pub admin_status: [u8; 0x4],
    pub reserved_at_18: [u8; 0x4],
    pub oper_status: [u8; 0x4],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ptys_reg_bits {
    pub reserved_at_0: [u8; 0x1],
    pub an_disable_admin: [u8; 0x1],
    pub an_disable_cap: [u8; 0x1],
    pub reserved_at_3: [u8; 0x5],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub plane_ind: [u8; 0x4],
    pub reserved_at_1c: [u8; 0x1],
    pub proto_mask: [u8; 0x3],
    pub an_status: [u8; 0x4],
    pub reserved_at_24: [u8; 0xc],
    pub data_rate_oper: [u8; 0x10],
    pub ext_eth_proto_capability: [u8; 0x20],
    pub eth_proto_capability: [u8; 0x20],
    pub ib_link_width_capability: [u8; 0x10],
    pub ib_proto_capability: [u8; 0x10],
    pub ext_eth_proto_admin: [u8; 0x20],
    pub eth_proto_admin: [u8; 0x20],
    pub ib_link_width_admin: [u8; 0x10],
    pub ib_proto_admin: [u8; 0x10],
    pub ext_eth_proto_oper: [u8; 0x20],
    pub eth_proto_oper: [u8; 0x20],
    pub ib_link_width_oper: [u8; 0x10],
    pub ib_proto_oper: [u8; 0x10],
    pub reserved_at_160: [u8; 0x8],
    pub lane_rate_oper: [u8; 0x14],
    pub connector_type: [u8; 0x4],
    pub eth_proto_lp_advertise: [u8; 0x20],
    pub reserved_at_1a0: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mlcr_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x20],
    pub beacon_duration: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub beacon_remain: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ptas_reg_bits {
    pub reserved_at_0: [u8; 0x20],
    pub algorithm_options: [u8; 0x10],
    pub reserved_at_30: [u8; 0x4],
    pub repetitions_mode: [u8; 0x4],
    pub num_of_repetitions: [u8; 0x8],
    pub grade_version: [u8; 0x8],
    pub height_grade_type: [u8; 0x4],
    pub phase_grade_type: [u8; 0x4],
    pub height_grade_weight: [u8; 0x8],
    pub phase_grade_weight: [u8; 0x8],
    pub gisim_measure_bits: [u8; 0x10],
    pub adaptive_tap_measure_bits: [u8; 0x10],
    pub ber_bath_high_error_threshold: [u8; 0x10],
    pub ber_bath_mid_error_threshold: [u8; 0x10],
    pub ber_bath_low_error_threshold: [u8; 0x10],
    pub one_ratio_high_threshold: [u8; 0x10],
    pub one_ratio_high_mid_threshold: [u8; 0x10],
    pub one_ratio_low_mid_threshold: [u8; 0x10],
    pub one_ratio_low_threshold: [u8; 0x10],
    pub ndeo_error_threshold: [u8; 0x10],
    pub mixer_offset_step_size: [u8; 0x10],
    pub reserved_at_110: [u8; 0x8],
    pub mix90_phase_for_voltage_bath: [u8; 0x8],
    pub mixer_offset_start: [u8; 0x10],
    pub mixer_offset_end: [u8; 0x10],
    pub reserved_at_140: [u8; 0x15],
    pub ber_test_time: [u8; 0xb],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pspa_reg_bits {
    pub swid: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub sub_port: [u8; 0x8],
    pub reserved_at_18: [u8; 0x8],
    pub reserved_at_20: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pqdr_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x5],
    pub prio: [u8; 0x3],
    pub reserved_at_18: [u8; 0x6],
    pub mode: [u8; 0x2],
    pub reserved_at_20: [u8; 0x20],
    pub reserved_at_40: [u8; 0x10],
    pub min_threshold: [u8; 0x10],
    pub reserved_at_60: [u8; 0x10],
    pub max_threshold: [u8; 0x10],
    pub reserved_at_80: [u8; 0x10],
    pub mark_probability_denominator: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ppsc_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x60],
    pub reserved_at_80: [u8; 0x1c],
    pub wrps_admin: [u8; 0x4],
    pub reserved_at_a0: [u8; 0x1c],
    pub wrps_status: [u8; 0x4],
    pub reserved_at_c0: [u8; 0x8],
    pub up_threshold: [u8; 0x8],
    pub reserved_at_d0: [u8; 0x8],
    pub down_threshold: [u8; 0x8],
    pub reserved_at_e0: [u8; 0x20],
    pub reserved_at_100: [u8; 0x1c],
    pub srps_admin: [u8; 0x4],
    pub reserved_at_120: [u8; 0x1c],
    pub srps_status: [u8; 0x4],
    pub reserved_at_140: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pplr_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x8],
    pub lb_cap: [u8; 0x8],
    pub reserved_at_30: [u8; 0x8],
    pub lb_en: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pplm_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x20],
    pub port_profile_mode: [u8; 0x8],
    pub static_port_profile: [u8; 0x8],
    pub active_port_profile: [u8; 0x8],
    pub reserved_at_58: [u8; 0x8],
    pub retransmission_active: [u8; 0x8],
    pub fec_mode_active: [u8; 0x18],
    pub rs_fec_correction_bypass_cap: [u8; 0x4],
    pub reserved_at_84: [u8; 0x8],
    pub fec_override_cap_56g: [u8; 0x4],
    pub fec_override_cap_100g: [u8; 0x4],
    pub fec_override_cap_50g: [u8; 0x4],
    pub fec_override_cap_25g: [u8; 0x4],
    pub fec_override_cap_10g_40g: [u8; 0x4],
    pub rs_fec_correction_bypass_admin: [u8; 0x4],
    pub reserved_at_a4: [u8; 0x8],
    pub fec_override_admin_56g: [u8; 0x4],
    pub fec_override_admin_100g: [u8; 0x4],
    pub fec_override_admin_50g: [u8; 0x4],
    pub fec_override_admin_25g: [u8; 0x4],
    pub fec_override_admin_10g_40g: [u8; 0x4],
    pub fec_override_cap_400g_8x: [u8; 0x10],
    pub fec_override_cap_200g_4x: [u8; 0x10],
    pub fec_override_cap_100g_2x: [u8; 0x10],
    pub fec_override_cap_50g_1x: [u8; 0x10],
    pub fec_override_admin_400g_8x: [u8; 0x10],
    pub fec_override_admin_200g_4x: [u8; 0x10],
    pub fec_override_admin_100g_2x: [u8; 0x10],
    pub fec_override_admin_50g_1x: [u8; 0x10],
    pub fec_override_cap_800g_8x: [u8; 0x10],
    pub fec_override_cap_400g_4x: [u8; 0x10],
    pub fec_override_cap_200g_2x: [u8; 0x10],
    pub fec_override_cap_100g_1x: [u8; 0x10],
    pub reserved_at_180: [u8; 0xa0],
    pub fec_override_admin_800g_8x: [u8; 0x10],
    pub fec_override_admin_400g_4x: [u8; 0x10],
    pub fec_override_admin_200g_2x: [u8; 0x10],
    pub fec_override_admin_100g_1x: [u8; 0x10],
    pub reserved_at_260: [u8; 0x60],
    pub fec_override_cap_1600g_8x: [u8; 0x10],
    pub fec_override_cap_800g_4x: [u8; 0x10],
    pub fec_override_cap_400g_2x: [u8; 0x10],
    pub fec_override_cap_200g_1x: [u8; 0x10],
    pub fec_override_admin_1600g_8x: [u8; 0x10],
    pub fec_override_admin_800g_4x: [u8; 0x10],
    pub fec_override_admin_400g_2x: [u8; 0x10],
    pub fec_override_admin_200g_1x: [u8; 0x10],
    pub reserved_at_340: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ppcnt_reg_bits {
    pub swid: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub pnat: [u8; 0x2],
    pub reserved_at_12: [u8; 0x8],
    pub grp: [u8; 0x6],
    pub clr: [u8; 0x1],
    pub reserved_at_21: [u8; 0x13],
    pub plane_ind: [u8; 0x4],
    pub reserved_at_38: [u8; 0x3],
    pub prio_tc: [u8; 0x5],
    pub counter_set: mlx5_ifc_eth_cntrs_grp_data_layout_auto_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mpein_reg_bits {
    pub reserved_at_0: [u8; 0x2],
    pub depth: [u8; 0x6],
    pub pcie_index: [u8; 0x8],
    pub node: [u8; 0x8],
    pub reserved_at_18: [u8; 0x8],
    pub capability_mask: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub link_width_enabled: [u8; 0x8],
    pub link_speed_enabled: [u8; 0x10],
    pub lane0_physical_position: [u8; 0x8],
    pub link_width_active: [u8; 0x8],
    pub link_speed_active: [u8; 0x10],
    pub num_of_pfs: [u8; 0x10],
    pub num_of_vfs: [u8; 0x10],
    pub bdf0: [u8; 0x10],
    pub reserved_at_b0: [u8; 0x10],
    pub max_read_request_size: [u8; 0x4],
    pub max_payload_size: [u8; 0x4],
    pub reserved_at_c8: [u8; 0x5],
    pub pwr_status: [u8; 0x3],
    pub port_type: [u8; 0x4],
    pub reserved_at_d4: [u8; 0xb],
    pub lane_reversal: [u8; 0x1],
    pub reserved_at_e0: [u8; 0x14],
    pub pci_power: [u8; 0xc],
    pub reserved_at_100: [u8; 0x20],
    pub device_status: [u8; 0x10],
    pub port_state: [u8; 0x8],
    pub reserved_at_138: [u8; 0x8],
    pub reserved_at_140: [u8; 0x10],
    pub receiver_detect_result: [u8; 0x10],
    pub reserved_at_160: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mpcnt_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub pcie_index: [u8; 0x8],
    pub reserved_at_10: [u8; 0xa],
    pub grp: [u8; 0x6],
    pub clr: [u8; 0x1],
    pub reserved_at_21: [u8; 0x1f],
    pub counter_set: mlx5_ifc_pcie_cntrs_grp_data_layout_auto_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ppad_reg_bits {
    pub reserved_at_0: [u8; 0x3],
    pub single_mac: [u8; 0x1],
    pub reserved_at_4: [u8; 0x4],
    pub local_port: [u8; 0x8],
    pub mac_47_32: [u8; 0x10],
    pub mac_31_0: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pmtu_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub max_mtu: [u8; 0x10],
    pub reserved_at_30: [u8; 0x10],
    pub admin_mtu: [u8; 0x10],
    pub reserved_at_50: [u8; 0x10],
    pub oper_mtu: [u8; 0x10],
    pub reserved_at_70: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pmpr_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub module: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x18],
    pub attenuation_5g: [u8; 0x8],
    pub reserved_at_40: [u8; 0x18],
    pub attenuation_7g: [u8; 0x8],
    pub reserved_at_60: [u8; 0x18],
    pub attenuation_12g: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pmpe_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub module: [u8; 0x8],
    pub reserved_at_10: [u8; 0xc],
    pub module_status: [u8; 0x4],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pmpc_reg_bits {
    pub module_state_updated: [u8; 32][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pmlpn_reg_bits {
    pub reserved_at_0: [u8; 0x4],
    pub mlpn_status: [u8; 0x4],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub e: [u8; 0x1],
    pub reserved_at_21: [u8; 0x1f],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pmlp_reg_bits {
    pub rxtx: [u8; 0x1],
    pub reserved_at_1: [u8; 0x7],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub width: [u8; 0x8],
    pub lane0_module_mapping: [u8; 0x20],
    pub lane1_module_mapping: [u8; 0x20],
    pub lane2_module_mapping: [u8; 0x20],
    pub lane3_module_mapping: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x160],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pmaos_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub module: [u8; 0x8],
    pub reserved_at_10: [u8; 0x4],
    pub admin_status: [u8; 0x4],
    pub reserved_at_18: [u8; 0x4],
    pub oper_status: [u8; 0x4],
    pub ase: [u8; 0x1],
    pub ee: [u8; 0x1],
    pub reserved_at_22: [u8; 0x1c],
    pub e: [u8; 0x2],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_plpc_reg_bits {
    pub reserved_at_0: [u8; 0x4],
    pub profile_id: [u8; 0xc],
    pub reserved_at_10: [u8; 0x4],
    pub proto_mask: [u8; 0x4],
    pub reserved_at_18: [u8; 0x8],
    pub reserved_at_20: [u8; 0x10],
    pub lane_speed: [u8; 0x10],
    pub reserved_at_40: [u8; 0x17],
    pub lpbf: [u8; 0x1],
    pub fec_mode_policy: [u8; 0x8],
    pub retransmission_capability: [u8; 0x8],
    pub fec_mode_capability: [u8; 0x18],
    pub retransmission_support_admin: [u8; 0x8],
    pub fec_mode_support_admin: [u8; 0x18],
    pub retransmission_request_admin: [u8; 0x8],
    pub fec_mode_request_admin: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_plib_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub ib_port: [u8; 0x8],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_plbf_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0xd],
    pub lbf_mode: [u8; 0x3],
    pub reserved_at_20: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pipg_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub dic: [u8; 0x1],
    pub reserved_at_21: [u8; 0x19],
    pub ipg: [u8; 0x4],
    pub reserved_at_3e: [u8; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pifr_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0xe0],
    pub port_filter: [u8; 8][0x20],
    pub port_filter_update_en: [u8; 8][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pfcc_reg_bits {
    pub reserved_at_0: [u8; 0x4],
    pub buf_ownership: [u8; 0x2],
    pub reserved_at_6: [u8; 0x2],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0xa],
    pub cable_length_mask: [u8; 0x1],
    pub ppan_mask_n: [u8; 0x1],
    pub minor_stall_mask: [u8; 0x1],
    pub critical_stall_mask: [u8; 0x1],
    pub reserved_at_1e: [u8; 0x2],
    pub ppan: [u8; 0x4],
    pub reserved_at_24: [u8; 0x4],
    pub prio_mask_tx: [u8; 0x8],
    pub reserved_at_30: [u8; 0x8],
    pub prio_mask_rx: [u8; 0x8],
    pub pptx: [u8; 0x1],
    pub aptx: [u8; 0x1],
    pub pptx_mask_n: [u8; 0x1],
    pub reserved_at_43: [u8; 0x5],
    pub pfctx: [u8; 0x8],
    pub reserved_at_50: [u8; 0x10],
    pub pprx: [u8; 0x1],
    pub aprx: [u8; 0x1],
    pub pprx_mask_n: [u8; 0x1],
    pub reserved_at_63: [u8; 0x5],
    pub pfcrx: [u8; 0x8],
    pub reserved_at_70: [u8; 0x10],
    pub device_stall_minor_watermark: [u8; 0x10],
    pub device_stall_critical_watermark: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x18],
    pub cable_length: [u8; 0x8],
    pub reserved_at_c0: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pelc_reg_bits {
    pub op: [u8; 0x4],
    pub reserved_at_4: [u8; 0x4],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub op_admin: [u8; 0x8],
    pub op_capability: [u8; 0x8],
    pub op_request: [u8; 0x8],
    pub op_active: [u8; 0x8],
    pub admin: [u8; 0x40],
    pub capability: [u8; 0x40],
    pub request: [u8; 0x40],
    pub active: [u8; 0x40],
    pub reserved_at_140: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_peir_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0xc],
    pub error_count: [u8; 0x4],
    pub reserved_at_30: [u8; 0x10],
    pub reserved_at_40: [u8; 0xc],
    pub lane: [u8; 0x4],
    pub reserved_at_50: [u8; 0x8],
    pub error_type: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mpegc_reg_bits {
    pub reserved_at_0: [u8; 0x30],
    pub field_select: [u8; 0x10],
    pub tx_overflow_sense: [u8; 0x1],
    pub mark_cqe: [u8; 0x1],
    pub mark_cnp: [u8; 0x1],
    pub reserved_at_43: [u8; 0x1b],
    pub tx_lossy_overflow_oper: [u8; 0x2],
    pub reserved_at_60: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mpir_reg_bits {
    pub sdm: [u8; 0x1],
    pub reserved_at_1: [u8; 0x1b],
    pub host_buses: [u8; 0x4],
    pub reserved_at_20: [u8; 0x20],
    pub local_port: [u8; 0x8],
    pub reserved_at_28: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtutc_reg_bits {
    pub reserved_at_0: [u8; 0x5],
    pub freq_adj_units: [u8; 0x3],
    pub reserved_at_8: [u8; 0x3],
    pub log_max_freq_adjustment: [u8; 0x5],
    pub reserved_at_10: [u8; 0xc],
    pub operation: [u8; 0x4],
    pub freq_adjustment: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub utc_sec: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x2],
    pub utc_nsec: [u8; 0x1e],
    pub time_adjustment: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pcam_enhanced_features_bits {
    pub reserved_at_0: [u8; 0x10],
    pub ppcnt_recovery_counters: [u8; 0x1],
    pub reserved_at_11: [u8; 0x7],
    pub cable_length: [u8; 0x1],
    pub reserved_at_19: [u8; 0x4],
    pub fec_200G_per_lane_in_pplm: [u8; 0x1],
    pub reserved_at_1e: [u8; 0x2a],
    pub fec_100G_per_lane_in_pplm: [u8; 0x1],
    pub reserved_at_49: [u8; 0x2],
    pub shp_pbmc_pbsr_support: [u8; 0x1],
    pub reserved_at_4c: [u8; 0x7],
    pub buffer_ownership: [u8; 0x1],
    pub resereved_at_54: [u8; 0x14],
    pub fec_50G_per_lane_in_pplm: [u8; 0x1],
    pub reserved_at_69: [u8; 0x4],
    pub rx_icrc_encapsulated_counter: [u8; 0x1],
    pub reserved_at_6e: [u8; 0x4],
    pub ptys_extended_ethernet: [u8; 0x1],
    pub reserved_at_73: [u8; 0x3],
    pub pfcc_mask: [u8; 0x1],
    pub reserved_at_77: [u8; 0x3],
    pub per_lane_error_counters: [u8; 0x1],
    pub rx_buffer_fullness_counters: [u8; 0x1],
    pub ptys_connector_type: [u8; 0x1],
    pub reserved_at_7d: [u8; 0x1],
    pub ppcnt_discard_group: [u8; 0x1],
    pub ppcnt_statistical_group: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pcam_regs_5000_to_507f_bits {
    pub port_access_reg_cap_mask_127_to_96: [u8; 0x20],
    pub port_access_reg_cap_mask_95_to_64: [u8; 0x20],
    pub port_access_reg_cap_mask_63: [u8; 0x1],
    pub pphcr: [u8; 0x1],
    pub port_access_reg_cap_mask_61_to_36: [u8; 0x1a],
    pub pplm: [u8; 0x1],
    pub port_access_reg_cap_mask_34_to_32: [u8; 0x3],
    pub port_access_reg_cap_mask_31_to_13: [u8; 0x13],
    pub pbmc: [u8; 0x1],
    pub pptb: [u8; 0x1],
    pub port_access_reg_cap_mask_10_to_09: [u8; 0x2],
    pub ppcnt: [u8; 0x1],
    pub port_access_reg_cap_mask_07_to_00: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pcam_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub feature_group: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub access_reg_group: [u8; 0x8],
    pub reserved_at_20: [u8; 0x20],
    pub regs_5000_to_507f: mlx5_ifc_pcam_regs_5000_to_507f_bits,
    pub reserved_at_0: [u8; 0x80],
    pub port_access_reg_cap_mask: },
    pub reserved_at_c0: [u8; 0x80],
    pub enhanced_features: mlx5_ifc_pcam_enhanced_features_bits,
    pub reserved_at_0: [u8; 0x80],
    pub feature_cap_mask: },
    pub reserved_at_1c0: [u8; 0xc0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcam_enhanced_features_bits {
    pub reserved_at_0: [u8; 0x50],
    pub mtutc_freq_adj_units: [u8; 0x1],
    pub mtutc_time_adjustment_extended_range: [u8; 0x1],
    pub reserved_at_52: [u8; 0xb],
    pub mcia_32dwords: [u8; 0x1],
    pub out_pulse_duration_ns: [u8; 0x1],
    pub npps_period: [u8; 0x1],
    pub reserved_at_60: [u8; 0xa],
    pub reset_state: [u8; 0x1],
    pub ptpcyc2realtime_modify: [u8; 0x1],
    pub reserved_at_6c: [u8; 0x2],
    pub pci_status_and_power: [u8; 0x1],
    pub reserved_at_6f: [u8; 0x5],
    pub mark_tx_action_cnp: [u8; 0x1],
    pub mark_tx_action_cqe: [u8; 0x1],
    pub dynamic_tx_overflow: [u8; 0x1],
    pub reserved_at_77: [u8; 0x4],
    pub pcie_outbound_stalled: [u8; 0x1],
    pub tx_overflow_buffer_pkt: [u8; 0x1],
    pub mtpps_enh_out_per_adj: [u8; 0x1],
    pub mtpps_fs: [u8; 0x1],
    pub pcie_performance_group: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcam_access_reg_bits {
    pub reserved_at_0: [u8; 0x1c],
    pub mcda: [u8; 0x1],
    pub mcc: [u8; 0x1],
    pub mcqi: [u8; 0x1],
    pub mcqs: [u8; 0x1],
    pub regs_95_to_90: [u8; 0x6],
    pub mpir: [u8; 0x1],
    pub regs_88_to_87: [u8; 0x2],
    pub mpegc: [u8; 0x1],
    pub mtutc: [u8; 0x1],
    pub regs_84_to_68: [u8; 0x11],
    pub tracer_registers: [u8; 0x4],
    pub regs_63_to_46: [u8; 0x12],
    pub mrtc: [u8; 0x1],
    pub regs_44_to_41: [u8; 0x4],
    pub mfrl: [u8; 0x1],
    pub regs_39_to_32: [u8; 0x8],
    pub regs_31_to_11: [u8; 0x15],
    pub mtmp: [u8; 0x1],
    pub regs_9_to_0: [u8; 0xa],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcam_access_reg_bits1 {
    pub regs_127_to_96: [u8; 0x20],
    pub regs_95_to_64: [u8; 0x20],
    pub regs_63_to_32: [u8; 0x20],
    pub regs_31_to_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcam_access_reg_bits2 {
    pub regs_127_to_99: [u8; 0x1d],
    pub mirc: [u8; 0x1],
    pub regs_97_to_96: [u8; 0x2],
    pub regs_95_to_87: [u8; 0x09],
    pub synce_registers: [u8; 0x2],
    pub regs_84_to_64: [u8; 0x15],
    pub regs_63_to_32: [u8; 0x20],
    pub regs_31_to_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcam_access_reg_bits3 {
    pub regs_127_to_96: [u8; 0x20],
    pub regs_95_to_64: [u8; 0x20],
    pub regs_63_to_32: [u8; 0x20],
    pub regs_31_to_3: [u8; 0x1d],
    pub mrtcq: [u8; 0x1],
    pub mtctr: [u8; 0x1],
    pub mtptm: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcam_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub feature_group: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub access_reg_group: [u8; 0x8],
    pub reserved_at_20: [u8; 0x20],
    pub access_regs: mlx5_ifc_mcam_access_reg_bits,
    pub access_regs1: mlx5_ifc_mcam_access_reg_bits1,
    pub access_regs2: mlx5_ifc_mcam_access_reg_bits2,
    pub access_regs3: mlx5_ifc_mcam_access_reg_bits3,
    pub reserved_at_0: [u8; 0x80],
    pub mng_access_reg_cap_mask: },
    pub reserved_at_c0: [u8; 0x80],
    pub enhanced_features: mlx5_ifc_mcam_enhanced_features_bits,
    pub reserved_at_0: [u8; 0x80],
    pub mng_feature_cap_mask: },
    pub reserved_at_1c0: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qcam_access_reg_cap_mask {
    pub qcam_access_reg_cap_mask_127_to_20: [u8; 0x6C],
    pub qpdpm: [u8; 0x1],
    pub qcam_access_reg_cap_mask_18_to_4: [u8; 0x0F],
    pub qdpm: [u8; 0x1],
    pub qpts: [u8; 0x1],
    pub qcap: [u8; 0x1],
    pub qcam_access_reg_cap_mask_0: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qcam_qos_feature_cap_mask {
    pub qcam_qos_feature_cap_mask_127_to_5: [u8; 0x7B],
    pub qetcr_qshr_max_bw_val_msb: [u8; 0x1],
    pub qcam_qos_feature_cap_mask_3_to_1: [u8; 0x3],
    pub qpts_trust_both: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qcam_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub feature_group: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub access_reg_group: [u8; 0x8],
    pub reserved_at_20: [u8; 0x20],
    pub reg_cap: mlx5_ifc_qcam_access_reg_cap_mask,
    pub reserved_at_0: [u8; 0x80],
    pub qos_access_reg_cap_mask: },
    pub reserved_at_c0: [u8; 0x80],
    pub feature_cap: mlx5_ifc_qcam_qos_feature_cap_mask,
    pub reserved_at_0: [u8; 0x80],
    pub qos_feature_cap_mask: },
    pub reserved_at_1c0: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_core_dump_reg_bits {
    pub reserved_at_0: [u8; 0x18],
    pub core_dump_type: [u8; 0x8],
    pub reserved_at_20: [u8; 0x30],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x8],
    pub qpn: [u8; 0x18],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pcap_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub port_capability_mask: [u8; 4][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_paos_reg_bits {
    pub swid: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x4],
    pub admin_status: [u8; 0x4],
    pub reserved_at_18: [u8; 0x4],
    pub oper_status: [u8; 0x4],
    pub ase: [u8; 0x1],
    pub ee: [u8; 0x1],
    pub reserved_at_22: [u8; 0x1c],
    pub e: [u8; 0x2],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pamp_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub opamp_group: [u8; 0x8],
    pub reserved_at_10: [u8; 0xc],
    pub opamp_group_type: [u8; 0x4],
    pub start_index: [u8; 0x10],
    pub reserved_at_30: [u8; 0x4],
    pub num_of_indices: [u8; 0xc],
    pub index_data: [u8; 18][0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pcmr_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub entropy_force_cap: [u8; 0x1],
    pub entropy_calc_cap: [u8; 0x1],
    pub entropy_gre_calc_cap: [u8; 0x1],
    pub reserved_at_23: [u8; 0xf],
    pub rx_ts_over_crc_cap: [u8; 0x1],
    pub reserved_at_33: [u8; 0xb],
    pub fcs_cap: [u8; 0x1],
    pub reserved_at_3f: [u8; 0x1],
    pub entropy_force: [u8; 0x1],
    pub entropy_calc: [u8; 0x1],
    pub entropy_gre_calc: [u8; 0x1],
    pub reserved_at_43: [u8; 0xf],
    pub rx_ts_over_crc: [u8; 0x1],
    pub reserved_at_53: [u8; 0xb],
    pub fcs_chk: [u8; 0x1],
    pub reserved_at_5f: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_lane_2_module_mapping_bits {
    pub reserved_at_0: [u8; 0x4],
    pub rx_lane: [u8; 0x4],
    pub reserved_at_8: [u8; 0x4],
    pub tx_lane: [u8; 0x4],
    pub reserved_at_10: [u8; 0x8],
    pub module: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_bufferx_reg_bits {
    pub reserved_at_0: [u8; 0x6],
    pub lossy: [u8; 0x1],
    pub epsb: [u8; 0x1],
    pub reserved_at_8: [u8; 0x8],
    pub size: [u8; 0x10],
    pub xoff_threshold: [u8; 0x10],
    pub xon_threshold: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_node_in_bits {
    pub node_description: [u8; 64][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_register_power_settings_bits {
    pub reserved_at_0: [u8; 0x18],
    pub power_settings_level: [u8; 0x8],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_register_host_endianness_bits {
    pub he: [u8; 0x1],
    pub reserved_at_1: [u8; 0x1f],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_umr_pointer_desc_argument_bits {
    pub reserved_at_0: [u8; 0x20],
    pub mkey: [u8; 0x20],
    pub addressh_63_32: [u8; 0x20],
    pub addressl_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ud_adrs_vector_bits {
    pub dc_key: [u8; 0x40],
    pub ext: [u8; 0x1],
    pub reserved_at_41: [u8; 0x7],
    pub destination_qp_dct: [u8; 0x18],
    pub static_rate: [u8; 0x4],
    pub sl_eth_prio: [u8; 0x4],
    pub fl: [u8; 0x1],
    pub mlid: [u8; 0x7],
    pub rlid_udp_sport: [u8; 0x10],
    pub reserved_at_80: [u8; 0x20],
    pub rmac_47_16: [u8; 0x20],
    pub rmac_15_0: [u8; 0x10],
    pub tclass: [u8; 0x8],
    pub hop_limit: [u8; 0x8],
    pub reserved_at_e0: [u8; 0x1],
    pub grh: [u8; 0x1],
    pub reserved_at_e2: [u8; 0x2],
    pub src_addr_index: [u8; 0x8],
    pub flow_label: [u8; 0x14],
    pub rgid_rip: [u8; 16][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pages_req_event_bits {
    pub reserved_at_0: [u8; 0x10],
    pub function_id: [u8; 0x10],
    pub num_pages: [u8; 0x20],
    pub reserved_at_40: [u8; 0xa0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_eqe_bits {
    pub reserved_at_0: [u8; 0x8],
    pub event_type: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub event_sub_type: [u8; 0x8],
    pub reserved_at_20: [u8; 0xe0],
    pub event_data: mlx5_ifc_event_auto_bits,
    pub reserved_at_1e0: [u8; 0x10],
    pub signature: [u8; 0x8],
    pub reserved_at_1f8: [u8; 0x7],
    pub owner: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cmd_queue_entry_bits {
    pub type: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub input_length: [u8; 0x20],
    pub input_mailbox_pointer_63_32: [u8; 0x20],
    pub input_mailbox_pointer_31_9: [u8; 0x17],
    pub reserved_at_77: [u8; 0x9],
    pub command_input_inline_data: [u8; 16][0x8],
    pub command_output_inline_data: [u8; 16][0x8],
    pub output_mailbox_pointer_63_32: [u8; 0x20],
    pub output_mailbox_pointer_31_9: [u8; 0x17],
    pub reserved_at_1b7: [u8; 0x9],
    pub output_length: [u8; 0x20],
    pub token: [u8; 0x8],
    pub signature: [u8; 0x8],
    pub reserved_at_1f0: [u8; 0x8],
    pub status: [u8; 0x7],
    pub ownership: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cmd_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub command_output: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cmd_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub command: [u8; ][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_cmd_if_box_bits {
    pub mailbox_data: [u8; 512][0x8],
    pub reserved_at_1000: [u8; 0x180],
    pub next_pointer_63_32: [u8; 0x20],
    pub next_pointer_31_10: [u8; 0x16],
    pub reserved_at_11b6: [u8; 0xa],
    pub block_number: [u8; 0x20],
    pub reserved_at_11e0: [u8; 0x8],
    pub token: [u8; 0x8],
    pub ctrl_signature: [u8; 0x8],
    pub signature: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtt_bits {
    pub ptag_63_32: [u8; 0x20],
    pub ptag_31_8: [u8; 0x18],
    pub reserved_at_38: [u8; 0x6],
    pub wr_en: [u8; 0x1],
    pub rd_en: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_wol_rol_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x10],
    pub rol_mode: [u8; 0x8],
    pub wol_mode: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_wol_rol_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_wol_rol_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_wol_rol_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub rol_mode_valid: [u8; 0x1],
    pub wol_mode_valid: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub rol_mode: [u8; 0x8],
    pub wol_mode: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_initial_seg_bits {
    pub fw_rev_minor: [u8; 0x10],
    pub fw_rev_major: [u8; 0x10],
    pub cmd_interface_rev: [u8; 0x10],
    pub fw_rev_subminor: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub cmdq_phy_addr_63_32: [u8; 0x20],
    pub cmdq_phy_addr_31_12: [u8; 0x14],
    pub reserved_at_b4: [u8; 0x2],
    pub nic_interface: [u8; 0x2],
    pub log_cmdq_size: [u8; 0x4],
    pub log_cmdq_stride: [u8; 0x4],
    pub command_doorbell_vector: [u8; 0x20],
    pub reserved_at_e0: [u8; 0xf00],
    pub initializing: [u8; 0x1],
    pub reserved_at_fe1: [u8; 0x4],
    pub nic_interface_supported: [u8; 0x3],
    pub embedded_cpu: [u8; 0x1],
    pub reserved_at_fe9: [u8; 0x17],
    pub health_buffer: mlx5_ifc_health_buffer_bits,
    pub no_dram_nic_offset: [u8; 0x20],
    pub reserved_at_1220: [u8; 0x6e40],
    pub reserved_at_8060: [u8; 0x1f],
    pub clear_int: [u8; 0x1],
    pub health_syndrome: [u8; 0x8],
    pub health_counter: [u8; 0x18],
    pub reserved_at_80a0: [u8; 0x17fc0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtpps_reg_bits {
    pub reserved_at_0: [u8; 0xc],
    pub cap_number_of_pps_pins: [u8; 0x4],
    pub reserved_at_10: [u8; 0x4],
    pub cap_max_num_of_pps_in_pins: [u8; 0x4],
    pub reserved_at_18: [u8; 0x4],
    pub cap_max_num_of_pps_out_pins: [u8; 0x4],
    pub reserved_at_20: [u8; 0x13],
    pub cap_log_min_npps_period: [u8; 0x5],
    pub reserved_at_38: [u8; 0x3],
    pub cap_log_min_out_pulse_duration_ns: [u8; 0x5],
    pub reserved_at_40: [u8; 0x4],
    pub cap_pin_3_mode: [u8; 0x4],
    pub reserved_at_48: [u8; 0x4],
    pub cap_pin_2_mode: [u8; 0x4],
    pub reserved_at_50: [u8; 0x4],
    pub cap_pin_1_mode: [u8; 0x4],
    pub reserved_at_58: [u8; 0x4],
    pub cap_pin_0_mode: [u8; 0x4],
    pub reserved_at_60: [u8; 0x4],
    pub cap_pin_7_mode: [u8; 0x4],
    pub reserved_at_68: [u8; 0x4],
    pub cap_pin_6_mode: [u8; 0x4],
    pub reserved_at_70: [u8; 0x4],
    pub cap_pin_5_mode: [u8; 0x4],
    pub reserved_at_78: [u8; 0x4],
    pub cap_pin_4_mode: [u8; 0x4],
    pub field_select: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x20],
    pub npps_period: [u8; 0x40],
    pub enable: [u8; 0x1],
    pub reserved_at_101: [u8; 0xb],
    pub pattern: [u8; 0x4],
    pub reserved_at_110: [u8; 0x4],
    pub pin_mode: [u8; 0x4],
    pub pin: [u8; 0x8],
    pub reserved_at_120: [u8; 0x2],
    pub out_pulse_duration_ns: [u8; 0x1e],
    pub time_stamp: [u8; 0x40],
    pub out_pulse_duration: [u8; 0x10],
    pub out_periodic_adjustment: [u8; 0x10],
    pub enhanced_out_periodic_adjustment: [u8; 0x20],
    pub reserved_at_1c0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtppse_reg_bits {
    pub reserved_at_0: [u8; 0x18],
    pub pin: [u8; 0x8],
    pub event_arm: [u8; 0x1],
    pub reserved_at_21: [u8; 0x1b],
    pub event_generation_mode: [u8; 0x4],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcqs_reg_bits {
    pub last_index_flag: [u8; 0x1],
    pub reserved_at_1: [u8; 0x7],
    pub fw_device: [u8; 0x8],
    pub component_index: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub identifier: [u8; 0x10],
    pub reserved_at_40: [u8; 0x17],
    pub component_status: [u8; 0x5],
    pub component_update_state: [u8; 0x4],
    pub last_update_state_changer_type: [u8; 0x4],
    pub last_update_state_changer_host_id: [u8; 0x4],
    pub reserved_at_68: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcqi_cap_bits {
    pub supported_info_bitmask: [u8; 0x20],
    pub component_size: [u8; 0x20],
    pub max_component_size: [u8; 0x20],
    pub log_mcda_word_size: [u8; 0x4],
    pub reserved_at_64: [u8; 0xc],
    pub mcda_max_write_size: [u8; 0x10],
    pub rd_en: [u8; 0x1],
    pub reserved_at_81: [u8; 0x1],
    pub match_chip_id: [u8; 0x1],
    pub match_psid: [u8; 0x1],
    pub check_user_timestamp: [u8; 0x1],
    pub match_base_guid_mac: [u8; 0x1],
    pub reserved_at_86: [u8; 0x1a],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcqi_version_bits {
    pub reserved_at_0: [u8; 0x2],
    pub build_time_valid: [u8; 0x1],
    pub user_defined_time_valid: [u8; 0x1],
    pub reserved_at_4: [u8; 0x14],
    pub version_string_length: [u8; 0x8],
    pub version: [u8; 0x20],
    pub build_time: [u8; 0x40],
    pub user_defined_time: [u8; 0x40],
    pub build_tool_version: [u8; 0x20],
    pub reserved_at_e0: [u8; 0x20],
    pub version_string: [u8; 92][0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcqi_activation_method_bits {
    pub pending_server_ac_power_cycle: [u8; 0x1],
    pub pending_server_dc_power_cycle: [u8; 0x1],
    pub pending_server_reboot: [u8; 0x1],
    pub pending_fw_reset: [u8; 0x1],
    pub auto_activate: [u8; 0x1],
    pub all_hosts_sync: [u8; 0x1],
    pub device_hw_reset: [u8; 0x1],
    pub reserved_at_7: [u8; 0x19],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_mcqi_reg_data_bits {
    pub mcqi_caps: mlx5_ifc_mcqi_cap_bits,
    pub mcqi_version: mlx5_ifc_mcqi_version_bits,
    pub mcqi_activation_mathod: mlx5_ifc_mcqi_activation_method_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcqi_reg_bits {
    pub read_pending_component: [u8; 0x1],
    pub reserved_at_1: [u8; 0xf],
    pub component_index: [u8; 0x10],
    pub reserved_at_20: [u8; 0x20],
    pub reserved_at_40: [u8; 0x1b],
    pub info_type: [u8; 0x5],
    pub info_size: [u8; 0x20],
    pub offset: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x10],
    pub data_size: [u8; 0x10],
    pub data: [mlx5_ifc_mcqi_reg_data_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcc_reg_bits {
    pub reserved_at_0: [u8; 0x4],
    pub time_elapsed_since_last_cmd: [u8; 0xc],
    pub reserved_at_10: [u8; 0x8],
    pub instruction: [u8; 0x8],
    pub reserved_at_20: [u8; 0x10],
    pub component_index: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub update_handle: [u8; 0x18],
    pub handle_owner_type: [u8; 0x4],
    pub handle_owner_host_id: [u8; 0x4],
    pub reserved_at_68: [u8; 0x1],
    pub control_progress: [u8; 0x7],
    pub error_code: [u8; 0x8],
    pub reserved_at_78: [u8; 0x4],
    pub control_state: [u8; 0x4],
    pub component_size: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcda_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub update_handle: [u8; 0x18],
    pub offset: [u8; 0x20],
    pub reserved_at_40: [u8; 0x10],
    pub size: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub data: [u8; ][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mfrl_reg_bits {
    pub reserved_at_0: [u8; 0x20],
    pub reserved_at_20: [u8; 0x2],
    pub pci_sync_for_fw_update_start: [u8; 0x1],
    pub pci_sync_for_fw_update_resp: [u8; 0x2],
    pub rst_type_sel: [u8; 0x3],
    pub pci_reset_req_method: [u8; 0x3],
    pub reserved_at_2b: [u8; 0x1],
    pub reset_state: [u8; 0x4],
    pub reset_type: [u8; 0x8],
    pub reset_level: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mirc_reg_bits {
    pub reserved_at_0: [u8; 0x18],
    pub status_code: [u8; 0x8],
    pub reserved_at_20: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pddr_monitor_opcode_bits {
    pub reserved_at_0: [u8; 0x10],
    pub monitor_opcode: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_pddr_troubleshooting_page_status_opcode_auto_bits {
    pub pddr_monitor_opcode: mlx5_ifc_pddr_monitor_opcode_bits,
    pub reserved_at_0: [u8; 0x20],
}

// Monitor opcodes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pddr_troubleshooting_page_bits {
    pub reserved_at_0: [u8; 0x10],
    pub group_opcode: [u8; 0x10],
    pub status_opcode: mlx5_ifc_pddr_troubleshooting_page_status_opcode_auto_bits,
    pub reserved_at_40: [u8; 0x20],
    pub status_message: [u8; 59][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_pddr_reg_page_data_auto_bits {
    pub pddr_troubleshooting_page: mlx5_ifc_pddr_troubleshooting_page_bits,
    pub reserved_at_0: [u8; 0x7c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pddr_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub pnat: [u8; 0x2],
    pub reserved_at_12: [u8; 0xe],
    pub reserved_at_20: [u8; 0x18],
    pub page_select: [u8; 0x8],
    pub page_data: mlx5_ifc_pddr_reg_page_data_auto_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mrtc_reg_bits {
    pub time_synced: [u8; 0x1],
    pub reserved_at_1: [u8; 0x1f],
    pub reserved_at_20: [u8; 0x20],
    pub time_h: [u8; 0x20],
    pub time_l: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtcap_reg_bits {
    pub reserved_at_0: [u8; 0x19],
    pub sensor_count: [u8; 0x7],
    pub reserved_at_20: [u8; 0x20],
    pub sensor_map: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtmp_reg_bits {
    pub reserved_at_0: [u8; 0x14],
    pub sensor_index: [u8; 0xc],
    pub reserved_at_20: [u8; 0x10],
    pub temperature: [u8; 0x10],
    pub mte: [u8; 0x1],
    pub mtr: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub max_temperature: [u8; 0x10],
    pub tee: [u8; 0x2],
    pub reserved_at_62: [u8; 0xe],
    pub temp_threshold_hi: [u8; 0x10],
    pub reserved_at_80: [u8; 0x10],
    pub temp_threshold_lo: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x20],
    pub sensor_name_hi: [u8; 0x20],
    pub sensor_name_lo: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtptm_reg_bits {
    pub reserved_at_0: [u8; 0x10],
    pub psta: [u8; 0x1],
    pub reserved_at_11: [u8; 0xf],
    pub reserved_at_20: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtctr_reg_bits {
    pub first_clock_timestamp_request: [u8; 0x8],
    pub second_clock_timestamp_request: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub first_clock_valid: [u8; 0x1],
    pub second_clock_valid: [u8; 0x1],
    pub reserved_at_22: [u8; 0x1e],
    pub first_clock_timestamp: [u8; 0x40],
    pub second_clock_timestamp: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_bin_range_layout_bits {
    pub reserved_at_0: [u8; 0xa],
    pub high_val: [u8; 0x6],
    pub reserved_at_10: [u8; 0xa],
    pub low_val: [u8; 0x6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pphcr_reg_bits {
    pub active_hist_type: [u8; 0x4],
    pub reserved_at_4: [u8; 0x4],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x8],
    pub num_of_bins: [u8; 0x8],
    pub reserved_at_30: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub bin_range: [mlx5_ifc_bin_range_layout_bits; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_ports_control_registers_document_bits {
    pub bufferx_reg: mlx5_ifc_bufferx_reg_bits,
    pub eth_2819_cntrs_grp_data_layout: mlx5_ifc_eth_2819_cntrs_grp_data_layout_bits,
    pub eth_2863_cntrs_grp_data_layout: mlx5_ifc_eth_2863_cntrs_grp_data_layout_bits,
    pub eth_3635_cntrs_grp_data_layout: mlx5_ifc_eth_3635_cntrs_grp_data_layout_bits,
    pub eth_802_3_cntrs_grp_data_layout: mlx5_ifc_eth_802_3_cntrs_grp_data_layout_bits,
    pub eth_extended_cntrs_grp_data_layout: mlx5_ifc_eth_extended_cntrs_grp_data_layout_bits,
    pub eth_per_prio_grp_data_layout: mlx5_ifc_eth_per_prio_grp_data_layout_bits,
    pub eth_per_tc_prio_grp_data_layout: mlx5_ifc_eth_per_tc_prio_grp_data_layout_bits,
    pub eth_per_tc_congest_prio_grp_data_layout: mlx5_ifc_eth_per_tc_congest_prio_grp_data_layout_bits,
    pub lane_2_module_mapping: mlx5_ifc_lane_2_module_mapping_bits,
    pub pamp_reg: mlx5_ifc_pamp_reg_bits,
    pub paos_reg: mlx5_ifc_paos_reg_bits,
    pub pcap_reg: mlx5_ifc_pcap_reg_bits,
    pub pddr_monitor_opcode: mlx5_ifc_pddr_monitor_opcode_bits,
    pub pddr_reg: mlx5_ifc_pddr_reg_bits,
    pub pddr_troubleshooting_page: mlx5_ifc_pddr_troubleshooting_page_bits,
    pub peir_reg: mlx5_ifc_peir_reg_bits,
    pub pelc_reg: mlx5_ifc_pelc_reg_bits,
    pub pfcc_reg: mlx5_ifc_pfcc_reg_bits,
    pub ib_port_cntrs_grp_data_layout: mlx5_ifc_ib_port_cntrs_grp_data_layout_bits,
    pub phys_layer_cntrs: mlx5_ifc_phys_layer_cntrs_bits,
    pub pifr_reg: mlx5_ifc_pifr_reg_bits,
    pub pipg_reg: mlx5_ifc_pipg_reg_bits,
    pub plbf_reg: mlx5_ifc_plbf_reg_bits,
    pub plib_reg: mlx5_ifc_plib_reg_bits,
    pub plpc_reg: mlx5_ifc_plpc_reg_bits,
    pub pmaos_reg: mlx5_ifc_pmaos_reg_bits,
    pub pmlp_reg: mlx5_ifc_pmlp_reg_bits,
    pub pmlpn_reg: mlx5_ifc_pmlpn_reg_bits,
    pub pmpc_reg: mlx5_ifc_pmpc_reg_bits,
    pub pmpe_reg: mlx5_ifc_pmpe_reg_bits,
    pub pmpr_reg: mlx5_ifc_pmpr_reg_bits,
    pub pmtu_reg: mlx5_ifc_pmtu_reg_bits,
    pub ppad_reg: mlx5_ifc_ppad_reg_bits,
    pub ppcnt_reg: mlx5_ifc_ppcnt_reg_bits,
    pub mpein_reg: mlx5_ifc_mpein_reg_bits,
    pub mpcnt_reg: mlx5_ifc_mpcnt_reg_bits,
    pub pplm_reg: mlx5_ifc_pplm_reg_bits,
    pub pplr_reg: mlx5_ifc_pplr_reg_bits,
    pub ppsc_reg: mlx5_ifc_ppsc_reg_bits,
    pub pqdr_reg: mlx5_ifc_pqdr_reg_bits,
    pub pspa_reg: mlx5_ifc_pspa_reg_bits,
    pub ptas_reg: mlx5_ifc_ptas_reg_bits,
    pub ptys_reg: mlx5_ifc_ptys_reg_bits,
    pub mlcr_reg: mlx5_ifc_mlcr_reg_bits,
    pub pude_reg: mlx5_ifc_pude_reg_bits,
    pub pvlc_reg: mlx5_ifc_pvlc_reg_bits,
    pub slrg_reg: mlx5_ifc_slrg_reg_bits,
    pub sltp_reg: mlx5_ifc_sltp_reg_bits,
    pub mtpps_reg: mlx5_ifc_mtpps_reg_bits,
    pub mtppse_reg: mlx5_ifc_mtppse_reg_bits,
    pub fpga_access_reg: mlx5_ifc_fpga_access_reg_bits,
    pub fpga_ctrl_bits: mlx5_ifc_fpga_ctrl_bits,
    pub fpga_cap_bits: mlx5_ifc_fpga_cap_bits,
    pub mcqi_reg: mlx5_ifc_mcqi_reg_bits,
    pub mcc_reg: mlx5_ifc_mcc_reg_bits,
    pub mcda_reg: mlx5_ifc_mcda_reg_bits,
    pub mirc_reg: mlx5_ifc_mirc_reg_bits,
    pub mfrl_reg: mlx5_ifc_mfrl_reg_bits,
    pub mtutc_reg: mlx5_ifc_mtutc_reg_bits,
    pub mrtc_reg: mlx5_ifc_mrtc_reg_bits,
    pub mtcap_reg: mlx5_ifc_mtcap_reg_bits,
    pub mtmp_reg: mlx5_ifc_mtmp_reg_bits,
    pub mtptm_reg: mlx5_ifc_mtptm_reg_bits,
    pub mtctr_reg: mlx5_ifc_mtctr_reg_bits,
    pub pphcr_reg: mlx5_ifc_pphcr_reg_bits,
    pub reserved_at_0: [u8; 0x60e0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_debug_enhancements_document_bits {
    pub health_buffer: mlx5_ifc_health_buffer_bits,
    pub reserved_at_0: [u8; 0x200],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_uplink_pci_interface_document_bits {
    pub initial_seg: mlx5_ifc_initial_seg_bits,
    pub reserved_at_0: [u8; 0x20060],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_flow_table_root_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_set_flow_table_root_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub other_eswitch: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x10],
    pub eswitch_owner_vhca_id: [u8; 0x10],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x7],
    pub table_of_other_vport: [u8; 0x1],
    pub table_vport_number: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x8],
    pub underlay_qpn: [u8; 0x18],
    pub table_eswitch_owner_vhca_id_valid: [u8; 0x1],
    pub reserved_at_e1: [u8; 0xf],
    pub table_eswitch_owner_vhca_id: [u8; 0x10],
    pub reserved_at_100: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_flow_table_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_flow_table_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub other_vport: [u8; 0x1],
    pub other_eswitch: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vport_number: [u8; 0x10],
    pub reserved_at_60: [u8; 0x10],
    pub modify_field_select: [u8; 0x10],
    pub table_type: [u8; 0x8],
    pub reserved_at_88: [u8; 0x8],
    pub eswitch_owner_vhca_id: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x8],
    pub table_id: [u8; 0x18],
    pub flow_table_context: mlx5_ifc_flow_table_context_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ets_tcn_config_reg_bits {
    pub g: [u8; 0x1],
    pub b: [u8; 0x1],
    pub r: [u8; 0x1],
    pub reserved_at_3: [u8; 0x9],
    pub group: [u8; 0x4],
    pub reserved_at_10: [u8; 0x9],
    pub bw_allocation: [u8; 0x7],
    pub reserved_at_20: [u8; 0xc],
    pub max_bw_units: [u8; 0x4],
    pub max_bw_value: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ets_global_config_reg_bits {
    pub reserved_at_0: [u8; 0x2],
    pub r: [u8; 0x1],
    pub reserved_at_3: [u8; 0x1d],
    pub reserved_at_20: [u8; 0xc],
    pub max_bw_units: [u8; 0x4],
    pub reserved_at_30: [u8; 0x8],
    pub max_bw_value: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qetc_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub port_number: [u8; 0x8],
    pub reserved_at_10: [u8; 0x30],
    pub tc_configuration: [mlx5_ifc_ets_tcn_config_reg_bits; 0x8],
    pub global_configuration: mlx5_ifc_ets_global_config_reg_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qpdpm_dscp_reg_bits {
    pub e: [u8; 0x1],
    pub reserved_at_01: [u8; 0x0b],
    pub prio: [u8; 0x04],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qpdpm_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub dscp: [mlx5_ifc_qpdpm_dscp_reg_bits; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qpts_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x2d],
    pub trust_state: [u8; 0x3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pptb_reg_bits {
    pub reserved_at_0: [u8; 0x2],
    pub mm: [u8; 0x2],
    pub reserved_at_4: [u8; 0x4],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x6],
    pub cm: [u8; 0x1],
    pub um: [u8; 0x1],
    pub pm: [u8; 0x8],
    pub prio_x_buff: [u8; 0x20],
    pub pm_msb: [u8; 0x8],
    pub reserved_at_48: [u8; 0x10],
    pub ctrl_buff: [u8; 0x4],
    pub untagged_buff: [u8; 0x4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sbcam_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub feature_group: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub access_reg_group: [u8; 0x8],
    pub reserved_at_20: [u8; 0x20],
    pub sb_access_reg_cap_mask: [u8; 4][0x20],
    pub reserved_at_c0: [u8; 0x80],
    pub sb_feature_cap_mask: [u8; 4][0x20],
    pub reserved_at_1c0: [u8; 0x40],
    pub cap_total_buffer_size: [u8; 0x20],
    pub cap_cell_size: [u8; 0x10],
    pub cap_max_pg_buffers: [u8; 0x8],
    pub cap_num_pool_supported: [u8; 0x8],
    pub reserved_at_240: [u8; 0x8],
    pub cap_sbsr_stat_size: [u8; 0x8],
    pub cap_max_tclass_data: [u8; 0x8],
    pub cap_max_cpu_ingress_tclass_sb: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pbmc_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub reserved_at_10: [u8; 0x10],
    pub xoff_timer_value: [u8; 0x10],
    pub xoff_refresh: [u8; 0x10],
    pub reserved_at_40: [u8; 0x9],
    pub fullness_threshold: [u8; 0x7],
    pub port_buffer_size: [u8; 0x10],
    pub buffer: [mlx5_ifc_bufferx_reg_bits; 10],
    pub shared_headroom_pool: mlx5_ifc_bufferx_reg_bits,
    pub reserved_at_320: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sbpr_reg_bits {
    pub desc: [u8; 0x1],
    pub snap: [u8; 0x1],
    pub reserved_at_2: [u8; 0x4],
    pub dir: [u8; 0x2],
    pub reserved_at_8: [u8; 0x14],
    pub pool: [u8; 0x4],
    pub infi_size: [u8; 0x1],
    pub reserved_at_21: [u8; 0x7],
    pub size: [u8; 0x18],
    pub reserved_at_40: [u8; 0x1c],
    pub mode: [u8; 0x4],
    pub reserved_at_60: [u8; 0x8],
    pub buff_occupancy: [u8; 0x18],
    pub clr: [u8; 0x1],
    pub reserved_at_81: [u8; 0x7],
    pub max_buff_occupancy: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x8],
    pub ext_buff_occupancy: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sbcm_reg_bits {
    pub desc: [u8; 0x1],
    pub snap: [u8; 0x1],
    pub reserved_at_2: [u8; 0x6],
    pub local_port: [u8; 0x8],
    pub pnat: [u8; 0x2],
    pub pg_buff: [u8; 0x6],
    pub reserved_at_18: [u8; 0x6],
    pub dir: [u8; 0x2],
    pub reserved_at_20: [u8; 0x1f],
    pub exc: [u8; 0x1],
    pub reserved_at_40: [u8; 0x40],
    pub reserved_at_80: [u8; 0x8],
    pub buff_occupancy: [u8; 0x18],
    pub clr: [u8; 0x1],
    pub reserved_at_a1: [u8; 0x7],
    pub max_buff_occupancy: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x8],
    pub min_buff: [u8; 0x18],
    pub infi_max: [u8; 0x1],
    pub reserved_at_e1: [u8; 0x7],
    pub max_buff: [u8; 0x18],
    pub reserved_at_100: [u8; 0x20],
    pub reserved_at_120: [u8; 0x1c],
    pub pool: [u8; 0x4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_qtct_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub port_number: [u8; 0x8],
    pub reserved_at_10: [u8; 0xd],
    pub prio: [u8; 0x3],
    pub reserved_at_20: [u8; 0x1d],
    pub tclass: [u8; 0x3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mcia_reg_bits {
    pub l: [u8; 0x1],
    pub reserved_at_1: [u8; 0x7],
    pub module: [u8; 0x8],
    pub reserved_at_10: [u8; 0x8],
    pub status: [u8; 0x8],
    pub i2c_device_address: [u8; 0x8],
    pub page_number: [u8; 0x8],
    pub device_address: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub size: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub dwords: [u8; 0x400],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dcbx_param_bits {
    pub dcbx_cee_cap: [u8; 0x1],
    pub dcbx_ieee_cap: [u8; 0x1],
    pub dcbx_standby_cap: [u8; 0x1],
    pub reserved_at_3: [u8; 0x5],
    pub port_number: [u8; 0x8],
    pub reserved_at_10: [u8; 0xa],
    pub max_application_table_size: [u8; 6],
    pub reserved_at_20: [u8; 0x15],
    pub version_oper: [u8; 0x3],
    pub reserved_at_38: [u8; 5],
    pub version_admin: [u8; 0x3],
    pub willing_admin: [u8; 0x1],
    pub reserved_at_41: [u8; 0x3],
    pub pfc_cap_oper: [u8; 0x4],
    pub reserved_at_48: [u8; 0x4],
    pub pfc_cap_admin: [u8; 0x4],
    pub reserved_at_50: [u8; 0x4],
    pub num_of_tc_oper: [u8; 0x4],
    pub reserved_at_58: [u8; 0x4],
    pub num_of_tc_admin: [u8; 0x4],
    pub remote_willing: [u8; 0x1],
    pub reserved_at_61: [u8; 3],
    pub remote_pfc_cap: [u8; 4],
    pub reserved_at_68: [u8; 0x14],
    pub remote_num_of_tc: [u8; 0x4],
    pub reserved_at_80: [u8; 0x18],
    pub error: [u8; 0x8],
    pub reserved_at_a0: [u8; 0x160],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_lagc_bits {
    pub fdb_selection_mode: [u8; 0x1],
    pub reserved_at_1: [u8; 0x14],
    pub port_select_mode: [u8; 0x3],
    pub reserved_at_18: [u8; 0x5],
    pub lag_state: [u8; 0x3],
    pub reserved_at_20: [u8; 0xc],
    pub active_port: [u8; 0x4],
    pub reserved_at_30: [u8; 0x4],
    pub tx_remap_affinity_2: [u8; 0x4],
    pub reserved_at_38: [u8; 0x4],
    pub tx_remap_affinity_1: [u8; 0x4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_lag_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_lag_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub ctx: mlx5_ifc_lagc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_lag_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_lag_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub field_select: [u8; 0x20],
    pub ctx: mlx5_ifc_lagc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_lag_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub ctx: mlx5_ifc_lagc_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_lag_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_lag_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_lag_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_vport_lag_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_vport_lag_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_vport_lag_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_vport_lag_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_memic_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub reserved_at_60: [u8; 0x18],
    pub memic_operation_type: [u8; 0x8],
    pub memic_start_addr: [u8; 0x40],
    pub reserved_at_c0: [u8; 0x140],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_memic_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
    pub memic_operation_addr: [u8; 0x40],
    pub reserved_at_c0: [u8; 0x140],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_memic_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_30: [u8; 0x20],
    pub reserved_at_40: [u8; 0x18],
    pub log_memic_addr_alignment: [u8; 0x8],
    pub range_start_addr: [u8; 0x40],
    pub range_size: [u8; 0x20],
    pub memic_size: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_memic_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub memic_start_addr: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_memic_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub memic_start_addr: [u8; 0x40],
    pub memic_size: [u8; 0x20],
    pub reserved_at_e0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_memic_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_umem_bits {
    pub reserved_at_0: [u8; 0x80],
    pub ats: [u8; 0x1],
    pub reserved_at_81: [u8; 0x1a],
    pub log_page_size: [u8; 0x5],
    pub page_offset: [u8; 0x20],
    pub num_of_mtt: [u8; 0x40],
    pub mtt: [mlx5_ifc_mtt_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_uctx_bits {
    pub cap: [u8; 0x20],
    pub reserved_at_20: [u8; 0x160],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sw_icm_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x18],
    pub log_sw_icm_size: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
    pub sw_icm_start_addr: [u8; 0x40],
    pub reserved_at_c0: [u8; 0x140],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_geneve_tlv_option_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x18],
    pub geneve_option_fte_index: [u8; 0x8],
    pub option_class: [u8; 0x10],
    pub option_type: [u8; 0x8],
    pub reserved_at_78: [u8; 0x3],
    pub option_data_length: [u8; 0x5],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_umem_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub umem: mlx5_ifc_umem_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_umem_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x8],
    pub umem_id: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_umem_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub umem_id: [u8; 0x18],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_umem_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_uctx_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
    pub uctx: mlx5_ifc_uctx_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_uctx_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_uctx_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_destroy_uctx_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_sw_icm_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub sw_icm: mlx5_ifc_sw_icm_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_geneve_tlv_option_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub geneve_tlv_opt: mlx5_ifc_geneve_tlv_option_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtrc_string_db_param_bits {
    pub string_db_base_address: [u8; 0x20],
    pub reserved_at_20: [u8; 0x8],
    pub string_db_size: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtrc_cap_bits {
    pub trace_owner: [u8; 0x1],
    pub trace_to_memory: [u8; 0x1],
    pub reserved_at_2: [u8; 0x4],
    pub trc_ver: [u8; 0x2],
    pub reserved_at_8: [u8; 0x14],
    pub num_string_db: [u8; 0x4],
    pub first_string_trace: [u8; 0x8],
    pub num_string_trace: [u8; 0x8],
    pub reserved_at_30: [u8; 0x28],
    pub log_max_trace_buffer_size: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
    pub string_db_param: [mlx5_ifc_mtrc_string_db_param_bits; 8],
    pub reserved_at_280: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtrc_conf_bits {
    pub reserved_at_0: [u8; 0x1c],
    pub trace_mode: [u8; 0x4],
    pub reserved_at_20: [u8; 0x18],
    pub log_trace_buffer_size: [u8; 0x8],
    pub trace_mkey: [u8; 0x20],
    pub reserved_at_60: [u8; 0x3a0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtrc_stdb_bits {
    pub string_db_index: [u8; 0x4],
    pub reserved_at_4: [u8; 0x4],
    pub read_size: [u8; 0x18],
    pub start_offset: [u8; 0x20],
    pub string_db_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mtrc_ctrl_bits {
    pub trace_status: [u8; 0x2],
    pub reserved_at_2: [u8; 0x2],
    pub arm_event: [u8; 0x1],
    pub reserved_at_5: [u8; 0xb],
    pub modify_field_select: [u8; 0x10],
    pub reserved_at_20: [u8; 0x2b],
    pub current_timestamp52_32: [u8; 0x15],
    pub current_timestamp31_0: [u8; 0x20],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_host_params_context_bits {
    pub host_number: [u8; 0x8],
    pub reserved_at_8: [u8; 0x5],
    pub host_pf_not_exist: [u8; 0x1],
    pub reserved_at_14: [u8; 0x1],
    pub host_pf_disabled: [u8; 0x1],
    pub host_num_of_vfs: [u8; 0x10],
    pub host_total_vfs: [u8; 0x10],
    pub host_pci_bus: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub host_pci_device: [u8; 0x10],
    pub reserved_at_60: [u8; 0x10],
    pub host_pci_function: [u8; 0x10],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ifc_vhca_state {
    MLX5_VHCA_STATE_INVALID = 0x0,
    MLX5_VHCA_STATE_ALLOCATED = 0x1,
    MLX5_VHCA_STATE_ACTIVE = 0x2,
    MLX5_VHCA_STATE_IN_USE = 0x3,
    MLX5_VHCA_STATE_TEARDOWN_REQUEST = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_network_function_params_bits {
    pub host_number: [u8; 0x8],
    pub pci_pf_type: [u8; 0x4],
    pub reserved_at_c: [u8; 0x4],
    pub pci_num_vfs: [u8; 0x10],
    pub pci_total_vfs: [u8; 0x10],
    pub pci_bus: [u8; 0x8],
    pub pci_device_function: [u8; 0x8],
    pub vhca_id: [u8; 0x10],
    pub vhca_state: [u8; 0x4],
    pub reserved_at_54: [u8; 0xc],
    pub reserved_at_60: [u8; 0xa],
    pub esw_vport_manual: [u8; 0x1],
    pub pci_bus_assigned: [u8; 0x1],
    pub pci_vf_info_valid: [u8; 0x1],
    pub reserved_at_6d: [u8; 0x13],
    pub pci_vf_stride: [u8; 0x10],
    pub pci_first_vf_offset: [u8; 0x10],
    pub reserved_at_a0: [u8; 0x160],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ifc_net_function_params_bits {
    pub host_params_context: mlx5_ifc_host_params_context_bits,
    pub network_function_params: mlx5_ifc_network_function_params_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_esw_functions_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_esw_functions_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub net_function_num: [u8; 0x8],
    pub reserved_at_68: [u8; 0x18],
    pub reserved_at_80: [u8; 0x380],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sf_partition_bits {
    pub reserved_at_0: [u8; 0x10],
    pub log_num_sf: [u8; 0x8],
    pub log_sf_bar_size: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_sf_partitions_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x18],
    pub num_sf_partitions: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
    pub sf_partition: [mlx5_ifc_sf_partition_bits; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_sf_partitions_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_sf_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_dealloc_sf_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_sf_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_alloc_sf_in_bits {
    pub opcode: [u8; 0x10],
    pub reserved_at_10: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub function_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_affiliated_event_header_bits {
    pub reserved_at_0: [u8; 0x10],
    pub obj_type: [u8; 0x10],
    pub obj_id: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ipsec_aso_bits {
    pub valid: [u8; 0x1],
    pub reserved_at_201: [u8; 0x1],
    pub mode: [u8; 0x2],
    pub window_sz: [u8; 0x2],
    pub soft_lft_arm: [u8; 0x1],
    pub hard_lft_arm: [u8; 0x1],
    pub remove_flow_enable: [u8; 0x1],
    pub esn_event_arm: [u8; 0x1],
    pub reserved_at_20a: [u8; 0x16],
    pub remove_flow_pkt_cnt: [u8; 0x20],
    pub remove_flow_soft_lft: [u8; 0x20],
    pub reserved_at_260: [u8; 0x80],
    pub mode_parameter: [u8; 0x20],
    pub replay_protection_window: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_ipsec_obj_bits {
    pub modify_field_select: [u8; 0x40],
    pub full_offload: [u8; 0x1],
    pub reserved_at_41: [u8; 0x1],
    pub esn_en: [u8; 0x1],
    pub esn_overlap: [u8; 0x1],
    pub reserved_at_44: [u8; 0x2],
    pub icv_length: [u8; 0x2],
    pub reserved_at_48: [u8; 0x4],
    pub aso_return_reg: [u8; 0x4],
    pub reserved_at_50: [u8; 0x10],
    pub esn_msb: [u8; 0x20],
    pub reserved_at_80: [u8; 0x8],
    pub dekn: [u8; 0x18],
    pub salt: [u8; 0x20],
    pub implicit_iv: [u8; 0x40],
    pub reserved_at_100: [u8; 0x8],
    pub ipsec_aso_access_pd: [u8; 0x18],
    pub reserved_at_120: [u8; 0xe0],
    pub ipsec_aso: mlx5_ifc_ipsec_aso_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_ipsec_obj_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub ipsec_object: mlx5_ifc_ipsec_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_ipsec_obj_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
    pub ipsec_object: mlx5_ifc_ipsec_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_ipsec_obj_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub ipsec_object: mlx5_ifc_ipsec_obj_bits,
}

pub const MLX5_MACSEC_ASO_INC_SN: c_uint = 0x2;
pub const MLX5_MACSEC_ASO_REG_C_4_5: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_macsec_aso_bits {
    pub valid: [u8; 0x1],
    pub reserved_at_1: [u8; 0x1],
    pub mode: [u8; 0x2],
    pub window_size: [u8; 0x2],
    pub soft_lifetime_arm: [u8; 0x1],
    pub hard_lifetime_arm: [u8; 0x1],
    pub remove_flow_enable: [u8; 0x1],
    pub epn_event_arm: [u8; 0x1],
    pub reserved_at_a: [u8; 0x16],
    pub remove_flow_packet_count: [u8; 0x20],
    pub remove_flow_soft_lifetime: [u8; 0x20],
    pub reserved_at_60: [u8; 0x80],
    pub mode_parameter: [u8; 0x20],
    pub replay_protection_window: [u8; 8][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_macsec_offload_obj_bits {
    pub modify_field_select: [u8; 0x40],
    pub confidentiality_en: [u8; 0x1],
    pub reserved_at_41: [u8; 0x1],
    pub epn_en: [u8; 0x1],
    pub epn_overlap: [u8; 0x1],
    pub reserved_at_44: [u8; 0x2],
    pub confidentiality_offset: [u8; 0x2],
    pub reserved_at_48: [u8; 0x4],
    pub aso_return_reg: [u8; 0x4],
    pub reserved_at_50: [u8; 0x10],
    pub epn_msb: [u8; 0x20],
    pub reserved_at_80: [u8; 0x8],
    pub dekn: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x20],
    pub sci: [u8; 0x40],
    pub reserved_at_100: [u8; 0x8],
    pub macsec_aso_access_pd: [u8; 0x18],
    pub reserved_at_120: [u8; 0x60],
    pub salt: [u8; 3][0x20],
    pub reserved_at_1e0: [u8; 0x20],
    pub macsec_aso: mlx5_ifc_macsec_aso_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_macsec_obj_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub macsec_object: mlx5_ifc_macsec_offload_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_macsec_obj_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub macsec_object: mlx5_ifc_macsec_offload_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_macsec_obj_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
    pub macsec_object: mlx5_ifc_macsec_offload_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_wrapped_dek_bits {
    pub gcm_iv: [u8; 0x60],
    pub reserved_at_60: [u8; 0x20],
    pub const0: [u8; 0x1],
    pub key_size: [u8; 0x1],
    pub reserved_at_82: [u8; 0x2],
    pub key2_invalid: [u8; 0x1],
    pub reserved_at_85: [u8; 0x3],
    pub pd: [u8; 0x18],
    pub key_purpose: [u8; 0x5],
    pub reserved_at_a5: [u8; 0x13],
    pub kek_id: [u8; 0x8],
    pub reserved_at_c0: [u8; 0x40],
    pub key1: [u8; 0x8][0x20],
    pub key2: [u8; 0x8][0x20],
    pub reserved_at_300: [u8; 0x40],
    pub const1: [u8; 0x1],
    pub reserved_at_341: [u8; 0x1f],
    pub reserved_at_360: [u8; 0x20],
    pub auth_tag: [u8; 0x80],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_encryption_key_obj_bits {
    pub modify_field_select: [u8; 0x40],
    pub state: [u8; 0x8],
    pub sw_wrapped: [u8; 0x1],
    pub reserved_at_49: [u8; 0xb],
    pub key_size: [u8; 0x4],
    pub reserved_at_58: [u8; 0x4],
    pub key_purpose: [u8; 0x4],
    pub reserved_at_60: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub reserved_at_80: [u8; 0x100],
    pub opaque: [u8; 0x40],
    pub reserved_at_1c0: [u8; 0x40],
    pub key: [u8; 8][0x80],
    pub sw_wrapped_dek: [u8; 8][0x80],
    pub reserved_at_a00: [u8; 0x600],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_encryption_key_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub encryption_key_object: mlx5_ifc_encryption_key_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_encryption_key_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub encryption_key_object: mlx5_ifc_encryption_key_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_meter_parameters_bits {
    pub valid: [u8; 0x1],
    pub bucket_overflow: [u8; 0x1],
    pub start_color: [u8; 0x2],
    pub both_buckets_on_green: [u8; 0x1],
    pub reserved_at_5: [u8; 0x1],
    pub meter_mode: [u8; 0x2],
    pub reserved_at_8: [u8; 0x18],
    pub reserved_at_20: [u8; 0x20],
    pub reserved_at_40: [u8; 0x3],
    pub cbs_exponent: [u8; 0x5],
    pub cbs_mantissa: [u8; 0x8],
    pub reserved_at_50: [u8; 0x3],
    pub cir_exponent: [u8; 0x5],
    pub cir_mantissa: [u8; 0x8],
    pub reserved_at_60: [u8; 0x20],
    pub reserved_at_80: [u8; 0x3],
    pub ebs_exponent: [u8; 0x5],
    pub ebs_mantissa: [u8; 0x8],
    pub reserved_at_90: [u8; 0x3],
    pub eir_exponent: [u8; 0x5],
    pub eir_mantissa: [u8; 0x8],
    pub reserved_at_a0: [u8; 0x60],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_flow_meter_aso_obj_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x40],
    pub reserved_at_80: [u8; 0x8],
    pub meter_aso_access_pd: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x160],
    pub flow_meter_parameters: [mlx5_ifc_flow_meter_parameters_bits; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_flow_meter_aso_obj_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub flow_meter_aso_obj: mlx5_ifc_flow_meter_aso_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_int_kek_obj_bits {
    pub modify_field_select: [u8; 0x40],
    pub state: [u8; 0x8],
    pub auto_gen: [u8; 0x1],
    pub reserved_at_49: [u8; 0xb],
    pub key_size: [u8; 0x4],
    pub reserved_at_58: [u8; 0x8],
    pub reserved_at_60: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub reserved_at_80: [u8; 0x180],
    pub key: [u8; 8][0x80],
    pub reserved_at_600: [u8; 0x200],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_int_kek_obj_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub int_kek_object: mlx5_ifc_int_kek_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_int_kek_obj_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
    pub int_kek_object: mlx5_ifc_int_kek_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_sampler_obj_bits {
    pub modify_field_select: [u8; 0x40],
    pub table_type: [u8; 0x8],
    pub level: [u8; 0x8],
    pub reserved_at_50: [u8; 0xf],
    pub ignore_flow_level: [u8; 0x1],
    pub sample_ratio: [u8; 0x20],
    pub reserved_at_80: [u8; 0x8],
    pub sample_table_id: [u8; 0x18],
    pub reserved_at_a0: [u8; 0x8],
    pub default_table_id: [u8; 0x18],
    pub sw_steering_icm_address_rx: [u8; 0x40],
    pub sw_steering_icm_address_tx: [u8; 0x40],
    pub reserved_at_140: [u8; 0xa0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_sampler_obj_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub sampler_object: mlx5_ifc_sampler_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_sampler_obj_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
    pub sampler_object: mlx5_ifc_sampler_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tls_static_params_bits {
    pub const_2: [u8; 0x2],
    pub tls_version: [u8; 0x4],
    pub const_1: [u8; 0x2],
    pub reserved_at_8: [u8; 0x14],
    pub encryption_standard: [u8; 0x4],
    pub reserved_at_20: [u8; 0x20],
    pub initial_record_number: [u8; 0x40],
    pub resync_tcp_sn: [u8; 0x20],
    pub gcm_iv: [u8; 0x20],
    pub implicit_iv: [u8; 0x40],
    pub reserved_at_100: [u8; 0x8],
    pub dek_index: [u8; 0x18],
    pub reserved_at_120: [u8; 0xe0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_tls_progress_params_bits {
    pub next_record_tcp_sn: [u8; 0x20],
    pub hw_resync_tcp_sn: [u8; 0x20],
    pub record_tracker_state: [u8; 0x2],
    pub auth_state: [u8; 0x2],
    pub reserved_at_44: [u8; 0x4],
    pub hw_offset_record_number: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_suspend_vhca_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_suspend_vhca_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resume_vhca_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_resume_vhca_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vhca_migration_state_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub incremental: [u8; 0x1],
    pub chunk: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_vhca_migration_state_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x20],
    pub migration_state: [u8; 0x4],
    pub reserved_at_64: [u8; 0x1c],
    pub required_umem_size: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x20],
    pub remaining_total_size: [u8; 0x40],
    pub reserved_at_100: [u8; 0x100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_save_vhca_state_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub incremental: [u8; 0x1],
    pub set_track: [u8; 0x1],
    pub reserved_at_42: [u8; 0xe],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub va: [u8; 0x40],
    pub mkey: [u8; 0x20],
    pub size: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_save_vhca_state_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub actual_image_size: [u8; 0x20],
    pub next_required_umem_size: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_load_vhca_state_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x10],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub va: [u8; 0x40],
    pub mkey: [u8; 0x20],
    pub size: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_load_vhca_state_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_adv_rdma_cap_bits {
    pub rdma_transport_manager: [u8; 0x1],
    pub rdma_transport_manager_other_eswitch: [u8; 0x1],
    pub reserved_at_2: [u8; 0x1e],
    pub rcx_type: [u8; 0x8],
    pub reserved_at_28: [u8; 0x2],
    pub ps_entry_log_max_value: [u8; 0x6],
    pub reserved_at_30: [u8; 0x6],
    pub qp_max_ps_num_entry: [u8; 0xa],
    pub mp_max_num_queues: [u8; 0x8],
    pub ps_user_context_max_log_size: [u8; 0x8],
    pub message_based_qp_and_striding_wq: [u8; 0x8],
    pub reserved_at_58: [u8; 0x8],
    pub max_receive_send_message_size_stride: [u8; 0x10],
    pub reserved_at_70: [u8; 0x10],
    pub max_receive_send_message_size_byte: [u8; 0x20],
    pub reserved_at_a0: [u8; 0x160],
    pub rdma_transport_rx_flow_table_properties: mlx5_ifc_flow_table_prop_layout_bits,
    pub rdma_transport_tx_flow_table_properties: mlx5_ifc_flow_table_prop_layout_bits,
    pub rdma_transport_rx_ft_field_support_2: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub rdma_transport_tx_ft_field_support_2: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub rdma_transport_rx_ft_field_bitmask_support_2: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub rdma_transport_tx_ft_field_bitmask_support_2: mlx5_ifc_flow_table_fields_supported_2_bits,
    pub reserved_at_800: [u8; 0x3800],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_adv_virtualization_cap_bits {
    pub reserved_at_0: [u8; 0x3],
    pub pg_track_log_max_num: [u8; 0x5],
    pub pg_track_max_num_range: [u8; 0x8],
    pub pg_track_log_min_addr_space: [u8; 0x8],
    pub pg_track_log_max_addr_space: [u8; 0x8],
    pub reserved_at_20: [u8; 0x3],
    pub pg_track_log_min_msg_size: [u8; 0x5],
    pub reserved_at_28: [u8; 0x3],
    pub pg_track_log_max_msg_size: [u8; 0x5],
    pub reserved_at_30: [u8; 0x3],
    pub pg_track_log_min_page_size: [u8; 0x5],
    pub reserved_at_38: [u8; 0x3],
    pub pg_track_log_max_page_size: [u8; 0x5],
    pub reserved_at_40: [u8; 0x7c0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_page_track_report_entry_bits {
    pub dirty_address_high: [u8; 0x20],
    pub dirty_address_low: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_page_track_range_bits {
    pub start_address: [u8; 0x40],
    pub length: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_page_track_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x10],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub state: [u8; 0x4],
    pub track_type: [u8; 0x4],
    pub log_addr_space_size: [u8; 0x8],
    pub reserved_at_90: [u8; 0x3],
    pub log_page_size: [u8; 0x5],
    pub reserved_at_98: [u8; 0x3],
    pub log_msg_size: [u8; 0x5],
    pub reserved_at_a0: [u8; 0x8],
    pub reporting_qpn: [u8; 0x18],
    pub reserved_at_c0: [u8; 0x18],
    pub num_ranges: [u8; 0x8],
    pub reserved_at_e0: [u8; 0x20],
    pub range_start_address: [u8; 0x40],
    pub length: [u8; 0x40],
    pub track_range: [mlx5_ifc_page_track_range_bits; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_create_page_track_obj_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_page_track_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_modify_page_track_obj_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_page_track_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_query_page_track_obj_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_page_track_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_msecq_reg_bits {
    pub reserved_at_0: [u8; 0x20],
    pub reserved_at_20: [u8; 0x12],
    pub network_option: [u8; 0x2],
    pub local_ssm_code: [u8; 0x4],
    pub local_enhanced_ssm_code: [u8; 0x8],
    pub local_clock_identity: [u8; 0x40],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_msees_admin_status {
    MLX5_MSEES_ADMIN_STATUS_FREE_RUNNING		= 0x0,
    MLX5_MSEES_ADMIN_STATUS_TRACK			= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_msees_oper_status {
    MLX5_MSEES_OPER_STATUS_FREE_RUNNING		= 0x0,
    MLX5_MSEES_OPER_STATUS_SELF_TRACK		= 0x1,
    MLX5_MSEES_OPER_STATUS_OTHER_TRACK		= 0x2,
    MLX5_MSEES_OPER_STATUS_HOLDOVER			= 0x3,
    MLX5_MSEES_OPER_STATUS_FAIL_HOLDOVER		= 0x4,
    MLX5_MSEES_OPER_STATUS_FAIL_FREE_RUNNING	= 0x5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_msees_failure_reason {
    MLX5_MSEES_FAILURE_REASON_UNDEFINED_ERROR		= 0x0,
    MLX5_MSEES_FAILURE_REASON_PORT_DOWN			= 0x1,
    MLX5_MSEES_FAILURE_REASON_TOO_HIGH_FREQUENCY_DIFF	= 0x2,
    MLX5_MSEES_FAILURE_REASON_NET_SYNCHRONIZER_DEVICE_ERROR	= 0x3,
    MLX5_MSEES_FAILURE_REASON_LACK_OF_RESOURCES		= 0x4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_msees_reg_bits {
    pub reserved_at_0: [u8; 0x8],
    pub local_port: [u8; 0x8],
    pub pnat: [u8; 0x2],
    pub lp_msb: [u8; 0x2],
    pub reserved_at_14: [u8; 0xc],
    pub field_select: [u8; 0x20],
    pub admin_status: [u8; 0x4],
    pub oper_status: [u8; 0x4],
    pub ho_acq: [u8; 0x1],
    pub reserved_at_49: [u8; 0xc],
    pub admin_freq_measure: [u8; 0x1],
    pub oper_freq_measure: [u8; 0x1],
    pub failure_reason: [u8; 0x9],
    pub frequency_diff: [u8; 0x20],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_mrtcq_reg_bits {
    pub reserved_at_0: [u8; 0x40],
    pub rt_clock_identity: [u8; 0x40],
    pub reserved_at_80: [u8; 0x180],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pcie_cong_event_obj_bits {
    pub modify_select_field: [u8; 0x40],
    pub inbound_event_en: [u8; 0x1],
    pub outbound_event_en: [u8; 0x1],
    pub reserved_at_42: [u8; 0x1e],
    pub reserved_at_60: [u8; 0x1],
    pub inbound_cong_state: [u8; 0x3],
    pub reserved_at_64: [u8; 0x1],
    pub outbound_cong_state: [u8; 0x3],
    pub reserved_at_68: [u8; 0x18],
    pub inbound_cong_low_threshold: [u8; 0x10],
    pub inbound_cong_high_threshold: [u8; 0x10],
    pub outbound_cong_low_threshold: [u8; 0x10],
    pub outbound_cong_high_threshold: [u8; 0x10],
    pub reserved_at_e0: [u8; 0x340],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pcie_cong_event_cmd_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub cong_obj: mlx5_ifc_pcie_cong_event_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_pcie_cong_event_cmd_out_bits {
    pub hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
    pub cong_obj: mlx5_ifc_pcie_cong_event_obj_bits,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_pcie_cong_event_mod_field {
    MLX5_PCIE_CONG_EVENT_MOD_EVENT_EN = BIT(0),
    MLX5_PCIE_CONG_EVENT_MOD_THRESH   = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_psp_rotate_key_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_psp_rotate_key_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x40],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_psp_gen_spi_in_key_size {
    MLX5_PSP_GEN_SPI_IN_KEY_SIZE_128 = 0x0,
    MLX5_PSP_GEN_SPI_IN_KEY_SIZE_256 = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_key_spi_bits {
    pub spi: [u8; 0x20],
    pub reserved_at_20: [u8; 0x60],
    pub key: [u8; 8][0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_psp_gen_spi_in_bits {
    pub opcode: [u8; 0x10],
    pub uid: [u8; 0x10],
    pub reserved_at_20: [u8; 0x10],
    pub op_mod: [u8; 0x10],
    pub reserved_at_40: [u8; 0x20],
    pub key_size: [u8; 0x2],
    pub reserved_at_62: [u8; 0xe],
    pub num_of_spi: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_psp_gen_spi_out_bits {
    pub status: [u8; 0x8],
    pub reserved_at_8: [u8; 0x18],
    pub syndrome: [u8; 0x20],
    pub reserved_at_40: [u8; 0x10],
    pub num_of_spi: [u8; 0x10],
    pub reserved_at_60: [u8; 0x20],
    pub key_spi: [mlx5_ifc_key_spi_bits; ],
}
