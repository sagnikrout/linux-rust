//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_tc_lib.h
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
// Copyright (C) 2019-2021, Intel Corporation.

pub const ICE_TC_FLOWER_MASK_32: c_uint = 0xFFFFFFFF;
pub const ICE_IPV6_HDR_TC_MASK: c_uint = 0xFF00000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_indr_block_priv {
    pub netdev: *mut net_device,
    pub np: *mut ice_netdev_priv,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_flower_action {
// forward action specific params
    pub /: *mut *mut u32 tc_class; / forward to hw_tc,
    pub rsvd: u32,
    pub tc: },
    pub /: *mut *mut u16 queue; / forward to queue,
// To add filter in HW, absolute queue number in global
// space of queues (between 0...N) is needed
//
    pub hw_queue: u16,
    pub q: },
    pub fwd: },
    pub fltr_act: ice_sw_fwd_act_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_vlan_hdr {
    pub /: *mut *mut __be16 vlan_id; / Only last 12 bits valid,
    pub /: *mut *mut __be16 vlan_prio; / Only last 3 bits valid (valid values: 0..7),
    pub vlan_tpid: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_pppoe_hdr {
    pub session_id: __be16,
    pub ppp_proto: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_l2_hdr {
    pub dst_mac: [u8; ETH_ALEN],
    pub src_mac: [u8; ETH_ALEN],
    pub /: *mut *mut __be16 n_proto; / Ethernet Protocol,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_l3_hdr {
    pub /: *mut *mut u8 ip_proto; / IPPROTO value,
    pub dst_ip: in_addr,
    pub src_ip: in_addr,
    pub v4: },
    pub dst_ip6: in6_addr,
    pub src_ip6: in6_addr,
    pub v6: },
    pub ip: },

    pub tos: u8,
    pub ttl: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_l2tpv3_hdr {
    pub session_id: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_l4_hdr {
    pub dst_port: __be16,
    pub src_port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_flower_lyr_2_4_hdrs {
// L2 layer fields with their mask
    pub l2_key: ice_tc_l2_hdr,
    pub l2_mask: ice_tc_l2_hdr,
    pub vlan_hdr: ice_tc_vlan_hdr,
    pub cvlan_hdr: ice_tc_vlan_hdr,
    pub pppoe_hdr: ice_tc_pppoe_hdr,
    pub l2tpv3_hdr: ice_tc_l2tpv3_hdr,
// L3 (IPv4[6]) layer fields with their mask
    pub l3_key: ice_tc_l3_hdr,
    pub l3_mask: ice_tc_l3_hdr,
// L4 layer fields with their mask
    pub l4_key: ice_tc_l4_hdr,
    pub l4_mask: ice_tc_l4_hdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_eswitch_fltr_direction {
    ICE_ESWITCH_FLTR_INGRESS,
    ICE_ESWITCH_FLTR_EGRESS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_flower_fltr {
    pub tc_flower_node: hlist_node,
// cookie becomes filter_rule_id if rule is added successfully
    pub cookie: c_ulong,
// add_adv_rule returns information like recipe ID, rule_id. Store
// those values since they are needed to remove advanced rule
//
    pub rid: u16,
    pub rule_id: u16,
// VSI handle of the destination VSI (it could be main PF VSI, CHNL_VSI,
// VF VSI)
//
    pub dest_vsi_handle: u16,
// ptr to destination VSI
    pub dest_vsi: *mut ice_vsi,
// direction of fltr for eswitch use case
    pub direction: ice_eswitch_fltr_direction,
// Parsed TC flower configuration params
    pub outer_headers: ice_tc_flower_lyr_2_4_hdrs,
    pub inner_headers: ice_tc_flower_lyr_2_4_hdrs,
    pub src_vsi: *mut ice_vsi,
    pub tenant_id: __be32,
    pub gtp_pdu_info_keys: gtp_pdu_session_info,
    pub gtp_pdu_info_masks: gtp_pdu_session_info,
    pub pfcp_meta_keys: pfcp_metadata,
    pub pfcp_meta_masks: pfcp_metadata,
    pub flags: u32,
    pub tunnel_type: u8,
    pub action: ice_tc_flower_action,
// cache ptr which is used wherever needed to communicate netlink
// messages
//
    pub extack: *mut netlink_ext_ack,
}

//
// ice_is_chnl_fltr - is this a valid channel filter
// @f: Pointer to tc-flower filter
//
// Criteria to determine of given filter is valid channel filter
// or not is based on its destination.
// For forward to VSI action, if destination is valid hw_tc (aka tc_class)
// and in supported range of TCs for ADQ, then return true.
// For forward to queue, as long as dest_vsi is valid and it is of type
// VSI_CHNL (PF ADQ VSI is of type VSI_CHNL), return true.
// NOTE: For forward to queue, correct dest_vsi is still set in tc_fltr based
// on destination queue specified.
//
// ice_chnl_dmac_fltr_cnt - DMAC based CHNL filter count
// @pf: Pointer to PF
//
extern "C" {
    pub fn ice_replay_tc_fltrs(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_is_tunnel_supported(dev: *mut net_device) -> bool;
}
extern "C" {
    pub fn ice_drop_vf_tx_lldp(vsi: *mut ice_vsi, init: bool) -> c_int;
}
extern "C" {
    pub fn ice_pass_vf_tx_lldp(vsi: *mut ice_vsi, deinit: bool) -> c_int;
}
