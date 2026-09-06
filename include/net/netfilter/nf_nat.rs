//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_nat.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_nat_manip_type {
    NF_NAT_MANIP_SRC,
    NF_NAT_MANIP_DST
}

// SRC manip occurs POST_ROUTING or LOCAL_IN

// per conntrack: nat application helper private data
#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_nat_help {
// insert nat helper private data here

    pub nat_pptp_info: nf_nat_pptp,

}

// The structure embedded in the conntrack structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_nat {
    pub help: nf_conntrack_nat_help,

    pub masq_index: c_int,

}

// Set up the info structure to map into this range.

extern "C" {
    pub fn nf_ct_ext_find(_arg: ct, _arg: NF_CT_EXT_NAT) -> return;
}

extern "C" {
    pub fn nf_nat_ipv4_register_fn(net: *mut net, ops: *const nf_hook_ops) -> c_int;
}
extern "C" {
    pub fn nf_nat_ipv4_unregister_fn(net: *mut net, ops: *const nf_hook_ops);
}
extern "C" {
    pub fn nf_nat_ipv6_register_fn(net: *mut net, ops: *const nf_hook_ops) -> c_int;
}
extern "C" {
    pub fn nf_nat_ipv6_unregister_fn(net: *mut net, ops: *const nf_hook_ops);
}
extern "C" {
    pub fn nf_nat_inet_register_fn(net: *mut net, ops: *const nf_hook_ops) -> c_int;
}
extern "C" {
    pub fn nf_nat_inet_unregister_fn(net: *mut net, ops: *const nf_hook_ops);
}
