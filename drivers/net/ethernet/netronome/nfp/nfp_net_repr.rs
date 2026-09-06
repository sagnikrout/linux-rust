//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_net_repr.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017-2018 Netronome Systems, Inc.

//
// struct nfp_reprs - container for representor netdevs
// @num_reprs:	Number of elements in reprs array
// @reprs:	Array of representor netdevs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_reprs {
    pub num_reprs: c_uint,
    pub __counted_by(num_reprs): *mut *mut net_device __rcu reprs[],
}

//
// struct nfp_repr_pcpu_stats
// @rx_packets:	Received packets
// @rx_bytes:	Received bytes
// @tx_packets:	Transmitted packets
// @tx_bytes:	Transmitted dropped
// @tx_drops:	Packets dropped on transmit
// @syncp:	Reference count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_repr_pcpu_stats {
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_drops: u64,
    pub syncp: u64_stats_sync,
}

//
// struct nfp_repr - priv data for representor netdevs
// @netdev:	Back pointer to netdev
// @dst:	Destination for packet TX
// @port:	Port of representor
// @app:	APP handle
// @stats:	Statistic of packets hitting CPU
// @app_priv:	Pointer for APP data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_repr {
    pub netdev: *mut net_device,
    pub dst: *mut metadata_dst,
    pub port: *mut nfp_port,
    pub app: *mut nfp_app,
    pub stats: *mut nfp_repr_pcpu_stats __percpu,
    pub app_priv: *mut c_void,
}

//
// enum nfp_repr_type - type of representor
// @NFP_REPR_TYPE_PHYS_PORT:	external NIC port
// @NFP_REPR_TYPE_PF:		physical function
// @NFP_REPR_TYPE_VF:		virtual function
// @__NFP_REPR_TYPE_MAX:	number of representor types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_repr_type {
    NFP_REPR_TYPE_PHYS_PORT,
    NFP_REPR_TYPE_PF,
    NFP_REPR_TYPE_VF,

    __NFP_REPR_TYPE_MAX,
}

extern "C" {
    pub fn nfp_repr_inc_rx_stats(netdev: *mut net_device, len: c_uint);
}
extern "C" {
    pub fn nfp_repr_free(netdev: *mut net_device);
}
extern "C" {
    pub fn nfp_repr_clean_and_free(repr: *mut nfp_repr);
}
extern "C" {
    pub fn nfp_reprs_clean_and_free(app: *mut nfp_app, reprs: *mut nfp_reprs);
}
extern "C" {
    pub fn nfp_reprs_resync_phys_ports(app: *mut nfp_app) -> c_int;
}
extern "C" {
    pub fn nfp_repr_alloc_mqs(_arg: app, _arg: 1, _arg: 1) -> return;
}
