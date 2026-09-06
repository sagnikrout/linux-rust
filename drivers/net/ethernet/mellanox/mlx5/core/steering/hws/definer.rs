//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/definer.h
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
// Max available selecotrs
pub const DW_SELECTORS: c_int = 9;
pub const BYTE_SELECTORS: c_int = 8;
// Selectors based on match TAG
pub const DW_SELECTORS_MATCH: c_int = 6;
pub const DW_SELECTORS_LIMITED: c_int = 3;
// Selectors based on range TAG
pub const DW_SELECTORS_RANGE: c_int = 2;
pub const BYTE_SELECTORS_RANGE: c_int = 8;
pub const HWS_NUM_OF_FLEX_PARSERS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_definer_fname {
    MLX5HWS_DEFINER_FNAME_ETH_SMAC_47_16_O,
    MLX5HWS_DEFINER_FNAME_ETH_SMAC_47_16_I,
    MLX5HWS_DEFINER_FNAME_ETH_SMAC_15_0_O,
    MLX5HWS_DEFINER_FNAME_ETH_SMAC_15_0_I,
    MLX5HWS_DEFINER_FNAME_ETH_DMAC_47_16_O,
    MLX5HWS_DEFINER_FNAME_ETH_DMAC_47_16_I,
    MLX5HWS_DEFINER_FNAME_ETH_DMAC_15_0_O,
    MLX5HWS_DEFINER_FNAME_ETH_DMAC_15_0_I,
    MLX5HWS_DEFINER_FNAME_ETH_TYPE_O,
    MLX5HWS_DEFINER_FNAME_ETH_TYPE_I,
    MLX5HWS_DEFINER_FNAME_ETH_L3_TYPE_O,
    MLX5HWS_DEFINER_FNAME_ETH_L3_TYPE_I,
    MLX5HWS_DEFINER_FNAME_VLAN_TYPE_O,
    MLX5HWS_DEFINER_FNAME_VLAN_TYPE_I,
    MLX5HWS_DEFINER_FNAME_VLAN_FIRST_PRIO_O,
    MLX5HWS_DEFINER_FNAME_VLAN_FIRST_PRIO_I,
    MLX5HWS_DEFINER_FNAME_VLAN_CFI_O,
    MLX5HWS_DEFINER_FNAME_VLAN_CFI_I,
    MLX5HWS_DEFINER_FNAME_VLAN_ID_O,
    MLX5HWS_DEFINER_FNAME_VLAN_ID_I,
    MLX5HWS_DEFINER_FNAME_VLAN_SECOND_TYPE_O,
    MLX5HWS_DEFINER_FNAME_VLAN_SECOND_TYPE_I,
    MLX5HWS_DEFINER_FNAME_VLAN_SECOND_PRIO_O,
    MLX5HWS_DEFINER_FNAME_VLAN_SECOND_PRIO_I,
    MLX5HWS_DEFINER_FNAME_VLAN_SECOND_CFI_O,
    MLX5HWS_DEFINER_FNAME_VLAN_SECOND_CFI_I,
    MLX5HWS_DEFINER_FNAME_VLAN_SECOND_ID_O,
    MLX5HWS_DEFINER_FNAME_VLAN_SECOND_ID_I,
    MLX5HWS_DEFINER_FNAME_IPV4_IHL_O,
    MLX5HWS_DEFINER_FNAME_IPV4_IHL_I,
    MLX5HWS_DEFINER_FNAME_IP_DSCP_O,
    MLX5HWS_DEFINER_FNAME_IP_DSCP_I,
    MLX5HWS_DEFINER_FNAME_IP_ECN_O,
    MLX5HWS_DEFINER_FNAME_IP_ECN_I,
    MLX5HWS_DEFINER_FNAME_IP_TTL_O,
    MLX5HWS_DEFINER_FNAME_IP_TTL_I,
    MLX5HWS_DEFINER_FNAME_IPV4_DST_O,
    MLX5HWS_DEFINER_FNAME_IPV4_DST_I,
    MLX5HWS_DEFINER_FNAME_IPV4_SRC_O,
    MLX5HWS_DEFINER_FNAME_IPV4_SRC_I,
    MLX5HWS_DEFINER_FNAME_IP_VERSION_O,
    MLX5HWS_DEFINER_FNAME_IP_VERSION_I,
    MLX5HWS_DEFINER_FNAME_IP_FRAG_O,
    MLX5HWS_DEFINER_FNAME_IP_FRAG_I,
    MLX5HWS_DEFINER_FNAME_IP_LEN_O,
    MLX5HWS_DEFINER_FNAME_IP_LEN_I,
    MLX5HWS_DEFINER_FNAME_IP_TOS_O,
    MLX5HWS_DEFINER_FNAME_IP_TOS_I,
    MLX5HWS_DEFINER_FNAME_IPV6_FLOW_LABEL_O,
    MLX5HWS_DEFINER_FNAME_IPV6_FLOW_LABEL_I,
    MLX5HWS_DEFINER_FNAME_IPV6_DST_127_96_O,
    MLX5HWS_DEFINER_FNAME_IPV6_DST_95_64_O,
    MLX5HWS_DEFINER_FNAME_IPV6_DST_63_32_O,
    MLX5HWS_DEFINER_FNAME_IPV6_DST_31_0_O,
    MLX5HWS_DEFINER_FNAME_IPV6_DST_127_96_I,
    MLX5HWS_DEFINER_FNAME_IPV6_DST_95_64_I,
    MLX5HWS_DEFINER_FNAME_IPV6_DST_63_32_I,
    MLX5HWS_DEFINER_FNAME_IPV6_DST_31_0_I,
    MLX5HWS_DEFINER_FNAME_IPV6_SRC_127_96_O,
    MLX5HWS_DEFINER_FNAME_IPV6_SRC_95_64_O,
    MLX5HWS_DEFINER_FNAME_IPV6_SRC_63_32_O,
    MLX5HWS_DEFINER_FNAME_IPV6_SRC_31_0_O,
    MLX5HWS_DEFINER_FNAME_IPV6_SRC_127_96_I,
    MLX5HWS_DEFINER_FNAME_IPV6_SRC_95_64_I,
    MLX5HWS_DEFINER_FNAME_IPV6_SRC_63_32_I,
    MLX5HWS_DEFINER_FNAME_IPV6_SRC_31_0_I,
    MLX5HWS_DEFINER_FNAME_IP_PROTOCOL_O,
    MLX5HWS_DEFINER_FNAME_IP_PROTOCOL_I,
    MLX5HWS_DEFINER_FNAME_L4_SPORT_O,
    MLX5HWS_DEFINER_FNAME_L4_SPORT_I,
    MLX5HWS_DEFINER_FNAME_L4_DPORT_O,
    MLX5HWS_DEFINER_FNAME_L4_DPORT_I,
    MLX5HWS_DEFINER_FNAME_TCP_FLAGS_I,
    MLX5HWS_DEFINER_FNAME_TCP_FLAGS_O,
    MLX5HWS_DEFINER_FNAME_TCP_SEQ_NUM,
    MLX5HWS_DEFINER_FNAME_TCP_ACK_NUM,
    MLX5HWS_DEFINER_FNAME_GTP_TEID,
    MLX5HWS_DEFINER_FNAME_GTP_MSG_TYPE,
    MLX5HWS_DEFINER_FNAME_GTP_EXT_FLAG,
    MLX5HWS_DEFINER_FNAME_GTP_NEXT_EXT_HDR,
    MLX5HWS_DEFINER_FNAME_GTP_EXT_HDR_PDU,
    MLX5HWS_DEFINER_FNAME_GTP_EXT_HDR_QFI,
    MLX5HWS_DEFINER_FNAME_GTPU_DW0,
    MLX5HWS_DEFINER_FNAME_GTPU_FIRST_EXT_DW0,
    MLX5HWS_DEFINER_FNAME_GTPU_DW2,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER_0,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER_1,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER_2,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER_3,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER_4,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER_5,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER_6,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER_7,
    MLX5HWS_DEFINER_FNAME_VPORT_REG_C_0,
    MLX5HWS_DEFINER_FNAME_VXLAN_FLAGS,
    MLX5HWS_DEFINER_FNAME_VXLAN_VNI,
    MLX5HWS_DEFINER_FNAME_VXLAN_GPE_FLAGS,
    MLX5HWS_DEFINER_FNAME_VXLAN_GPE_RSVD0,
    MLX5HWS_DEFINER_FNAME_VXLAN_GPE_PROTO,
    MLX5HWS_DEFINER_FNAME_VXLAN_GPE_VNI,
    MLX5HWS_DEFINER_FNAME_VXLAN_GPE_RSVD1,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_LEN,
    MLX5HWS_DEFINER_FNAME_GENEVE_OAM,
    MLX5HWS_DEFINER_FNAME_GENEVE_PROTO,
    MLX5HWS_DEFINER_FNAME_GENEVE_VNI,
    MLX5HWS_DEFINER_FNAME_SOURCE_QP,
    MLX5HWS_DEFINER_FNAME_SOURCE_GVMI,
    MLX5HWS_DEFINER_FNAME_REG_0,
    MLX5HWS_DEFINER_FNAME_REG_1,
    MLX5HWS_DEFINER_FNAME_REG_2,
    MLX5HWS_DEFINER_FNAME_REG_3,
    MLX5HWS_DEFINER_FNAME_REG_4,
    MLX5HWS_DEFINER_FNAME_REG_5,
    MLX5HWS_DEFINER_FNAME_REG_6,
    MLX5HWS_DEFINER_FNAME_REG_7,
    MLX5HWS_DEFINER_FNAME_REG_8,
    MLX5HWS_DEFINER_FNAME_REG_9,
    MLX5HWS_DEFINER_FNAME_REG_10,
    MLX5HWS_DEFINER_FNAME_REG_11,
    MLX5HWS_DEFINER_FNAME_REG_A,
    MLX5HWS_DEFINER_FNAME_REG_B,
    MLX5HWS_DEFINER_FNAME_GRE_KEY_PRESENT,
    MLX5HWS_DEFINER_FNAME_GRE_C,
    MLX5HWS_DEFINER_FNAME_GRE_K,
    MLX5HWS_DEFINER_FNAME_GRE_S,
    MLX5HWS_DEFINER_FNAME_GRE_PROTOCOL,
    MLX5HWS_DEFINER_FNAME_GRE_OPT_KEY,
    MLX5HWS_DEFINER_FNAME_GRE_OPT_SEQ,
    MLX5HWS_DEFINER_FNAME_GRE_OPT_CHECKSUM,
    MLX5HWS_DEFINER_FNAME_INTEGRITY_O,
    MLX5HWS_DEFINER_FNAME_INTEGRITY_I,
    MLX5HWS_DEFINER_FNAME_ICMP_DW1,
    MLX5HWS_DEFINER_FNAME_ICMP_DW2,
    MLX5HWS_DEFINER_FNAME_ICMP_DW3,
    MLX5HWS_DEFINER_FNAME_IPSEC_SPI,
    MLX5HWS_DEFINER_FNAME_IPSEC_SEQUENCE_NUMBER,
    MLX5HWS_DEFINER_FNAME_IPSEC_SYNDROME,
    MLX5HWS_DEFINER_FNAME_MPLS0_O,
    MLX5HWS_DEFINER_FNAME_MPLS1_O,
    MLX5HWS_DEFINER_FNAME_MPLS2_O,
    MLX5HWS_DEFINER_FNAME_MPLS3_O,
    MLX5HWS_DEFINER_FNAME_MPLS4_O,
    MLX5HWS_DEFINER_FNAME_MPLS0_I,
    MLX5HWS_DEFINER_FNAME_MPLS1_I,
    MLX5HWS_DEFINER_FNAME_MPLS2_I,
    MLX5HWS_DEFINER_FNAME_MPLS3_I,
    MLX5HWS_DEFINER_FNAME_MPLS4_I,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER0_OK,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER1_OK,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER2_OK,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER3_OK,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER4_OK,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER5_OK,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER6_OK,
    MLX5HWS_DEFINER_FNAME_FLEX_PARSER7_OK,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS0_O,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS1_O,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS2_O,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS3_O,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS4_O,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS0_I,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS1_I,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS2_I,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS3_I,
    MLX5HWS_DEFINER_FNAME_OKS2_MPLS4_I,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_OK_0,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_OK_1,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_OK_2,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_OK_3,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_OK_4,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_OK_5,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_OK_6,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_OK_7,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_DW_0,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_DW_1,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_DW_2,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_DW_3,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_DW_4,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_DW_5,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_DW_6,
    MLX5HWS_DEFINER_FNAME_GENEVE_OPT_DW_7,
    MLX5HWS_DEFINER_FNAME_IB_L4_OPCODE,
    MLX5HWS_DEFINER_FNAME_IB_L4_QPN,
    MLX5HWS_DEFINER_FNAME_IB_L4_A,
    MLX5HWS_DEFINER_FNAME_RANDOM_NUM,
    MLX5HWS_DEFINER_FNAME_PTYPE_L2_O,
    MLX5HWS_DEFINER_FNAME_PTYPE_L2_I,
    MLX5HWS_DEFINER_FNAME_PTYPE_L3_O,
    MLX5HWS_DEFINER_FNAME_PTYPE_L3_I,
    MLX5HWS_DEFINER_FNAME_PTYPE_L4_O,
    MLX5HWS_DEFINER_FNAME_PTYPE_L4_I,
    MLX5HWS_DEFINER_FNAME_PTYPE_L4_EXT_O,
    MLX5HWS_DEFINER_FNAME_PTYPE_L4_EXT_I,
    MLX5HWS_DEFINER_FNAME_PTYPE_FRAG_O,
    MLX5HWS_DEFINER_FNAME_PTYPE_FRAG_I,
    MLX5HWS_DEFINER_FNAME_TNL_HDR_0,
    MLX5HWS_DEFINER_FNAME_TNL_HDR_1,
    MLX5HWS_DEFINER_FNAME_TNL_HDR_2,
    MLX5HWS_DEFINER_FNAME_TNL_HDR_3,
    MLX5HWS_DEFINER_FNAME_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_definer_match_criteria {
    MLX5HWS_DEFINER_MATCH_CRITERIA_EMPTY = 0,
    MLX5HWS_DEFINER_MATCH_CRITERIA_OUTER = 1 << 0,
    MLX5HWS_DEFINER_MATCH_CRITERIA_MISC = 1 << 1,
    MLX5HWS_DEFINER_MATCH_CRITERIA_INNER = 1 << 2,
    MLX5HWS_DEFINER_MATCH_CRITERIA_MISC2 = 1 << 3,
    MLX5HWS_DEFINER_MATCH_CRITERIA_MISC3 = 1 << 4,
    MLX5HWS_DEFINER_MATCH_CRITERIA_MISC4 = 1 << 5,
    MLX5HWS_DEFINER_MATCH_CRITERIA_MISC5 = 1 << 6,
    MLX5HWS_DEFINER_MATCH_CRITERIA_MISC6 = 1 << 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_definer_type {
    MLX5HWS_DEFINER_TYPE_MATCH,
    MLX5HWS_DEFINER_TYPE_JUMBO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_definer_match_flag {
    MLX5HWS_DEFINER_MATCH_FLAG_TNL_VXLAN_GPE = 1 << 0,
    MLX5HWS_DEFINER_MATCH_FLAG_TNL_GENEVE = 1 << 1,
    MLX5HWS_DEFINER_MATCH_FLAG_TNL_GTPU = 1 << 2,
    MLX5HWS_DEFINER_MATCH_FLAG_TNL_GRE = 1 << 3,
    MLX5HWS_DEFINER_MATCH_FLAG_TNL_VXLAN = 1 << 4,
    MLX5HWS_DEFINER_MATCH_FLAG_TNL_HEADER_0_1 = 1 << 5,

    MLX5HWS_DEFINER_MATCH_FLAG_TNL_GRE_OPT_KEY = 1 << 6,
    MLX5HWS_DEFINER_MATCH_FLAG_TNL_HEADER_2 = 1 << 7,

    MLX5HWS_DEFINER_MATCH_FLAG_TNL_MPLS_OVER_GRE = 1 << 8,
    MLX5HWS_DEFINER_MATCH_FLAG_TNL_MPLS_OVER_UDP = 1 << 9,

    MLX5HWS_DEFINER_MATCH_FLAG_ICMPV4 = 1 << 10,
    MLX5HWS_DEFINER_MATCH_FLAG_ICMPV6 = 1 << 11,
    MLX5HWS_DEFINER_MATCH_FLAG_TCP_O = 1 << 12,
    MLX5HWS_DEFINER_MATCH_FLAG_TCP_I = 1 << 13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_definer_fc {
    pub ctx: *mut mlx5hws_context,
// Source
    pub s_byte_off: u32,
    pub s_bit_off: c_int,
    pub s_bit_mask: u32,
// Destination
    pub byte_off: u32,
    pub bit_off: c_int,
    pub bit_mask: u32,
    pub fname: mlx5hws_definer_fname,
    pub tag): *mut u8,
    pub tag): *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_eth_l2_bits {
    pub dmac_47_16: [u8; 0x20],
    pub dmac_15_0: [u8; 0x10],
    pub l3_ethertype: [u8; 0x10],
    pub reserved_at_40: [u8; 0x1],
    pub sx_sniffer: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub ip_fragmented: [u8; 0x1],
    pub qp_type: [u8; 0x2],
    pub encap_type: [u8; 0x2],
    pub port_number: [u8; 0x2],
    pub l3_type: [u8; 0x2],
    pub l4_type_bwc: [u8; 0x2],
    pub first_vlan_qualifier: [u8; 0x2],
    pub first_priority: [u8; 0x3],
    pub first_cfi: [u8; 0x1],
    pub first_vlan_id: [u8; 0xc],
    pub l4_type: [u8; 0x4],
    pub reserved_at_64: [u8; 0x2],
    pub ipsec_layer: [u8; 0x2],
    pub l2_type: [u8; 0x2],
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
pub struct mlx5_ifc_definer_hl_eth_l2_src_bits {
    pub smac_47_16: [u8; 0x20],
    pub smac_15_0: [u8; 0x10],
    pub loopback_syndrome: [u8; 0x8],
    pub l3_type: [u8; 0x2],
    pub l4_type_bwc: [u8; 0x2],
    pub first_vlan_qualifier: [u8; 0x2],
    pub ip_fragmented: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_ib_l2_bits {
    pub sx_sniffer: [u8; 0x1],
    pub force_lb: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub reserved_at_3: [u8; 0x3],
    pub port_number: [u8; 0x2],
    pub sl: [u8; 0x4],
    pub qp_type: [u8; 0x2],
    pub lnh: [u8; 0x2],
    pub dlid: [u8; 0x10],
    pub vl: [u8; 0x4],
    pub lrh_packet_length: [u8; 0xc],
    pub slid: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_eth_l3_bits {
    pub ip_version: [u8; 0x4],
    pub ihl: [u8; 0x4],
    pub tos: [u8; 0x8],
    pub dscp: [u8; 0x6],
    pub ecn: [u8; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_eth_l4_bits {
    pub source_port: [u8; 0x10],
    pub destination_port: [u8; 0x10],
    pub data_offset: [u8; 0x4],
    pub l4_ok: [u8; 0x1],
    pub l3_ok: [u8; 0x1],
    pub ip_fragmented: [u8; 0x1],
    pub tcp_ns: [u8; 0x1],
    pub tcp_flags: [u8; 0x8],
    pub tcp_cwr: [u8; 0x1],
    pub tcp_ece: [u8; 0x1],
    pub tcp_urg: [u8; 0x1],
    pub tcp_ack: [u8; 0x1],
    pub tcp_psh: [u8; 0x1],
    pub tcp_rst: [u8; 0x1],
    pub tcp_syn: [u8; 0x1],
    pub tcp_fin: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_src_qp_gvmi_bits {
    pub loopback_syndrome: [u8; 0x8],
    pub l3_type: [u8; 0x2],
    pub l4_type_bwc: [u8; 0x2],
    pub first_vlan_qualifier: [u8; 0x2],
    pub reserved_at_e: [u8; 0x1],
    pub functional_lb: [u8; 0x1],
    pub source_gvmi: [u8; 0x10],
    pub force_lb: [u8; 0x1],
    pub ip_fragmented: [u8; 0x1],
    pub source_is_requestor: [u8; 0x1],
    pub reserved_at_23: [u8; 0x5],
    pub source_qp: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_ib_l4_bits {
    pub opcode: [u8; 0x8],
    pub qp: [u8; 0x18],
    pub se: [u8; 0x1],
    pub migreq: [u8; 0x1],
    pub ackreq: [u8; 0x1],
    pub fecn: [u8; 0x1],
    pub becn: [u8; 0x1],
    pub bth: [u8; 0x1],
    pub deth: [u8; 0x1],
    pub dcceth: [u8; 0x1],
    pub reserved_at_28: [u8; 0x2],
    pub pad_count: [u8; 0x2],
    pub tver: [u8; 0x4],
    pub p_key: [u8; 0x10],
    pub reserved_at_40: [u8; 0x8],
    pub deth_source_qp: [u8; 0x18],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_integrity_ok1_bits {
    MLX5HWS_DEFINER_OKS1_FIRST_L4_OK = 24,
    MLX5HWS_DEFINER_OKS1_FIRST_L3_OK = 25,
    MLX5HWS_DEFINER_OKS1_SECOND_L4_OK = 26,
    MLX5HWS_DEFINER_OKS1_SECOND_L3_OK = 27,
    MLX5HWS_DEFINER_OKS1_FIRST_L4_CSUM_OK = 28,
    MLX5HWS_DEFINER_OKS1_FIRST_IPV4_CSUM_OK = 29,
    MLX5HWS_DEFINER_OKS1_SECOND_L4_CSUM_OK = 30,
    MLX5HWS_DEFINER_OKS1_SECOND_IPV4_CSUM_OK = 31,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_oks1_bits {
    pub oks1_bits: [u8; 0x20],
    pub second_ipv4_checksum_ok: [u8; 0x1],
    pub second_l4_checksum_ok: [u8; 0x1],
    pub first_ipv4_checksum_ok: [u8; 0x1],
    pub first_l4_checksum_ok: [u8; 0x1],
    pub second_l3_ok: [u8; 0x1],
    pub second_l4_ok: [u8; 0x1],
    pub first_l3_ok: [u8; 0x1],
    pub first_l4_ok: [u8; 0x1],
    pub flex_parser7_steering_ok: [u8; 0x1],
    pub flex_parser6_steering_ok: [u8; 0x1],
    pub flex_parser5_steering_ok: [u8; 0x1],
    pub flex_parser4_steering_ok: [u8; 0x1],
    pub flex_parser3_steering_ok: [u8; 0x1],
    pub flex_parser2_steering_ok: [u8; 0x1],
    pub flex_parser1_steering_ok: [u8; 0x1],
    pub flex_parser0_steering_ok: [u8; 0x1],
    pub second_ipv6_extension_header_vld: [u8; 0x1],
    pub first_ipv6_extension_header_vld: [u8; 0x1],
    pub l3_tunneling_ok: [u8; 0x1],
    pub l2_tunneling_ok: [u8; 0x1],
    pub second_tcp_ok: [u8; 0x1],
    pub second_udp_ok: [u8; 0x1],
    pub second_ipv4_ok: [u8; 0x1],
    pub second_ipv6_ok: [u8; 0x1],
    pub second_l2_ok: [u8; 0x1],
    pub vxlan_ok: [u8; 0x1],
    pub gre_ok: [u8; 0x1],
    pub first_tcp_ok: [u8; 0x1],
    pub first_udp_ok: [u8; 0x1],
    pub first_ipv4_ok: [u8; 0x1],
    pub first_ipv6_ok: [u8; 0x1],
    pub first_l2_ok: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_oks2_bits {
    pub reserved_at_0: [u8; 0xa],
    pub second_mpls_ok: [u8; 0x1],
    pub second_mpls4_s_bit: [u8; 0x1],
    pub second_mpls4_qualifier: [u8; 0x1],
    pub second_mpls3_s_bit: [u8; 0x1],
    pub second_mpls3_qualifier: [u8; 0x1],
    pub second_mpls2_s_bit: [u8; 0x1],
    pub second_mpls2_qualifier: [u8; 0x1],
    pub second_mpls1_s_bit: [u8; 0x1],
    pub second_mpls1_qualifier: [u8; 0x1],
    pub second_mpls0_s_bit: [u8; 0x1],
    pub second_mpls0_qualifier: [u8; 0x1],
    pub first_mpls_ok: [u8; 0x1],
    pub first_mpls4_s_bit: [u8; 0x1],
    pub first_mpls4_qualifier: [u8; 0x1],
    pub first_mpls3_s_bit: [u8; 0x1],
    pub first_mpls3_qualifier: [u8; 0x1],
    pub first_mpls2_s_bit: [u8; 0x1],
    pub first_mpls2_qualifier: [u8; 0x1],
    pub first_mpls1_s_bit: [u8; 0x1],
    pub first_mpls1_qualifier: [u8; 0x1],
    pub first_mpls0_s_bit: [u8; 0x1],
    pub first_mpls0_qualifier: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_voq_bits {
    pub reserved_at_0: [u8; 0x18],
    pub ecn_ok: [u8; 0x1],
    pub congestion: [u8; 0x1],
    pub profile: [u8; 0x2],
    pub internal_prio: [u8; 0x4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_ipv4_src_dst_bits {
    pub source_address: [u8; 0x20],
    pub destination_address: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_random_number_bits {
    pub random_number: [u8; 0x10],
    pub reserved: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_ipv6_addr_bits {
    pub ipv6_address_127_96: [u8; 0x20],
    pub ipv6_address_95_64: [u8; 0x20],
    pub ipv6_address_63_32: [u8; 0x20],
    pub ipv6_address_31_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_tcp_icmp_header_bits {
    pub icmp_dw1: [u8; 0x20],
    pub icmp_dw2: [u8; 0x20],
    pub icmp_dw3: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_tunnel_header_bits {
    pub tunnel_header_0: [u8; 0x20],
    pub tunnel_header_1: [u8; 0x20],
    pub tunnel_header_2: [u8; 0x20],
    pub tunnel_header_3: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_ipsec_bits {
    pub spi: [u8; 0x20],
    pub sequence_number: [u8; 0x20],
    pub reserved: [u8; 0x10],
    pub ipsec_syndrome: [u8; 0x8],
    pub next_header: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_metadata_bits {
    pub metadata_to_cqe: [u8; 0x20],
    pub general_purpose: [u8; 0x20],
    pub acomulated_hash: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_flex_parser_bits {
    pub flex_parser_7: [u8; 0x20],
    pub flex_parser_6: [u8; 0x20],
    pub flex_parser_5: [u8; 0x20],
    pub flex_parser_4: [u8; 0x20],
    pub flex_parser_3: [u8; 0x20],
    pub flex_parser_2: [u8; 0x20],
    pub flex_parser_1: [u8; 0x20],
    pub flex_parser_0: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_registers_bits {
    pub register_c_10: [u8; 0x20],
    pub register_c_11: [u8; 0x20],
    pub register_c_8: [u8; 0x20],
    pub register_c_9: [u8; 0x20],
    pub register_c_6: [u8; 0x20],
    pub register_c_7: [u8; 0x20],
    pub register_c_4: [u8; 0x20],
    pub register_c_5: [u8; 0x20],
    pub register_c_2: [u8; 0x20],
    pub register_c_3: [u8; 0x20],
    pub register_c_0: [u8; 0x20],
    pub register_c_1: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_mpls_bits {
    pub mpls0_label: [u8; 0x20],
    pub mpls1_label: [u8; 0x20],
    pub mpls2_label: [u8; 0x20],
    pub mpls3_label: [u8; 0x20],
    pub mpls4_label: [u8; 0x20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_definer_hl_bits {
    pub eth_l2_outer: mlx5_ifc_definer_hl_eth_l2_bits,
    pub eth_l2_inner: mlx5_ifc_definer_hl_eth_l2_bits,
    pub eth_l2_src_outer: mlx5_ifc_definer_hl_eth_l2_src_bits,
    pub eth_l2_src_inner: mlx5_ifc_definer_hl_eth_l2_src_bits,
    pub ib_l2: mlx5_ifc_definer_hl_ib_l2_bits,
    pub eth_l3_outer: mlx5_ifc_definer_hl_eth_l3_bits,
    pub eth_l3_inner: mlx5_ifc_definer_hl_eth_l3_bits,
    pub eth_l4_outer: mlx5_ifc_definer_hl_eth_l4_bits,
    pub eth_l4_inner: mlx5_ifc_definer_hl_eth_l4_bits,
    pub source_qp_gvmi: mlx5_ifc_definer_hl_src_qp_gvmi_bits,
    pub ib_l4: mlx5_ifc_definer_hl_ib_l4_bits,
    pub oks1: mlx5_ifc_definer_hl_oks1_bits,
    pub oks2: mlx5_ifc_definer_hl_oks2_bits,
    pub voq: mlx5_ifc_definer_hl_voq_bits,
    pub reserved_at_480: [u8; 0x380],
    pub ipv4_src_dest_outer: mlx5_ifc_definer_hl_ipv4_src_dst_bits,
    pub ipv4_src_dest_inner: mlx5_ifc_definer_hl_ipv4_src_dst_bits,
    pub ipv6_dst_outer: mlx5_ifc_definer_hl_ipv6_addr_bits,
    pub ipv6_dst_inner: mlx5_ifc_definer_hl_ipv6_addr_bits,
    pub ipv6_src_outer: mlx5_ifc_definer_hl_ipv6_addr_bits,
    pub ipv6_src_inner: mlx5_ifc_definer_hl_ipv6_addr_bits,
    pub unsupported_dest_ib_l3: [u8; 0x80],
    pub unsupported_source_ib_l3: [u8; 0x80],
    pub unsupported_udp_misc_outer: [u8; 0x20],
    pub unsupported_udp_misc_inner: [u8; 0x20],
    pub tcp_icmp: mlx5_ifc_definer_tcp_icmp_header_bits,
    pub tunnel_header: mlx5_ifc_definer_hl_tunnel_header_bits,
    pub mpls_outer: mlx5_ifc_definer_hl_mpls_bits,
    pub mpls_inner: mlx5_ifc_definer_hl_mpls_bits,
    pub unsupported_config_headers_outer: [u8; 0x80],
    pub unsupported_config_headers_inner: [u8; 0x80],
    pub random_number: mlx5_ifc_definer_hl_random_number_bits,
    pub ipsec: mlx5_ifc_definer_hl_ipsec_bits,
    pub metadata: mlx5_ifc_definer_hl_metadata_bits,
    pub unsupported_utc_timestamp: [u8; 0x40],
    pub unsupported_free_running_timestamp: [u8; 0x40],
    pub flex_parser: mlx5_ifc_definer_hl_flex_parser_bits,
    pub registers: mlx5_ifc_definer_hl_registers_bits,
// Reserved in case header layout on future HW
    pub unsupported_reserved: [u8; 0xd40],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_definer_gtp {
    MLX5HWS_DEFINER_GTP_EXT_HDR_BIT = 0x04,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_gtp_bits {
    pub version: [u8; 0x3],
    pub proto_type: [u8; 0x1],
    pub reserved1: [u8; 0x1],
    pub msg_flags: [u8; 0x3],
    pub ext_hdr_flag: [u8; 0x1],
    pub seq_num_flag: [u8; 0x1],
    pub pdu_flag: [u8; 0x1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_opt_gtp_bits {
    pub seq_num: [u8; 0x10],
    pub pdu_num: [u8; 0x8],
    pub next_ext_hdr_type: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_gtp_psc_bits {
    pub len: [u8; 0x8],
    pub pdu_type: [u8; 0x4],
    pub flags: [u8; 0x4],
    pub qfi: [u8; 0x8],
    pub reserved2: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_ipv6_vtc_bits {
    pub version: [u8; 0x4],
    pub tos: [u8; 0x8],
    pub dscp: [u8; 0x6],
    pub ecn: [u8; 0x2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_ipv6_routing_ext_bits {
    pub next_hdr: [u8; 0x8],
    pub hdr_len: [u8; 0x8],
    pub type: [u8; 0x8],
    pub segments_left: [u8; 0x8],
    pub flags: [u8; 0x20],
    pub last_entry: [u8; 0x8],
    pub flag: [u8; 0x8],
    pub tag: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_vxlan_bits {
    pub flags: [u8; 0x8],
    pub reserved1: [u8; 0x18],
    pub vni: [u8; 0x18],
    pub reserved2: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_vxlan_gpe_bits {
    pub flags: [u8; 0x8],
    pub rsvd0: [u8; 0x10],
    pub protocol: [u8; 0x8],
    pub vni: [u8; 0x18],
    pub rsvd1: [u8; 0x8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_gre_bits {
    pub c_rsvd0_ver: [u8; 0x10],
    pub gre_c_present: [u8; 0x1],
    pub reserved_at_1: [u8; 0x1],
    pub gre_k_present: [u8; 0x1],
    pub gre_s_present: [u8; 0x1],
    pub reserved_at_4: [u8; 0x9],
    pub version: [u8; 0x3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_geneve_bits {
    pub ver_opt_len_o_c_rsvd: [u8; 0x10],
    pub version: [u8; 0x2],
    pub opt_len: [u8; 0x6],
    pub o_flag: [u8; 0x1],
    pub c_flag: [u8; 0x1],
    pub reserved_at_a: [u8; 0x6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_geneve_opt_bits {
    pub class: [u8; 0x10],
    pub type: [u8; 0x8],
    pub reserved: [u8; 0x3],
    pub len: [u8; 0x5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ifc_header_icmp_bits {
    pub icmp_dw1: [u8; 0x20],
    pub type: [u8; 0x8],
    pub code: [u8; 0x8],
    pub cksum: [u8; 0x10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_definer {
    pub type: mlx5hws_definer_type,
    pub dw_selector: [u8; DW_SELECTORS],
    pub byte_selector: [u8; BYTE_SELECTORS],
    pub mask: mlx5hws_rule_match_tag,
    pub obj_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_definer_cache {
    pub list_head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_definer_cache_item {
    pub definer: mlx5hws_definer,
    pub /: *mut *mut u32 refcount; / protected by context ctrl lock,
    pub list_node: list_head,
}

extern "C" {
    pub fn mlx5hws_definer_get_id(definer: *mut mlx5hws_definer) -> c_int;
}
extern "C" {
    pub fn mlx5hws_definer_init_cache(cache: *mut mlx5hws_definer_cache) -> c_int;
}
extern "C" {
    pub fn mlx5hws_definer_uninit_cache(cache: *mut mlx5hws_definer_cache);
}
