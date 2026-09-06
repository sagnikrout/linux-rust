//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/mscc/ocelot_vcap.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Microsemi Ocelot Switch driver
// Copyright (c) 2019 Microsemi Corporation
//

// Cookie definitions for private VCAP filters installed by the driver.
// Must be unique per VCAP block.
//

// =================================================================
// VCAP Common
// =================================================================
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_props {
    pub /: *mut *mut u16 tg_width; / Type-group width (in bits),
    pub /: *mut *mut u16 sw_count; / Sub word count,
    pub /: *mut *mut u16 entry_count; / Entry count,
    pub /: *mut *mut u16 entry_words; / Number of entry words,
    pub /: *mut *mut u16 entry_width; / Entry width (in bits),
    pub /: *mut *mut u16 action_count; / Action count,
    pub /: *mut *mut u16 action_words; / Number of action words,
    pub /: *mut *mut u16 action_width; / Action width (in bits),
    pub /: *mut *mut u16 action_type_width; / Action type width (in bits),
    pub /: *mut *mut u16 width; / Action type width (in bits),
    pub /: *mut *mut u16 count; / Action type sub word count,
    pub action_table: [}; 2],
    pub /: *mut *mut u16 counter_words; / Number of counter words,
    pub /: *mut *mut u16 counter_width; / Counter width (in bits),
    pub target: ocelot_target,
    pub keys: *const vcap_field,
    pub actions: *const vcap_field,
}

// VCAP Type-Group values

pub const VCAP_CACHE_ENTRY_DAT_RSZ: c_uint = 0x4;
pub const VCAP_CACHE_MASK_DAT_RSZ: c_uint = 0x4;
pub const VCAP_CACHE_ACTION_DAT_RSZ: c_uint = 0x4;
pub const VCAP_CACHE_CNT_DAT_RSZ: c_uint = 0x4;

// =================================================================
// VCAP IS2
// =================================================================
//
// IS2 half key types
pub const IS2_TYPE_ETYPE: c_int = 0;
pub const IS2_TYPE_LLC: c_int = 1;
pub const IS2_TYPE_SNAP: c_int = 2;
pub const IS2_TYPE_ARP: c_int = 3;
pub const IS2_TYPE_IP_UDP_TCP: c_int = 4;
pub const IS2_TYPE_IP_OTHER: c_int = 5;
pub const IS2_TYPE_IPV6: c_int = 6;
pub const IS2_TYPE_OAM: c_int = 7;
pub const IS2_TYPE_SMAC_SIP6: c_int = 8;

// IS2 half key type mask for matching any IP
pub const IS2_TYPE_MASK_IP_ANY: c_uint = 0xe;
// IS2 MASK_MODE values
pub const IS2_ACT_MASK_MODE_NONE: c_int = 0;
pub const IS2_ACT_MASK_MODE_FILTER: c_int = 1;
pub const IS2_ACT_MASK_MODE_POLICY: c_int = 2;
pub const IS2_ACT_MASK_MODE_REDIR: c_int = 3;
// IS2 REW_OP values
pub const IS2_ACT_REW_OP_NONE: c_int = 0;
pub const IS2_ACT_REW_OP_PTP_ONE: c_int = 2;
pub const IS2_ACT_REW_OP_PTP_TWO: c_int = 3;
pub const IS2_ACT_REW_OP_SPECIAL: c_int = 8;
pub const IS2_ACT_REW_OP_PTP_ORG: c_int = 9;

pub const VCAP_PORT_WIDTH: c_int = 4;
// IS2 quarter key - SMAC_SIP4
pub const IS2_QKO_IGR_PORT: c_int = 0;

pub const IS2_QKL_L2_SMAC: c_int = 48;

pub const IS2_QKL_L3_IP4_SIP: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is2_half_key_field {
// Common
    VCAP_IS2_TYPE,
    VCAP_IS2_HK_FIRST,
    VCAP_IS2_HK_PAG,
    VCAP_IS2_HK_RSV1,
    VCAP_IS2_HK_IGR_PORT_MASK,
    VCAP_IS2_HK_RSV2,
    VCAP_IS2_HK_HOST_MATCH,
    VCAP_IS2_HK_L2_MC,
    VCAP_IS2_HK_L2_BC,
    VCAP_IS2_HK_VLAN_TAGGED,
    VCAP_IS2_HK_VID,
    VCAP_IS2_HK_DEI,
    VCAP_IS2_HK_PCP,
// MAC_ETYPE / MAC_LLC / MAC_SNAP / OAM common
    VCAP_IS2_HK_L2_DMAC,
    VCAP_IS2_HK_L2_SMAC,
// MAC_ETYPE (TYPE=000)
    VCAP_IS2_HK_MAC_ETYPE_ETYPE,
    VCAP_IS2_HK_MAC_ETYPE_L2_PAYLOAD0,
    VCAP_IS2_HK_MAC_ETYPE_L2_PAYLOAD1,
    VCAP_IS2_HK_MAC_ETYPE_L2_PAYLOAD2,
// MAC_LLC (TYPE=001)
    VCAP_IS2_HK_MAC_LLC_DMAC,
    VCAP_IS2_HK_MAC_LLC_SMAC,
    VCAP_IS2_HK_MAC_LLC_L2_LLC,
// MAC_SNAP (TYPE=010)
    VCAP_IS2_HK_MAC_SNAP_SMAC,
    VCAP_IS2_HK_MAC_SNAP_DMAC,
    VCAP_IS2_HK_MAC_SNAP_L2_SNAP,
// MAC_ARP (TYPE=011)
    VCAP_IS2_HK_MAC_ARP_SMAC,
    VCAP_IS2_HK_MAC_ARP_ADDR_SPACE_OK,
    VCAP_IS2_HK_MAC_ARP_PROTO_SPACE_OK,
    VCAP_IS2_HK_MAC_ARP_LEN_OK,
    VCAP_IS2_HK_MAC_ARP_TARGET_MATCH,
    VCAP_IS2_HK_MAC_ARP_SENDER_MATCH,
    VCAP_IS2_HK_MAC_ARP_OPCODE_UNKNOWN,
    VCAP_IS2_HK_MAC_ARP_OPCODE,
    VCAP_IS2_HK_MAC_ARP_L3_IP4_DIP,
    VCAP_IS2_HK_MAC_ARP_L3_IP4_SIP,
    VCAP_IS2_HK_MAC_ARP_DIP_EQ_SIP,
// IP4_TCP_UDP / IP4_OTHER common
    VCAP_IS2_HK_IP4,
    VCAP_IS2_HK_L3_FRAGMENT,
    VCAP_IS2_HK_L3_FRAG_OFS_GT0,
    VCAP_IS2_HK_L3_OPTIONS,
    VCAP_IS2_HK_IP4_L3_TTL_GT0,
    VCAP_IS2_HK_L3_TOS,
    VCAP_IS2_HK_L3_IP4_DIP,
    VCAP_IS2_HK_L3_IP4_SIP,
    VCAP_IS2_HK_DIP_EQ_SIP,
// IP4_TCP_UDP (TYPE=100)
    VCAP_IS2_HK_TCP,
    VCAP_IS2_HK_L4_SPORT,
    VCAP_IS2_HK_L4_DPORT,
    VCAP_IS2_HK_L4_RNG,
    VCAP_IS2_HK_L4_SPORT_EQ_DPORT,
    VCAP_IS2_HK_L4_SEQUENCE_EQ0,
    VCAP_IS2_HK_L4_URG,
    VCAP_IS2_HK_L4_ACK,
    VCAP_IS2_HK_L4_PSH,
    VCAP_IS2_HK_L4_RST,
    VCAP_IS2_HK_L4_SYN,
    VCAP_IS2_HK_L4_FIN,
    VCAP_IS2_HK_L4_1588_DOM,
    VCAP_IS2_HK_L4_1588_VER,
// IP4_OTHER (TYPE=101)
    VCAP_IS2_HK_IP4_L3_PROTO,
    VCAP_IS2_HK_L3_PAYLOAD,
// IP6_STD (TYPE=110)
    VCAP_IS2_HK_IP6_L3_TTL_GT0,
    VCAP_IS2_HK_IP6_L3_PROTO,
    VCAP_IS2_HK_L3_IP6_SIP,
// OAM (TYPE=111)
    VCAP_IS2_HK_OAM_MEL_FLAGS,
    VCAP_IS2_HK_OAM_VER,
    VCAP_IS2_HK_OAM_OPCODE,
    VCAP_IS2_HK_OAM_FLAGS,
    VCAP_IS2_HK_OAM_MEPID,
    VCAP_IS2_HK_OAM_CCM_CNTS_EQ0,
    VCAP_IS2_HK_OAM_IS_Y1731,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcap_field {
    pub offset: c_int,
    pub length: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is2_action_field {
    VCAP_IS2_ACT_HIT_ME_ONCE,
    VCAP_IS2_ACT_CPU_COPY_ENA,
    VCAP_IS2_ACT_CPU_QU_NUM,
    VCAP_IS2_ACT_MASK_MODE,
    VCAP_IS2_ACT_MIRROR_ENA,
    VCAP_IS2_ACT_LRN_DIS,
    VCAP_IS2_ACT_POLICE_ENA,
    VCAP_IS2_ACT_POLICE_IDX,
    VCAP_IS2_ACT_POLICE_VCAP_ONLY,
    VCAP_IS2_ACT_PORT_MASK,
    VCAP_IS2_ACT_REW_OP,
    VCAP_IS2_ACT_SMAC_REPLACE_ENA,
    VCAP_IS2_ACT_RSV,
    VCAP_IS2_ACT_ACL_ID,
    VCAP_IS2_ACT_HIT_CNT,
}

// =================================================================
// VCAP IS1
// =================================================================
//
// IS1 half key types
pub const IS1_TYPE_S1_NORMAL: c_int = 0;
pub const IS1_TYPE_S1_5TUPLE_IP4: c_int = 1;
// IS1 full key types
pub const IS1_TYPE_S1_NORMAL_IP6: c_int = 0;
pub const IS1_TYPE_S1_7TUPLE: c_int = 1;
pub const IS2_TYPE_S1_5TUPLE_IP6: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is1_half_key_field {
    VCAP_IS1_HK_TYPE,
    VCAP_IS1_HK_LOOKUP,
    VCAP_IS1_HK_IGR_PORT_MASK,
    VCAP_IS1_HK_RSV,
    VCAP_IS1_HK_OAM_Y1731,
    VCAP_IS1_HK_L2_MC,
    VCAP_IS1_HK_L2_BC,
    VCAP_IS1_HK_IP_MC,
    VCAP_IS1_HK_VLAN_TAGGED,
    VCAP_IS1_HK_VLAN_DBL_TAGGED,
    VCAP_IS1_HK_TPID,
    VCAP_IS1_HK_VID,
    VCAP_IS1_HK_DEI,
    VCAP_IS1_HK_PCP,
// Specific Fields for IS1 Half Key S1_NORMAL
    VCAP_IS1_HK_L2_SMAC,
    VCAP_IS1_HK_ETYPE_LEN,
    VCAP_IS1_HK_ETYPE,
    VCAP_IS1_HK_IP_SNAP,
    VCAP_IS1_HK_IP4,
    VCAP_IS1_HK_L3_FRAGMENT,
    VCAP_IS1_HK_L3_FRAG_OFS_GT0,
    VCAP_IS1_HK_L3_OPTIONS,
    VCAP_IS1_HK_L3_DSCP,
    VCAP_IS1_HK_L3_IP4_SIP,
    VCAP_IS1_HK_TCP_UDP,
    VCAP_IS1_HK_TCP,
    VCAP_IS1_HK_L4_SPORT,
    VCAP_IS1_HK_L4_RNG,
// Specific Fields for IS1 Half Key S1_5TUPLE_IP4
    VCAP_IS1_HK_IP4_INNER_TPID,
    VCAP_IS1_HK_IP4_INNER_VID,
    VCAP_IS1_HK_IP4_INNER_DEI,
    VCAP_IS1_HK_IP4_INNER_PCP,
    VCAP_IS1_HK_IP4_IP4,
    VCAP_IS1_HK_IP4_L3_FRAGMENT,
    VCAP_IS1_HK_IP4_L3_FRAG_OFS_GT0,
    VCAP_IS1_HK_IP4_L3_OPTIONS,
    VCAP_IS1_HK_IP4_L3_DSCP,
    VCAP_IS1_HK_IP4_L3_IP4_DIP,
    VCAP_IS1_HK_IP4_L3_IP4_SIP,
    VCAP_IS1_HK_IP4_L3_PROTO,
    VCAP_IS1_HK_IP4_TCP_UDP,
    VCAP_IS1_HK_IP4_TCP,
    VCAP_IS1_HK_IP4_L4_RNG,
    VCAP_IS1_HK_IP4_IP_PAYLOAD_S1_5TUPLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is1_action_field {
    VCAP_IS1_ACT_DSCP_ENA,
    VCAP_IS1_ACT_DSCP_VAL,
    VCAP_IS1_ACT_QOS_ENA,
    VCAP_IS1_ACT_QOS_VAL,
    VCAP_IS1_ACT_DP_ENA,
    VCAP_IS1_ACT_DP_VAL,
    VCAP_IS1_ACT_PAG_OVERRIDE_MASK,
    VCAP_IS1_ACT_PAG_VAL,
    VCAP_IS1_ACT_RSV,
    VCAP_IS1_ACT_VID_REPLACE_ENA,
    VCAP_IS1_ACT_VID_ADD_VAL,
    VCAP_IS1_ACT_FID_SEL,
    VCAP_IS1_ACT_FID_VAL,
    VCAP_IS1_ACT_PCP_DEI_ENA,
    VCAP_IS1_ACT_PCP_VAL,
    VCAP_IS1_ACT_DEI_VAL,
    VCAP_IS1_ACT_VLAN_POP_CNT_ENA,
    VCAP_IS1_ACT_VLAN_POP_CNT,
    VCAP_IS1_ACT_CUSTOM_ACE_TYPE_ENA,
    VCAP_IS1_ACT_HIT_STICKY,
}

// =================================================================
// VCAP ES0
// =================================================================
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_es0_key_field {
    VCAP_ES0_EGR_PORT,
    VCAP_ES0_IGR_PORT,
    VCAP_ES0_RSV,
    VCAP_ES0_L2_MC,
    VCAP_ES0_L2_BC,
    VCAP_ES0_VID,
    VCAP_ES0_DP,
    VCAP_ES0_PCP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_es0_action_field {
    VCAP_ES0_ACT_PUSH_OUTER_TAG,
    VCAP_ES0_ACT_PUSH_INNER_TAG,
    VCAP_ES0_ACT_TAG_A_TPID_SEL,
    VCAP_ES0_ACT_TAG_A_VID_SEL,
    VCAP_ES0_ACT_TAG_A_PCP_SEL,
    VCAP_ES0_ACT_TAG_A_DEI_SEL,
    VCAP_ES0_ACT_TAG_B_TPID_SEL,
    VCAP_ES0_ACT_TAG_B_VID_SEL,
    VCAP_ES0_ACT_TAG_B_PCP_SEL,
    VCAP_ES0_ACT_TAG_B_DEI_SEL,
    VCAP_ES0_ACT_VID_A_VAL,
    VCAP_ES0_ACT_PCP_A_VAL,
    VCAP_ES0_ACT_DEI_A_VAL,
    VCAP_ES0_ACT_VID_B_VAL,
    VCAP_ES0_ACT_PCP_B_VAL,
    VCAP_ES0_ACT_DEI_B_VAL,
    VCAP_ES0_ACT_RSV,
    VCAP_ES0_ACT_HIT_STICKY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_ipv4 {
    pub addr: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_vcap_bit {
    OCELOT_VCAP_BIT_ANY,
    OCELOT_VCAP_BIT_0,
    OCELOT_VCAP_BIT_1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_u8 {
    pub value: [u8; 1],
    pub mask: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_u16 {
    pub value: [u8; 2],
    pub mask: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_u24 {
    pub value: [u8; 3],
    pub mask: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_u32 {
    pub value: [u8; 4],
    pub mask: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_u40 {
    pub value: [u8; 5],
    pub mask: [u8; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_u48 {
    pub value: [u8; 6],
    pub mask: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_u64 {
    pub value: [u8; 8],
    pub mask: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_u128 {
    pub value: [u8; 16],
    pub mask: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_vid {
    pub value: u16,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_ipv4 {
    pub value: ocelot_ipv4,
    pub mask: ocelot_ipv4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_udp_tcp {
    pub value: u16,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_port {
    pub value: u8,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_vcap_key_type {
    OCELOT_VCAP_KEY_ANY,
    OCELOT_VCAP_KEY_ETYPE,
    OCELOT_VCAP_KEY_LLC,
    OCELOT_VCAP_KEY_SNAP,
    OCELOT_VCAP_KEY_ARP,
    OCELOT_VCAP_KEY_IPV4,
    OCELOT_VCAP_KEY_IPV6
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_key_vlan {
    pub /: *mut *mut ocelot_vcap_vid vid; / VLAN ID (12 bit),
    pub /: *mut *mut ocelot_vcap_u8 pcp; / PCP (3 bit),
    pub /: *mut *mut ocelot_vcap_bit dei; / DEI,
    pub /: *mut *mut ocelot_vcap_bit tagged; / Tagged/untagged frame,
    pub tpid: ocelot_vcap_bit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_key_etype {
    pub dmac: ocelot_vcap_u48,
    pub smac: ocelot_vcap_u48,
    pub etype: ocelot_vcap_u16,
    pub /: *mut *mut ocelot_vcap_u16 data; / MAC data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_key_llc {
    pub dmac: ocelot_vcap_u48,
    pub smac: ocelot_vcap_u48,
// LLC header: DSAP at byte 0, SSAP at byte 1, Control at byte 2
    pub llc: ocelot_vcap_u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_key_snap {
    pub dmac: ocelot_vcap_u48,
    pub smac: ocelot_vcap_u48,
// SNAP header: Organization Code at byte 0, Type at byte 3
    pub snap: ocelot_vcap_u40,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_key_arp {
    pub smac: ocelot_vcap_u48,
    pub /: *mut *mut ocelot_vcap_bit arp; / Opcode ARP/RARP,
    pub /: *mut *mut ocelot_vcap_bit req; / Opcode request/reply,
    pub /: *mut *mut ocelot_vcap_bit unknown; / Opcode unknown,
    pub /: *mut *mut ocelot_vcap_bit smac_match; / Sender MAC matches SMAC,
    pub /: *mut *mut ocelot_vcap_bit dmac_match; / Target MAC matches DMAC,
// < Protocol addr. length 4, hardware length 6
    pub length: ocelot_vcap_bit,
    pub /: *mut *mut ocelot_vcap_bit ip; / Protocol address type IP,
    pub /: *mut *mut ocelot_vcap_bit ethernet; / Hardware address type Ethernet,
    pub /: *mut *mut ocelot_vcap_ipv4 sip; / Sender IP address,
    pub /: *mut *mut ocelot_vcap_ipv4 dip; / Target IP address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_key_ipv4 {
    pub /: *mut *mut ocelot_vcap_bit ttl; / TTL zero,
    pub /: *mut *mut ocelot_vcap_bit fragment; / Fragment,
    pub /: *mut *mut ocelot_vcap_bit options; / Header options,
    pub ds: ocelot_vcap_u8,
    pub /: *mut *mut ocelot_vcap_u8 proto; / Protocol,
    pub /: *mut *mut ocelot_vcap_ipv4 sip; / Source IP address,
    pub /: *mut *mut ocelot_vcap_ipv4 dip; / Destination IP address,
    pub /: *mut *mut ocelot_vcap_u48 data; / Not UDP/TCP: IP data,
    pub /: *mut *mut ocelot_vcap_udp_tcp sport; / UDP/TCP: Source port,
    pub /: *mut *mut ocelot_vcap_udp_tcp dport; / UDP/TCP: Destination port,
    pub tcp_fin: ocelot_vcap_bit,
    pub tcp_syn: ocelot_vcap_bit,
    pub tcp_rst: ocelot_vcap_bit,
    pub tcp_psh: ocelot_vcap_bit,
    pub tcp_ack: ocelot_vcap_bit,
    pub tcp_urg: ocelot_vcap_bit,
    pub /: *mut *mut ocelot_vcap_bit sip_eq_dip; / SIP equals DIP,
    pub /: *mut *mut ocelot_vcap_bit sport_eq_dport; / SPORT equals DPORT,
    pub /: *mut *mut ocelot_vcap_bit seq_zero; / TCP sequence number is zero,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_key_ipv6 {
    pub /: *mut *mut ocelot_vcap_u8 proto; / IPv6 protocol,
    pub /: *mut *mut ocelot_vcap_u128 sip; / IPv6 source (byte 0-7 ignored),
    pub /: *mut *mut ocelot_vcap_u128 dip; / IPv6 destination (byte 0-7 ignored),
    pub /: *mut *mut ocelot_vcap_bit ttl; / TTL zero,
    pub ds: ocelot_vcap_u8,
    pub /: *mut *mut ocelot_vcap_u48 data; / Not UDP/TCP: IP data,
    pub sport: ocelot_vcap_udp_tcp,
    pub dport: ocelot_vcap_udp_tcp,
    pub tcp_fin: ocelot_vcap_bit,
    pub tcp_syn: ocelot_vcap_bit,
    pub tcp_rst: ocelot_vcap_bit,
    pub tcp_psh: ocelot_vcap_bit,
    pub tcp_ack: ocelot_vcap_bit,
    pub tcp_urg: ocelot_vcap_bit,
    pub /: *mut *mut ocelot_vcap_bit sip_eq_dip; / SIP equals DIP,
    pub /: *mut *mut ocelot_vcap_bit sport_eq_dport; / SPORT equals DPORT,
    pub /: *mut *mut ocelot_vcap_bit seq_zero; / TCP sequence number is zero,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_mask_mode {
    OCELOT_MASK_MODE_NONE,
    OCELOT_MASK_MODE_PERMIT_DENY,
    OCELOT_MASK_MODE_POLICY,
    OCELOT_MASK_MODE_REDIRECT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_es0_vid_sel {
    OCELOT_ES0_VID_PLUS_CLASSIFIED_VID = 0,
    OCELOT_ES0_VID = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_es0_pcp_sel {
    OCELOT_CLASSIFIED_PCP = 0,
    OCELOT_ES0_PCP = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_es0_tag {
    OCELOT_NO_ES0_TAG,
    OCELOT_ES0_TAG,
    OCELOT_FORCE_PORT_TAG,
    OCELOT_FORCE_UNTAG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_tag_tpid_sel {
    OCELOT_TAG_TPID_SEL_8021Q,
    OCELOT_TAG_TPID_SEL_8021AD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_action {
// VCAP ES0
    pub push_outer_tag: ocelot_es0_tag,
    pub push_inner_tag: ocelot_es0_tag,
    pub tag_a_tpid_sel: ocelot_tag_tpid_sel,
    pub tag_a_vid_sel: c_int,
    pub tag_a_pcp_sel: c_int,
    pub vid_a_val: u16,
    pub pcp_a_val: u8,
    pub dei_a_val: u8,
    pub tag_b_tpid_sel: ocelot_tag_tpid_sel,
    pub tag_b_vid_sel: c_int,
    pub tag_b_pcp_sel: c_int,
    pub vid_b_val: u16,
    pub pcp_b_val: u8,
    pub dei_b_val: u8,
}

// VCAP IS1
// VCAP IS2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_stats {
    pub bytes: u64,
    pub pkts: u64,
    pub used: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocelot_vcap_filter_type {
    OCELOT_VCAP_FILTER_DUMMY,
    OCELOT_VCAP_FILTER_PAG,
    OCELOT_VCAP_FILTER_OFFLOAD,
    OCELOT_PSFP_FILTER_OFFLOAD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_id {
    pub cookie: c_ulong,
    pub tc_offload: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_vcap_filter {
    pub list: list_head,
    pub type: ocelot_vcap_filter_type,
    pub block_id: c_int,
    pub goto_target: c_int,
    pub lookup: c_int,
    pub pag: u8,
    pub prio: u16,
    pub id: ocelot_vcap_id,
    pub action: ocelot_vcap_action,
    pub stats: ocelot_vcap_stats,
// For VCAP IS1 and IS2
    pub take_ts: bool,
    pub is_trap: bool,
    pub ingress_port_mask: c_ulong,
// For VCAP ES0
    pub ingress_port: ocelot_vcap_port,
// For VCAP IS2 mirrors and ES0
    pub egress_port: ocelot_vcap_port,
    pub dmac_mc: ocelot_vcap_bit,
    pub dmac_bc: ocelot_vcap_bit,
    pub vlan: ocelot_vcap_key_vlan,
    pub key_type: ocelot_vcap_key_type,
// OCELOT_VCAP_KEY_ANY: No specific fields
    pub etype: ocelot_vcap_key_etype,
    pub llc: ocelot_vcap_key_llc,
    pub snap: ocelot_vcap_key_snap,
    pub arp: ocelot_vcap_key_arp,
    pub ipv4: ocelot_vcap_key_ipv4,
    pub ipv6: ocelot_vcap_key_ipv6,
    pub key: },
}
