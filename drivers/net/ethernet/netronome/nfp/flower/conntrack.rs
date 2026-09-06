//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/flower/conntrack.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2021 Corigine, Inc.
pub const __NFP_FLOWER_CONNTRACK_H__: c_int = 1;

pub const NFP_FL_CT_NO_TUN: c_uint = 0xff;

// _out = false;					\
// _out = true;			\
//
// struct nfp_fl_ct_zone_entry - Zone entry containing conntrack flow information
// @zone:	The zone number, used as lookup key in hashtable
// @hash_node:	Used by the hashtable
// @priv:	Pointer to nfp_flower_priv data
// @nft:	Pointer to nf_flowtable for this zone
//
// @pre_ct_list:	The pre_ct_list of nfp_fl_ct_flow_entry entries
// @pre_ct_count:	Keep count of the number of pre_ct entries
//
// @post_ct_list:	The post_ct_list of nfp_fl_ct_flow_entry entries
// @post_ct_count:	Keep count of the number of post_ct entries
//
// @tc_merge_tb:	The table of merged tc flows
// @tc_merge_count:	Keep count of the number of merged tc entries
//
// @nft_flows_list:	The list of nft relatednfp_fl_ct_flow_entry entries
// @nft_flows_count:	Keep count of the number of nft_flow entries
//
// @nft_merge_tb:	The table of merged tc+nft flows
// @nft_merge_count:	Keep count of the number of merged tc+nft entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_ct_zone_entry {
    pub zone: u16,
    pub hash_node: rhash_head,
    pub priv: *mut nfp_flower_priv,
    pub nft: *mut nf_flowtable,
    pub pre_ct_list: list_head,
    pub pre_ct_count: c_uint,
    pub post_ct_list: list_head,
    pub post_ct_count: c_uint,
    pub tc_merge_tb: rhashtable,
    pub tc_merge_count: c_uint,
    pub nft_flows_list: list_head,
    pub nft_flows_count: c_uint,
    pub nft_merge_tb: rhashtable,
    pub nft_merge_count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ct_entry_type {
    CT_TYPE_PRE_CT,
    CT_TYPE_NFT,
    CT_TYPE_POST_CT,
    _CT_TYPE_MAX,
}

pub const NFP_MAX_RECIRC_CT_ZONES: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_nfp_layer_name {
    FLOW_PAY_META_TCI =    0,
    FLOW_PAY_INPORT,
    FLOW_PAY_EXT_META,
    FLOW_PAY_MAC_MPLS,
    FLOW_PAY_L4,
    FLOW_PAY_IPV4,
    FLOW_PAY_IPV6,
    FLOW_PAY_CT,
    FLOW_PAY_GRE,
    FLOW_PAY_QINQ,
    FLOW_PAY_UDP_TUN,
    FLOW_PAY_GENEVE_OPT,

    _FLOW_PAY_LAYERS_MAX
}

// NFP flow entry flags.

//
// struct nfp_fl_ct_flow_entry - Flow entry containing conntrack flow information
// @cookie:	Flow cookie, same as original TC flow, used as key
// @list_node:	Used by the list
// @chain_index:	Chain index of the original flow
// @goto_chain_index:	goto chain index of the flow
// @netdev:	netdev structure.
// @zt:		Reference to the zone table this belongs to
// @children:	List of tc_merge flows this flow forms part of
// @rule:	Reference to the original TC flow rule
// @stats:	Used to cache stats for updating
// @prev_m_entries:	Array of all previous nft_tc_merge entries
// @num_prev_m_entries:	The number of all previous nft_tc_merge entries
// @tun_offset: Used to indicate tunnel action offset in action list
// @flags:	Used to indicate flow flag like NAT which used by merge.
// @type:	Type of ct-entry from enum ct_entry_type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_ct_flow_entry {
    pub cookie: c_ulong,
    pub list_node: list_head,
    pub chain_index: u32,
    pub goto_chain_index: u32,
    pub netdev: *mut net_device,
    pub zt: *mut nfp_fl_ct_zone_entry,
    pub children: list_head,
    pub rule: *mut flow_rule,
    pub stats: flow_stats,
    pub 1]: *mut *mut nfp_fl_nft_tc_merge prev_m_entries[NFP_MAX_RECIRC_CT_ZONES -,
    pub num_prev_m_entries: u8,
    pub tun: u8 tun_offset; // Set to NFP_FL_CT_NO_TUN if no,
    pub flags: u8,
    pub type: u8,
}

//
// struct nfp_fl_ct_tc_merge - Merge of two flows from tc
// @cookie:		Flow cookie, combination of pre and post ct cookies
// @hash_node:		Used by the hashtable
// @pre_ct_list:	This entry is part of a pre_ct_list
// @post_ct_list:	This entry is part of a post_ct_list
// @zt:			Reference to the zone table this belongs to
// @pre_ct_parent:	The pre_ct_parent
// @post_ct_parent:	The post_ct_parent
// @children:		List of nft merged entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_ct_tc_merge {
    pub cookie: [c_ulong; 2],
    pub hash_node: rhash_head,
    pub pre_ct_list: list_head,
    pub post_ct_list: list_head,
    pub zt: *mut nfp_fl_ct_zone_entry,
    pub pre_ct_parent: *mut nfp_fl_ct_flow_entry,
    pub post_ct_parent: *mut nfp_fl_ct_flow_entry,
    pub children: list_head,
}

//
// struct nfp_fl_nft_tc_merge - Merge of tc_merge flows with nft flow
// @netdev:		Ingress netdev name
// @cookie:		Flow cookie, combination of tc_merge and nft cookies
// @hash_node:		Used by the hashtable
// @zt:	Reference to the zone table this belongs to
// @nft_flow_list:	This entry is part of a nft_flows_list
// @tc_merge_list:	This entry is part of a ct_merge_list
// @tc_m_parent:	The tc_merge parent
// @nft_parent:	The nft_entry parent
// @tc_flower_cookie:	The cookie of the flow offloaded to the nfp
// @flow_pay:	Reference to the offloaded flow struct
// @next_pre_ct_entry:	Reference to the next ct zone pre ct entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_nft_tc_merge {
    pub netdev: *mut net_device,
    pub cookie: [c_ulong; 3],
    pub hash_node: rhash_head,
    pub zt: *mut nfp_fl_ct_zone_entry,
    pub nft_flow_list: list_head,
    pub tc_merge_list: list_head,
    pub tc_m_parent: *mut nfp_fl_ct_tc_merge,
    pub nft_parent: *mut nfp_fl_ct_flow_entry,
    pub tc_flower_cookie: c_ulong,
    pub flow_pay: *mut nfp_fl_payload,
    pub next_pre_ct_entry: *mut nfp_fl_ct_flow_entry,
}

//
// struct nfp_fl_ct_map_entry - Map between flow cookie and specific ct_flow
// @cookie:	Flow cookie, same as original TC flow, used as key
// @hash_node:	Used by the hashtable
// @ct_entry:	Pointer to corresponding ct_entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_fl_ct_map_entry {
    pub cookie: c_ulong,
    pub hash_node: rhash_head,
    pub ct_entry: *mut nfp_fl_ct_flow_entry,
}

extern "C" {
    pub fn is_pre_ct_flow(flow: *mut flow_cls_offload) -> bool;
}
extern "C" {
    pub fn is_post_ct_flow(flow: *mut flow_cls_offload) -> bool;
}
//
// nfp_fl_ct_handle_pre_ct() - Handles -trk conntrack rules
// @priv:	Pointer to app priv
// @netdev:	netdev structure.
// @flow:	TC flower classifier offload structure.
// @extack:	Extack pointer for errors
// @m_entry:previous nfp_fl_nft_tc_merge entry
//
// Adds a new entry to the relevant zone table and tries to
// merge with other +trk+est entries and offload if possible.
//
// Return: negative value on error, 0 if configured successfully.
//
// nfp_fl_ct_handle_post_ct() - Handles +trk+est conntrack rules
// @priv:	Pointer to app priv
// @netdev:	netdev structure.
// @flow:	TC flower classifier offload structure.
// @extack:	Extack pointer for errors
//
// Adds a new entry to the relevant zone table and tries to
// merge with other -trk entries and offload if possible.
//
// Return: negative value on error, 0 if configured successfully.
//
// nfp_fl_create_new_pre_ct() - create next ct_zone -trk conntrack rules
// @m_entry:previous nfp_fl_nft_tc_merge entry
//
// Create a new pre_ct entry from previous nfp_fl_nft_tc_merge entry
// to the next relevant zone table. Try to merge with other +trk+est
// entries and offload if possible. The created new pre_ct entry is
// linked to the previous nfp_fl_nft_tc_merge entry.
//
// Return: negative value on error, 0 if configured successfully.
//
extern "C" {
    pub fn nfp_fl_create_new_pre_ct(m_entry: *mut nfp_fl_nft_tc_merge) -> c_int;
}
//
// nfp_fl_ct_clean_flow_entry() - Free a nfp_fl_ct_flow_entry
// @entry:	Flow entry to cleanup
//
extern "C" {
    pub fn nfp_fl_ct_clean_flow_entry(entry: *mut nfp_fl_ct_flow_entry);
}
//
// nfp_fl_ct_del_flow() - Handle flow_del callbacks for conntrack
// @ct_map_ent:	ct map entry for the flow that needs deleting
//
extern "C" {
    pub fn nfp_fl_ct_del_flow(ct_map_ent: *mut nfp_fl_ct_map_entry) -> c_int;
}
//
// nfp_fl_ct_handle_nft_flow() - Handle flower flow callbacks for nft table
// @type:	Type provided by callback
// @type_data:	Callback data
// @cb_priv:	Pointer to data provided when registering the callback, in this
// case it's the zone table.
//
// nfp_fl_ct_stats() - Handle flower stats callbacks for ct flows
// @flow:	TC flower classifier offload structure.
// @ct_map_ent:	ct map entry for the flow that needs deleting
//
