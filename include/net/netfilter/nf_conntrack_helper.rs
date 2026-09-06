//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_helper.h
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
// connection tracking helpers.
//
// 16 Dec 2003: Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
// - generalize L3 protocol dependent part.
//
// Derived from include/linux/netfiter_ipv4/ip_conntrack_helper.h
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_ct_helper_flags {
    NF_CT_HELPER_F_USERSPACE	= (1 << 0),
    NF_CT_HELPER_F_CONFIGURED	= (1 << 1),
}

pub const NF_CT_HELPER_NAME_LEN: c_int = 16;
// Must be kept in sync with the classes defined by helpers
pub const NF_CT_MAX_EXPECT_CLASSES: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_helper {
    pub /: *mut *mut hlist_node hnode; / Internal use.,
    pub rcu: rcu_head,
    pub /: *mut *mut char name[NF_CT_HELPER_NAME_LEN]; / name of the module,
    pub /: *mut *mut *mut module me; / pointer to self,
    pub expect_policy: [nf_conntrack_expect_policy; NF_CT_MAX_EXPECT_CLASSES],
    pub ct_refcnt: refcount_t,
    pub /: *mut *mut *mut u8 nfproto; / NFPROTO_, can be NFPROTO_UNSPEC,
    pub /: *mut *mut u8 l4proto; / IPPROTO_UDP/TCP,
// Function to call when data passes; return verdict
    pub conntrackinfo): ip_conntrack_info,
    pub ct): *mut *mut void (destroy)(struct nf_conn,
    pub ct): *mut *mut *mut int (from_nlattr)(struct nlattr attr, struct nf_conn,
    pub ct): *const *const *const int (to_nlattr)(struct sk_buff skb, struct nf_conn,
    pub expect_class_max: c_uint,
    pub flags: c_uint,
// For user-space helpers:
    pub queue_num: c_uint,
// length of userspace private data stored in nf_conn_help->data
    pub data_len: u16,
// name of NAT helper module
    pub nat_mod_name: [c_char; NF_CT_HELPER_NAME_LEN],
}

// nf_conn feature for connections that have a helper
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_help {
// Helper. if any
    pub helper: *mut nf_conntrack_helper __rcu,
    pub expectations: hlist_head,
// Current number of expected connections
    pub expecting: [u8; NF_CT_MAX_EXPECT_CLASSES],
// private helper information.
    pub __aligned(8): char data[32],
}

extern "C" {
    pub fn nf_conntrack_helper_put(helper: *mut nf_conntrack_helper);
}
extern "C" {
    pub fn nf_conntrack_helper_register(: *mut nf_conntrack_helper, : *mut nf_conntrack_helper) -> c_int;
}
extern "C" {
    pub fn __nf_conntrack_helper_register(: *mut nf_conntrack_helper) -> c_int;
}
extern "C" {
    pub fn nf_conntrack_helper_unregister(: *mut nf_conntrack_helper);
}
extern "C" {
    pub fn nf_conntrack_helper_release(: *mut nf_conntrack_helper);
}

extern "C" {
    pub fn nf_ct_helper_destroy(ct: *mut nf_conn);
}
extern "C" {
    pub fn nf_ct_ext_find(_arg: ct, _arg: NF_CT_EXT_HELPER) -> return;
}
extern "C" {
    pub fn nf_conntrack_helper_init() -> c_int;
}
extern "C" {
    pub fn nf_conntrack_helper_fini();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_helper_expectfn {
    pub head: list_head,
    pub name: *const c_char,
    pub exp): *mut *mut *mut void (expectfn)(struct nf_conn ct, struct nf_conntrack_expect,
}

extern "C" {
    pub fn nf_ct_helper_expectfn_register(n: *mut nf_ct_helper_expectfn);
}
extern "C" {
    pub fn nf_ct_helper_expectfn_unregister(n: *mut nf_ct_helper_expectfn);
}
extern "C" {
    pub fn nf_ct_helper_expectfn_destroy(n: *const nf_ct_helper_expectfn);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_nat_helper {
    pub list: list_head,
    pub /: *mut *mut char mod_name[NF_CT_HELPER_NAME_LEN]; / module name,
    pub /: *mut *mut *mut module module; / pointer to self,
}

extern "C" {
    pub fn nf_nat_helper_register(nat: *mut nf_conntrack_nat_helper);
}
extern "C" {
    pub fn nf_nat_helper_unregister(nat: *mut nf_conntrack_nat_helper);
}
extern "C" {
    pub fn nf_nat_helper_put(helper: *mut nf_conntrack_helper);
}
