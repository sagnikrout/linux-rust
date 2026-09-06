//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/tc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Driver for Solarflare network controllers and boards
// Copyright 2019 Solarflare Communications Inc.
// Copyright 2020-2022 Xilinx Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

//
// struct efx_tc_mac_pedit_action - mac pedit action fields
//
// @h_addr:	mac address field of ethernet header
// @linkage:	rhashtable reference
// @ref:	reference count
// @fw_id:	index of this entry in firmware MAC address table
//
// MAC address edits are indirected through a table in the hardware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_mac_pedit_action {
    pub h_addr: [u8; ETH_ALEN],
    pub linkage: rhash_head,
    pub ref: refcount_t,
    pub /: *mut *mut u32 fw_id; / index of this entry in firmware MAC address table,
}

//
// struct efx_tc_action_set - collection of tc action fields
//
// @vlan_push: the number of vlan headers to push
// @vlan_pop: the number of vlan headers to pop
// @decap: used to indicate a tunnel header decapsulation should take place
// @do_nat: perform NAT/NPT with values returned by conntrack match
// @do_ttl_dec: used to indicate IP TTL / Hop Limit should be decremented
// @deliver: used to indicate a deliver action should take place
// @vlan_tci: tci fields for vlan push actions
// @vlan_proto: ethernet types for vlan push actions
// @count: counter mapping
// @encap_md: encap entry in tc_encap_ht table
// @encap_user: linked list of encap users (encap_md->users)
// @user: owning action-set-list. Only populated if @encap_md is; used by efx_tc_update_encap() fallback handling
// @count_user: linked list of counter users (counter->users)
// @dest_mport: destination mport
// @src_mac: source mac entry in tc_mac_ht table
// @dst_mac: destination mac entry in tc_mac_ht table
// @fw_id: index of this entry in firmware actions table
// @list: linked list of tc actions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_action_set {
    pub vlan_push:2: u16,
    pub vlan_pop:2: u16,
    pub decap:1: u16,
    pub do_nat:1: u16,
    pub do_ttl_dec:1: u16,
    pub deliver:1: u16,
    pub vlan_tci: [__be16; 2],
    pub vlan_proto: [__be16; 2],
    pub count: *mut efx_tc_counter_index,
    pub encap_md: *mut efx_tc_encap_action,
    pub encap_user: list_head,
    pub user: *mut efx_tc_action_set_list,
    pub count_user: list_head,
    pub dest_mport: u32,
    pub src_mac: *mut efx_tc_mac_pedit_action,
    pub dst_mac: *mut efx_tc_mac_pedit_action,
    pub fw_id: u32,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_match_fields {
// L1
    pub ingress_port: u32,
    pub /: *mut *mut u8 recirc_id; / mapped from (u32) TC chain_index to smaller space,
// L2 (inner when encap)
    pub eth_proto: __be16,
    pub vlan_proto: [__be16 vlan_tci[2],; 2],
    pub eth_daddr: [u8 eth_saddr[ETH_ALEN],; ETH_ALEN],
// L3 (when IP)
    pub ip_ttl: u8 ip_proto, ip_tos,,
    pub dst_ip: __be32 src_ip,,

    pub dst_ip6: in6_addr src_ip6,,

    pub ip_firstfrag: bool ip_frag,,
// L4
    pub /: *mut *mut __be16 l4_sport, l4_dport; / Ports (UDP, TCP),
    pub tcp_flags: __be16,
    pub /: *mut *mut bool tcp_syn_fin_rst; / true if ANY of SYN/FIN/RST are set,
// Encap.  The following are *outer* fields.  Note that there are no
// outer eth (L2) fields; this is because TC doesn't have them.
//
    pub enc_dst_ip: __be32 enc_src_ip,,
    pub enc_dst_ip6: in6_addr enc_src_ip6,,
    pub enc_ip_ttl: u8 enc_ip_tos,,
    pub enc_dport: __be16 enc_sport,,
    pub /: *mut *mut __be32 enc_keyid; / e.g. VNI, VSID,
// Conntrack.
    pub ct_state_est:1: u16 ct_state_trk:1,,
    pub ct_mark: u32,
    pub ct_zone: u16,
}

//
// enum efx_tc_em_pseudo_type - &struct efx_tc_encap_match pseudo type
//
// These are used to classify "pseudo" encap matches, which don't refer
// to an entry in hardware but rather indicate that a section of the
// match space is in use by another Outer Rule.
//
// @EFX_TC_EM_DIRECT: real HW entry in Outer Rule table; not a pseudo.
// Hardware index in &struct efx_tc_encap_match.fw_id is valid.
// @EFX_TC_EM_PSEUDO_MASK: registered by an encap match which includes a
// match on an optional field (currently ip_tos and/or udp_sport),
// to prevent an overlapping encap match _without_ optional fields.
// The pseudo encap match may be referenced again by an encap match
// with different values for these fields, but all masks must match the
// first (stored in our child_* fields).
// @EFX_TC_EM_PSEUDO_OR: registered by an fLHS rule that fits in the OR
// table.  The &struct efx_tc_lhs_rule already holds the HW OR entry.
// Only one reference to this encap match may exist.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_tc_em_pseudo_type {
    EFX_TC_EM_DIRECT,
    EFX_TC_EM_PSEUDO_MASK,
    EFX_TC_EM_PSEUDO_OR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_encap_match {
    pub dst_ip: __be32 src_ip,,
    pub dst_ip6: in6_addr src_ip6,,
    pub udp_dport: __be16,
    pub udp_sport_mask: __be16 udp_sport,,
    pub ip_tos_mask: u8 ip_tos,,
    pub linkage: rhash_head,
    pub tun_type: efx_encap_type,
    pub child_ip_tos_mask: u8,
    pub child_udp_sport_mask: __be16,
    pub ref: refcount_t,
    pub type: efx_tc_em_pseudo_type,
    pub /: *mut *mut u32 fw_id; / index of this entry in firmware encap match table,
    pub /: *mut *mut *mut efx_tc_encap_match pseudo; / Referenced pseudo EM if needed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_recirc_id {
    pub chain_index: u32,
    pub net_dev: *mut net_device,
    pub linkage: rhash_head,
    pub ref: refcount_t,
    pub /: *mut *mut u8 fw_id; / index allocated for use in the MAE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_match {
    pub value: efx_tc_match_fields,
    pub mask: efx_tc_match_fields,
    pub encap: *mut efx_tc_encap_match,
    pub rid: *mut efx_tc_recirc_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_action_set_list {
    pub list: list_head,
    pub fw_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_lhs_action {
    pub tun_type: efx_encap_type,
    pub rid: *mut efx_tc_recirc_id,
    pub zone: *mut efx_tc_ct_zone,
    pub count: *mut efx_tc_counter_index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_flow_rule {
    pub cookie: c_ulong,
    pub linkage: rhash_head,
    pub match: efx_tc_match,
    pub acts: efx_tc_action_set_list,
    pub /: *mut *mut *mut efx_tc_action_set_list fallback; / what to use when unready?,
    pub fw_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_lhs_rule {
    pub cookie: c_ulong,
    pub match: efx_tc_match,
    pub lhs_act: efx_tc_lhs_action,
    pub linkage: rhash_head,
    pub fw_id: u32,
    pub /: *mut *mut bool is_ar; / Action Rule (for OR-AR-CT-AR sequence),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efx_tc_rule_prios {
    EFX_TC_PRIO_TC, /* Rule inserted by TC */
    EFX_TC_PRIO_DFLT, /* Default switch rule; one of efx_tc_default_rules */
    EFX_TC_PRIO__NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_table_field_fmt {
    pub field_id: u16,
    pub lbn: u16,
    pub width: u16,
    pub masking: u8,
    pub scheme: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_table_desc {
    pub type: u16,
    pub key_width: u16,
    pub resp_width: u16,
    pub n_keys: u16,
    pub n_resps: u16,
    pub n_prios: u16,
    pub flags: u8,
    pub scheme: u8,
    pub keys: *mut efx_tc_table_field_fmt,
    pub resps: *mut efx_tc_table_field_fmt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_table_ct {
    pub desc: efx_tc_table_desc,
    pub hooked: bool,
    pub eth_proto_idx: u8,
    pub ip_proto_idx: u8,
    pub /: *mut *mut u8 src_ip_idx; / either v4 or v6,
    pub dst_ip_idx: u8,
    pub l4_sport_idx: u8,
    pub l4_dport_idx: u8,
    pub /: *mut *mut u8 zone_idx; / for TABLE_FIELD_ID_DOMAIN,
    pub keys: },
    pub dnat_idx: u8,
    pub nat_ip_idx: u8,
    pub l4_natport_idx: u8,
    pub mark_idx: u8,
    pub counter_id_idx: u8,
    pub resps: },
}

//
// struct efx_tc_state - control plane data for TC offload
//
// @caps: MAE capabilities reported by MCDI
// @block_list: List of &struct efx_tc_block_binding
// @mutex: Used to serialise operations on TC hashtables
// @counter_ht: Hashtable of TC counters (FW IDs and counter values)
// @counter_id_ht: Hashtable mapping TC counter cookies to counters
// @encap_ht: Hashtable of TC encap actions
// @mac_ht: Hashtable of MAC address entries (for pedits)
// @encap_match_ht: Hashtable of TC encap matches
// @match_action_ht: Hashtable of TC match-action rules
// @lhs_rule_ht: Hashtable of TC left-hand (act ct & goto chain) rules
// @ct_zone_ht: Hashtable of TC conntrack flowtable bindings
// @ct_ht: Hashtable of TC conntrack flow entries
// @neigh_ht: Hashtable of neighbour watches (&struct efx_neigh_binder)
// @recirc_ht: Hashtable of recirculation ID mappings (&struct efx_tc_recirc_id)
// @recirc_ida: Recirculation ID allocator
// @meta_ct: MAE table layout for conntrack table
// @reps_mport_id: MAE port allocated for representor RX
// @reps_filter_uc: VNIC filter for representor unicast RX (promisc)
// @reps_filter_mc: VNIC filter for representor multicast RX (allmulti)
// @reps_mport_vport_id: vport_id for representor RX filters
// @flush_counters: counters have been stopped, waiting for drain
// @flush_gen: final generation count per type array as reported by
// MC_CMD_MAE_COUNTERS_STREAM_STOP
// @seen_gen: most recent generation count per type as seen by efx_tc_rx()
// @flush_wq: wait queue used by efx_mae_stop_counters() to wait for
// MAE counters RXQ to finish draining
// @dflt: Match-action rules for default switching; at priority
// %EFX_TC_PRIO_DFLT.  Named by *ingress* port
// @dflt.pf: rule for traffic ingressing from PF (egresses to wire)
// @dflt.wire: rule for traffic ingressing from wire (egresses to PF)
// @facts: Fallback action-set-lists for unready rules.  Named by *egress* port
// @facts.pf: action-set-list for unready rules on PF netdev, hence applying to
// traffic from wire, and egressing to PF
// @facts.reps: action-set-list for unready rules on representors, hence
// applying to traffic from representees, and egressing to the reps mport
// @up: have TC datastructures been set up?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_state {
    pub caps: *mut mae_caps,
    pub block_list: list_head,
    pub mutex: mutex,
    pub counter_ht: rhashtable,
    pub counter_id_ht: rhashtable,
    pub encap_ht: rhashtable,
    pub mac_ht: rhashtable,
    pub encap_match_ht: rhashtable,
    pub match_action_ht: rhashtable,
    pub lhs_rule_ht: rhashtable,
    pub ct_zone_ht: rhashtable,
    pub ct_ht: rhashtable,
    pub neigh_ht: rhashtable,
    pub recirc_ht: rhashtable,
    pub recirc_ida: ida,
    pub meta_ct: efx_tc_table_ct,
    pub reps_mport_vport_id: u32 reps_mport_id,,
    pub reps_filter_mc: s32 reps_filter_uc,,
    pub flush_counters: bool,
    pub flush_gen: [u32; EFX_TC_COUNTER_TYPE_MAX],
    pub seen_gen: [u32; EFX_TC_COUNTER_TYPE_MAX],
    pub flush_wq: wait_queue_head_t,
    pub pf: efx_tc_flow_rule,
    pub wire: efx_tc_flow_rule,
    pub dflt: },
    pub pf: efx_tc_action_set_list,
    pub reps: efx_tc_action_set_list,
    pub facts: },
    pub up: bool,
}

extern "C" {
    pub fn efx_tc_indr_netdev_type(net_dev: *mut net_device) -> efx_encap_type;
}
extern "C" {
    pub fn efx_tc_flower_external_mport(efx: *mut efx_nic, efv: *mut efx_rep) -> i64;
}
extern "C" {
    pub fn efx_tc_configure_default_rule_rep(efv: *mut efx_rep) -> c_int;
}
extern "C" {
    pub fn efx_tc_insert_rep_filters(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_tc_remove_rep_filters(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_init_tc(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_fini_tc(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_init_struct_tc(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_fini_struct_tc(efx: *mut efx_nic);
}
