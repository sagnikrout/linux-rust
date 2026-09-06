//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_arfs.h
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
// Copyright (C) 2018-2020, Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_arfs_fltr_state {
    ICE_ARFS_INACTIVE,
    ICE_ARFS_ACTIVE,
    ICE_ARFS_TODEL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_arfs_entry {
    pub fltr_info: ice_fdir_fltr,
    pub list_entry: hlist_node,
    pub /: *mut *mut u64 time_activated; / only valid for UDP flows,
    pub flow_id: u32,
// fltr_state = 0 - ICE_ARFS_INACTIVE:
// filter needs to be updated or programmed in HW.
// fltr_state = 1 - ICE_ARFS_ACTIVE:
// filter is active and programmed in HW.
// fltr_state = 2 - ICE_ARFS_TODEL:
// filter has been deleted from HW and needs to be removed from
// the aRFS hash table.
//
    pub fltr_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_arfs_entry_ptr {
    pub arfs_entry: *mut ice_arfs_entry,
    pub list_entry: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_arfs_active_fltr_cntrs {
    pub active_tcpv4_cnt: core::sync::atomic::AtomicI32,
    pub active_tcpv6_cnt: core::sync::atomic::AtomicI32,
    pub active_udpv4_cnt: core::sync::atomic::AtomicI32,
    pub active_udpv6_cnt: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn ice_clear_arfs(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_init_arfs(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_sync_arfs_fltrs(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_set_cpu_rx_rmap(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_remove_arfs(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_rebuild_arfs(pf: *mut ice_pf);
}

