//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_tc.h
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
// Copyright (c) 2016, Mellanox Technologies. All rights reserved.
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

pub const MLX5E_TC_FLOW_ID_MASK: c_uint = 0x0000ffff;

extern "C" {
    pub fn mlx5e_tc_num_filters(priv: *mut mlx5e_priv, flags: c_ulong) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tc_update_priv {
    pub fwd_dev: *mut net_device,
    pub skb_done: bool,
    pub forward_tx: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_nic_flow_attr {
    pub flow_tag: u32,
    pub hairpin_tirn: u32,
    pub hairpin_ft: *mut mlx5_flow_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_attr {
    pub action: u32,
    pub tc_act_cookies: [c_ulong; TCA_ACT_MAX_PRIO],
    pub counter: *mut mlx5_fc,
    pub modify_hdr: *mut mlx5_modify_hdr,
    pub /: *mut *mut *mut mlx5e_mod_hdr_handle mh; / attached mod header instance,
    pub /: *mut *mut *mut mlx5e_mod_hdr_handle slow_mh; / attached mod header instance for slow path,
    pub ct_attr: mlx5_ct_attr,
    pub sample_attr: mlx5e_sample_attr,
    pub meter_attr: mlx5e_meter_attr,
    pub parse_attr: *mut mlx5e_tc_flow_parse_attr,
    pub chain: u32,
    pub prio: u16,
    pub tc_act_cookies_count: u16,
    pub dest_chain: u32,
    pub ft: *mut mlx5_flow_table,
    pub dest_ft: *mut mlx5_flow_table,
    pub extra_split_ft: *mut mlx5_flow_table,
    pub inner_match_level: u8,
    pub outer_match_level: u8,
    pub tun_ip_version: u8,
    pub /: *mut *mut int tunnel_id; / mapped tunnel id,
    pub flags: u32,
    pub exe_aso_type: u32,
    pub list: list_head,
    pub post_act_handle: *mut mlx5e_post_act_handle,
    pub branch_true: *mut mlx5_flow_attr,
    pub branch_false: *mut mlx5_flow_attr,
    pub jumping_attr: *mut mlx5_flow_attr,
    pub act_id_restore_rule: *mut mlx5_flow_handle,
// keep this union last
    pub esw_attr): DECLARE_FLEX_ARRAY(struct mlx5_esw_flow_attr,,
    pub nic_attr): DECLARE_FLEX_ARRAY(struct mlx5_nic_flow_attr,,
}

// Returns true if any of the flags that require skipping further TC/NF processing are set.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_rx_tun_attr {
    pub decap_vport: u16,
    pub v4: __be32,
    pub v6: in6_addr,
    pub /: *mut *mut } src_ip; / Valid if decap_vport is not zero,
    pub v4: __be32,
    pub v6: in6_addr,
    pub /: *mut *mut } dst_ip; / Valid if decap_vport is not zero,
}

pub const MLX5E_TC_TABLE_CHAIN_TAG_BITS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tunnel_match_key {
    pub enc_control: flow_dissector_key_control,
    pub enc_key_id: flow_dissector_key_keyid,
    pub enc_tp: flow_dissector_key_ports,
    pub enc_ip: flow_dissector_key_ip,
    pub enc_ipv4: flow_dissector_key_ipv4_addrs,
    pub enc_ipv6: flow_dissector_key_ipv6_addrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tunnel_match_enc_opts {
    pub key: flow_dissector_key_enc_opts,
    pub mask: flow_dissector_key_enc_opts,
}

// Tunnel_id mapping is TUNNEL_INFO_BITS + ENC_OPTS_BITS.
// Upper TUNNEL_INFO_BITS for general tunnel info.
// Lower ENC_OPTS_BITS bits for enc_opts.
//
pub const TUNNEL_INFO_BITS: c_int = 12;

pub const ENC_OPTS_BITS: c_int = 11;

extern "C" {
    pub fn mlx5e_tc_esw_init(uplink_priv: *mut mlx5_rep_uplink_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_tc_esw_cleanup(uplink_priv: *mut mlx5_rep_uplink_priv);
}
extern "C" {
    pub fn mlx5e_tc_ht_init(tc_ht: *mut rhashtable) -> c_int;
}
extern "C" {
    pub fn mlx5e_tc_ht_cleanup(tc_ht: *mut rhashtable);
}
extern "C" {
    pub fn mlx5e_encap_take(e: *mut mlx5e_encap_entry) -> bool;
}
extern "C" {
    pub fn mlx5e_encap_put(priv: *mut mlx5e_priv, e: *mut mlx5e_encap_entry);
}
extern "C" {
    pub fn mlx5e_take_all_encap_flows(e: *mut mlx5e_encap_entry, flow_list: *mut list_head);
}
extern "C" {
    pub fn mlx5e_put_flow_list(priv: *mut mlx5e_priv, flow_list: *mut list_head);
}
extern "C" {
    pub fn mlx5e_tc_update_neigh_used_value(nhe: *mut mlx5e_neigh_hash_entry);
}
extern "C" {
    pub fn mlx5e_tc_reoffload_flows_work(work: *mut work_struct);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_tc_attr_to_reg {
    MAPPED_OBJ_TO_REG,
    VPORT_TO_REG,
    TUNNEL_TO_REG,
    CTSTATE_TO_REG,
    ZONE_TO_REG,
    ZONE_RESTORE_TO_REG,
    MARK_TO_REG,
    LABELS_TO_REG,
    FTEID_TO_REG,
    NIC_MAPPED_OBJ_TO_REG,
    NIC_ZONE_RESTORE_TO_REG,
    PACKET_COLOR_TO_REG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_tc_attr_to_reg_mapping {
    pub /: *mut *mut int mfield; / rewrite field,
    pub /: *mut *mut int moffset; / bit offset of mfield,
    pub /: *mut *mut int mlen; / bits to rewrite/match,
    pub /: *mut *mut int soffset; / byte offset of spec for match,
}

extern "C" {
    pub fn mlx5e_tc_nic_init(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_tc_nic_cleanup(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_tc_is_vf_tunnel(out_dev: *mut net_device, route_dev: *mut net_device) -> bool;
}

extern "C" {
    pub fn mlx5e_tc_table_free(tc: *mut mlx5e_tc_table);
}
extern "C" {
    pub fn mlx5e_tc_update_skb_nic(cqe: *mut mlx5_cqe64, skb: *mut sk_buff) -> bool;
}

