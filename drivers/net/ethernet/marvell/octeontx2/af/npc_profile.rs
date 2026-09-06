//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/npc_profile.h
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


// SPDX-License-Identifier: GPL-2.0
// Marvell RVU Admin Function driver
//
// Copyright (C) 2018 Marvell.
//
pub const NPC_KPU_PROFILE_VER: c_uint = 0x0000000100070000;

pub const NPC_IH_W: c_uint = 0x8000;
pub const NPC_IH_UTAG: c_uint = 0x2000;
pub const NPC_ETYPE_IP: c_uint = 0x0800;
pub const NPC_ETYPE_IP6: c_uint = 0x86dd;
pub const NPC_ETYPE_ARP: c_uint = 0x0806;
pub const NPC_ETYPE_RARP: c_uint = 0x8035;
pub const NPC_ETYPE_NGIO: c_uint = 0x8842;
pub const NPC_ETYPE_MPLSU: c_uint = 0x8847;
pub const NPC_ETYPE_MPLSM: c_uint = 0x8848;
pub const NPC_ETYPE_ETAG: c_uint = 0x893f;
pub const NPC_ETYPE_CTAG: c_uint = 0x8100;
pub const NPC_ETYPE_SBTAG: c_uint = 0x88a8;
pub const NPC_ETYPE_ITAG: c_uint = 0x88e7;
pub const NPC_ETYPE_PTP: c_uint = 0x88f7;
pub const NPC_ETYPE_FCOE: c_uint = 0x8906;
pub const NPC_ETYPE_QINQ: c_uint = 0x9100;
pub const NPC_ETYPE_QINQ2: c_uint = 0x9200;
pub const NPC_ETYPE_TRANS_ETH_BR: c_uint = 0x6558;
pub const NPC_ETYPE_PPP: c_uint = 0x880b;
pub const NPC_ETYPE_NSH: c_uint = 0x894f;
pub const NPC_ETYPE_DSA: c_uint = 0xdada;
pub const NPC_ETYPE_PPPOE: c_uint = 0x8864;
pub const NPC_ETYPE_ERSPA: c_uint = 0x88be;
pub const NPC_ETYPE_FP: c_uint = 0x8903;
pub const NPC_PPP_IP: c_uint = 0x0021;
pub const NPC_PPP_IP6: c_uint = 0x0057;
pub const NPC_IPNH_HOP: c_int = 0;
pub const NPC_IPNH_ICMP: c_int = 1;
pub const NPC_IPNH_IGMP: c_int = 2;
pub const NPC_IPNH_IP: c_int = 4;
pub const NPC_IPNH_TCP: c_int = 6;
pub const NPC_IPNH_UDP: c_int = 17;
pub const NPC_IPNH_IP6: c_int = 41;
pub const NPC_IPNH_ROUT: c_int = 43;
pub const NPC_IPNH_FRAG: c_int = 44;
pub const NPC_IPNH_GRE: c_int = 47;
pub const NPC_IPNH_ESP: c_int = 50;
pub const NPC_IPNH_AH: c_int = 51;
pub const NPC_IPNH_ICMP6: c_int = 58;
pub const NPC_IPNH_NONH: c_int = 59;
pub const NPC_IPNH_DEST: c_int = 60;
pub const NPC_IPNH_SCTP: c_int = 132;
pub const NPC_IPNH_MOBILITY: c_int = 135;
pub const NPC_IPNH_MPLS: c_int = 137;
pub const NPC_IPNH_HOSTID: c_int = 139;
pub const NPC_IPNH_SHIM6: c_int = 140;
pub const NPC_IPNH_CUSTOM: c_int = 253;
pub const NPC_IP6_ROUTE_TYPE: c_int = 4;
pub const NPC_UDP_PORT_PTP_E: c_int = 319;
pub const NPC_UDP_PORT_PTP_G: c_int = 320;
pub const NPC_UDP_PORT_GTPC: c_int = 2123;
pub const NPC_UDP_PORT_GTPU: c_int = 2152;
pub const NPC_UDP_PORT_VXLAN: c_int = 4789;
pub const NPC_UDP_PORT_VXLANGPE: c_int = 4790;
pub const NPC_UDP_PORT_GENEVE: c_int = 6081;
pub const NPC_UDP_PORT_MPLS: c_int = 6635;
pub const NPC_UDP_PORT_ESP: c_int = 4500;
pub const NPC_UDP_PORT_ROCEV2: c_int = 4791;
pub const NPC_VXLANGPE_NP_IP: c_uint = 0x1;
pub const NPC_VXLANGPE_NP_IP6: c_uint = 0x2;
pub const NPC_VXLANGPE_NP_ETH: c_uint = 0x3;
pub const NPC_VXLANGPE_NP_NSH: c_uint = 0x4;
pub const NPC_VXLANGPE_NP_MPLS: c_uint = 0x5;
pub const NPC_VXLANGPE_NP_GBP: c_uint = 0x6;
pub const NPC_VXLANGPE_NP_VBNG: c_uint = 0x7;
pub const NPC_NSH_NP_IP: c_uint = 0x1;
pub const NPC_NSH_NP_IP6: c_uint = 0x2;
pub const NPC_NSH_NP_ETH: c_uint = 0x3;
pub const NPC_NSH_NP_NSH: c_uint = 0x4;
pub const NPC_NSH_NP_MPLS: c_uint = 0x5;
pub const NPC_TCP_PORT_HTTP: c_int = 80;
pub const NPC_TCP_PORT_HTTPS: c_int = 443;
pub const NPC_TCP_PORT_PPTP: c_int = 1723;
pub const NPC_MPLS_S: c_uint = 0x0100;
pub const NPC_IP_TTL_MASK: c_uint = 0xff00;
pub const NPC_IP_VER_4: c_uint = 0x4000;
pub const NPC_IP_VER_6: c_uint = 0x6000;
pub const NPC_IP_VER_MASK: c_uint = 0xf000;
pub const NPC_IP_HDR_LEN_5: c_uint = 0x0500;
pub const NPC_IP_HDR_LEN_MASK: c_uint = 0x0f00;
pub const NPC_IP_HDR_MF: c_uint = 0x2000;
pub const NPC_IP_HDR_FRAGOFF: c_uint = 0x1fff;
pub const NPC_IP6_HOP_MASK: c_uint = 0x00ff;
pub const NPC_IP6_FRAG_FRAGOFF: c_uint = 0xfff8;

pub const NPC_GRE_VER_MASK: c_uint = 0x0003;
pub const NPC_GRE_VER_1: c_uint = 0x0001;
pub const NPC_VXLAN_I: c_uint = 0x0800;

pub const NPC_VXLANGPE_NP_MASK: c_uint = 0x00ff;
pub const NPC_NSH_NP_MASK: c_uint = 0x00ff;

pub const NPC_GTP_MT_G_PDU: c_uint = 0xff;
pub const NPC_GTP_MT_MASK: c_uint = 0xff;
pub const NPC_TCP_FLAGS_FIN: c_uint = 0x0001;
pub const NPC_TCP_FLAGS_SYN: c_uint = 0x0002;
pub const NPC_TCP_FLAGS_RST: c_uint = 0x0004;
pub const NPC_TCP_FLAGS_PSH: c_uint = 0x0008;
pub const NPC_TCP_FLAGS_ACK: c_uint = 0x0010;
pub const NPC_TCP_FLAGS_URG: c_uint = 0x0020;
pub const NPC_TCP_FLAGS_MASK: c_uint = 0x003f;
pub const NPC_TCP_DATA_OFFSET_5: c_uint = 0x5000;
pub const NPC_TCP_DATA_OFFSET_MASK: c_uint = 0xf000;
pub const NPC_DSA_EXTEND: c_uint = 0x1000;
pub const NPC_DSA_EDSA: c_uint = 0x8000;
pub const NPC_DSA_FDSA: c_uint = 0xc000;
pub const NPC_KEXOF_DMAC: c_int = 9;
pub const MKEX_SIGN: c_uint = 0x19bbfdbd15f;

// Rx parse key extract nibble enable

// Tx parse key extract nibble enable

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_parser_state {
    NPC_S_NA = 0,
    NPC_S_KPU1_ETHER,
    NPC_S_KPU1_IH_NIX,
    NPC_S_KPU1_IH,
    NPC_S_KPU1_EXDSA,
    NPC_S_KPU1_HIGIG2,
    NPC_S_KPU1_IH_NIX_HIGIG2,
    NPC_S_KPU1_CUSTOM_PRE_L2,
    NPC_S_KPU1_CPT_HDR,
    NPC_S_KPU1_VLAN_EXDSA,
    NPC_S_KPU2_CTAG,
    NPC_S_KPU2_CTAG2,
    NPC_S_KPU2_SBTAG,
    NPC_S_KPU2_QINQ,
    NPC_S_KPU2_ETAG,
    NPC_S_KPU2_EXDSA,
    NPC_S_KPU2_CPT_CTAG,
    NPC_S_KPU2_CPT_QINQ,
    NPC_S_KPU2_CPT_CTAG2,
    NPC_S_KPU2_CPT_SBTAG,
    NPC_S_KPU2_MT,
    NPC_S_KPU3_CTAG,
    NPC_S_KPU3_STAG,
    NPC_S_KPU3_QINQ,
    NPC_S_KPU3_CTAG_C,
    NPC_S_KPU3_STAG_C,
    NPC_S_KPU3_QINQ_C,
    NPC_S_KPU3_DSA,
    NPC_S_KPU3_VLAN_EXDSA,
    NPC_S_KPU3_CPT_QINQ,
    NPC_S_KPU3_CPT_CTAG,
    NPC_S_KPU3_CPT_STAG,
    NPC_S_KPU4_MPLS,
    NPC_S_KPU4_NSH,
    NPC_S_KPU4_FDSA,
    NPC_S_KPU4_VLAN_EXDSA,
    NPC_S_KPU4_PPPOE,
    NPC_S_KPU5_IP,
    NPC_S_KPU5_IP6,
    NPC_S_KPU5_ARP,
    NPC_S_KPU5_RARP,
    NPC_S_KPU5_PTP,
    NPC_S_KPU5_FCOE,
    NPC_S_KPU5_MPLS,
    NPC_S_KPU5_MPLS_PL,
    NPC_S_KPU5_NSH,
    NPC_S_KPU5_CPT_IP,
    NPC_S_KPU5_CPT_IP6,
    NPC_S_KPU5_NGIO,
    NPC_S_KPU6_IP6_EXT,
    NPC_S_KPU6_IP6_HOP_DEST,
    NPC_S_KPU6_IP6_ROUT,
    NPC_S_KPU6_IP6_FRAG,
    NPC_S_KPU6_IP6_CPT_FRAG,
    NPC_S_KPU6_IP6_CPT_HOP_DEST,
    NPC_S_KPU6_IP6_CPT_ROUT,
    NPC_S_KPU7_IP6_EXT,
    NPC_S_KPU7_IP6_ROUT,
    NPC_S_KPU7_IP6_FRAG,
    NPC_S_KPU7_CPT_IP6_FRAG,
    NPC_S_KPU8_TCP,
    NPC_S_KPU8_UDP,
    NPC_S_KPU8_SCTP,
    NPC_S_KPU8_ICMP,
    NPC_S_KPU8_IGMP,
    NPC_S_KPU8_ICMP6,
    NPC_S_KPU8_GRE,
    NPC_S_KPU8_AH,
    NPC_S_KPU8_CUSTOM,
    NPC_S_KPU9_TU_MPLS_IN_GRE,
    NPC_S_KPU9_TU_MPLS_IN_NSH,
    NPC_S_KPU9_TU_MPLS_IN_IP,
    NPC_S_KPU9_TU_MPLS_IN_UDP,
    NPC_S_KPU9_TU_NSH_IN_GRE,
    NPC_S_KPU9_VXLAN,
    NPC_S_KPU9_VXLANGPE,
    NPC_S_KPU9_GENEVE,
    NPC_S_KPU9_GTPC,
    NPC_S_KPU9_GTPU,
    NPC_S_KPU9_ESP,
    NPC_S_KPU9_CUSTOM,
    NPC_S_KPU10_TU_MPLS_IN_VXLANGPE,
    NPC_S_KPU10_TU_MPLS_PL,
    NPC_S_KPU10_TU_MPLS,
    NPC_S_KPU10_TU_NSH_IN_VXLANGPE,
    NPC_S_KPU11_TU_ETHER,
    NPC_S_KPU11_TU_PPP,
    NPC_S_KPU11_TU_MPLS_IN_NSH,
    NPC_S_KPU11_TU_MPLS_PL,
    NPC_S_KPU11_TU_MPLS,
    NPC_S_KPU11_TU_ETHER_IN_NSH,
    NPC_S_KPU12_TU_IP,
    NPC_S_KPU12_TU_IP6,
    NPC_S_KPU12_TU_ARP,
    NPC_S_KPU13_TU_IP6_EXT,
    NPC_S_KPU14_TU_IP6_EXT,
    NPC_S_KPU15_TU_TCP,
    NPC_S_KPU15_TU_UDP,
    NPC_S_KPU15_TU_SCTP,
    NPC_S_KPU15_TU_ICMP,
    NPC_S_KPU15_TU_IGMP,
    NPC_S_KPU15_TU_ICMP6,
    NPC_S_KPU15_TU_ESP,
    NPC_S_KPU15_TU_AH,
    NPC_S_KPU16_HTTP_DATA,
    NPC_S_KPU16_HTTPS_DATA,
    NPC_S_KPU16_PPTP_DATA,
    NPC_S_KPU16_TCP_DATA,
    NPC_S_KPU16_UDP_DATA,
    NPC_S_KPU16_UDP_PTP,
    NPC_S_LAST /* has to be the last item */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu9_extra_parser_state {
    NPC_S_KPU9_ROCEV2 = NPC_S_LAST + 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu4_extra_parser_state {
    NPC_S_KPU4_SBTAG_PTP = NPC_S_LAST + 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_la_uflag {
    NPC_F_LA_U_HAS_TAG = 0x10,
    NPC_F_LA_U_HAS_IH_NIX = 0x20,
    NPC_F_LA_U_HAS_HIGIG2 = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_la_lflag {
    NPC_F_LA_L_UNK_ETYPE = 1,
    NPC_F_LA_L_WITH_VLAN,
    NPC_F_LA_L_WITH_ETAG,
    NPC_F_LA_L_WITH_MPLS,
    NPC_F_LA_L_WITH_NSH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lb_uflag {
    NPC_F_LB_U_UNK_ETYPE = 0x80,
    NPC_F_LB_U_MORE_TAG = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lb_lflag {
    NPC_F_LB_L_WITH_CTAG = 1,
    NPC_F_LB_L_WITH_CTAG_UNK,
    NPC_F_LB_L_WITH_STAG_CTAG,
    NPC_F_LB_L_WITH_STAG_STAG,
    NPC_F_LB_L_WITH_QINQ_CTAG,
    NPC_F_LB_L_WITH_QINQ_QINQ,
    NPC_F_LB_L_WITH_ITAG,
    NPC_F_LB_L_WITH_ITAG_STAG,
    NPC_F_LB_L_WITH_ITAG_CTAG,
    NPC_F_LB_L_WITH_ITAG_UNK,
    NPC_F_LB_L_WITH_BTAG_ITAG,
    NPC_F_LB_L_WITH_STAG,
    NPC_F_LB_L_WITH_QINQ,
    NPC_F_LB_L_DSA,
    NPC_F_LB_L_DSA_VLAN,
    NPC_F_LB_L_EDSA,
    NPC_F_LB_L_EDSA_VLAN,
    NPC_F_LB_L_EXDSA,
    NPC_F_LB_L_EXDSA_VLAN,
    NPC_F_LB_L_FDSA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_cn20k_kpu_lc_uflag {
    NPC_CN20K_F_LC_U_MPLS_IN_IP = 0x20,
    NPC_CN20K_F_LC_U_IP6_TUN_IP6 = 0x40,
    NPC_CN20K_F_LC_U_IP6_MPLS_IN_IP = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_cn20k_kpu_lc_lflag {
    NPC_CN20K_F_LC_L_IP_FRAG = 2,
    NPC_CN20K_F_LC_L_IP6_FRAG,
    NPC_CN20K_F_LC_L_6TO4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lc_uflag {
    NPC_F_LC_U_UNK_PROTO = 0x10,
    NPC_F_LC_U_IP_FRAG = 0x20,
    NPC_F_LC_U_IP6_FRAG = 0x40,
    NPC_F_LC_L_6TO4 = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lc_lflag {
    NPC_F_LC_L_IP_IN_IP = 1,
    NPC_F_LC_L_MPLS_IN_IP,
    NPC_F_LC_L_IP6_TUN_IP6,
    NPC_F_LC_L_IP6_MPLS_IN_IP,
    NPC_F_LC_L_MPLS_4_LABELS,
    NPC_F_LC_L_MPLS_3_LABELS,
    NPC_F_LC_L_MPLS_2_LABELS,
    NPC_F_LC_L_EXT_HOP,
    NPC_F_LC_L_EXT_DEST,
    NPC_F_LC_L_EXT_ROUT,
    NPC_F_LC_L_EXT_MOBILITY,
    NPC_F_LC_L_EXT_HOSTID,
    NPC_F_LC_L_EXT_SHIM6,
    NPC_F_LC_L_IP6_SRH_SEG_1,
    NPC_F_LC_L_IP6_SRH_SEG_2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_ld_lflag {
    NPC_F_LD_L_TCP_UNK_PORT = 1,
    NPC_F_LD_L_TCP_HAS_OPTIONS,
    NPC_F_LD_L_TCP_UNK_PORT_HAS_OPTIONS,
    NPC_F_LD_L_UDP_UNK_PORT,
    NPC_F_LD_L_GRE_NVGRE,
    NPC_F_LD_L_GRE_HAS_SRE,
    NPC_F_LD_L_GRE_HAS_CSUM,
    NPC_F_LD_L_GRE_HAS_KEY,
    NPC_F_LD_L_GRE_HAS_SEQ,
    NPC_F_LD_L_GRE_HAS_CSUM_KEY,
    NPC_F_LD_L_GRE_HAS_CSUM_SEQ,
    NPC_F_LD_L_GRE_HAS_KEY_SEQ,
    NPC_F_LD_L_GRE_HAS_CSUM_KEY_SEQ,
    NPC_F_LD_L_GRE_HAS_ROUTE,
    NPC_F_LD_L_GRE_UNK_PROTO,
    NPC_F_LD_L_GRE_VER1,
    NPC_F_LD_L_GRE_VER1_HAS_SEQ,
    NPC_F_LD_L_GRE_VER1_HAS_ACK,
    NPC_F_LD_L_GRE_VER1_HAS_SEQ_ACK,
    NPC_F_LD_L_GRE_VER1_UNK_PROTO,
    NPC_F_LD_L_MPLS_4_LABELS,
    NPC_F_LD_L_MPLS_3_LABELS,
    NPC_F_LD_L_MPLS_2_LABELS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_le_lflag {
    NPC_F_LE_L_VXLAN_NOVNI,
    NPC_F_LE_L_VXLANGPE_NOVNI,
    NPC_F_LE_L_VXLANGPE_UNK,
    NPC_F_LE_L_VXLANGPE_NONP,
    NPC_F_LE_L_GENEVE_OAM,
    NPC_F_LE_L_GENEVE_CRI_OPT,
    NPC_F_LE_L_GENEVE_OAM_CRI_OPT,
    NPC_F_LE_L_GTPU_G_PDU,
    NPC_F_LE_L_GTPU_UNK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lf_uflag {
    NPC_F_LF_U_UNK_ETYPE = 0x10,
    NPC_F_LF_U_HAS_TAG = 0x20,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lf_lflag {
    NPC_F_LF_L_WITH_CTAG = 1,
    NPC_F_LF_L_WITH_STAG_CTAG,
    NPC_F_LF_L_WITH_STAG,
    NPC_F_LF_L_WITH_QINQ_CTAG,
    NPC_F_LF_L_WITH_QINQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lg_uflag {
    NPC_F_LG_U_UNK_IP_PROTO = 0x10,
    NPC_F_LG_U_IP_HAS_OPTIONS = 0x20,
    NPC_F_LG_U_IP6_HAS_EXT = 0x40,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lh_uflag {
    NPC_F_LH_U_TCP_HAS_OPTIONS = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_lh_lflag {
    NPC_F_LH_L_TCP_HTTP = 1,
    NPC_F_LH_L_TCP_HTTPS,
    NPC_F_LH_L_TCP_PPTP,
    NPC_F_LH_L_TCP_UNK_PORT,
    NPC_F_LH_L_UDP_UNK_PORT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_kpu_err_code {
    NPC_EC_NOERR = 0, /* has to be zero */
    NPC_EC_UNK,
    NPC_EC_IH_LENGTH,
    NPC_EC_EDSA_UNK,
    NPC_EC_L2_K1,
    NPC_EC_L2_K2,
    NPC_EC_L2_K3,
    NPC_EC_L2_K3_ETYPE_UNK,
    NPC_EC_L2_K4,
    NPC_EC_MPLS_2MANY,
    NPC_EC_MPLS_UNK,
    NPC_EC_NSH_UNK,
    NPC_EC_IP_TTL_0,
    NPC_EC_IP_FRAG_OFFSET_1,
    NPC_EC_IP_VER,
    NPC_EC_IP6_HOP_0,
    NPC_EC_IP6_VER,
    NPC_EC_TCP_FLAGS_FIN_ONLY,
    NPC_EC_TCP_FLAGS_ZERO,
    NPC_EC_TCP_FLAGS_RST_FIN,
    NPC_EC_TCP_FLAGS_URG_SYN,
    NPC_EC_TCP_FLAGS_RST_SYN,
    NPC_EC_TCP_FLAGS_SYN_FIN,
    NPC_EC_VXLAN,
    NPC_EC_NVGRE,
    NPC_EC_GRE,
    NPC_EC_GRE_VER1,
    NPC_EC_L4,
    NPC_EC_OIP4_CSUM,
    NPC_EC_IIP4_CSUM,
    NPC_EC_LAST /* has to be the last item */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NPC_ERRLEV_E {
    NPC_ERRLEV_RE = 0,
    NPC_ERRLEV_LA = 1,
    NPC_ERRLEV_LB = 2,
    NPC_ERRLEV_LC = 3,
    NPC_ERRLEV_LD = 4,
    NPC_ERRLEV_LE = 5,
    NPC_ERRLEV_LF = 6,
    NPC_ERRLEV_LG = 7,
    NPC_ERRLEV_LH = 8,
    NPC_ERRLEV_R9 = 9,
    NPC_ERRLEV_R10 = 10,
    NPC_ERRLEV_R11 = 11,
    NPC_ERRLEV_R12 = 12,
    NPC_ERRLEV_R13 = 13,
    NPC_ERRLEV_R14 = 14,
    NPC_ERRLEV_NIX = 15,
    NPC_ERRLEV_ENUM_LAST = 16,
}

// nibble: LA..LE (ltype only) + Error code + Channel
// nibble: LA..LE (ltype only)
// Default RX MCAM KEX profile
// Layer A: Ethernet:
// DMAC: 6 bytes, KW1[55:8]
// Ethertype: 2 bytes, KW0[55:40]
// DMAC: 6 bytes, KW1[55:8]
// Ethertype: 2 bytes, KW0[55:40]
// Layer A: HiGig2:
// Classification: 2 bytes, KW1[23:8]
// VID: 2 bytes, KW1[39:24]
// Layer B: Single VLAN (CTAG)
// CTAG VLAN: 2 bytes, KW1[7:0], KW0[63:56]
// Ethertype: 2 bytes, KW0[55:40]
// Layer B: Stacked VLAN (STAG|QinQ)
// Outer VLAN: 2 bytes, KW1[7:0], KW0[63:56]
// Ethertype: 2 bytes, KW0[55:40]
// SWITCH PORT: 1 byte, KW0[63:56]
// Ethertype: 2 bytes, KW0[55:40]
// Layer C: IPv4
// SIP+DIP: 8 bytes, KW2[63:0]
// TOS: 1 byte, KW1[63:56]
// Layer C: IPv6
// Everything up to SADDR: 8 bytes, KW2[63:0]
// Layer D:UDP
// SPORT+DPORT: 4 bytes, KW3[31:0]
// Layer D:TCP
// SPORT+DPORT: 4 bytes, KW3[31:0]
// Default TX MCAM KEX profile
// Layer A: NIX_INST_HDR_S + Ethernet
// NIX appends 8 bytes of NIX_INST_HDR_S at the
// start of each TX packet supplied to NPC.
//
// PF_FUNC: 2B , KW0 [47:32]
// DMAC: 6 bytes, KW1[63:16]
// Layer A: HiGig2:
// PF_FUNC: 2B , KW0 [47:32]
// VID: 2 bytes, KW1[31:16]
// Layer B: Single VLAN (CTAG)
// CTAG VLAN[2..3] KW0[63:48]
// CTAG VLAN[2..3] KW1[15:0]
// Layer B: Stacked VLAN (STAG|QinQ)
// Outer VLAN: 2 bytes, KW0[63:48]
// Outer VLAN: 2 Bytes, KW1[15:0]
// Layer C: IPv4
// SIP+DIP: 8 bytes, KW2[63:0]
// Layer C: IPv6
// Everything up to SADDR: 8 bytes, KW2[63:0]
// Layer D:UDP
// SPORT+DPORT: 4 bytes, KW3[31:0]
// Layer D:TCP
// SPORT+DPORT: 4 bytes, KW3[31:0]
