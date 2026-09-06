//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_fdir.h
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
// Copyright (C) 2018-2020, Intel Corporation.
pub const ICE_FDIR_TUN_PKT_OFF: c_int = 50;

// macros for offsets into packets for flow director programming
pub const ICE_ETH_TYPE_F_OFFSET: c_int = 12;
pub const ICE_ETH_VLAN_TCI_OFFSET: c_int = 14;
pub const ICE_ETH_TYPE_VLAN_OFFSET: c_int = 16;
pub const ICE_IPV4_SRC_ADDR_OFFSET: c_int = 26;
pub const ICE_IPV4_DST_ADDR_OFFSET: c_int = 30;
pub const ICE_IPV4_TCP_SRC_PORT_OFFSET: c_int = 34;
pub const ICE_IPV4_TCP_DST_PORT_OFFSET: c_int = 36;
pub const ICE_IPV4_UDP_SRC_PORT_OFFSET: c_int = 34;
pub const ICE_IPV4_UDP_DST_PORT_OFFSET: c_int = 36;
pub const ICE_IPV4_SCTP_SRC_PORT_OFFSET: c_int = 34;
pub const ICE_IPV4_SCTP_DST_PORT_OFFSET: c_int = 36;
pub const ICE_IPV4_PROTO_OFFSET: c_int = 23;
pub const ICE_IPV6_SRC_ADDR_OFFSET: c_int = 22;
pub const ICE_IPV6_DST_ADDR_OFFSET: c_int = 38;
pub const ICE_IPV6_TCP_SRC_PORT_OFFSET: c_int = 54;
pub const ICE_IPV6_TCP_DST_PORT_OFFSET: c_int = 56;
pub const ICE_IPV6_UDP_SRC_PORT_OFFSET: c_int = 54;
pub const ICE_IPV6_UDP_DST_PORT_OFFSET: c_int = 56;
pub const ICE_IPV6_SCTP_SRC_PORT_OFFSET: c_int = 54;
pub const ICE_IPV6_SCTP_DST_PORT_OFFSET: c_int = 56;
pub const ICE_MAC_ETHTYPE_OFFSET: c_int = 12;
pub const ICE_IPV4_TOS_OFFSET: c_int = 15;
pub const ICE_IPV4_TTL_OFFSET: c_int = 22;
pub const ICE_IPV6_TC_OFFSET: c_int = 14;
pub const ICE_IPV6_HLIM_OFFSET: c_int = 21;
pub const ICE_IPV6_PROTO_OFFSET: c_int = 20;
pub const ICE_IPV4_GTPU_TEID_OFFSET: c_int = 46;
pub const ICE_IPV4_GTPU_QFI_OFFSET: c_int = 56;
pub const ICE_IPV4_L2TPV3_SESS_ID_OFFSET: c_int = 34;
pub const ICE_IPV6_L2TPV3_SESS_ID_OFFSET: c_int = 54;
pub const ICE_IPV4_ESP_SPI_OFFSET: c_int = 34;
pub const ICE_IPV6_ESP_SPI_OFFSET: c_int = 54;
pub const ICE_IPV4_AH_SPI_OFFSET: c_int = 38;
pub const ICE_IPV6_AH_SPI_OFFSET: c_int = 58;
pub const ICE_IPV4_NAT_T_ESP_SPI_OFFSET: c_int = 42;
pub const ICE_IPV6_NAT_T_ESP_SPI_OFFSET: c_int = 62;
pub const ICE_FDIR_MAX_FLTRS: c_int = 16384;
// IP v4 has 2 flag bits that enable fragment processing: DF and MF. DF
// requests that the packet not be fragmented. MF indicates that a packet has
// been fragmented.
//
pub const ICE_FDIR_IPV4_PKT_FLAG_MF: c_uint = 0x20;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_fltr_prgm_desc_dest {
    ICE_FLTR_PRGM_DESC_DEST_DROP_PKT,
    ICE_FLTR_PRGM_DESC_DEST_DIRECT_PKT_QINDEX,
    ICE_FLTR_PRGM_DESC_DEST_DIRECT_PKT_QGROUP,
    ICE_FLTR_PRGM_DESC_DEST_DIRECT_PKT_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_fltr_prgm_desc_fd_status {
    ICE_FLTR_PRGM_DESC_FD_STATUS_NONE,
    ICE_FLTR_PRGM_DESC_FD_STATUS_FD_ID,
}

// Flow Director (FD) Filter Programming descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fd_fltr_desc_ctx {
    pub fdid: u32,
    pub qindex: u16,
    pub cnt_index: u16,
    pub fd_vsi: u16,
    pub flex_val: u16,
    pub comp_q: u8,
    pub comp_report: u8,
    pub fd_space: u8,
    pub cnt_ena: u8,
    pub evict_ena: u8,
    pub toq: u8,
    pub toq_prio: u8,
    pub dpu_recipe: u8,
    pub drop: u8,
    pub flex_prio: u8,
    pub flex_mdid: u8,
    pub dtype: u8,
    pub pcmd: u8,
    pub desc_prof_prio: u8,
    pub desc_prof: u8,
    pub swap: u8,
    pub fdid_prio: u8,
    pub fdid_mdid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_rx_flow_userdef {
    pub flex_word: u16,
    pub flex_offset: u16,
    pub flex_fltr: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fdir_v4 {
    pub dst_ip: __be32,
    pub src_ip: __be32,
    pub dst_port: __be16,
    pub src_port: __be16,
    pub l4_header: __be32,
    pub /: *mut *mut __be32 sec_parm_idx; / security parameter index,
    pub tos: u8,
    pub ip_ver: u8,
    pub proto: u8,
    pub ttl: u8,
}

pub const ICE_IPV6_ADDR_LEN_AS_U32: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fdir_v6 {
    pub dst_ip: [__be32; ICE_IPV6_ADDR_LEN_AS_U32],
    pub src_ip: [__be32; ICE_IPV6_ADDR_LEN_AS_U32],
    pub dst_port: __be16,
    pub src_port: __be16,
    pub /: *mut *mut __be32 l4_header; / next header,
    pub /: *mut *mut __be32 sec_parm_idx; / security parameter index,
    pub tc: u8,
    pub proto: u8,
    pub hlim: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fdir_udp_gtp {
    pub flags: u8,
    pub msg_type: u8,
    pub rsrvd_len: __be16,
    pub teid: __be32,
    pub rsrvd_seq_nbr: __be16,
    pub rsrvd_n_pdu_nbr: u8,
    pub rsrvd_next_ext_type: u8,
    pub rsvrd_ext_len: u8,
    pub rsvrd: u32,
    pub next_ext: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fdir_l2tpv3 {
    pub session_id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fdir_extra {
    pub /: *mut *mut u8 dst_mac[ETH_ALEN]; / dest MAC address,
    pub /: *mut *mut u8 src_mac[ETH_ALEN]; / src MAC address,
    pub /: *mut *mut __be16 ether_type; / for NON_IP_L2,
    pub /: *mut *mut u32 usr_def[2]; / user data,
    pub /: *mut *mut __be16 vlan_type; / VLAN ethertype,
    pub /: *mut *mut __be16 vlan_tag; / VLAN tag info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fdir_fltr {
    pub fltr_node: list_head,
    pub flow_type: ice_fltr_ptype,
    pub eth_mask: ethhdr eth,,
    pub v4: ice_fdir_v4,
    pub v6: ice_fdir_v6,
    pub mask: } ip,,
    pub gtpu_data: ice_fdir_udp_gtp,
    pub gtpu_mask: ice_fdir_udp_gtp,
    pub l2tpv3_data: ice_fdir_l2tpv3,
    pub l2tpv3_mask: ice_fdir_l2tpv3,
    pub ext_data: ice_fdir_extra,
    pub ext_mask: ice_fdir_extra,
// flex byte filter data
    pub flex_word: __be16,
// queue region size (=2^q_region)
    pub q_region: u8,
    pub flex_offset: u16,
    pub flex_fltr: u16,
// filter control
    pub q_index: i16,
    pub orig_q_index: u16,
    pub dest_vsi: u16,
    pub dest_ctl: u8,
    pub cnt_ena: u8,
    pub fltr_status: u8,
    pub cnt_index: u16,
    pub fltr_id: u32,
    pub fdid_prio: u8,
    pub comp_report: u8,
}

// Dummy packet filter definition structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fdir_base_pkt {
    pub flow: ice_fltr_ptype,
    pub pkt_len: u16,
    pub pkt: *const u8,
    pub tun_pkt_len: u16,
    pub tun_pkt: *const u8,
}

extern "C" {
    pub fn ice_alloc_fd_res_cntr(hw: *mut ice_hw, cntr_id: *mut u16) -> c_int;
}
extern "C" {
    pub fn ice_free_fd_res_cntr(hw: *mut ice_hw, cntr_id: u16) -> c_int;
}
extern "C" {
    pub fn ice_alloc_fd_guar_item(hw: *mut ice_hw, cntr_id: *mut u16, num_fltr: u16) -> c_int;
}
extern "C" {
    pub fn ice_alloc_fd_shrd_item(hw: *mut ice_hw, cntr_id: *mut u16, num_fltr: u16) -> c_int;
}
extern "C" {
    pub fn ice_get_fdir_cnt_all(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_fdir_num_avail_fltr(hw: *mut ice_hw, vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_fdir_is_dup_fltr(hw: *mut ice_hw, input: *mut ice_fdir_fltr) -> bool;
}
extern "C" {
    pub fn ice_fdir_has_frag(flow: ice_fltr_ptype) -> bool;
}
extern "C" {
    pub fn ice_fdir_list_add_fltr(hw: *mut ice_hw, input: *mut ice_fdir_fltr);
}
