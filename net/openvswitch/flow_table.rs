//! Automatically rewritten from C Header to Rust Module
//! Source: net/openvswitch/flow_table.h
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
// Copyright (c) 2007-2013 Nicira, Inc.
//
pub const FLOW_TABLE_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mask_cache_entry {
    pub skb_hash: u32,
    pub mask_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mask_cache {
    pub rcu: rcu_head,
    pub /: *mut *mut u32 cache_size; / Must be ^2 value.,
    pub mask_cache: *mut mask_cache_entry __percpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mask_count {
    pub index: c_int,
    pub counter: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mask_array_stats {
    pub syncp: u64_stats_sync,
    pub usage_cntrs: [u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mask_array {
    pub rcu: rcu_head,
    pub max: int count,,
    pub masks_usage_stats: *mut mask_array_stats __percpu,
    pub masks_usage_zero_cntr: *mut u64,
    pub __counted_by(max): *mut *mut sw_flow_mask __rcu masks[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct table_instance {
    pub buckets: *mut hlist_head,
    pub n_buckets: c_uint,
    pub rcu: rcu_head,
    pub node_ver: c_int,
    pub hash_seed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flow_table {
    pub ti: *mut table_instance __rcu,
    pub ufid_ti: *mut table_instance __rcu,
    pub mask_cache: *mut mask_cache __rcu,
    pub mask_array: *mut mask_array __rcu,
    pub last_rehash: c_ulong,
    pub count: c_uint,
    pub ufid_count: c_uint,
}

extern "C" {
    pub fn ovs_flow_init() -> c_int;
}
extern "C" {
    pub fn ovs_flow_exit();
}
extern "C" {
    pub fn ovs_flow_free(: *mut sw_flow, deferred: bool);
}
extern "C" {
    pub fn ovs_flow_tbl_init(: *mut flow_table) -> c_int;
}
extern "C" {
    pub fn ovs_flow_tbl_count(table: *const flow_table) -> c_int;
}
extern "C" {
    pub fn ovs_flow_tbl_destroy(table: *mut flow_table);
}
extern "C" {
    pub fn ovs_flow_tbl_flush(flow_table: *mut flow_table) -> c_int;
}
extern "C" {
    pub fn ovs_flow_tbl_remove(table: *mut flow_table, flow: *mut sw_flow);
}
extern "C" {
    pub fn ovs_flow_tbl_num_masks(table: *const flow_table) -> c_int;
}
extern "C" {
    pub fn ovs_flow_tbl_masks_cache_size(table: *const flow_table) -> u32;
}
extern "C" {
    pub fn ovs_flow_tbl_masks_cache_resize(table: *mut flow_table, size: u32) -> c_int;
}
extern "C" {
    pub fn ovs_flow_cmp(: *const sw_flow, : *const sw_flow_match) -> bool;
}
extern "C" {
    pub fn ovs_flow_masks_rebalance(table: *mut flow_table);
}
