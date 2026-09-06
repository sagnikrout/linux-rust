//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_expect.h
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
// connection tracking expectations.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect {
// Conntrack expectation list member
    pub lnode: hlist_node,
// Hash member
    pub hnode: hlist_node,
// Network namespace
    pub net: possible_net_t,
// We expect this tuple, with the following mask
    pub master_tuple: nf_conntrack_tuple,
    pub tuple: nf_conntrack_tuple,
    pub mask: nf_conntrack_tuple_mask,

    pub zone: nf_conntrack_zone,

// Usage count.
    pub use: refcount_t,
// Flags
    pub flags: c_uint,
// Expectation class
    pub class: c_uint,
// Event filter mask
    pub event_mask: u16,
// Function to call after setup and insertion
    pub this): *mut nf_conntrack_expect,
// Helper that created this expectation
    pub helper: *mut nf_conntrack_helper __rcu,
// Helper to assign to new connection
    pub assign_helper: *mut nf_conntrack_helper __rcu,
// The conntrack of the master connection
    pub master: *mut nf_conn,
// jiffies32 when this expectation expires
    pub timeout: u32,

    pub saved_addr: nf_inet_addr,
// This is the original per-proto part, used to map the
// expected connection the way the recipient expects.
    pub saved_proto: nf_conntrack_man_proto,
// Direction relative to the master connection.
    pub dir: ip_conntrack_dir,

    pub rcu: rcu_head,
}

extern "C" {
    pub fn read_pnet(_arg: &exp->net) -> return;
}

pub const NF_CT_EXP_POLICY_NAME_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect_policy {
    pub max_expected: c_uint,
    pub timeout: c_uint,
    pub name: [c_char; NF_CT_EXP_POLICY_NAME_LEN],
}

pub const NF_CT_EXPECT_CLASS_DEFAULT: c_int = 0;
pub const NF_CT_EXPECT_MAX_CNT: c_int = 255;
// Allow to reuse expectations with the same tuples from different master
// conntracks.
//
pub const NF_CT_EXP_F_SKIP_MASTER: c_uint = 0x1;
extern "C" {
    pub fn nf_conntrack_expect_pernet_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn nf_conntrack_expect_pernet_fini(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_expect_init() -> c_int;
}
extern "C" {
    pub fn nf_conntrack_expect_fini();
}
extern "C" {
    pub fn nf_ct_remove_expectations(ct: *mut nf_conn);
}
extern "C" {
    pub fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect);
}
extern "C" {
    pub fn nf_ct_expect_iterate_destroy(e: *mut *mut bool (iter)(struct nf_conntrack_expect, data): *mut c_void, data: *mut c_void);
}
// Allocate space for an expectation: this is mandatory before calling
extern "C" {
    pub fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);
}
extern "C" {
    pub fn nf_ct_expect_related_report(_arg: expect, _arg: 0, _arg: 0, _arg: flags) -> return;
}
extern "C" {
    pub fn nf_ct_expectation_gc(master_help: *mut nf_conn_help);
}
