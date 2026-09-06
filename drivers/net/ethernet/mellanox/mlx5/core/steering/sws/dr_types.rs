//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/sws/dr_types.h
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

pub const DR_RULE_MAX_STES: c_int = 18;
pub const DR_ACTION_MAX_STES: c_int = 5;
pub const DR_STE_SVLAN: c_uint = 0x1;
pub const DR_STE_CVLAN: c_uint = 0x2;

pub const DR_NUM_OF_FLEX_PARSERS: c_int = 8;
pub const DR_STE_MAX_FLEX_0_ID: c_int = 3;
pub const DR_STE_MAX_FLEX_1_ID: c_int = 7;
pub const DR_ACTION_CACHE_LINE_SIZE: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_icm_chunk_size {
    DR_CHUNK_SIZE_1,
    DR_CHUNK_SIZE_MIN = DR_CHUNK_SIZE_1, /* keep updated when changing */
    DR_CHUNK_SIZE_2,
    DR_CHUNK_SIZE_4,
    DR_CHUNK_SIZE_8,
    DR_CHUNK_SIZE_16,
    DR_CHUNK_SIZE_32,
    DR_CHUNK_SIZE_64,
    DR_CHUNK_SIZE_128,
    DR_CHUNK_SIZE_256,
    DR_CHUNK_SIZE_512,
    DR_CHUNK_SIZE_1K,
    DR_CHUNK_SIZE_2K,
    DR_CHUNK_SIZE_4K,
    DR_CHUNK_SIZE_8K,
    DR_CHUNK_SIZE_16K,
    DR_CHUNK_SIZE_32K,
    DR_CHUNK_SIZE_64K,
    DR_CHUNK_SIZE_128K,
    DR_CHUNK_SIZE_256K,
    DR_CHUNK_SIZE_512K,
    DR_CHUNK_SIZE_1024K,
    DR_CHUNK_SIZE_2048K,
    DR_CHUNK_SIZE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_icm_type {
    DR_ICM_TYPE_STE,
    DR_ICM_TYPE_MODIFY_ACTION,
    DR_ICM_TYPE_MODIFY_HDR_PTRN,
    DR_ICM_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_ste_ctx_action_cap {
    DR_STE_CTX_ACTION_CAP_NONE = 0,
    DR_STE_CTX_ACTION_CAP_TX_POP   = 1 << 0,
    DR_STE_CTX_ACTION_CAP_RX_PUSH  = 1 << 1,
    DR_STE_CTX_ACTION_CAP_RX_ENCAP = 1 << 2,
    DR_STE_CTX_ACTION_CAP_POP_MDFY = 1 << 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_matcher_criteria {
    DR_MATCHER_CRITERIA_EMPTY = 0,
    DR_MATCHER_CRITERIA_OUTER = 1 << 0,
    DR_MATCHER_CRITERIA_MISC = 1 << 1,
    DR_MATCHER_CRITERIA_INNER = 1 << 2,
    DR_MATCHER_CRITERIA_MISC2 = 1 << 3,
    DR_MATCHER_CRITERIA_MISC3 = 1 << 4,
    DR_MATCHER_CRITERIA_MISC4 = 1 << 5,
    DR_MATCHER_CRITERIA_MISC5 = 1 << 6,
    DR_MATCHER_CRITERIA_MAX = 1 << 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_action_type {
    DR_ACTION_TYP_TNL_L2_TO_L2,
    DR_ACTION_TYP_L2_TO_TNL_L2,
    DR_ACTION_TYP_TNL_L3_TO_L2,
    DR_ACTION_TYP_L2_TO_TNL_L3,
    DR_ACTION_TYP_DROP,
    DR_ACTION_TYP_QP,
    DR_ACTION_TYP_FT,
    DR_ACTION_TYP_CTR,
    DR_ACTION_TYP_TAG,
    DR_ACTION_TYP_MODIFY_HDR,
    DR_ACTION_TYP_VPORT,
    DR_ACTION_TYP_POP_VLAN,
    DR_ACTION_TYP_PUSH_VLAN,
    DR_ACTION_TYP_INSERT_HDR,
    DR_ACTION_TYP_REMOVE_HDR,
    DR_ACTION_TYP_SAMPLER,
    DR_ACTION_TYP_ASO_FLOW_METER,
    DR_ACTION_TYP_RANGE,
    DR_ACTION_TYP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_ipv {
    DR_RULE_IPV4,
    DR_RULE_IPV6,
    DR_RULE_IPV_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_ste {
// refcount: indicates the num of rules that using this ste
    pub refcount: u32,
// this ste is part of a rule, located in ste's chain
    pub ste_chain_location: u8,
// attached to the miss_list head at each htbl entry
    pub miss_list_node: list_head,
// this ste is member of htbl
    pub htbl: *mut mlx5dr_ste_htbl,
    pub next_htbl: *mut mlx5dr_ste_htbl,
// The rule this STE belongs to
    pub rule_rx_tx: *mut mlx5dr_rule_rx_tx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_ste_htbl_ctrl {
// total number of valid entries belonging to this hash table. This
// includes the non collision and collision entries
//
    pub num_of_valid_entries: c_uint,
// total number of collisions entries attached to this table
    pub num_of_collisions: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_ste_htbl {
    pub lu_type: u16,
    pub byte_mask: u16,
    pub refcount: u32,
    pub chunk: *mut mlx5dr_icm_chunk,
    pub pointing_ste: *mut mlx5dr_ste,
    pub ctrl: mlx5dr_ste_htbl_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_ste_send_info {
    pub ste: *mut mlx5dr_ste,
    pub send_list: list_head,
    pub size: u16,
    pub offset: u16,
    pub data_cont: [u8; DR_STE_SIZE],
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_ste_build {
    pub inner:1: u8,
    pub rx:1: u8,
    pub vhca_id_valid:1: u8,
    pub dmn: *mut mlx5dr_domain,
    pub caps: *mut mlx5dr_cmd_caps,
    pub lu_type: u16,
    pub byte_mask: u16,
    pub bit_mask: [u8; DR_STE_SIZE_MASK],
    pub tag): *mut u8,
}

extern "C" {
    pub fn mlx5dr_ste_htbl_free(htbl: *mut mlx5dr_ste_htbl) -> c_int;
}
// STE utils
extern "C" {
    pub fn mlx5dr_ste_calc_hash_index(hw_ste_p: *mut u8, htbl: *mut mlx5dr_ste_htbl) -> u32;
}
extern "C" {
    pub fn mlx5dr_ste_is_miss_addr_set(ste_ctx: *mut mlx5dr_ste_ctx, hw_ste_p: *mut u8) -> bool;
}
extern "C" {
    pub fn mlx5dr_ste_set_bit_mask(hw_ste_p: *mut u8, bit_mask: *mut u8);
}
extern "C" {
    pub fn mlx5dr_ste_get_icm_addr(ste: *mut mlx5dr_ste) -> u64;
}
extern "C" {
    pub fn mlx5dr_ste_get_mr_addr(ste: *mut mlx5dr_ste) -> u64;
}
pub const MLX5DR_MAX_VLANS: c_int = 2;
pub const MLX5DR_INVALID_PATTERN_INDEX: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_ste_actions_attr {
    pub modify_index: u32,
    pub modify_pat_idx: u32,
    pub modify_actions: u16,
    pub single_modify_action: *mut u8,
    pub decap_index: u32,
    pub decap_pat_idx: u32,
    pub decap_actions: u16,
    pub decap_with_vlan:1: u8,
    pub final_icm_addr: u64,
    pub flow_tag: u32,
    pub ctr_id: u32,
    pub gvmi: u16,
    pub hit_gvmi: u16,
    pub id: u32,
    pub size: u32,
    pub param_0: u8,
    pub param_1: u8,
    pub reformat: },
    pub count: c_int,
    pub headers: [u32; MLX5DR_MAX_VLANS],
    pub vlans: },
    pub obj_id: u32,
    pub offset: u32,
    pub dest_reg_id: u8,
    pub init_color: u8,
    pub aso_flow_meter: },
    pub miss_icm_addr: u64,
    pub definer_id: u32,
    pub min: u32,
    pub max: u32,
    pub range: },
}

extern "C" {
    pub fn mlx5dr_ste_alloc_modify_hdr(action: *mut mlx5dr_action) -> c_int;
}
extern "C" {
    pub fn mlx5dr_ste_free_modify_hdr(action: *mut mlx5dr_action);
}
// initial as 0, increased only when ste appears in a new rule
extern "C" {
    pub fn mlx5dr_ste_equal_tag(src: *mut c_void, dst: *mut c_void) -> bool;
}
// STE build functions
extern "C" {
    pub fn mlx5dr_ste_build_empty_always_hit(sb: *mut mlx5dr_ste_build, rx: bool);
}
// Actions utils
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_match_spec {
    pub /: *mut *mut u32 smac_47_16; / Source MAC address of incoming packet,
// Incoming packet Ethertype - this is the Ethertype
// following the last VLAN tag of the packet
//
    pub /: *mut *mut u32 smac_15_0:16; / Source MAC address of incoming packet,
    pub ethertype:16: u32,
    pub /: *mut *mut u32 dmac_47_16; / Destination MAC address of incoming packet,
    pub /: *mut *mut u32 dmac_15_0:16; / Destination MAC address of incoming packet,
// Priority of first VLAN tag in the incoming packet.
// Valid only when cvlan_tag==1 or svlan_tag==1
//
    pub first_prio:3: u32,
// CFI bit of first VLAN tag in the incoming packet.
// Valid only when cvlan_tag==1 or svlan_tag==1
//
    pub first_cfi:1: u32,
// VLAN ID of first VLAN tag in the incoming packet.
// Valid only when cvlan_tag==1 or svlan_tag==1
//
    pub first_vid:12: u32,
    pub /: *mut *mut u32 ip_protocol:8; / IP protocol,
// Differentiated Services Code Point derived from
// Traffic Class/TOS field of IPv6/v4
//
    pub ip_dscp:6: u32,
// Explicit Congestion Notification derived from
// Traffic Class/TOS field of IPv6/v4
//
    pub ip_ecn:2: u32,
// The first vlan in the packet is c-vlan (0x8100).
// cvlan_tag and svlan_tag cannot be set together
//
    pub cvlan_tag:1: u32,
// The first vlan in the packet is s-vlan (0x8a88).
// cvlan_tag and svlan_tag cannot be set together
//
    pub svlan_tag:1: u32,
    pub /: *mut *mut u32 frag:1; / Packet is an IP fragment,
    pub /: *mut *mut u32 ip_version:4; / IP version,
// TCP flags. ;Bit 0: FIN;Bit 1: SYN;Bit 2: RST;Bit 3: PSH;Bit 4: ACK;
// Bit 5: URG;Bit 6: ECE;Bit 7: CWR;Bit 8: NS
//
    pub tcp_flags:9: u32,
// TCP source port.;tcp and udp sport/dport are mutually exclusive
    pub tcp_sport:16: u32,
// TCP destination port.
// tcp and udp sport/dport are mutually exclusive
//
    pub tcp_dport:16: u32,
    pub reserved_auto1:16: u32,
    pub ipv4_ihl:4: u32,
    pub reserved_auto2:4: u32,
    pub ttl_hoplimit:8: u32,
// UDP source port.;tcp and udp sport/dport are mutually exclusive
    pub udp_sport:16: u32,
// UDP destination port.;tcp and udp sport/dport are mutually exclusive
    pub udp_dport:16: u32,
// IPv6 source address of incoming packets
// For IPv4 address use bits 31:0 (rest of the bits are reserved)
// This field should be qualified by an appropriate ethertype
//
    pub src_ip_127_96: u32,
// IPv6 source address of incoming packets
// For IPv4 address use bits 31:0 (rest of the bits are reserved)
// This field should be qualified by an appropriate ethertype
//
    pub src_ip_95_64: u32,
// IPv6 source address of incoming packets
// For IPv4 address use bits 31:0 (rest of the bits are reserved)
// This field should be qualified by an appropriate ethertype
//
    pub src_ip_63_32: u32,
// IPv6 source address of incoming packets
// For IPv4 address use bits 31:0 (rest of the bits are reserved)
// This field should be qualified by an appropriate ethertype
//
    pub src_ip_31_0: u32,
// IPv6 destination address of incoming packets
// For IPv4 address use bits 31:0 (rest of the bits are reserved)
// This field should be qualified by an appropriate ethertype
//
    pub dst_ip_127_96: u32,
// IPv6 destination address of incoming packets
// For IPv4 address use bits 31:0 (rest of the bits are reserved)
// This field should be qualified by an appropriate ethertype
//
    pub dst_ip_95_64: u32,
// IPv6 destination address of incoming packets
// For IPv4 address use bits 31:0 (rest of the bits are reserved)
// This field should be qualified by an appropriate ethertype
//
    pub dst_ip_63_32: u32,
// IPv6 destination address of incoming packets
// For IPv4 address use bits 31:0 (rest of the bits are reserved)
// This field should be qualified by an appropriate ethertype
//
    pub dst_ip_31_0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_match_misc {
// used with GRE, checksum exist when gre_c_present == 1
    pub gre_c_present:1: u32,
    pub reserved_auto1:1: u32,
// used with GRE, key exist when gre_k_present == 1
    pub gre_k_present:1: u32,
// used with GRE, sequence number exist when gre_s_present == 1
    pub gre_s_present:1: u32,
    pub source_vhca_port:4: u32,
    pub /: *mut *mut u32 source_sqn:24; / Source SQN,
    pub source_eswitch_owner_vhca_id:16: u32,
// Source port.;0xffff determines wire port
    pub source_port:16: u32,
// Priority of second VLAN tag in the outer header of the incoming packet.
// Valid only when outer_second_cvlan_tag ==1 or outer_second_svlan_tag ==1
//
    pub outer_second_prio:3: u32,
// CFI bit of first VLAN tag in the outer header of the incoming packet.
// Valid only when outer_second_cvlan_tag ==1 or outer_second_svlan_tag ==1
//
    pub outer_second_cfi:1: u32,
// VLAN ID of first VLAN tag the outer header of the incoming packet.
// Valid only when outer_second_cvlan_tag ==1 or outer_second_svlan_tag ==1
//
    pub outer_second_vid:12: u32,
// Priority of second VLAN tag in the inner header of the incoming packet.
// Valid only when inner_second_cvlan_tag ==1 or inner_second_svlan_tag ==1
//
    pub inner_second_prio:3: u32,
// CFI bit of first VLAN tag in the inner header of the incoming packet.
// Valid only when inner_second_cvlan_tag ==1 or inner_second_svlan_tag ==1
//
    pub inner_second_cfi:1: u32,
// VLAN ID of first VLAN tag the inner header of the incoming packet.
// Valid only when inner_second_cvlan_tag ==1 or inner_second_svlan_tag ==1
//
    pub inner_second_vid:12: u32,
    pub outer_second_cvlan_tag:1: u32,
    pub inner_second_cvlan_tag:1: u32,
// The second vlan in the outer header of the packet is c-vlan (0x8100).
// outer_second_cvlan_tag and outer_second_svlan_tag cannot be set together
//
    pub outer_second_svlan_tag:1: u32,
// The second vlan in the inner header of the packet is c-vlan (0x8100).
// inner_second_cvlan_tag and inner_second_svlan_tag cannot be set together
//
    pub inner_second_svlan_tag:1: u32,
// The second vlan in the outer header of the packet is s-vlan (0x8a88).
// outer_second_cvlan_tag and outer_second_svlan_tag cannot be set together
//
    pub reserved_auto2:12: u32,
// The second vlan in the inner header of the packet is s-vlan (0x8a88).
// inner_second_cvlan_tag and inner_second_svlan_tag cannot be set together
//
    pub /: *mut *mut u32 gre_protocol:16; / GRE Protocol (outer),
    pub /: *mut *mut u32 gre_key_h:24; / GRE Key[31:8] (outer),
    pub /: *mut *mut u32 gre_key_l:8; / GRE Key [7:0] (outer),
    pub /: *mut *mut u32 vxlan_vni:24; / VXLAN VNI (outer),
    pub reserved_auto3:8: u32,
    pub /: *mut *mut u32 geneve_vni:24; / GENEVE VNI field (outer),
    pub reserved_auto4:6: u32,
    pub geneve_tlv_option_0_exist:1: u32,
    pub /: *mut *mut u32 geneve_oam:1; / GENEVE OAM field (outer),
    pub reserved_auto5:12: u32,
    pub /: *mut *mut u32 outer_ipv6_flow_label:20; / Flow label of incoming IPv6 packet (outer),
    pub reserved_auto6:12: u32,
    pub /: *mut *mut u32 inner_ipv6_flow_label:20; / Flow label of incoming IPv6 packet (inner),
    pub reserved_auto7:10: u32,
    pub /: *mut *mut u32 geneve_opt_len:6; / GENEVE OptLen (outer),
    pub /: *mut *mut u32 geneve_protocol_type:16; / GENEVE protocol type (outer),
    pub reserved_auto8:8: u32,
    pub /: *mut *mut u32 bth_dst_qp:24; / Destination QP in BTH header,
    pub reserved_auto9: u32,
    pub outer_esp_spi: u32,
    pub reserved_auto10: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_match_misc2 {
    pub /: *mut *mut u32 outer_first_mpls_label:20; / First MPLS LABEL (outer),
    pub /: *mut *mut u32 outer_first_mpls_exp:3; / First MPLS EXP (outer),
    pub /: *mut *mut u32 outer_first_mpls_s_bos:1; / First MPLS S_BOS (outer),
    pub /: *mut *mut u32 outer_first_mpls_ttl:8; / First MPLS TTL (outer),
    pub /: *mut *mut u32 inner_first_mpls_label:20; / First MPLS LABEL (inner),
    pub /: *mut *mut u32 inner_first_mpls_exp:3; / First MPLS EXP (inner),
    pub /: *mut *mut u32 inner_first_mpls_s_bos:1; / First MPLS S_BOS (inner),
    pub /: *mut *mut u32 inner_first_mpls_ttl:8; / First MPLS TTL (inner),
    pub /: *mut *mut u32 outer_first_mpls_over_gre_label:20; / last MPLS LABEL (outer),
    pub /: *mut *mut u32 outer_first_mpls_over_gre_exp:3; / last MPLS EXP (outer),
    pub /: *mut *mut u32 outer_first_mpls_over_gre_s_bos:1; / last MPLS S_BOS (outer),
    pub /: *mut *mut u32 outer_first_mpls_over_gre_ttl:8; / last MPLS TTL (outer),
    pub /: *mut *mut u32 outer_first_mpls_over_udp_label:20; / last MPLS LABEL (outer),
    pub /: *mut *mut u32 outer_first_mpls_over_udp_exp:3; / last MPLS EXP (outer),
    pub /: *mut *mut u32 outer_first_mpls_over_udp_s_bos:1; / last MPLS S_BOS (outer),
    pub /: *mut *mut u32 outer_first_mpls_over_udp_ttl:8; / last MPLS TTL (outer),
    pub /: *mut *mut u32 metadata_reg_c_7; / metadata_reg_c_7,
    pub /: *mut *mut u32 metadata_reg_c_6; / metadata_reg_c_6,
    pub /: *mut *mut u32 metadata_reg_c_5; / metadata_reg_c_5,
    pub /: *mut *mut u32 metadata_reg_c_4; / metadata_reg_c_4,
    pub /: *mut *mut u32 metadata_reg_c_3; / metadata_reg_c_3,
    pub /: *mut *mut u32 metadata_reg_c_2; / metadata_reg_c_2,
    pub /: *mut *mut u32 metadata_reg_c_1; / metadata_reg_c_1,
    pub /: *mut *mut u32 metadata_reg_c_0; / metadata_reg_c_0,
    pub /: *mut *mut u32 metadata_reg_a; / metadata_reg_a,
    pub reserved_auto1: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_match_misc3 {
    pub inner_tcp_seq_num: u32,
    pub outer_tcp_seq_num: u32,
    pub inner_tcp_ack_num: u32,
    pub outer_tcp_ack_num: u32,
    pub reserved_auto1:8: u32,
    pub outer_vxlan_gpe_vni:24: u32,
    pub outer_vxlan_gpe_next_protocol:8: u32,
    pub outer_vxlan_gpe_flags:8: u32,
    pub reserved_auto2:16: u32,
    pub icmpv4_header_data: u32,
    pub icmpv6_header_data: u32,
    pub icmpv4_type: u8,
    pub icmpv4_code: u8,
    pub icmpv6_type: u8,
    pub icmpv6_code: u8,
    pub geneve_tlv_option_0_data: u32,
    pub gtpu_teid: u32,
    pub gtpu_msg_type: u8,
    pub gtpu_msg_flags: u8,
    pub reserved_auto3:16: u32,
    pub gtpu_dw_2: u32,
    pub gtpu_first_ext_dw_0: u32,
    pub gtpu_dw_0: u32,
    pub reserved_auto4: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_match_misc4 {
    pub prog_sample_field_value_0: u32,
    pub prog_sample_field_id_0: u32,
    pub prog_sample_field_value_1: u32,
    pub prog_sample_field_id_1: u32,
    pub prog_sample_field_value_2: u32,
    pub prog_sample_field_id_2: u32,
    pub prog_sample_field_value_3: u32,
    pub prog_sample_field_id_3: u32,
    pub reserved_auto1: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_match_misc5 {
    pub macsec_tag_0: u32,
    pub macsec_tag_1: u32,
    pub macsec_tag_2: u32,
    pub macsec_tag_3: u32,
    pub tunnel_header_0: u32,
    pub tunnel_header_1: u32,
    pub tunnel_header_2: u32,
    pub tunnel_header_3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_match_param {
    pub outer: mlx5dr_match_spec,
    pub misc: mlx5dr_match_misc,
    pub inner: mlx5dr_match_spec,
    pub misc2: mlx5dr_match_misc2,
    pub misc3: mlx5dr_match_misc3,
    pub misc4: mlx5dr_match_misc4,
    pub misc5: mlx5dr_match_misc5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_esw_caps {
    pub drop_icm_address_rx: u64,
    pub drop_icm_address_tx: u64,
    pub uplink_icm_address_rx: u64,
    pub uplink_icm_address_tx: u64,
    pub sw_owner:1: u8,
    pub sw_owner_v2:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_cmd_vport_cap {
    pub vport_gvmi: u16,
    pub vhca_gvmi: u16,
    pub num: u16,
    pub icm_address_rx: u64,
    pub icm_address_tx: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_roce_cap {
    pub roce_en:1: u8,
    pub fl_rc_qp_when_roce_disabled:1: u8,
    pub fl_rc_qp_when_roce_enabled:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_vports {
    pub esw_manager_caps: mlx5dr_cmd_vport_cap,
    pub uplink_caps: mlx5dr_cmd_vport_cap,
    pub vports_caps_xa: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_cmd_caps {
    pub gvmi: u16,
    pub nic_rx_drop_address: u64,
    pub nic_tx_drop_address: u64,
    pub nic_tx_allow_address: u64,
    pub esw_rx_drop_address: u64,
    pub esw_tx_drop_address: u64,
    pub log_icm_size: u32,
    pub hdr_modify_icm_addr: u64,
    pub log_modify_pattern_icm_size: u32,
    pub hdr_modify_pattern_icm_addr: u64,
    pub flex_protocols: u32,
    pub flex_parser_id_icmp_dw0: u8,
    pub flex_parser_id_icmp_dw1: u8,
    pub flex_parser_id_icmpv6_dw0: u8,
    pub flex_parser_id_icmpv6_dw1: u8,
    pub flex_parser_id_geneve_tlv_option_0: u8,
    pub flex_parser_id_mpls_over_gre: u8,
    pub flex_parser_id_mpls_over_udp: u8,
    pub flex_parser_id_gtpu_dw_0: u8,
    pub flex_parser_id_gtpu_teid: u8,
    pub flex_parser_id_gtpu_dw_2: u8,
    pub flex_parser_id_gtpu_first_ext_dw_0: u8,
    pub flex_parser_ok_bits_supp: u8,
    pub max_ft_level: u8,
    pub roce_min_src_udp: u16,
    pub sw_format_ver: u8,
    pub eswitch_manager: bool,
    pub rx_sw_owner: bool,
    pub tx_sw_owner: bool,
    pub fdb_sw_owner: bool,
    pub rx_sw_owner_v2:1: u8,
    pub tx_sw_owner_v2:1: u8,
    pub fdb_sw_owner_v2:1: u8,
    pub esw_caps: mlx5dr_esw_caps,
    pub vports: mlx5dr_vports,
    pub prio_tag_required: bool,
    pub roce_caps: mlx5dr_roce_cap,
    pub log_header_modify_argument_granularity: u16,
    pub log_header_modify_argument_max_alloc: u16,
    pub support_modify_argument: bool,
    pub is_ecpf:1: u8,
    pub isolate_vl_tc:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_domain_nic_type {
    DR_DOMAIN_NIC_TYPE_RX,
    DR_DOMAIN_NIC_TYPE_TX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_domain_rx_tx {
    pub drop_icm_addr: u64,
    pub default_icm_addr: u64,
    pub type: mlx5dr_domain_nic_type,
    pub /: *mut *mut mutex mutex; / protect rx/tx domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_domain_info {
    pub supp_sw_steering: bool,
    pub max_inline_size: u32,
    pub max_send_wr: u32,
    pub max_log_sw_icm_sz: u32,
    pub max_log_action_icm_sz: u32,
    pub max_log_modify_hdr_pattern_icm_sz: u32,
    pub rx: mlx5dr_domain_rx_tx,
    pub tx: mlx5dr_domain_rx_tx,
    pub caps: mlx5dr_cmd_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_domain {
    pub mdev: *mut mlx5_core_dev,
    pub pdn: u32,
    pub uar: *mut mlx5_uars_page,
    pub type: mlx5dr_domain_type,
    pub refcount: refcount_t,
    pub ste_icm_pool: *mut mlx5dr_icm_pool,
    pub action_icm_pool: *mut mlx5dr_icm_pool,
    pub send_info_pool_rx: *mut mlx5dr_send_info_pool,
    pub send_info_pool_tx: *mut mlx5dr_send_info_pool,
    pub chunks_kmem_cache: *mut kmem_cache,
    pub htbls_kmem_cache: *mut kmem_cache,
    pub ptrn_mgr: *mut mlx5dr_ptrn_mgr,
    pub arg_mgr: *mut mlx5dr_arg_mgr,
    pub send_ring: *mut mlx5dr_send_ring,
    pub info: mlx5dr_domain_info,
    pub csum_fts_xa: xarray,
    pub ste_ctx: *mut mlx5dr_ste_ctx,
    pub dbg_tbl_list: list_head,
    pub dump_info: mlx5dr_dbg_dump_info,
    pub definers_xa: xarray,
    pub peer_dmn_xa: xarray,
// memory management statistics
    pub num_buddies: [u32; DR_ICM_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_table_rx_tx {
    pub s_anchor: *mut mlx5dr_ste_htbl,
    pub nic_dmn: *mut mlx5dr_domain_rx_tx,
    pub default_icm_addr: u64,
    pub nic_matcher_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_table {
    pub dmn: *mut mlx5dr_domain,
    pub rx: mlx5dr_table_rx_tx,
    pub tx: mlx5dr_table_rx_tx,
    pub level: u32,
    pub table_type: u32,
    pub table_id: u32,
    pub flags: u32,
    pub matcher_list: list_head,
    pub miss_action: *mut mlx5dr_action,
    pub refcount: refcount_t,
    pub dbg_node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_matcher_rx_tx {
    pub s_htbl: *mut mlx5dr_ste_htbl,
    pub e_anchor: *mut mlx5dr_ste_htbl,
    pub ste_builder: *mut mlx5dr_ste_build,
    pub num_of_builders: u8,
    pub num_of_builders_arr: [u8; DR_RULE_IPV_MAX][DR_RULE_IPV_MAX],
    pub nic_tbl: *mut mlx5dr_table_rx_tx,
    pub prio: u32,
    pub list_node: list_head,
    pub rules: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_matcher {
    pub tbl: *mut mlx5dr_table,
    pub rx: mlx5dr_matcher_rx_tx,
    pub tx: mlx5dr_matcher_rx_tx,
    pub /: *mut *mut list_head list_node; / Used for both matchers and dbg managing,
    pub prio: u32,
    pub mask: mlx5dr_match_param,
    pub match_criteria: u8,
    pub refcount: refcount_t,
    pub dbg_rule_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_ste_action_modify_field {
    pub hw_field: u16,
    pub start: u8,
    pub end: u8,
    pub l3_type: u8,
    pub l4_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_ptrn_obj {
    pub chunk: *mut mlx5dr_icm_chunk,
    pub data: *mut u8,
    pub num_of_actions: u16,
    pub index: u32,
    pub refcount: refcount_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_arg_obj {
    pub obj_id: u32,
    pub obj_offset: u32,
    pub list_node: list_head,
    pub log_chunk_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_rewrite {
    pub dmn: *mut mlx5dr_domain,
    pub chunk: *mut mlx5dr_icm_chunk,
    pub data: *mut u8,
    pub num_of_actions: u16,
    pub index: u32,
    pub single_action_opt:1: u8,
    pub allow_rx:1: u8,
    pub allow_tx:1: u8,
    pub modify_ttl:1: u8,
    pub ptrn: *mut mlx5dr_ptrn_obj,
    pub arg: *mut mlx5dr_arg_obj,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_reformat {
    pub dmn: *mut mlx5dr_domain,
    pub id: u32,
    pub size: u32,
    pub param_0: u8,
    pub param_1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_sampler {
    pub dmn: *mut mlx5dr_domain,
    pub rx_icm_addr: u64,
    pub tx_icm_addr: u64,
    pub sampler_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_dest_tbl {
    pub is_fw_tbl:1: u8,
    pub is_wire_ft:1: u8,
    pub tbl: *mut mlx5dr_table,
    pub dmn: *mut mlx5dr_domain,
    pub id: u32,
    pub group_id: u32,
    pub type: fs_flow_table_type,
    pub rx_icm_addr: u64,
    pub tx_icm_addr: u64,
    pub ref_actions: *mut mlx5dr_action,
    pub num_of_ref_actions: u32,
    pub fw_tbl: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_range {
    pub dmn: *mut mlx5dr_domain,
    pub hit_tbl_action: *mut mlx5dr_action,
    pub miss_tbl_action: *mut mlx5dr_action,
    pub definer_id: u32,
    pub min: u32,
    pub max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_ctr {
    pub ctr_id: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_vport {
    pub dmn: *mut mlx5dr_domain,
    pub caps: *mut mlx5dr_cmd_vport_cap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_push_vlan {
    pub /: *mut *mut u32 vlan_hdr; / tpid_pcp_dei_vid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_flow_tag {
    pub flow_tag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_rule_action_member {
    pub action: *mut mlx5dr_action,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action_aso_flow_meter {
    pub dmn: *mut mlx5dr_domain,
    pub obj_id: u32,
    pub offset: u32,
    pub dest_reg_id: u8,
    pub init_color: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_action {
    pub action_type: mlx5dr_action_type,
    pub refcount: refcount_t,
    pub data: *mut c_void,
    pub rewrite: *mut mlx5dr_action_rewrite,
    pub reformat: *mut mlx5dr_action_reformat,
    pub sampler: *mut mlx5dr_action_sampler,
    pub dest_tbl: *mut mlx5dr_action_dest_tbl,
    pub ctr: *mut mlx5dr_action_ctr,
    pub vport: *mut mlx5dr_action_vport,
    pub push_vlan: *mut mlx5dr_action_push_vlan,
    pub flow_tag: *mut mlx5dr_action_flow_tag,
    pub aso: *mut mlx5dr_action_aso_flow_meter,
    pub range: *mut mlx5dr_action_range,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5dr_connect_type {
    CONNECT_HIT	= 1,
    CONNECT_MISS	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_htbl_connect_info {
    pub type: mlx5dr_connect_type,
    pub hit_next_htbl: *mut mlx5dr_ste_htbl,
    pub miss_icm_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_rule_rx_tx {
    pub nic_matcher: *mut mlx5dr_matcher_rx_tx,
    pub last_rule_ste: *mut mlx5dr_ste,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_rule {
    pub matcher: *mut mlx5dr_matcher,
    pub rx: mlx5dr_rule_rx_tx,
    pub tx: mlx5dr_rule_rx_tx,
    pub rule_actions_list: list_head,
    pub dbg_node: list_head,
    pub flow_source: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_icm_chunk {
    pub buddy_mem: *mut mlx5dr_icm_buddy_mem,
// indicates the index of this chunk in the whole memory,
// used for deleting the chunk from the buddy
//
    pub seg: c_uint,
    pub size: mlx5dr_icm_chunk_size,
// Memory optimisation
    pub ste_arr: *mut mlx5dr_ste,
    pub hw_ste_arr: *mut u8,
    pub miss_list: *mut list_head,
}

extern "C" {
    pub fn mlx5dr_icm_pool_get_chunk_mr_addr(chunk: *mut mlx5dr_icm_chunk) -> u64;
}
extern "C" {
    pub fn mlx5dr_icm_pool_get_chunk_rkey(chunk: *mut mlx5dr_icm_chunk) -> u32;
}
extern "C" {
    pub fn mlx5dr_icm_pool_get_chunk_icm_addr(chunk: *mut mlx5dr_icm_chunk) -> u64;
}
extern "C" {
    pub fn mlx5dr_icm_pool_get_chunk_num_of_entries(chunk: *mut mlx5dr_icm_chunk) -> u32;
}
extern "C" {
    pub fn mlx5dr_icm_pool_get_chunk_byte_size(chunk: *mut mlx5dr_icm_chunk) -> u32;
}
extern "C" {
    pub fn mlx5dr_icm_pool_free_htbl(pool: *mut mlx5dr_icm_pool, htbl: *mut mlx5dr_ste_htbl);
}
// Threshold is 50%, one is added to table of size 1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_cmd_query_flow_table_details {
    pub status: u8,
    pub level: u8,
    pub sw_owner_icm_root_1: u64,
    pub sw_owner_icm_root_0: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_cmd_create_flow_table_attr {
    pub table_type: u32,
    pub uid: u16,
    pub icm_addr_rx: u64,
    pub icm_addr_tx: u64,
    pub level: u8,
    pub sw_owner: bool,
    pub term_tbl: bool,
    pub decap_en: bool,
    pub reformat_en: bool,
}

// internal API functions
extern "C" {
    pub fn mlx5dr_cmd_sync_steering(mdev: *mut mlx5_core_dev) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_cmd_gid_attr {
    pub gid: [u8; 16],
    pub mac: [u8; 6],
    pub roce_ver: u32,
}

extern "C" {
    pub fn mlx5dr_definer_put(dmn: *mut mlx5dr_domain, definer_id: u32);
}
extern "C" {
    pub fn mlx5dr_icm_pool_destroy(pool: *mut mlx5dr_icm_pool);
}
extern "C" {
    pub fn mlx5dr_icm_free_chunk(chunk: *mut mlx5dr_icm_chunk);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_qp {
    pub mdev: *mut mlx5_core_dev,
    pub wq: mlx5_wq_qp,
    pub uar: *mut mlx5_uars_page,
    pub wq_ctrl: mlx5_wq_ctrl,
    pub qpn: u32,
    pub head: c_uint,
    pub pc: c_uint,
    pub cc: c_uint,
    pub size: c_uint,
    pub wqe_head: *mut c_uint,
    pub wqe_cnt: c_uint,
    pub sq: },
    pub pc: c_uint,
    pub cc: c_uint,
    pub size: c_uint,
    pub wqe_cnt: c_uint,
    pub rq: },
    pub max_inline_data: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_cq {
    pub mdev: *mut mlx5_core_dev,
    pub wq: mlx5_cqwq,
    pub wq_ctrl: mlx5_wq_ctrl,
    pub mcq: mlx5_core_cq,
    pub qp: *mut mlx5dr_qp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_mr {
    pub mdev: *mut mlx5_core_dev,
    pub mkey: u32,
    pub dma_addr: dma_addr_t,
    pub addr: *mut c_void,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_send_ring {
    pub cq: *mut mlx5dr_cq,
    pub qp: *mut mlx5dr_qp,
    pub mr: *mut mlx5dr_mr,
// How much wqes are waiting for completion
    pub pending_wqe: u32,
// Signal request per this trash hold value
    pub signal_th: u16,
// Each post_send_size less than max_post_send_size
    pub max_post_send_size: u32,
// manage the send queue
    pub tx_head: u32,
    pub buf: *mut c_void,
    pub buf_size: u32,
    pub sync_buff: *mut u8,
    pub sync_mr: *mut mlx5dr_mr,
    pub /: *mut *mut spinlock_t lock; / Protect the data path of the send ring,
    pub /: *mut *mut bool err_state; / send_ring is not usable in err state,
}

extern "C" {
    pub fn mlx5dr_send_ring_alloc(dmn: *mut mlx5dr_domain) -> c_int;
}
extern "C" {
    pub fn mlx5dr_send_info_pool_create(dmn: *mut mlx5dr_domain) -> c_int;
}
extern "C" {
    pub fn mlx5dr_send_info_pool_destroy(dmn: *mut mlx5dr_domain);
}
extern "C" {
    pub fn mlx5dr_send_info_free(ste_send_info: *mut mlx5dr_ste_send_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_cmd_ft_info {
    pub id: u32,
    pub vport: u16,
    pub type: fs_flow_table_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_cmd_flow_destination_hw_info {
    pub type: mlx5_flow_destination_type,
    pub tir_num: u32,
    pub ft_num: u32,
    pub ft_id: u32,
    pub counter_id: u32,
    pub sampler_id: u32,
    pub num: u16,
    pub vhca_id: u16,
    pub reformat_id: u32,
    pub flags: u8,
    pub vport: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_cmd_fte_info {
    pub dests_size: u32,
    pub index: u32,
    pub flow_context: mlx5_flow_context,
    pub val: *mut u32,
    pub action: mlx5_flow_act,
    pub dest_arr: *mut mlx5dr_cmd_flow_destination_hw_info,
    pub ignore_flow_level: bool,
}

extern "C" {
    pub fn mlx5dr_ste_supp_ttl_cs_recalc(caps: *mut mlx5dr_cmd_caps) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5dr_fw_recalc_cs_ft {
    pub rx_icm_addr: u64,
    pub table_id: u32,
    pub group_id: u32,
    pub modify_hdr_id: u32,
}

extern "C" {
    pub fn mlx5dr_domain_is_support_ptrn_arg(dmn: *mut mlx5dr_domain) -> bool;
}
extern "C" {
    pub fn mlx5dr_ptrn_mgr_destroy(mgr: *mut mlx5dr_ptrn_mgr);
}
extern "C" {
    pub fn mlx5dr_arg_mgr_destroy(mgr: *mut mlx5dr_arg_mgr);
}
extern "C" {
    pub fn mlx5dr_arg_get_obj_id(arg_obj: *mut mlx5dr_arg_obj) -> u32;
}
