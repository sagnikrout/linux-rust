//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ip_fib.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET  is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the Forwarding Information Base.
//
// Authors:	A.N.Kuznetsov, <kuznet@ms2.inr.ac.ru>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_config {
    pub fc_dst_len: u8,
    pub fc_dscp: dscp_t,
    pub fc_protocol: u8,
    pub fc_scope: u8,
    pub fc_type: u8,
    pub fc_gw_family: u8,
// 2 bytes unused
    pub fc_table: u32,
    pub fc_dst: __be32,
    pub fc_gw4: __be32,
    pub fc_gw6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_nh_exception {
    pub fnhe_next: *mut fib_nh_exception __rcu,
    pub fnhe_genid: c_int,
    pub fnhe_daddr: __be32,
    pub fnhe_pmtu: u32,
    pub fnhe_mtu_locked: bool,
    pub fnhe_gw: __be32,
    pub fnhe_expires: c_ulong,
    pub fnhe_rth_input: *mut rtable __rcu,
    pub fnhe_rth_output: *mut rtable __rcu,
    pub fnhe_stamp: c_ulong,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fnhe_hash_bucket {
    pub chain: *mut fib_nh_exception __rcu,
}

pub const FNHE_HASH_SHIFT: c_int = 11;

pub const FNHE_RECLAIM_DEPTH: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_nh_common {
    pub nhc_dev: *mut net_device,
    pub nhc_dev_tracker: netdevice_tracker,
    pub nhc_oif: c_int,
    pub nhc_scope: c_uchar,
    pub nhc_family: u8,
    pub nhc_gw_family: u8,
    pub nhc_flags: c_uchar,
    pub nhc_lwtstate: *mut lwtunnel_state,
    pub ipv4: __be32,
    pub ipv6: in6_addr,
    pub nhc_gw: },
    pub nhc_weight: c_int,
    pub nhc_upper_bound: core::sync::atomic::AtomicI32,
// v4 specific, but allows fib6_nh with v4 routes
    pub nhc_pcpu_rth_output: *mut *mut rtable __rcu  __percpu,
    pub nhc_rth_input: *mut rtable __rcu,
    pub nhc_exceptions: *mut fnhe_hash_bucket __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_nh {
    pub nh_common: fib_nh_common,
    pub nh_hash: hlist_node,
    pub nh_parent: *mut fib_info,

    pub nh_tclassid: __u32,

    pub nh_saddr: __be32,
    pub nh_saddr_genid: c_int,

}

//
// This structure contains data shared by many of routes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_info {
    pub fib_hash: hlist_node,
    pub fib_lhash: hlist_node,
    pub nh_list: list_head,
    pub fib_net: *mut net,
    pub fib_treeref: refcount_t,
    pub fib_clntref: refcount_t,
    pub fib_flags: c_uint,
    pub fib_dead: c_uchar,
    pub fib_protocol: c_uchar,
    pub fib_scope: c_uchar,
    pub fib_type: c_uchar,
    pub fib_prefsrc: __be32,
    pub fib_tb_id: u32,
    pub fib_priority: u32,
    pub fib_metrics: *mut dst_metrics,

    pub fib_nhs: c_int,
    pub fib_nh_is_v6: bool,
    pub nh_updated: bool,
    pub pfsrc_removed: bool,
    pub nh: *mut nexthop,
    pub rcu: rcu_head,
    pub __counted_by(fib_nhs): fib_nh fib_nh[],
}

extern "C" {
    pub fn fib4_semantics_init(net: *mut net) -> int __net_init;
}
extern "C" {
    pub fn fib4_semantics_exit(net: *mut net) -> void __net_exit;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_result {
    pub prefix: __be32,
    pub prefixlen: c_uchar,
    pub nh_sel: c_uchar,
    pub type: c_uchar,
    pub scope: c_uchar,
    pub tclassid: u32,
    pub dscp: dscp_t,
    pub nhc: *mut fib_nh_common,
    pub fi: *mut fib_info,
    pub table: *mut fib_table,
    pub fa_head: *mut hlist_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_result_nl {
    pub up*/: *mut *mut __be32 fl_addr; / To be looked,
    pub fl_mark: u32,
    pub fl_tos: c_uchar,
    pub fl_scope: c_uchar,
    pub tb_id_in: c_uchar,
    pub /: *mut *mut unsigned char tb_id; / Results,
    pub prefixlen: c_uchar,
    pub nh_sel: c_uchar,
    pub type: c_uchar,
    pub scope: c_uchar,
    pub err: c_int,
}

pub const FIB_TABLE_HASHSZ: c_int = 256;

pub const FIB_TABLE_HASHSZ: c_int = 2;

extern "C" {
    pub fn fib_result_prefsrc(net: *mut net, res: *mut fib_result) -> __be32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_rt_info {
    pub fi: *mut fib_info,
    pub tb_id: u32,
    pub dst: __be32,
    pub dst_len: c_int,
    pub dscp: dscp_t,
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_entry_notifier_info {
    pub /: *mut *mut fib_notifier_info info; / must be first,
    pub dst: u32,
    pub dst_len: c_int,
    pub fi: *mut fib_info,
    pub dscp: dscp_t,
    pub type: u8,
    pub tb_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_nh_notifier_info {
    pub /: *mut *mut fib_notifier_info info; / must be first,
    pub fib_nh: *mut fib_nh,
}

extern "C" {
    pub fn fib4_notifier_init(net: *mut net) -> int __net_init;
}
extern "C" {
    pub fn fib4_notifier_exit(net: *mut net) -> void __net_exit;
}
extern "C" {
    pub fn fib_info_notify_update(net: *mut net, info: *mut nl_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_table {
    pub tb_hlist: hlist_node,
    pub tb_id: u32,
    pub tb_num_default: c_int,
    pub rcu: rcu_head,
    pub tb_data: *mut c_ulong,
    pub __data: [c_ulong; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_dump_filter {
    pub table_id: u32,
// filter_set is an optimization that an entry is set
    pub filter_set: bool,
    pub dump_routes: bool,
    pub dump_exceptions: bool,
    pub protocol: c_uchar,
    pub rt_type: c_uchar,
    pub flags: c_uint,
    pub dev: *mut net_device,
}

extern "C" {
    pub fn fib_table_flush(net: *mut net, table: *mut fib_table, flush_all: bool) -> c_int;
}
extern "C" {
    pub fn fib_table_flush_external(table: *mut fib_table);
}
extern "C" {
    pub fn fib_free_table(tb: *mut fib_table);
}

// Only fib4_rules_init() adds fib_table.
extern "C" {
    pub fn hlist_entry(_arg: tb_hlist, fib_table: struct, _arg: tb_hlist) -> return;
}
extern "C" {
    pub fn fib_get_table(_arg: net, _arg: id) -> return;
}

extern "C" {
    pub fn fib4_rules_init(net: *mut net) -> int __net_init;
}
extern "C" {
    pub fn fib4_rules_exit(net: *mut net) -> void __net_exit;
}
extern "C" {
    pub fn __fib_lookup(_arg: net, _arg: flp, _arg: res, _arg: flags) -> return;
}
extern "C" {
    pub fn fib4_rule_default(rule: *const fib_rule) -> bool;
}
extern "C" {
    pub fn fib4_rules_seq_read(net: *const net) -> c_uint;
}

// Exported by fib_frontend.c
extern "C" {
    pub fn ip_fib_init();
}
extern "C" {
    pub fn fib_compute_spec_dst(skb: *mut sk_buff) -> __be32;
}
extern "C" {
    pub fn fib_info_nh_uses_dev(fi: *mut fib_info, dev: *const net_device) -> bool;
}

extern "C" {
    pub fn atomic_read(_arg: &net->ipv4.fib_num_tclassid_users) -> return;
}

extern "C" {
    pub fn fib_unmerge(net: *mut net) -> c_int;
}
// Exported by fib_semantics.c
extern "C" {
    pub fn ip_fib_check_default(gw: __be32, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn fib_sync_down_dev(dev: *mut net_device, event: c_ulong, force: bool) -> c_int;
}
extern "C" {
    pub fn fib_sync_down_addr(dev: *mut net_device, local: __be32) -> c_int;
}
extern "C" {
    pub fn fib_sync_up(dev: *mut net_device, nh_flags: c_uchar) -> c_int;
}
extern "C" {
    pub fn fib_sync_mtu(dev: *mut net_device, orig_mtu: u32);
}
extern "C" {
    pub fn fib_nhc_update_mtu(nhc: *mut fib_nh_common, new: u32, orig: u32);
}
// Fields used for sysctl_fib_multipath_hash_fields.
// Common to IPv4 and IPv6.
//
// Add new fields at the end. This is user API.
//

extern "C" {
    pub fn flow_hash_from_keys_seed(_arg: keys, _arg: &hash_key) -> return;
}

extern "C" {
    pub fn flow_hash_from_keys(_arg: keys) -> return;
}

extern "C" {
    pub fn fib_nh_release(net: *mut net, fib_nh: *mut fib_nh);
}
extern "C" {
    pub fn fib_nh_common_release(nhc: *mut fib_nh_common);
}
// Exported by fib_trie.c
extern "C" {
    pub fn fib_alias_hw_flags_set(net: *mut net, fri: *const fib_rt_info);
}
extern "C" {
    pub fn fib_trie_init();
}

// itag = nh->nh_tclassid << 16;
// itag = 0;

// itag = (rtag<<16);
// itag |= (rtag>>16);

extern "C" {
    pub fn fib_flush(net: *mut net);
}
extern "C" {
    pub fn free_fib_info(fi: *mut fib_info);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &fi->fib_clntref) -> return;
}

extern "C" {
    pub fn fib_proc_init(net: *mut net) -> int __net_init;
}
extern "C" {
    pub fn fib_proc_exit(net: *mut net) -> void __net_exit;
}

extern "C" {
    pub fn ip_mtu_from_fib_result(res: *mut fib_result, daddr: __be32) -> u32;
}
