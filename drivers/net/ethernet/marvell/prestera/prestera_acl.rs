//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/prestera/prestera_acl.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2020-2021 Marvell International Ltd. All rights reserved.

pub const PRESTERA_ACL_KEYMASK_PCL_ID: c_uint = 0x3FF;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_acl_match_type {
    PRESTERA_ACL_RULE_MATCH_TYPE_PCL_ID,
    PRESTERA_ACL_RULE_MATCH_TYPE_ETH_TYPE,
    PRESTERA_ACL_RULE_MATCH_TYPE_ETH_DMAC_0,
    PRESTERA_ACL_RULE_MATCH_TYPE_ETH_DMAC_1,
    PRESTERA_ACL_RULE_MATCH_TYPE_ETH_SMAC_0,
    PRESTERA_ACL_RULE_MATCH_TYPE_ETH_SMAC_1,
    PRESTERA_ACL_RULE_MATCH_TYPE_IP_PROTO,
    PRESTERA_ACL_RULE_MATCH_TYPE_SYS_PORT,
    PRESTERA_ACL_RULE_MATCH_TYPE_SYS_DEV,
    PRESTERA_ACL_RULE_MATCH_TYPE_IP_SRC,
    PRESTERA_ACL_RULE_MATCH_TYPE_IP_DST,
    PRESTERA_ACL_RULE_MATCH_TYPE_L4_PORT_SRC,
    PRESTERA_ACL_RULE_MATCH_TYPE_L4_PORT_DST,
    PRESTERA_ACL_RULE_MATCH_TYPE_L4_PORT_RANGE_SRC,
    PRESTERA_ACL_RULE_MATCH_TYPE_L4_PORT_RANGE_DST,
    PRESTERA_ACL_RULE_MATCH_TYPE_VLAN_ID,
    PRESTERA_ACL_RULE_MATCH_TYPE_VLAN_TPID,
    PRESTERA_ACL_RULE_MATCH_TYPE_ICMP_TYPE,
    PRESTERA_ACL_RULE_MATCH_TYPE_ICMP_CODE,

    __PRESTERA_ACL_RULE_MATCH_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_acl_rule_action {
    PRESTERA_ACL_RULE_ACTION_ACCEPT = 0,
    PRESTERA_ACL_RULE_ACTION_DROP = 1,
    PRESTERA_ACL_RULE_ACTION_TRAP = 2,
    PRESTERA_ACL_RULE_ACTION_JUMP = 5,
    PRESTERA_ACL_RULE_ACTION_COUNT = 7,
    PRESTERA_ACL_RULE_ACTION_POLICE = 8,

    PRESTERA_ACL_RULE_ACTION_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_acl_match {
    pub key: [__be32; __PRESTERA_ACL_RULE_MATCH_TYPE_MAX],
    pub mask: [__be32; __PRESTERA_ACL_RULE_MATCH_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_acl_action_jump {
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_acl_action_police {
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_acl_action_count {
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_acl_rule_entry_key {
    pub prio: u32,
    pub match: prestera_acl_match,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_acl_hw_action_info {
    pub id: prestera_acl_rule_action,
    pub police: prestera_acl_action_police,
    pub count: prestera_acl_action_count,
    pub jump: prestera_acl_action_jump,
}

// This struct (arg) used only to be passed as parameter for
// acl_rule_entry_create. Must be flat. Can contain object keys, which will be
// resolved to object links, before saving to acl_rule_entry struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_acl_rule_entry_arg {
    pub vtcam_id: u32,
    pub valid:1: u8,
    pub trap: } accept, drop,,
    pub i: prestera_acl_action_jump,
    pub valid:1: u8,
    pub jump: },
    pub valid:1: u8,
    pub rate: u64,
    pub burst: u64,
    pub ingress: bool,
    pub police: },
    pub valid:1: u8,
    pub client: u32,
    pub count: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_acl_rule {
    pub /: *mut *mut rhash_head ht_node; / Member of acl HT,
    pub list: list_head,
    pub ruleset: *mut prestera_acl_ruleset,
    pub jump_ruleset: *mut prestera_acl_ruleset,
    pub cookie: c_ulong,
    pub chain_index: u32,
    pub priority: u32,
    pub re_key: prestera_acl_rule_entry_key,
    pub re_arg: prestera_acl_rule_entry_arg,
    pub re: *mut prestera_acl_rule_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_acl_iface {
    pub port: *mut prestera_port,
    pub index: u32,
}

extern "C" {
    pub fn prestera_acl_init(sw: *mut prestera_switch) -> c_int;
}
extern "C" {
    pub fn prestera_acl_fini(sw: *mut prestera_switch);
}
extern "C" {
    pub fn prestera_acl_rule_destroy(rule: *mut prestera_acl_rule);
}
extern "C" {
    pub fn prestera_acl_ruleset_is_offload(ruleset: *mut prestera_acl_ruleset) -> bool;
}
extern "C" {
    pub fn prestera_acl_ruleset_offload(ruleset: *mut prestera_acl_ruleset) -> c_int;
}
extern "C" {
    pub fn prestera_acl_ruleset_put(ruleset: *mut prestera_acl_ruleset);
}
extern "C" {
    pub fn prestera_acl_ruleset_index_get(ruleset: *const prestera_acl_ruleset) -> u32;
}
extern "C" {
    pub fn prestera_acl_vtcam_id_put(acl: *mut prestera_acl, vtcam_id: u32) -> c_int;
}
extern "C" {
    pub fn prestera_acl_chain_to_client(chain_index: u32, ingress: bool, client: *mut u32) -> c_int;
}
