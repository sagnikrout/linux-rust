//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack.h
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
// Connection state tracking for netfilter.  This is separated from,
// but required by, the (future) NAT layer; it can also be used by an iptables
// extension.
//
// 16 Dec 2003: Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
// - generalize L3 protocol dependent part.
//
// Derived from include/linux/netfiter_ipv4/ip_conntrack.h
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_udp {
    pub stream_ts: c_ulong,
}

// per conntrack: protocol private data
#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_proto {
// insert conntrack proto private data here
    pub sctp: ip_ct_sctp,
    pub tcp: ip_ct_tcp,
    pub udp: nf_ct_udp,
    pub gre: nf_ct_gre,
    pub tmpl_padto: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_expect_proto {
// insert expect proto private data here
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_net_ecache {
    pub dwork: delayed_work,
    pub dying_lock: spinlock_t,
    pub dying_list: hlist_nulls_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_net {
// only used when new connection is allocated:
    pub count: core::sync::atomic::AtomicI32,
    pub expect_count: c_uint,
// only used from work queues, configuration plane, and so on:
    pub users4: c_uint,
    pub users6: c_uint,
    pub users_bridge: c_uint,

    pub sysctl_header: *mut ctl_table_header,

    pub ecache: nf_conntrack_net_ecache,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
// Usage count in here is 1 for hash table, 1 per skb,
// plus 1 for any connection(s) we are `master' for
//
// Hint, SKB address this struct and refcnt via skb->_nfct and
// helpers nf_conntrack_get() and nf_conntrack_put().
// Helper nf_ct_put() equals nf_conntrack_put() by dec refcnt,
// except that the latter uses internal indirection and does not
// result in a conntrack module dependency.
// beware nf_ct_get() is different and don't inc refcnt.
//
    pub ct_general: nf_conntrack,
    pub lock: spinlock_t,
// jiffies32 when this ct is considered dead
    pub timeout: u32,

    pub zone: nf_conntrack_zone,

// XXX should I move this to the tail ? - Y.K
// These are my tuples; original and reply
    pub tuplehash: [nf_conntrack_tuple_hash; IP_CT_DIR_MAX],
// Have we seen traffic both ways yet? (bitset)
    pub status: c_ulong,
    pub ct_net: possible_net_t,

    pub nat_bysource: hlist_node,

// all members below initialized via memset
    pub __nfct_init_offset: { },
// If we were expected by an expectation, this will be it
    pub master: *mut nf_conn,

    pub mark: u_int32_t,

    pub secmark: u_int32_t,

// Extensions
    pub ext: *mut nf_ct_ext,
// Storage reserved for other modules, must be the last member
    pub proto: nf_conntrack_proto,
}

extern "C" {
    pub fn container_of(_arg: nfct, nf_conn: struct, _arg: ct_general) -> return;
}

// get master conntrack via master expectation

extern "C" {
    pub fn read_pnet(_arg: &ct->ct_net) -> return;
}
// Is this tuple taken? (ignoring any belonging to the given
// Return conntrack_info and tuple hash for given skb.
// ctinfo = nfct & NFCT_INFOMASK;
extern "C" {
    pub fn nf_ct_destroy(nfct: *mut nf_conntrack);
}
extern "C" {
    pub fn nf_conntrack_tcp_set_closing(ct: *mut nf_conn);
}
// decrement reference count on a conntrack
// load module; enable/disable conntrack in this namespace
extern "C" {
    pub fn nf_ct_netns_get(net: *mut net, nfproto: u8) -> c_int;
}
extern "C" {
    pub fn nf_ct_netns_put(net: *mut net, nfproto: u8);
}
//
// Allocate a hashtable of hlist_head (if nulls == 0),
// or hlist_nulls_head (if nulls == 1)
//
extern "C" {
    pub fn nf_conntrack_hash_check_insert(ct: *mut nf_conn) -> c_int;
}
extern "C" {
    pub fn nf_ct_delete(ct: *mut nf_conn, pid: u32, report: c_int) -> bool;
}
// Refresh conntrack for this many jiffies and do accounting
// Refresh conntrack for this many jiffies
// kill conntrack and do accounting
// kill conntrack without accounting
extern "C" {
    pub fn nf_ct_delete(_arg: ct, _arg: 0, _arg: 0) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_iter_data {
    pub net: *mut net,
    pub data: *mut c_void,
    pub portid: u32,
    pub report: c_int,
}

// Iterate over all conntracks: if iter returns true, it's deleted.
// also set unconfirmed conntracks as dying. Only use in module exit path.
extern "C" {
    pub fn nf_conntrack_free(ct: *mut nf_conn);
}
extern "C" {
    pub fn test_bit(_arg: IPS_TEMPLATE_BIT, _arg: &ct->status) -> return;
}
// It's confirmed if it is, or has been in the hash table.
extern "C" {
    pub fn test_bit(_arg: IPS_CONFIRMED_BIT, _arg: &ct->status) -> return;
}
extern "C" {
    pub fn test_bit(_arg: IPS_DYING_BIT, _arg: &ct->status) -> return;
}
// Packet is received from loopback
// Must be unconfirmed, so not in hash table yet

// jiffies until ct expires, 0 if already expired
extern "C" {
    pub fn max(_arg: timeout, _arg: 0) -> return;
}
// use after obtaining a reference count
// load ct->timeout after is_confirmed() test.
// Pairs with __nf_conntrack_confirm() which:
// 1. Increases ct->timeout value
// 2. Inserts ct into rcu hlist
// 3. Sets the confirmed bit
// 4. Unlocks the hlist lock
//
extern "C" {
    pub fn nf_ct_is_expired(!nf_ct_is_dying(ct: ct) &&) -> return;
}

extern "C" {
    pub fn nf_conntrack_set_hashsize(val: *const c_char, kp: *const kernel_param) -> c_int;
}
extern "C" {
    pub fn nf_conntrack_hash_resize(hashsize: c_uint) -> c_int;
}
// must be called with rcu read lock held
// hash = hptr;
// hsize = hsz;
extern "C" {
    pub fn nf_ct_tmpl_free(tmpl: *mut nf_conn);
}
extern "C" {
    pub fn nf_ct_get_id(ct: *const nf_conn) -> u32;
}
extern "C" {
    pub fn nf_conntrack_count(net: *const net) -> u32;
}
extern "C" {
    pub fn net_generic(_arg: net, _arg: nf_conntrack_net_id) -> return;
}
extern "C" {
    pub fn nf_ct_skb_network_trim(skb: *mut sk_buff, family: c_int) -> c_int;
}

