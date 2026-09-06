//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_ecache.h
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
// connection tracking event cache.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_ct_ecache_state {
    NFCT_ECACHE_DESTROY_FAIL,	/* tried but failed to send destroy event */
    NFCT_ECACHE_DESTROY_SENT,	/* sent destroy event after failure */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_ecache {
    pub /: *mut *mut unsigned long cache; / bitops want long,

    pub /: *mut *mut local64_t timestamp; / event timestamp, in nanoseconds,

    pub /: *mut *mut u16 ctmask; / bitmask of ct events to be delivered,
    pub /: *mut *mut u16 expmask; / bitmask of expect events to be delivered,
    pub /: *mut *mut u32 missed; / missed events,
    pub /: *mut *mut u32 portid; / netlink portid of destroyer,
}

extern "C" {
    pub fn nf_ct_ext_find(_arg: ct, _arg: NF_CT_EXT_ECACHE) -> return;
}

extern "C" {
    pub fn nf_ct_ext_exist(_arg: ct, _arg: NF_CT_EXT_ECACHE) -> return;
}

// This structure is passed to event handler
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_event {
    pub ct: *mut nf_conn,
    pub portid: u32,
    pub report: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_exp_event {
    pub exp: *mut nf_conntrack_expect,
    pub portid: u32,
    pub report: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_event_notifier {
    pub item): *const *const int (ct_event)(unsigned int events, struct nf_ct_event,
    pub item): *const *const int (exp_event)(unsigned int events, struct nf_exp_event,
}

extern "C" {
    pub fn nf_conntrack_unregister_notifier(net: *mut net);
}
extern "C" {
    pub fn nf_ct_deliver_cached_events(ct: *mut nf_conn);
}
extern "C" {
    pub fn nf_ct_ecache_ext_add(ct: *mut nf_conn, ctmask: u16, expmask: u16, gfp: gfp_t) -> bool;
}

// renew only if this is the first cached event, so that the
// timestamp reflects the first, not the last, generated event.
//

extern "C" {
    pub fn nf_conntrack_eventmask_report(event: 1 <<, _arg: ct, _arg: portid, _arg: report) -> return;
}

extern "C" {
    pub fn nf_conntrack_eventmask_report(event: 1 <<, _arg: ct, _arg: 0, _arg: 0) -> return;
}

extern "C" {
    pub fn nf_conntrack_ecache_work(net: *mut net, state: nf_ct_ecache_state);
}
extern "C" {
    pub fn nf_conntrack_ecache_pernet_init(net: *mut net);
}
extern "C" {
    pub fn nf_conntrack_ecache_pernet_fini(net: *mut net);
}

