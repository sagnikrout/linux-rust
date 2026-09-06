//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ip6_fib.h
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
// Linux INET6 implementation
//
// Authors:
// Pedro Roque		<roque@di.fc.ul.pt>
//

pub const FIB6_TABLE_HASHSZ: c_int = 256;

pub const FIB6_TABLE_HASHSZ: c_int = 1;

pub const RT6_DEBUG: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_config {
    pub fc_table: u32,
    pub fc_metric: u32,
    pub fc_dst_len: c_int,
    pub fc_src_len: c_int,
    pub fc_ifindex: c_int,
    pub fc_flags: u32,
    pub fc_protocol: u32,
    pub /: *mut *mut u16 fc_type; / only 8 bits are used,
    pub 14: __unused :,
    pub fc_nh_id: u32,
    pub fc_dst: in6_addr,
    pub fc_src: in6_addr,
    pub fc_prefsrc: in6_addr,
    pub fc_gateway: in6_addr,
    pub fc_expires: c_ulong,
    pub fc_mx: *mut nlattr,
    pub fc_mx_len: c_int,
    pub fc_mp_len: c_int,
    pub fc_mp: *mut nlattr,
    pub fc_nlinfo: nl_info,
    pub fc_encap: *mut nlattr,
    pub fc_encap_type: u16,
    pub fc_is_fdb: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_node {
    pub parent: *mut fib6_node __rcu,
    pub left: *mut fib6_node __rcu,
    pub right: *mut fib6_node __rcu,

    pub subtree: *mut fib6_node __rcu,

    pub leaf: *mut fib6_info __rcu,
    pub /: *mut *mut __u16 fn_bit; / bit key,
    pub fn_flags: __u16,
    pub fn_sernum: c_int,
    pub rr_ptr: *mut fib6_info __rcu,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_gc_args {
    pub timeout: c_int,
    pub more: c_int,
}

//
// routing information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt6key {
    pub addr: in6_addr,
    pub plen: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt6_exception_bucket {
    pub chain: hlist_head,
    pub depth: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt6_exception {
    pub hlist: hlist_node,
    pub rt6i: *mut rt6_info,
    pub stamp: c_ulong,
    pub rcu: rcu_head,
}

pub const FIB6_EXCEPTION_BUCKET_SIZE_SHIFT: c_int = 10;

pub const FIB6_MAX_DEPTH: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_nh {
    pub nh_common: fib_nh_common,

    pub last_probe: c_ulong,

    pub rt6i_pcpu: *mut *mut rt6_info  __percpu,
    pub rt6i_exception_bucket: *mut rt6_exception_bucket __rcu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_info {
    pub fib6_table: *mut fib6_table,
    pub fib6_next: *mut fib6_info __rcu,
    pub fib6_node: *mut fib6_node __rcu,
// Multipath routes:
// siblings is a list of fib6_info that have the same metric/weight,
// destination, but not the same gateway. nsiblings is just a cache
// to speed up lookup.
//
    pub fib6_siblings: list_head,
    pub nh_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt6_info {
    pub dst: dst_entry,
    pub from: *mut fib6_info __rcu,
    pub sernum: c_int,
    pub rt6i_dst: rt6key,
    pub rt6i_src: rt6key,
    pub rt6i_gateway: in6_addr,
    pub rt6i_idev: *mut inet6_dev,
    pub rt6i_flags: u32,
// more non-fragment space at head required
    pub rt6i_nfheader_len: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_result {
    pub nh: *mut fib6_nh,
    pub f6i: *mut fib6_info,
    pub fib6_flags: u32,
    pub fib6_type: u8,
    pub rt6: *mut rt6_info,
}

// The callers should hold f6i->fib6_table->tb6_lock if a route has ever
// been added to a table before.
//
// The callers should hold f6i->fib6_table->tb6_lock if a route has ever
// been added to a table before.
//
extern "C" {
    pub fn time_after(_arg: jiffies, _arg: f6i->expires) -> return;
}
// Function to safely get fn->fn_sernum for passed in rt
// and store result in passed in cookie.
// Return true if we can get cookie safely
// Return false if not
//
// cookie = READ_ONCE(fn->fn_sernum);
// pairs with smp_wmb() in __fib6_update_sernum_upto_root()
// dst_release() accepts a NULL parameter.
// We rely on dst being first structure in struct rt6_info
//
extern "C" {
    pub fn fib6_info_destroy_rcu(head: *mut rcu_head);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &f6i->fib6_ref) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fib6_walk_state {

    FWS_S,

    FWS_L,
    FWS_R,
    FWS_C,
    FWS_U
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_walker {
    pub lh: list_head,
    pub node: *mut *mut fib6_node root,,
    pub leaf: *mut fib6_info,
    pub state: fib6_walk_state,
    pub skip: c_uint,
    pub count: c_uint,
    pub skip_in_node: c_uint,
    pub ): *mut *mut int (func)(struct fib6_walker,
    pub args: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt6_statistics {
    pub /: *mut *mut __u32 fib_nodes; / all fib6 nodes,
    pub /: *mut *mut __u32 fib_route_nodes; / intermediate nodes,
    pub /: *mut *mut __u32 fib_rt_entries; / rt entries in fib table,
    pub /: *mut *mut __u32 fib_rt_cache; / cached rt entries in exception table,
    pub /: *mut *mut __u32 fib_discarded_routes; / total number of routes delete,
// The following stat is not protected by any lock
    pub /: *mut *mut atomic_t fib_rt_alloc; / total number of routes alloced,
}

pub const RTN_TL_ROOT: c_uint = 0x0001;
pub const RTN_ROOT: c_uint = 0x0002		/* tree root node		*/;
pub const RTN_RTINFO: c_uint = 0x0004		/* node with valid routing info	*/;
//
// priority levels (or metrics)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_table {
    pub tb6_hlist: hlist_node,
    pub tb6_id: u32,
    pub tb6_lock: spinlock_t,
    pub tb6_root: fib6_node,
    pub tb6_peers: inet_peer_base,
    pub flags: c_uint,
    pub /: *mut *mut unsigned int fib_seq; / writes protected by rtnl_mutex,
    pub /: *mut *mut hlist_head tb6_gc_hlist; / GC candidates,

}

pub const FIB6_TABLE_MIN: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_entry_notifier_info {
    pub /: *mut *mut fib_notifier_info info; / must be first,
    pub rt: *mut fib6_info,
    pub nsiblings: c_uint,
}

//
// exported functions
//
// called with rcu lock held; can return error pointer
// caller needs to select path
//
// called with rcu lock held; caller needs to select path
// addr = from->fib6_prefsrc.addr;
// addr = in6addr_any;

extern "C" {
    pub fn fib6_nh_release(fib6_nh: *mut fib6_nh);
}
extern "C" {
    pub fn fib6_nh_release_dsts(fib6_nh: *mut fib6_nh);
}

extern "C" {
    pub fn call_fib6_entry_notifiers_replace(net: *mut net, rt: *mut fib6_info) -> c_int;
}

extern "C" {
    pub fn fib6_run_gc(expires: c_ulong, net: *mut net, force: bool);
}
extern "C" {
    pub fn fib6_gc_cleanup();
}
extern "C" {
    pub fn fib6_init() -> c_int;
}

// Add the route to the gc list if it is not already there
//
// The callers should hold f6i->fib6_table->tb6_lock.
//
// If fib6_node is null, the f6i is not in (or removed from) the
// table.
//
// There is a gap between finding the f6i from the table and
// calling this function without the protection of the tb6_lock.
// This check makes sure the f6i is not added to the gc list when
// it is not on the table.
//
// Remove the route from the gc list if it is on the list.
//
// The callers should hold f6i->fib6_table->tb6_lock.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_route_iter {
    pub p: seq_net_private,
    pub w: fib6_walker,
    pub skip: loff_t,
    pub tbl: *mut fib6_table,
    pub sernum: c_int,
}

extern "C" {
    pub fn fib6_notifier_init(net: *mut net) -> int __net_init;
}
extern "C" {
    pub fn fib6_notifier_exit(net: *mut net) -> void __net_exit;
}
extern "C" {
    pub fn fib6_tables_seq_read(net: *const net) -> c_uint;
}
extern "C" {
    pub fn fib6_update_sernum(net: *mut net, rt: *mut fib6_info);
}

extern "C" {
    pub fn fib6_update_sernum_upto_root(net: *mut net, rt: *mut fib6_info);
}

extern "C" {
    pub fn fib6_metric_set(f6i: *mut fib6_info, metric: c_int, val: u32);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__ipv6_route {
    pub meta): *mut *mut __bpf_md_ptr(struct bpf_iter_meta ,,
    pub rt): *mut *mut __bpf_md_ptr(struct fib6_info ,,
}

extern "C" {
    pub fn fib6_rules_init() -> c_int;
}
extern "C" {
    pub fn fib6_rules_cleanup();
}
extern "C" {
    pub fn fib6_rule_default(rule: *const fib_rule) -> bool;
}
extern "C" {
    pub fn fib6_rules_seq_read(net: *const net) -> c_uint;
}

