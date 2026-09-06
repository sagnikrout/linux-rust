//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_timeout.h
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

pub const CTNL_TIMEOUT_NAME_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_timeout {
    pub refcnt: refcount_t,
    pub l3num: __u16,
    pub l4proto: *const nf_conntrack_l4proto,
    pub rcu: rcu_head,
    pub data: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_timeout {
    pub timeout: *mut nf_ct_timeout __rcu,
}

extern "C" {
    pub fn nf_ct_ext_find(_arg: ct, _arg: NF_CT_EXT_TIMEOUT) -> return;
}

extern "C" {
    pub fn nf_ct_untimeout(net: *mut net, timeout: *mut nf_ct_timeout);
}
extern "C" {
    pub fn nf_ct_destroy_timeout(ct: *mut nf_conn);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_timeout_hooks {
    pub name): *const *const *const *const nf_ct_timeout (timeout_find_get)(net net, char,
    pub timeout): *mut *mut void (timeout_put)(struct nf_ct_timeout,
}

