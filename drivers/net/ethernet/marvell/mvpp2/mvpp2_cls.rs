//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/mvpp2/mvpp2_cls.h
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
//
// RSS and Classifier definitions for Marvell PPv2 Network Controller
//
// Copyright (C) 2014 Marvell
//
// Marcin Wojtas <mw@semihalf.com>
//

// Classifier constants
pub const MVPP2_CLS_FLOWS_TBL_SIZE: c_int = 512;
pub const MVPP2_CLS_FLOWS_TBL_DATA_WORDS: c_int = 3;
pub const MVPP2_CLS_LKP_TBL_SIZE: c_int = 64;
pub const MVPP2_CLS_RX_QUEUES: c_int = 256;
// Classifier flow constants
pub const MVPP2_FLOW_N_FIELDS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_cls_engine {
    MVPP22_CLS_ENGINE_C2 = 1,
    MVPP22_CLS_ENGINE_C3A,
    MVPP22_CLS_ENGINE_C3B,
    MVPP22_CLS_ENGINE_C4,
    MVPP22_CLS_ENGINE_C3HA = 6,
    MVPP22_CLS_ENGINE_C3HB = 7,
}

pub const MVPP22_CLS_HEK_N_FIELDS: c_int = 10;

// The fifth tuple in "5T" is the L4_Info field

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_cls_field_id {
    MVPP22_CLS_FIELD_MAC_DA = 0x03,
    MVPP22_CLS_FIELD_VLAN_PRI = 0x05,
    MVPP22_CLS_FIELD_VLAN = 0x06,
    MVPP22_CLS_FIELD_L3_PROTO = 0x0f,
    MVPP22_CLS_FIELD_IP4SA = 0x10,
    MVPP22_CLS_FIELD_IP4DA = 0x11,
    MVPP22_CLS_FIELD_IP6SA = 0x17,
    MVPP22_CLS_FIELD_IP6DA = 0x1a,
    MVPP22_CLS_FIELD_L4SIP = 0x1d,
    MVPP22_CLS_FIELD_L4DIP = 0x1e,
}

// Classifier C2 engine constants

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp22_cls_c2_action {
    MVPP22_C2_NO_UPD = 0,
    MVPP22_C2_NO_UPD_LOCK,
    MVPP22_C2_UPD,
    MVPP22_C2_UPD_LOCK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp22_cls_c2_fwd_action {
    MVPP22_C2_FWD_NO_UPD = 0,
    MVPP22_C2_FWD_NO_UPD_LOCK,
    MVPP22_C2_FWD_SW,
    MVPP22_C2_FWD_SW_LOCK,
    MVPP22_C2_FWD_HW,
    MVPP22_C2_FWD_HW_LOCK,
    MVPP22_C2_FWD_HW_LOW_LAT,
    MVPP22_C2_FWD_HW_LOW_LAT_LOCK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp22_cls_c2_color_action {
    MVPP22_C2_COL_NO_UPD = 0,
    MVPP22_C2_COL_NO_UPD_LOCK,
    MVPP22_C2_COL_GREEN,
    MVPP22_C2_COL_GREEN_LOCK,
    MVPP22_C2_COL_YELLOW,
    MVPP22_C2_COL_YELLOW_LOCK,
    MVPP22_C2_COL_RED,		/* Drop */
    MVPP22_C2_COL_RED_LOCK,		/* Drop */
}

pub const MVPP2_CLS_C2_TCAM_WORDS: c_int = 5;
pub const MVPP2_CLS_C2_ATTR_WORDS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_cls_c2_entry {
    pub index: u32,
// TCAM lookup key
    pub tcam: [u32; MVPP2_CLS_C2_TCAM_WORDS],
// Actions to perform upon TCAM match
    pub act: u32,
// Attributes relative to the actions to perform
    pub attr: [u32; MVPP2_CLS_C2_ATTR_WORDS],
// Entry validity
    pub valid: u8,
}

// Classifier C2 engine entries
pub const MVPP22_CLS_C2_N_ENTRIES: c_int = 256;
// Number of per-port dedicated entries in the C2 TCAM

// Each port has one range per flow type + one entry controlling the global RSS
// setting and the default rx queue
//

// Packet flow ID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_prs_flow {
    MVPP2_FL_START = 8,
    MVPP2_FL_IP4_TCP_NF_UNTAG = MVPP2_FL_START,
    MVPP2_FL_IP4_UDP_NF_UNTAG,
    MVPP2_FL_IP4_TCP_NF_TAG,
    MVPP2_FL_IP4_UDP_NF_TAG,
    MVPP2_FL_IP6_TCP_NF_UNTAG,
    MVPP2_FL_IP6_UDP_NF_UNTAG,
    MVPP2_FL_IP6_TCP_NF_TAG,
    MVPP2_FL_IP6_UDP_NF_TAG,
    MVPP2_FL_IP4_TCP_FRAG_UNTAG,
    MVPP2_FL_IP4_UDP_FRAG_UNTAG,
    MVPP2_FL_IP4_TCP_FRAG_TAG,
    MVPP2_FL_IP4_UDP_FRAG_TAG,
    MVPP2_FL_IP6_TCP_FRAG_UNTAG,
    MVPP2_FL_IP6_UDP_FRAG_UNTAG,
    MVPP2_FL_IP6_TCP_FRAG_TAG,
    MVPP2_FL_IP6_UDP_FRAG_TAG,
    MVPP2_FL_IP4_UNTAG, /* non-TCP, non-UDP, same for below */
    MVPP2_FL_IP4_TAG,
    MVPP2_FL_IP6_UNTAG,
    MVPP2_FL_IP6_TAG,
    MVPP2_FL_NON_IP_UNTAG,
    MVPP2_FL_NON_IP_TAG,
    MVPP2_FL_LAST,
}

// LU Type defined for all engines, and specified in the flow table
pub const MVPP2_CLS_LU_TYPE_MASK: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mvpp2_cls_lu_type {
// rule->loc is used as a lu-type for the entries 0 - 62.
    MVPP22_CLS_LU_TYPE_ALL = 63,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_cls_flow {
// The L2-L4 traffic flow type
    pub flow_type: c_int,
// The first id in the flow table for this flow
    pub flow_id: u16,
// The supported HEK fields for this flow
    pub supported_hash_opts: u16,
// The Header Parser result_info that matches this flow
    pub prs_ri: mvpp2_prs_result_info,
}

// Iterate on each classifier flow id. Sets 'i' to be the index of the first
// entry in the cls_flows table for each different flow_id.
// This relies on entries having the same flow_id in the cls_flows table being
// contiguous.
//

// Iterate on each classifier flow that has a given flow_type. Sets 'i' to be
// the index of the first entry in the cls_flow table for each different flow_id
// that has the given flow_type. This allows to operate on all flows that
// matches a given ethtool flow type.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_cls_flow_entry {
    pub index: u32,
    pub data: [u32; MVPP2_CLS_FLOWS_TBL_DATA_WORDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvpp2_cls_lookup_entry {
    pub lkpid: u32,
    pub way: u32,
    pub data: u32,
}

extern "C" {
    pub fn mvpp22_port_rss_init(port: *mut mvpp2_port) -> c_int;
}
extern "C" {
    pub fn mvpp22_port_rss_enable(port: *mut mvpp2_port) -> c_int;
}
extern "C" {
    pub fn mvpp22_port_rss_disable(port: *mut mvpp2_port) -> c_int;
}
extern "C" {
    pub fn mvpp22_port_rss_ctx_create(port: *mut mvpp2_port, rss_ctx: u32) -> c_int;
}
extern "C" {
    pub fn mvpp22_port_rss_ctx_delete(port: *mut mvpp2_port, rss_ctx: u32) -> c_int;
}
extern "C" {
    pub fn mvpp2_cls_init(priv: *mut mvpp2);
}
extern "C" {
    pub fn mvpp2_cls_port_config(port: *mut mvpp2_port);
}
extern "C" {
    pub fn mvpp2_cls_oversize_rxq_set(port: *mut mvpp2_port);
}
extern "C" {
    pub fn mvpp2_cls_flow_eng_get(fe: *mut mvpp2_cls_flow_entry) -> c_int;
}
extern "C" {
    pub fn mvpp2_flow_get_hek_fields(fe: *mut mvpp2_cls_flow_entry) -> u16;
}
extern "C" {
    pub fn mvpp2_cls_flow_hits(priv: *mut mvpp2, index: c_int) -> u32;
}
extern "C" {
    pub fn mvpp2_cls_lookup_hits(priv: *mut mvpp2, index: c_int) -> u32;
}
extern "C" {
    pub fn mvpp2_cls_c2_hit_count(priv: *mut mvpp2, c2_index: c_int) -> u32;
}
