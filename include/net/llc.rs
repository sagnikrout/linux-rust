//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/llc.h
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
// Copyright (c) 1997 by Procom Technology, Inc.
// 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_addr {
    pub lsap: c_uchar,
    pub mac: [c_uchar; IFHWADDRLEN],
}

pub const LLC_SAP_STATE_INACTIVE: c_int = 1;
pub const LLC_SAP_STATE_ACTIVE: c_int = 2;
pub const LLC_SK_DEV_HASH_BITS: c_int = 6;

pub const LLC_SK_LADDR_HASH_BITS: c_int = 6;

//
// struct llc_sap - Defines the SAP component
//
// @station - station this sap belongs to
// @state - sap state
// @p_bit - only lowest-order bit used
// @f_bit - only lowest-order bit used
// @laddr - SAP value in this 'lsap'
// @node - entry in station sap_list
// @sk_list - LLC sockets this one manages
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_sap {
    pub state: c_uchar,
    pub p_bit: c_uchar,
    pub f_bit: c_uchar,
    pub refcnt: refcount_t,
    pub orig_dev): *mut net_device,
    pub laddr: llc_addr,
    pub node: list_head,
    pub sk_lock: spinlock_t,
    pub sk_count: c_int,
    pub sk_laddr_hash: [hlist_nulls_head; LLC_SK_LADDR_HASH_ENTRIES],
    pub sk_dev_hash: [hlist_head; LLC_SK_DEV_HASH_ENTRIES],
    pub rcu: rcu_head,
}

extern "C" {
    pub fn llc_remove_pack(type: c_int);
}
extern "C" {
    pub fn llc_set_station_handler(skb): *mut *mut void (handler)(struct sk_buff);
}
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &sap->refcnt) -> return;
}
extern "C" {
    pub fn llc_sap_close(sap: *mut llc_sap);
}
extern "C" {
    pub fn llc_sap_handler(sap: *mut llc_sap, skb: *mut sk_buff);
}
extern "C" {
    pub fn llc_conn_handler(sap: *mut llc_sap, skb: *mut sk_buff);
}
extern "C" {
    pub fn llc_station_init();
}
extern "C" {
    pub fn llc_station_exit();
}

extern "C" {
    pub fn llc_proc_init() -> c_int;
}
extern "C" {
    pub fn llc_proc_exit();
}

extern "C" {
    pub fn llc_sysctl_init() -> c_int;
}
extern "C" {
    pub fn llc_sysctl_exit();
}

