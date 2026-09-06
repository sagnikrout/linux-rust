//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_core.h
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
// This header is used to share core functionality between the
// standalone connection tracking module, and the compatibility layer's use
// of connection tracking.
//
// 16 Dec 2003: Yasuyuki Kozakai @USAGI <yasuyuki.kozakai@toshiba.co.jp>
// - generalize L3 protocol dependent part.
//
// Derived from include/linux/netfiter_ipv4/ip_conntrack_core.h
//

// This header is used to share core functionality between the
extern "C" {
    pub fn nf_conntrack_init_net(net: *mut net) -> c_int;
}
extern "C" {
    pub fn nf_conntrack_cleanup_net(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_cleanup_net_list(net_exit_list: *mut list_head);
}
extern "C" {
    pub fn nf_conntrack_proto_pernet_init(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_proto_init() -> c_int;
}
extern "C" {
    pub fn nf_conntrack_proto_fini();
}
extern "C" {
    pub fn nf_conntrack_init_start() -> c_int;
}
extern "C" {
    pub fn nf_conntrack_cleanup_start();
}
extern "C" {
    pub fn nf_conntrack_init_end();
}
extern "C" {
    pub fn nf_conntrack_cleanup_end();
}
// Find a connection corresponding to a tuple.
extern "C" {
    pub fn __nf_conntrack_confirm(skb: *mut sk_buff) -> c_int;
}
// Confirm a connection: returns NF_DROP if packet must be dropped.
extern "C" {
    pub fn nf_confirm(priv: *mut c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> c_uint;
}
pub const CONNTRACK_LOCKS: c_int = 1024;
extern "C" {
    pub fn nf_conntrack_lock(lock: *mut spinlock_t);
}
// ctnetlink code shared by both ctnetlink and nf_conntrack_bpf
extern "C" {
    pub fn __nf_ct_change_timeout(ct: *mut nf_conn, cta_timeout: u64) -> c_int;
}
extern "C" {
    pub fn __nf_ct_change_status(ct: *mut nf_conn, on: c_ulong, off: c_ulong);
}
extern "C" {
    pub fn nf_ct_change_status_common(ct: *mut nf_conn, status: c_uint) -> c_int;
}
