//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter.h
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

extern "C" {
    pub fn netfilter_init() -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_hook_state {
    pub hook: u8,
    pub pf: u8,
    pub in: *mut net_device,
    pub out: *mut net_device,
    pub sk: *mut sock,
    pub net: *mut net,
    pub ): *mut *mut *mut *mut int (okfn)(struct net , struct sock , struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_hook_ops_type {
    NF_HOOK_OP_UNDEFINED,
    NF_HOOK_OP_NF_TABLES,
    NF_HOOK_OP_BPF,
    NF_HOOK_OP_NFT_FT,
    NF_HOOK_OP_NAT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_hook_ops {
    pub list: list_head,
    pub rcu: rcu_head,
// User fills in from here down.
    pub hook: *mut nf_hookfn,
    pub dev: *mut net_device,
    pub priv: *mut c_void,
    pub pf: u8,
    pub hook_ops_type:8: nf_hook_ops_type,
    pub hooknum: c_uint,
// Hooks are ordered in ascending priority.
    pub priority: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_hook_entry {
    pub hook: *mut nf_hookfn,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_hook_entries_rcu_head {
    pub head: rcu_head,
    pub allocation: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_hook_entries {
    pub num_hook_entries: u16,
// padding
    pub hooks: [nf_hook_entry; ],
// trailer: pointers to original orig_ops of each hook,
// followed by rcu_head and scratch space used for freeing
// the structure via call_rcu.
//
// This is not part of struct nf_hook_entry since its only
// needed in slow path (hook register/unregister):
// const struct nf_hook_ops     *orig_ops[]
//
// For the same reason, we store this at end -- its
// only needed when a hook is deleted, not during
// packet path processing:
// struct nf_hook_entries_rcu_head     head
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_lookup_hook_priv {
    pub entries: *mut nf_hook_entries __rcu,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_sockopt_ops {
    pub list: list_head,
    pub pf: u_int8_t,
// Non-inclusive ranges: use 0/0/NULL to never get called.
    pub set_optmin: c_int,
    pub set_optmax: c_int,
    pub len): c_uint,
    pub get_optmin: c_int,
    pub get_optmax: c_int,
    pub len): *mut *mut *mut *mut int (get)(struct sock sk, int optval, void __user user, int,
// Use the module struct to lock set/get code in place
    pub owner: *mut module,
}

// Function to register/unregister hook points.
extern "C" {
    pub fn nf_register_net_hook(net: *mut net, ops: *const nf_hook_ops) -> c_int;
}
extern "C" {
    pub fn nf_unregister_net_hook(net: *mut net, ops: *const nf_hook_ops);
}
// Functions to register get/setsockopt ranges (non-inclusive).  You
extern "C" {
    pub fn nf_register_sockopt(reg: *mut nf_sockopt_ops) -> c_int;
}
extern "C" {
    pub fn nf_unregister_sockopt(reg: *mut nf_sockopt_ops);
}

//
// nf_hook - call a netfilter hook
//
// Returns 1 if the hook has allowed the packet to pass.  The function
// okfn must be invoked by the caller in this case.  Any other return
// value indicates the packet has been consumed by the hook.
//

// Activate hook; either okfn or kfree_skb called, unless a hook
//
// RR:
//

// Call setsockopt()

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_hook {
    pub attr): *const nlattr,
    pub fl): *mut *mut *mut void (decode_session)(struct sk_buff skb, struct flowi,
    pub ct): *mut *mut void (remove_nat_bysrc)(struct nf_conn,
}

extern "C" {
    pub fn okfn(_arg: net, _arg: sk, _arg: skb) -> return;
}
extern "C" {
    pub fn okfn(_arg: net, _arg: sk, _arg: skb) -> return;
}
// nothing to do

extern "C" {
    pub fn nf_ct_attach(: *mut sk_buff, : *const sk_buff);
}
extern "C" {
    pub fn nf_ct_set_closing(nfct: *mut nf_conntrack);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_hook {
    pub skb): *mut *mut *mut int (update)(struct net net, struct sk_buff,
    pub ): *mut *mut void (destroy)(struct nf_conntrack,
    pub ): *const sk_buff,
    pub skb): *const *const *const void (attach)(struct sk_buff nskb, struct sk_buff,
    pub nfct): *mut *mut void (set_closing)(struct nf_conntrack,
    pub skb): *mut *mut int (confirm)(struct sk_buff,
    pub nfct): *const *const u32 (get_id)(struct nf_conntrack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfnl_ct_hook {
    pub ct): *const *const size_t (build_size)(struct nf_conn,
    pub ct_info_attr): u_int16_t ct_attr, u_int16_t,
    pub ct): *const *const *const int (parse)(struct nlattr attr, struct nf_conn,
    pub report): u32 portid, u32,
    pub off): ip_conntrack_info ctinfo, s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_defrag_hook {
    pub owner: *mut module,
    pub net): *mut *mut int (enable)(struct net,
    pub net): *mut *mut void (disable)(struct net,
}

//
// Contains bitmask of ctnetlink event subscribers, if any.
// Can't be pernet due to NETLINK_LISTEN_ALL_NSID setsockopt flag.
//
