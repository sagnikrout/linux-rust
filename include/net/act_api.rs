//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/act_api.h
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
// Public action API for classifiers/qdiscs
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcf_idrinfo {
    pub lock: mutex,
    pub action_idr: idr,
    pub net: *mut net,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_action {
    pub ops: *const tc_action_ops,
    pub /: *mut *mut __u32 type; / for backward compat(TCA_OLD_COMPAT),
    pub idrinfo: *mut tcf_idrinfo,
    pub tcfa_index: u32,
    pub tcfa_refcnt: refcount_t,
    pub tcfa_bindcnt: core::sync::atomic::AtomicI32,
    pub tcfa_action: c_int,
    pub tcfa_tm: tcf_t,
    pub tcfa_bstats: gnet_stats_basic_sync,
    pub tcfa_bstats_hw: gnet_stats_basic_sync,
    pub tcfa_drops: core::sync::atomic::AtomicI32,
    pub tcfa_overlimits: core::sync::atomic::AtomicI32,
    pub tcfa_rate_est: *mut net_rate_estimator __rcu,
    pub tcfa_lock: spinlock_t,
    pub cpu_bstats: *mut gnet_stats_basic_sync __percpu,
    pub cpu_bstats_hw: *mut gnet_stats_basic_sync __percpu,
    pub cpu_qstats: *mut gnet_stats_queue __percpu,
    pub user_cookie: *mut tc_cookie __rcu,
    pub goto_chain: *mut tcf_chain __rcu,
    pub tcfa_flags: u32,
    pub tcfa_rcu: rcu_head,
    pub hw_stats: u8,
    pub used_hw_stats: u8,
    pub used_hw_stats_valid: bool,
    pub in_hw_count: u32,
}

// Reserve 16 bits for user-space. See TCA_ACT_FLAGS_NO_PERCPU_STATS.
pub const TCA_ACT_FLAGS_USER_BITS: c_int = 16;
pub const TCA_ACT_FLAGS_USER_MASK: c_uint = 0xffff;

// Update lastuse only if needed, to avoid dirtying a cache line.
// We use a temp variable to avoid fetching jiffies twice.
//
extern "C" {
    pub fn void(priv: *mut *mut tc_action_priv_destructor)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_action_ops {
    pub head: list_head,
    pub kind: [c_char; IFNAMSIZ],
    pub /: *mut *mut tca_id id; / identifier should match kind,
    pub net_id: c_uint,
    pub size: usize,
    pub owner: *mut module,
    pub lock*/: *mut *mut *mut tcf_result ); / called under RCU BH,
    pub int): *mut *mut *mut *mut int (dump)(struct sk_buff , struct tc_action , int,,
    pub ): *mut *mut void (cleanup)(struct tc_action,
    pub index): *mut *mut *mut *mut *mut int (lookup)(struct net net, struct tc_action a, u32,
    pub extack): *mut u32 flags, struct netlink_ext_ack,
    pub ): *mut netlink_ext_ack,
    pub bool): *mut *mut *mut void (stats_update)(struct tc_action , u64, u64, u64, u64,,
    pub act): *const *const size_t (get_fill_size)(struct tc_action,
    pub destructor): *mut tc_action_priv_destructor,
    pub destructor): *mut tc_action_priv_destructor,
    pub extack): *mut netlink_ext_ack,
}

pub const ACT_P_BOUND: c_int = 0;
pub const ACT_P_CREATED: c_int = 1;
pub const ACT_P_DELETED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_action_net {
    pub idrinfo: *mut tcf_idrinfo,
    pub ops: *const tc_action_ops,
}

extern "C" {
    pub fn tcf_idr_search(tn: *mut tc_action_net, a: *mut tc_action, index: u32) -> c_int;
}
extern "C" {
    pub fn tcf_idr_insert_many(actions[]: *mut tc_action, init_res[]: c_int);
}
extern "C" {
    pub fn tcf_idr_cleanup(tn: *mut tc_action_net, index: u32);
}
extern "C" {
    pub fn tcf_idr_release(a: *mut tc_action, bind: bool) -> c_int;
}
extern "C" {
    pub fn tcf_register_action(a: *mut tc_action_ops, ops: *mut pernet_operations) -> c_int;
}

extern "C" {
    pub fn tcf_action_destroy(actions[]: *mut tc_action, bind: c_int) -> c_int;
}
extern "C" {
    pub fn tcf_action_dump_old(skb: *mut sk_buff, a: *mut tc_action, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tcf_action_copy_stats(: *mut sk_buff, : *mut tc_action, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn tcf_action_update_hw_stats(action: *mut tc_action) -> c_int;
}
// Range check for a control action supplied by user space.
//
// This is the same test tcf_action_check_ctrlact() applies to the primary
// control action, factored out for the *fallback* control actions
// (act_gact's TCA_GACT_PROB.paction and act_police's TCA_POLICE_RESULT),
// which must not reach tcf_action_check_ctrlact() because they have no
// goto_chain to allocate.  Without it, user space can store kernel-internal
// verdicts such as TC_ACT_CONSUMED, which is TC_ACT_VALUE_MAX + 1 and is
// deliberately not part of the UAPI value range.
//

extern "C" {
    pub fn tcf_dev_queue_xmit(skb: *mut sk_buff, skb): *mut *mut int (xmit)(struct sk_buff) -> c_int;
}

