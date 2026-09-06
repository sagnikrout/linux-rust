//! Automatically rewritten from C Header to Rust Module
//! Source: net/hsr/hsr_framereg.h
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
// Copyright 2011-2014 Autronica Fire and Security AS
//
// Author(s):
// 2011-2014 Arvid Brodin, arvid.brodin@alten.se
//
// include file for HSR and PRP.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_frame_info {
    pub skb_std: *mut sk_buff,
    pub skb_hsr: *mut sk_buff,
    pub skb_prp: *mut sk_buff,
    pub port_rcv: *mut hsr_port,
    pub node_src: *mut hsr_node,
    pub sequence_nr: u16,
    pub is_supervision: bool,
    pub is_proxy_supervision: bool,
    pub is_vlan: bool,
    pub is_local_dest: bool,
    pub is_local_exclusive: bool,
    pub is_from_san: bool,
    pub dst_in_node_db: bool,
    pub dst_in_proxy_node_db: bool,
}

extern "C" {
    pub fn hsr_del_self_node(hsr: *mut hsr_priv);
}
extern "C" {
    pub fn hsr_del_nodes(node_db: *mut list_head);
}
extern "C" {
    pub fn hsr_handle_sup_frame(frame: *mut hsr_frame_info);
}
extern "C" {
    pub fn hsr_addr_is_self(hsr: *mut hsr_priv, addr: *mut c_uchar) -> bool;
}
extern "C" {
    pub fn hsr_addr_is_redbox(hsr: *mut hsr_priv, addr: *mut c_uchar) -> bool;
}
extern "C" {
    pub fn hsr_addr_subst_source(node: *mut hsr_node, skb: *mut sk_buff);
}
extern "C" {
    pub fn hsr_register_frame_out(port: *mut hsr_port, frame: *mut hsr_frame_info) -> c_int;
}
extern "C" {
    pub fn hsr_prune_nodes(t: *mut timer_list);
}
extern "C" {
    pub fn hsr_prune_proxy_nodes(t: *mut timer_list);
}
extern "C" {
    pub fn prp_update_san_info(node: *mut hsr_node, is_sup: bool);
}
extern "C" {
    pub fn prp_register_frame_out(port: *mut hsr_port, frame: *mut hsr_frame_info) -> c_int;
}

pub const HSR_MAX_SEQ_BLOCKS: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_seq_block {
    pub time: c_ulong,
    pub block_idx: u16,
// Should be a flexible array member of what DECLARE_BITMAP() would
// produce.
//
    pub seq_nrs: [c_ulong; ][BITS_TO_LONGS(HSR_SEQ_BLOCK_SIZE)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_node {
    pub mac_list: list_head,
// Protect R/W access seq_blocks
    pub seq_out_lock: spinlock_t,
    pub macaddress_A: [c_uchar; ETH_ALEN],
    pub macaddress_B: [c_uchar; ETH_ALEN],
// Local slave through which AddrB frames are received from this node
    pub addr_B_port: hsr_port_type,
    pub time_in: [c_ulong; HSR_PT_PORTS],
    pub time_in_stale: [bool; HSR_PT_PORTS],
// if the node is a SAN
    pub san_a: bool,
    pub san_b: bool,
    pub removed: bool,
// Duplicate detection
    pub seq_blocks: xarray,
    pub block_buf: *mut c_void,
    pub next_block: c_uint,
    pub seq_port_cnt: c_uint,
    pub rcu_head: rcu_head,
}

extern "C" {
    pub fn struct_size_t(hsr_seq_block: struct, _arg: seq_nrs, _arg: node->seq_port_cnt) -> return;
}
