//! Automatically rewritten from C Header to Rust Module
//! Source: net/mac802154/llsec.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2014 Fraunhofer ITWM
//
// Written by:
// Phoebe Buckheister <phoebe.buckheister@itwm.fraunhofer.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac802154_llsec_key {
    pub key: ieee802154_llsec_key,
// one tfm for each authsize (4/8/16)
    pub tfm: [*mut crypto_aead; 3],
    pub tfm0: *mut crypto_sync_skcipher,
    pub ref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac802154_llsec_device_key {
    pub devkey: ieee802154_llsec_device_key,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac802154_llsec_device {
    pub dev: ieee802154_llsec_device,
    pub bucket_s: hlist_node,
    pub bucket_hw: hlist_node,
// protects dev.frame_counter and the elements of dev.keys
    pub lock: spinlock_t,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac802154_llsec_seclevel {
    pub level: ieee802154_llsec_seclevel,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac802154_llsec {
    pub params: ieee802154_llsec_params,
    pub table: ieee802154_llsec_table,
    pub 6): DECLARE_HASHTABLE(devices_short,,
    pub 6): DECLARE_HASHTABLE(devices_hw,,
// protects params, all other fields are fine with RCU
    pub lock: rwlock_t,
}

extern "C" {
    pub fn mac802154_llsec_init(sec: *mut mac802154_llsec);
}
extern "C" {
    pub fn mac802154_llsec_destroy(sec: *mut mac802154_llsec);
}
extern "C" {
    pub fn mac802154_llsec_encrypt(sec: *mut mac802154_llsec, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn mac802154_llsec_decrypt(sec: *mut mac802154_llsec, skb: *mut sk_buff) -> c_int;
}
