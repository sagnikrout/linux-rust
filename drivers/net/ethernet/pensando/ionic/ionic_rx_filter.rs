//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/pensando/ionic/ionic_rx_filter.h
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
// Copyright(c) 2017 - 2019 Pensando Systems, Inc

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ionic_filter_state {
    IONIC_FILTER_STATE_SYNCED,
    IONIC_FILTER_STATE_NEW,
    IONIC_FILTER_STATE_OLD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rx_filter {
    pub flow_id: u32,
    pub filter_id: u32,
    pub rxq_index: u16,
    pub state: ionic_filter_state,
    pub cmd: ionic_rx_filter_add_cmd,
    pub by_hash: hlist_node,
    pub by_id: hlist_node,
}

pub const IONIC_RX_FILTER_HASH_BITS: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_rx_filters {
    pub /: *mut *mut spinlock_t lock; / filter list lock,
    pub /: *mut *mut hlist_head by_hash[IONIC_RX_FILTER_HLISTS]; / by skb hash,
    pub /: *mut *mut hlist_head by_id[IONIC_RX_FILTER_HLISTS]; / by filter_id,
}

extern "C" {
    pub fn ionic_rx_filter_free(lif: *mut ionic_lif, f: *mut ionic_rx_filter);
}
extern "C" {
    pub fn ionic_rx_filter_replay(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_rx_filters_init(lif: *mut ionic_lif) -> c_int;
}
extern "C" {
    pub fn ionic_rx_filters_deinit(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_rx_filter_sync(lif: *mut ionic_lif);
}
extern "C" {
    pub fn ionic_lif_list_addr(lif: *mut ionic_lif, addr: *const u8, mode: bool) -> c_int;
}
extern "C" {
    pub fn ionic_lif_vlan_add(lif: *mut ionic_lif, vid: u16) -> c_int;
}
extern "C" {
    pub fn ionic_lif_vlan_del(lif: *mut ionic_lif, vid: u16) -> c_int;
}
