//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/nexthop.h
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
// Generic nexthop implementation
//
// Copyright (c) 2017-19 Cumulus Networks
// Copyright (c) 2017-19 David Ahern <dsa@cumulusnetworks.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_config {
    pub nh_id: u32,
    pub nh_family: u8,
    pub nh_protocol: u8,
    pub nh_blackhole: u8,
    pub nh_fdb: u8,
    pub nh_dst_port: __be16,
    pub nh_flags: u32,
    pub nh_ifindex: c_int,
    pub dev: *mut net_device,
    pub ipv4: __be32,
    pub ipv6: in6_addr,
    pub gw: },
    pub nh_grp: *mut nlattr,
    pub nh_grp_type: u16,
    pub nh_grp_res_num_buckets: u16,
    pub nh_grp_res_idle_timer: c_ulong,
    pub nh_grp_res_unbalanced_timer: c_ulong,
    pub nh_grp_res_has_num_buckets: bool,
    pub nh_grp_res_has_idle_timer: bool,
    pub nh_grp_res_has_unbalanced_timer: bool,
    pub nh_hw_stats: bool,
    pub nh_encap: *mut nlattr,
    pub nh_encap_type: u16,
    pub nlflags: u32,
    pub nlinfo: nl_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_info {
    pub /: *mut *mut hlist_node dev_hash; / entry on netns devhash,
    pub nh_parent: *mut nexthop,
    pub family: u8,
    pub reject_nh: bool,
    pub fdb_nh: bool,
    pub dst_port: __be16,
    pub fib_nhc: fib_nh_common,
    pub fib_nh: fib_nh,
    pub fib6_nh: fib6_nh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_res_bucket {
    pub nh_entry: *mut nh_grp_entry __rcu,
    pub used_time: atomic_long_t,
    pub migrated_time: c_ulong,
    pub occupied: bool,
    pub nh_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_res_table {
    pub net: *mut net,
    pub nhg_id: u32,
    pub upkeep_dw: delayed_work,
// List of NHGEs that have too few buckets ("uw" for underweight).
// Reclaimed buckets will be given to entries in this list.
//
    pub uw_nh_entries: list_head,
    pub unbalanced_since: c_ulong,
    pub idle_timer: u32,
    pub unbalanced_timer: u32,
    pub num_nh_buckets: u16,
    pub __counted_by(num_nh_buckets): nh_res_bucket nh_buckets[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_grp_entry_stats {
    pub packets: u64_stats_t,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_grp_entry {
    pub nh: *mut nexthop,
    pub stats: *mut nh_grp_entry_stats __percpu,
    pub weight: u16,
    pub upper_bound: core::sync::atomic::AtomicI32,
    pub hthr: },
// Member on uw_nh_entries.
    pub uw_nh_entry: list_head,
    pub count_buckets: u16,
    pub wants_buckets: u16,
    pub res: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_group {
    pub /: *mut *mut *mut nh_group spare; / spare group for removals,
    pub num_nh: u16,
    pub is_multipath: bool,
    pub hash_threshold: bool,
    pub resilient: bool,
    pub fdb_nh: bool,
    pub has_v4: bool,
    pub hw_stats: bool,
    pub res_table: *mut nh_res_table __rcu,
    pub __counted_by(num_nh): nh_grp_entry nh_entries[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nexthop {
    pub /: *mut *mut rb_node rb_node; / entry on netns rbtree,
    pub /: *mut *mut list_head fi_list; / v4 entries using nh,
    pub /: *mut *mut list_head f6i_list; / v6 entries using nh,
    pub /: *mut *mut list_head fdb_list; / fdb entries using this nh,
    pub /: *mut *mut list_head grp_list; / nh group entries using this nh,
    pub net: *mut net,
    pub id: u32,
    pub /: *mut *mut u8 protocol; / app managing this nh,
    pub nh_flags: u8,
    pub is_group: bool,
    pub dead: bool,
    pub /: *mut *mut spinlock_t lock; / protect dead and f6i_list,
    pub refcnt: refcount_t,
    pub rcu: rcu_head,
    pub nh_info: *mut nh_info __rcu,
    pub nh_grp: *mut nh_group __rcu,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nexthop_event_type {
    NEXTHOP_EVENT_DEL,
    NEXTHOP_EVENT_REPLACE,
    NEXTHOP_EVENT_RES_TABLE_PRE_REPLACE,
    NEXTHOP_EVENT_BUCKET_REPLACE,
    NEXTHOP_EVENT_HW_STATS_REPORT_DELTA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nh_notifier_info_type {
    NH_NOTIFIER_INFO_TYPE_SINGLE,
    NH_NOTIFIER_INFO_TYPE_GRP,
    NH_NOTIFIER_INFO_TYPE_RES_TABLE,
    NH_NOTIFIER_INFO_TYPE_RES_BUCKET,
    NH_NOTIFIER_INFO_TYPE_GRP_HW_STATS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_notifier_single_info {
    pub dev: *mut net_device,
    pub gw_family: u8,
    pub ipv4: __be32,
    pub ipv6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_notifier_grp_entry_info {
    pub weight: u16,
    pub nh: nh_notifier_single_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_notifier_grp_info {
    pub num_nh: u16,
    pub is_fdb: bool,
    pub hw_stats: bool,
    pub __counted_by(num_nh): nh_notifier_grp_entry_info nh_entries[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_notifier_res_bucket_info {
    pub bucket_index: u16,
    pub idle_timer_ms: c_uint,
    pub force: bool,
    pub old_nh: nh_notifier_single_info,
    pub new_nh: nh_notifier_single_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_notifier_res_table_info {
    pub num_nh_buckets: u16,
    pub hw_stats: bool,
    pub __counted_by(num_nh_buckets): nh_notifier_single_info nhs[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_notifier_grp_hw_stats_entry_info {
    pub id: u32,
    pub packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_notifier_grp_hw_stats_info {
    pub num_nh: u16,
    pub hw_stats_used: bool,
    pub __counted_by(num_nh): nh_notifier_grp_hw_stats_entry_info stats[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nh_notifier_info {
    pub net: *mut net,
    pub extack: *mut netlink_ext_ack,
    pub id: u32,
    pub type: nh_notifier_info_type,
    pub nh: *mut nh_notifier_single_info,
    pub nh_grp: *mut nh_notifier_grp_info,
    pub nh_res_table: *mut nh_notifier_res_table_info,
    pub nh_res_bucket: *mut nh_notifier_res_bucket_info,
    pub nh_grp_hw_stats: *mut nh_notifier_grp_hw_stats_info,
}

extern "C" {
    pub fn __unregister_nexthop_notifier(net: *mut net, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_nexthop_notifier(net: *mut net, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn nexthop_set_hw_flags(net: *mut net, id: u32, offload: bool, trap: bool);
}
// caller is holding rcu or rtnl; no reference taken to nexthop
extern "C" {
    pub fn nexthop_free_rcu(head: *mut rcu_head);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &nh->refcnt) -> return;
}
// for_nexthops macros in fib_semantics.c grabs a pointer to
// the nexthop before checking nhsel
//
// called with rcu lock
// called with rcu read lock or rtnl held
// called from fib_table_lookup with rcu_lock
// nhsel = i;
// nhsel = 0;
extern "C" {
    pub fn nexthop_num_path(_arg: fi->nh) -> return;
}
extern "C" {
    pub fn nexthop_fib_nhc(_arg: fi->nh, _arg: nhsel) -> return;
}
// only used when fib_nh is built into fib_info
//
// IPv6 variants
//
// Caller should either hold rcu_read_lock(), or RTNL.
// dst_port = nhi->dst_port;
