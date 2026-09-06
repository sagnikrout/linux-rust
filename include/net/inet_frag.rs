//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inet_frag.h
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

// Per netns frag queues directory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fqdir {
// sysctls
    pub high_thresh: c_long,
    pub low_thresh: c_long,
    pub timeout: c_int,
    pub max_dist: c_int,
    pub f: *mut inet_frags,
    pub net: *mut net,
    pub dead: bool,
    pub ____cacheline_aligned_in_smp: rhashtable rhashtable,
// Keep atomic mem on separate cachelines in structs that include it
    pub ____cacheline_aligned_in_smp: atomic_long_t mem,
    pub destroy_work: work_struct,
    pub free_list: llist_node,
}

//
// enum: fragment queue flags
//
// @INET_FRAG_FIRST_IN: first fragment has arrived
// @INET_FRAG_LAST_IN: final fragment has arrived
// @INET_FRAG_COMPLETE: frag queue has been processed and is due for destruction
// @INET_FRAG_HASH_DEAD: inet_frag_kill() has not removed fq from rhashtable
// @INET_FRAG_DROP: if skbs must be dropped (instead of being consumed)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_v4_compare_key {
    pub saddr: __be32,
    pub daddr: __be32,
    pub user: u32,
    pub vif: u32,
    pub id: __be16,
    pub protocol: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_v6_compare_key {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
    pub user: u32,
    pub id: __be32,
    pub iif: u32,
}

//
// struct inet_frag_queue - fragment queue
//
// @node: rhash node
// @key: keys identifying this frag.
// @timer: queue expiration timer
// @lock: spinlock protecting this frag
// @refcnt: reference count of the queue
// @rb_fragments: received fragments rb-tree root
// @fragments_tail: received fragments tail
// @last_run_head: the head of the last "run". see ip_fragment.c
// @stamp: timestamp of the last received fragment
// @len: total length of the original datagram
// @meat: length of received fragments so far
// @tstamp_type: stamp has a mono delivery time (EDT)
// @flags: fragment queue flags
// @max_size: maximum received fragment size
// @fqdir: pointer to struct fqdir
// @rcu: rcu head for freeing deferall
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_frag_queue {
    pub node: rhash_head,
    pub v4: frag_v4_compare_key,
    pub v6: frag_v6_compare_key,
    pub key: },
    pub timer: timer_list,
    pub lock: spinlock_t,
    pub refcnt: refcount_t,
    pub rb_fragments: rb_root,
    pub fragments_tail: *mut sk_buff,
    pub last_run_head: *mut sk_buff,
    pub stamp: ktime_t,
    pub len: c_int,
    pub meat: c_int,
    pub tstamp_type: u8,
    pub flags: __u8,
    pub max_size: u16,
    pub fqdir: *mut fqdir,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_frags {
    pub qsize: c_uint,
    pub arg): *const c_void,
    pub ): *mut *mut void (destructor)(struct inet_frag_queue,
    pub t): *mut *mut void (frag_expire)(struct timer_list,
    pub frags_cachep: *mut kmem_cache,
    pub frags_cache_name: *const c_char,
    pub rhash_params: rhashtable_params,
    pub refcnt: refcount_t,
    pub completion: completion,
}

extern "C" {
    pub fn inet_frags_init(: *mut inet_frags) -> c_int;
}
extern "C" {
    pub fn inet_frags_fini(: *mut inet_frags);
}
extern "C" {
    pub fn fqdir_init(fqdirp: *mut fqdir, f: *mut inet_frags, net: *mut net) -> c_int;
}
extern "C" {
    pub fn fqdir_pre_exit(fqdir: *mut fqdir);
}
extern "C" {
    pub fn fqdir_exit(fqdir: *mut fqdir);
}
extern "C" {
    pub fn inet_frag_kill(q: *mut inet_frag_queue, refs: *mut c_int);
}
extern "C" {
    pub fn inet_frag_destroy(q: *mut inet_frag_queue);
}
// Memory Tracking Functions.
extern "C" {
    pub fn atomic_long_read(_arg: &fqdir->mem) -> return;
}
// RFC 3168 support :
// We want to check ECN values of all fragments, do detect invalid combinations.
// In ipq->ecn, we store the OR value of each ip4_frag_ecn() fragment value.
//
pub const IPFRAG_ECN_NOT_ECT: c_uint = 0x01 /* one frag had ECN_NOT_ECT */;
pub const IPFRAG_ECN_ECT_1: c_uint = 0x02 /* one frag had ECN_ECT_1 */;
pub const IPFRAG_ECN_ECT_0: c_uint = 0x04 /* one frag had ECN_ECT_0 */;
pub const IPFRAG_ECN_CE: c_uint = 0x08 /* one frag had ECN_CE */;
// Return values of inet_frag_queue_insert()
pub const IPFRAG_OK: c_int = 0;
pub const IPFRAG_DUP: c_int = 1;
pub const IPFRAG_OVERLAP: c_int = 2;
