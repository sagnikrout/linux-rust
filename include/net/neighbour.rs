//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/neighbour.h
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
// Generic neighbour manipulation
//
// Authors:
// Pedro Roque		<roque@di.fc.ul.pt>
// Alexey Kuznetsov	<kuznet@ms2.inr.ac.ru>
//
// Changes:
//
// Harald Welte:		<laforge@gnumonks.org>
// - Add neighbour cache statistics like rtstat
//

//
// NUD stands for "neighbor unreachability detection"
//

// Following are used as a second way to access one of the above
// Following are used by "default" only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct neigh_parms {
    pub net: possible_net_t,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub list: list_head,
    pub ): *mut *mut int (neigh_setup)(struct neighbour,
    pub tbl: *mut neigh_table,
    pub sysctl_table: *mut c_void,
    pub dead: c_int,
    pub refcnt: refcount_t,
    pub rcu_head: rcu_head,
    pub reachable_time: c_int,
    pub qlen: u32,
    pub data: [c_int; NEIGH_VAR_DATA_MAX],
    pub NEIGH_VAR_DATA_MAX): DECLARE_BITMAP(data_state,,
}

// In ndo_neigh_setup, NEIGH_VAR_INIT should be used.
// In other cases, NEIGH_VAR_SET should be used.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct neigh_statistics {
    pub /: *mut *mut unsigned long allocs; / number of allocated neighs,
    pub /: *mut *mut unsigned long destroys; / number of destroyed neighs,
    pub /: *mut *mut unsigned long hash_grows; / number of hash resizes,
    pub /: *mut *mut unsigned long res_failed; / number of failed resolutions,
    pub /: *mut *mut unsigned long lookups; / number of lookups,
    pub /: *mut *mut unsigned long hits; / number of hits (among lookups),
    pub /: *mut *mut unsigned long rcv_probes_mcast; / number of received mcast ipv6,
    pub /: *mut *mut unsigned long rcv_probes_ucast; / number of received ucast ipv6,
    pub /: *mut *mut unsigned long periodic_gc_runs; / number of periodic GC runs,
    pub /: *mut *mut unsigned long forced_gc_runs; / number of forced GC runs,
    pub /: *mut *mut unsigned long unres_discards; / number of unresolved drops,
    pub /: *mut *mut unsigned long table_fulls; / times even gc couldn't help,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct neighbour {
    pub hash: hlist_node,
    pub dev_list: hlist_node,
    pub tbl: *mut neigh_table,
    pub parms: *mut neigh_parms,
    pub confirmed: c_ulong,
    pub updated: c_ulong,
    pub lock: rwlock_t,
    pub refcnt: refcount_t,
    pub arp_queue_len_bytes: c_uint,
    pub arp_queue: sk_buff_head,
    pub timer: timer_list,
    pub used: c_ulong,
    pub probes: core::sync::atomic::AtomicI32,
    pub nud_state: u8,
    pub type: u8,
    pub dead: u8,
    pub protocol: u8,
    pub flags: u32,
    pub ha_lock: seqlock_t,
    pub __aligned(8): unsigned char ha[ALIGN(MAX_ADDR_LEN, sizeof(unsigned long))],
    pub hh: hh_cache,
    pub ): *mut *mut *mut int (output)(struct neighbour , struct sk_buff,
    pub ops: *const neigh_ops,
    pub gc_list: list_head,
    pub managed_list: list_head,
    pub rcu: rcu_head,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub primary_key: [u8; ],
    pub __randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct neigh_ops {
    pub family: c_int,
    pub ): *mut *mut *mut void (solicit)(struct neighbour , struct sk_buff,
    pub ): *mut *mut *mut void (error_report)(struct neighbour , struct sk_buff,
    pub ): *mut *mut *mut int (output)(struct neighbour , struct sk_buff,
    pub ): *mut *mut *mut int (connected_output)(struct neighbour , struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pneigh_entry {
    pub next: *mut pneigh_entry __rcu,
    pub net: possible_net_t,
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub free_node: list_head,
    pub rcu: rcu_head,
}

//
// neighbour table manipulation
//
pub const NEIGH_NUM_HASH_RND: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct neigh_hash_table {
    pub hash_heads: *mut hlist_head,
    pub hash_shift: c_uint,
    pub hash_rnd: [__u32; NEIGH_NUM_HASH_RND],
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct neigh_table {
    pub family: c_int,
    pub entry_size: c_uint,
    pub key_len: c_uint,
    pub protocol: __be16,
    pub hash_rnd): *mut __u32,
    pub pkey): *const *const *const bool (key_eq)(struct neighbour , void,
    pub ): *mut *mut int (constructor)(struct neighbour,
    pub ): *mut *mut int (pconstructor)(struct pneigh_entry,
    pub ): *mut *mut void (pdestructor)(struct pneigh_entry,
    pub skb): *mut *mut void (proxy_redo)(struct sk_buff,
    pub pkey): *const *const int (is_multicast)(void,
    pub extack): *mut netlink_ext_ack,
    pub id: *mut c_char,
    pub parms: neigh_parms,
    pub parms_list: list_head,
    pub gc_interval: c_int,
    pub gc_thresh1: c_int,
    pub gc_thresh2: c_int,
    pub gc_thresh3: c_int,
    pub last_flush: c_ulong,
    pub gc_work: delayed_work,
    pub managed_work: delayed_work,
    pub proxy_timer: timer_list,
    pub proxy_queue: sk_buff_head,
    pub entries: core::sync::atomic::AtomicI32,
    pub gc_entries: core::sync::atomic::AtomicI32,
    pub gc_list: list_head,
    pub managed_list: list_head,
    pub lock: spinlock_t,
    pub last_rand: c_ulong,
    pub stats: *mut neigh_statistics __percpu,
    pub nht: *mut neigh_hash_table __rcu,
    pub phash_lock: mutex,
    pub phash_buckets: *mut pneigh_entry __rcu,
}

// flags for neigh_update()

// In-kernel representation for NDA_FLAGS_EXT flags:
pub const NTF_OLD_MASK: c_uint = 0xff;
pub const NTF_EXT_SHIFT: c_int = 8;

extern "C" {
    pub fn ___neigh_lookup_noref(_arg: tbl, _arg: tbl->key_eq, _arg: tbl->hash, _arg: pkey, _arg: dev) -> return;
}
// avoid dirtying neighbour
extern "C" {
    pub fn neigh_table_init(index: c_int, tbl: *mut neigh_table);
}
extern "C" {
    pub fn neigh_table_clear(index: c_int, tbl: *mut neigh_table) -> c_int;
}
extern "C" {
    pub fn __neigh_create(_arg: tbl, _arg: pkey, _arg: dev, _arg: true) -> return;
}
extern "C" {
    pub fn neigh_destroy(neigh: *mut neighbour);
}
extern "C" {
    pub fn __neigh_set_probe_once(neigh: *mut neighbour);
}
extern "C" {
    pub fn neigh_remove_one(ndel: *mut neighbour) -> bool;
}
extern "C" {
    pub fn neigh_changeaddr(tbl: *mut neigh_table, dev: *mut net_device);
}
extern "C" {
    pub fn neigh_ifdown(tbl: *mut neigh_table, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn neigh_carrier_down(tbl: *mut neigh_table, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn neigh_resolve_output(neigh: *mut neighbour, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn neigh_connected_output(neigh: *mut neighbour, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn neigh_direct_output(neigh: *mut neighbour, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn neigh_parms_release(tbl: *mut neigh_table, parms: *mut neigh_parms);
}
extern "C" {
    pub fn read_pnet(_arg: &parms->net) -> return;
}
extern "C" {
    pub fn neigh_rand_reach_time(base: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn read_pnet(_arg: &pneigh->net) -> return;
}
extern "C" {
    pub fn neigh_app_ns(n: *mut neighbour);
}
extern "C" {
    pub fn neigh_xmit(fam: c_int, : *mut net_device, : *const c_void, : *mut sk_buff) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct neigh_seq_state {
    pub p: seq_net_private,
    pub tbl: *mut neigh_table,
    pub nht: *mut neigh_hash_table,
    pub pos): *mut *mut neighbour n, loff_t,
    pub bucket: c_uint,
    pub flags: c_uint,
pub const NEIGH_SEQ_NEIGH_ONLY: c_uint = 0x00000001;
pub const NEIGH_SEQ_IS_PNEIGH: c_uint = 0x00000002;
pub const NEIGH_SEQ_SKIP_NOARP: c_uint = 0x00000004;
}

extern "C" {
    pub fn neigh_seq_stop(: *mut seq_file, : *mut c_void);
}
extern "C" {
    pub fn neigh_sysctl_unregister(p: *mut neigh_parms);
}
//
// Neighbour references
//

extern "C" {
    pub fn __neigh_event_send(_arg: neigh, _arg: skb, _arg: immediate_ok) -> return;
}
extern "C" {
    pub fn neigh_event_send_probe(_arg: neigh, _arg: skb, _arg: true) -> return;
}

// skb_push() would proceed silently if we have room for
// the unaligned size but not for the aligned size:
// check headroom explicitly.
//
// this is inlined by gcc
extern "C" {
    pub fn dev_queue_xmit(_arg: skb) -> return;
}
// n->nud_state and hh->hh_len could be changed under us.
// neigh_hh_output() is taking care of the race later.
//
extern "C" {
    pub fn neigh_hh_output(_arg: hh, _arg: skb) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: n->output)(n, _arg: skb) -> return;
}
extern "C" {
    pub fn neigh_create(_arg: tbl, _arg: pkey, _arg: dev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct neighbour_cb {
    pub sched_next: c_ulong,
    pub flags: c_uint,
}

pub const LOCALLY_ENQUEUED: c_uint = 0x1;

// notify = 1;
