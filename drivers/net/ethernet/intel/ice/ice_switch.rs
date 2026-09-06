//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_switch.h
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
// Copyright (c) 2018, Intel Corporation.

pub const ICE_SW_CFG_MAX_BUF_LEN: c_int = 2048;
pub const ICE_DFLT_VSI_INVAL: c_uint = 0xff;

pub const ICE_VSI_INVAL_ID: c_uint = 0xffff;
pub const ICE_INVAL_Q_HANDLE: c_uint = 0xFFFF;
// Switch Profile IDs for Profile related switch rules
pub const ICE_PROFID_IPV4_GTPC_TEID: c_int = 41;
pub const ICE_PROFID_IPV4_GTPC_NO_TEID: c_int = 42;
pub const ICE_PROFID_IPV4_GTPU_TEID: c_int = 43;
pub const ICE_PROFID_IPV6_GTPC_TEID: c_int = 44;
pub const ICE_PROFID_IPV6_GTPC_NO_TEID: c_int = 45;
pub const ICE_PROFID_IPV6_GTPU_TEID: c_int = 46;
pub const ICE_PROFID_IPV6_GTPU_IPV6_TCP_INNER: c_int = 70;
pub const ICE_PROFID_IPV4_PFCP_NODE: c_int = 79;
pub const ICE_PROFID_IPV6_PFCP_SESSION: c_int = 82;

pub const DUMMY_ETH_HDR_LEN: c_int = 16;
// VSI context structure for add/get/update/free operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vsi_ctx {
    pub vsi_num: u16,
    pub vsis_allocd: u16,
    pub vsis_unallocated: u16,
    pub flags: u16,
    pub info: ice_aqc_vsi_props,
    pub sched: ice_sched_vsi_info,
    pub alloc_from_pool: u8,
    pub vf_num: u8,
    pub num_lan_q_entries: [u16; ICE_MAX_TRAFFIC_CLASS],
    pub lan_q_ctx: [*mut ice_q_ctx; ICE_MAX_TRAFFIC_CLASS],
    pub num_rdma_q_entries: [u16; ICE_MAX_TRAFFIC_CLASS],
    pub rdma_q_ctx: [*mut ice_q_ctx; ICE_MAX_TRAFFIC_CLASS],
}

// Switch recipe ID enum values are specific to hardware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_sw_lkup_type {
    ICE_SW_LKUP_ETHERTYPE = 0,
    ICE_SW_LKUP_MAC = 1,
    ICE_SW_LKUP_MAC_VLAN = 2,
    ICE_SW_LKUP_PROMISC = 3,
    ICE_SW_LKUP_VLAN = 4,
    ICE_SW_LKUP_DFLT = 5,
    ICE_SW_LKUP_ETHERTYPE_MAC = 8,
    ICE_SW_LKUP_PROMISC_VLAN = 9,
    ICE_SW_LKUP_LAST
}

// type of filter src ID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_src_id {
    ICE_SRC_ID_UNKNOWN = 0,
    ICE_SRC_ID_VSI,
    ICE_SRC_ID_QUEUE,
    ICE_SRC_ID_LPORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fltr_info {
// Look up information: how to look up packet
    pub lkup_type: ice_sw_lkup_type,
// Forward action: filter action to do after lookup
    pub fltr_act: ice_sw_fwd_act_type,
// rule ID returned by firmware once filter rule is created
    pub fltr_rule_id: u16,
    pub flag: u16,
// Source VSI for LOOKUP_TX or source port for LOOKUP_RX
    pub src: u16,
    pub src_id: ice_src_id,
    pub mac_addr: [u8; ETH_ALEN],
    pub mac: },
    pub mac_addr: [u8; ETH_ALEN],
    pub vlan_id: u16,
    pub mac_vlan: },
    pub vlan_id: u16,
    pub tpid: u16,
    pub tpid_valid: u8,
    pub vlan: },
// Set lkup_type as ICE_SW_LKUP_ETHERTYPE
// if just using ethertype as filter. Set lkup_type as
// ICE_SW_LKUP_ETHERTYPE_MAC if MAC also needs to be
// passed in as filter.
//
    pub ethertype: u16,
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / optional,
    pub ethertype_mac: },
    pub using: *mut *mut } l_data; / Make sure to zero out the memory of l_data before,
// it or only set the data associated with lookup match
// rest everything should be zero
//
// Depending on filter action
// queue ID in case of ICE_FWD_TO_Q and starting
// queue ID in case of ICE_FWD_TO_QGRP.
//
    pub q_id:11: u16,
    pub hw_vsi_id:10: u16,
    pub vsi_list_id:10: u16,
    pub fwd_id: },
// Sw VSI handle
    pub vsi_handle: u16,
// Set to num_queues if action is ICE_FWD_TO_QGRP. This field
// determines the range of queues the packet needs to be forwarded to.
// Note that qgrp_size must be set to a power of 2.
//
    pub qgrp_size: u8,
// Rule creations populate these indicators basing on the switch type
    pub /: *mut *mut u8 lb_en; / Indicate if packet can be looped back,
    pub /: *mut *mut u8 lan_en; / Indicate if packet can be forwarded to the uplink,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_update_recipe_lkup_idx_params {
    pub rid: u16,
    pub fv_idx: u16,
    pub ignore_valid: bool,
    pub mask: u16,
    pub mask_valid: bool,
    pub lkup_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_adv_lkup_elem {
    pub type: ice_protocol_type,
    pub /: *mut *mut ice_prot_hdr h_u; / Header values,
// Used to iterate over the headers
    pub sizeof(u16)]: u16 h_raw[sizeof(union ice_prot_hdr) /,
}

// Used to iterate over header mask
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sw_act_ctrl {
// Source VSI for LOOKUP_TX or source port for LOOKUP_RX
    pub src: u16,
    pub flag: u16,
    pub fltr_act: ice_sw_fwd_act_type,
// Depending on filter action
// This is a queue ID in case of ICE_FWD_TO_Q and starting
// queue ID in case of ICE_FWD_TO_QGRP.
//
    pub q_id:11: u16,
    pub vsi_id:10: u16,
    pub hw_vsi_id:10: u16,
    pub vsi_list_id:10: u16,
    pub fwd_id: },
// software VSI handle
    pub vsi_handle: u16,
    pub qgrp_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_rule_query_data {
// Recipe ID for which the requested rule was added
    pub rid: u16,
// Rule ID that was added or is supposed to be removed
    pub rule_id: u16,
// vsi_handle for which Rule was added or is supposed to be removed
    pub vsi_handle: u16,
}

// This structure allows to pass info about lb_en and lan_en
// flags to ice_add_adv_rule. Values in act would be used
// only if act_valid was set to true, otherwise default
// values would be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_adv_rule_flags_info {
    pub act: u32,
    pub /: *mut *mut u8 act_valid; / indicate if flags in act are valid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_adv_rule_info {
// Store metadata values in rule info
    pub tun_type: ice_sw_tunnel_type,
    pub vlan_type: u16,
    pub fltr_rule_id: u16,
    pub priority: u32,
    pub need_pass_l2:1: u16,
    pub allow_pass_l2:1: u16,
    pub src_vsi: u16,
    pub sw_act: ice_sw_act_ctrl,
    pub flags_info: ice_adv_rule_flags_info,
}

// A collection of one or more four word recipe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sw_recipe {
// For a chained recipe the root recipe is what should be used for
// programming rules
//
    pub root_rid: u8,
    pub recp_created: u8,
// Number of extraction words
    pub n_ext_words: u8,
// Protocol ID and Offset pair (extraction word) to describe the
// recipe
//
    pub ext_words: [ice_fv_word; ICE_MAX_CHAIN_WORDS],
    pub word_masks: [u16; ICE_MAX_CHAIN_WORDS],
    pub fv_idx: [u8; ICE_MAX_CHAIN_WORDS],
    pub fv_mask: [u16; ICE_MAX_CHAIN_WORDS],
// Bit map specifying the IDs associated with this group of recipe
    pub ICE_MAX_NUM_RECIPES): DECLARE_BITMAP(r_bitmap,,
    pub tun_type: ice_sw_tunnel_type,
// List of type ice_fltr_mgmt_list_entry or adv_rule
    pub adv_rule: u8,
    pub filt_rules: list_head,
    pub filt_replay_rules: list_head,
    pub /: *mut *mut mutex filt_rule_lock; / protect filter rule structure,
// Profiles this recipe should be associated with
    pub fv_list: list_head,
// Profiles this recipe is associated with
    pub prof_ids: *mut u8 num_profs,,
// Bit map for possible result indexes
    pub ICE_MAX_FV_WORDS): DECLARE_BITMAP(res_idxs,,
// This allows user to specify the recipe priority.
// For now, this becomes 'fwd_priority' when recipe
// is created, usually recipes can have 'fwd' and 'join'
// priority.
//
    pub priority: u8,
    pub need_pass_l2:1: u8,
    pub allow_pass_l2:1: u8,
// This struct saves the fv_words for a given lookup
    pub lkup_exts: ice_prot_lkup_ext,
}

// Bookkeeping structure to hold bitmap of VSIs corresponding to VSI list ID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vsi_list_map_info {
    pub list_entry: list_head,
    pub ICE_MAX_VSI): DECLARE_BITMAP(vsi_map,,
    pub vsi_list_id: u16,
// counter to track how many rules are reusing this VSI list
    pub ref_cnt: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fltr_list_entry {
    pub list_entry: list_head,
    pub status: c_int,
    pub fltr_info: ice_fltr_info,
}

// This defines an entry in the list that maintains MAC or VLAN membership
// to HW list mapping, since multiple VSIs can subscribe to the same MAC or
// VLAN. As an optimization the VSI list should be created only when a
// second VSI becomes a subscriber to the same MAC address. VSI lists are always
// used for VLAN membership.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fltr_mgmt_list_entry {
// back pointer to VSI list ID to VSI list mapping
    pub vsi_list_info: *mut ice_vsi_list_map_info,
    pub vsi_count: u16,
pub const ICE_INVAL_LG_ACT_INDEX: c_uint = 0xffff;
    pub lg_act_idx: u16,
pub const ICE_INVAL_SW_MARKER_ID: c_uint = 0xffff;
    pub sw_marker_id: u16,
    pub list_entry: list_head,
    pub fltr_info: ice_fltr_info,
pub const ICE_INVAL_COUNTER_ID: c_uint = 0xff;
    pub counter_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_adv_fltr_mgmt_list_entry {
    pub list_entry: list_head,
    pub lkups: *mut ice_adv_lkup_elem,
    pub rule_info: ice_adv_rule_info,
    pub lkups_cnt: u16,
    pub vsi_list_info: *mut ice_vsi_list_map_info,
    pub vsi_count: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_promisc_flags {
    ICE_PROMISC_UCAST_RX = 0x1,
    ICE_PROMISC_UCAST_TX = 0x2,
    ICE_PROMISC_MCAST_RX = 0x4,
    ICE_PROMISC_MCAST_TX = 0x8,
    ICE_PROMISC_BCAST_RX = 0x10,
    ICE_PROMISC_BCAST_TX = 0x20,
    ICE_PROMISC_VLAN_RX = 0x40,
    ICE_PROMISC_VLAN_TX = 0x80,
}

// VSI related commands
extern "C" {
    pub fn ice_is_vsi_valid(hw: *mut ice_hw, vsi_handle: u16) -> bool;
}
extern "C" {
    pub fn ice_clear_all_vsi_ctx(hw: *mut ice_hw);
}
// Switch config
extern "C" {
    pub fn ice_get_initial_sw_cfg(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_share_res(hw: *mut ice_hw, type: u16, shared: u8, res_id: u16) -> c_int;
}
// Switch/bridge related commands
extern "C" {
    pub fn ice_rule_add_tunnel_metadata(lkup: *mut ice_adv_lkup_elem);
}
extern "C" {
    pub fn ice_rule_add_direction_metadata(lkup: *mut ice_adv_lkup_elem);
}
extern "C" {
    pub fn ice_rule_add_vlan_metadata(lkup: *mut ice_adv_lkup_elem);
}
extern "C" {
    pub fn ice_rule_add_src_vsi_metadata(lkup: *mut ice_adv_lkup_elem);
}
extern "C" {
    pub fn ice_update_sw_rule_bridge_mode(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_add_vlan(hw: *mut ice_hw, m_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ice_remove_vlan(hw: *mut ice_hw, v_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ice_add_mac(hw: *mut ice_hw, m_lst: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ice_remove_mac(hw: *mut ice_hw, m_lst: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ice_vlan_fltr_exist(hw: *mut ice_hw, vlan_id: u16, vsi_handle: u16) -> bool;
}
extern "C" {
    pub fn ice_add_eth_mac(hw: *mut ice_hw, em_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ice_remove_eth_mac(hw: *mut ice_hw, em_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ice_cfg_rdma_fltr(hw: *mut ice_hw, vsi_handle: u16, enable: bool) -> c_int;
}
extern "C" {
    pub fn ice_remove_vsi_fltr(hw: *mut ice_hw, vsi_handle: u16);
}
// Promisc/defport setup for VSIs
extern "C" {
    pub fn ice_init_def_sw_recp(hw: *mut ice_hw) -> c_int;
}
extern "C" {
    pub fn ice_get_hw_vsi_num(hw: *mut ice_hw, vsi_handle: u16) -> u16;
}
extern "C" {
    pub fn ice_replay_vsi_all_fltr(hw: *mut ice_hw, vsi_handle: u16) -> c_int;
}
extern "C" {
    pub fn ice_rm_all_sw_replay_rule_info(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_fill_eth_hdr(eth_hdr: *mut u8);
}
extern "C" {
    pub fn ice_change_proto_id_to_dvm();
}
extern "C" {
    pub fn ice_alloc_recipe(hw: *mut ice_hw, rid: *mut u16) -> c_int;
}
extern "C" {
    pub fn ice_init_chk_recipe_reuse_support(hw: *mut ice_hw);
}
