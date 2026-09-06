//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_repr.h
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
// Copyright (C) 2019-2021, Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_repr_pcpu_stats {
    pub syncp: u64_stats_sync,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_drops: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_repr_type {
    ICE_REPR_TYPE_VF,
    ICE_REPR_TYPE_SF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_repr {
    pub src_vsi: *mut ice_vsi,
    pub netdev: *mut net_device,
    pub dst: *mut metadata_dst,
    pub br_port: *mut ice_esw_br_port,
    pub stats: *mut ice_repr_pcpu_stats __percpu,
    pub id: u32,
    pub parent_mac: [u8; ETH_ALEN],
    pub type: ice_repr_type,
    pub vf: *mut ice_vf,
    pub sf: *mut ice_dynamic_port,
}

extern "C" {
    pub fn ice_repr_destroy(repr: *mut ice_repr);
}
extern "C" {
    pub fn ice_repr_start_tx_queues(repr: *mut ice_repr);
}
extern "C" {
    pub fn ice_repr_stop_tx_queues(repr: *mut ice_repr);
}
extern "C" {
    pub fn ice_is_port_repr_netdev(netdev: *const net_device) -> bool;
}
extern "C" {
    pub fn ice_repr_inc_rx_stats(netdev: *mut net_device, len: c_uint);
}
