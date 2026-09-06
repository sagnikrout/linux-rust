//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/tc_encap_actions.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2023, Advanced Micro Devices, Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

//
// struct efx_neigh_binder - driver state for a neighbour entry
// @net: the network namespace in which this neigh resides
// @dst_ip: the IPv4 destination address resolved by this neigh
// @dst_ip6: the IPv6 destination address resolved by this neigh
// @ha: the hardware (Ethernet) address of the neighbour
// @n_valid: true if the neighbour is in NUD_VALID state
// @lock: protects @ha and @n_valid
// @ttl: Time To Live associated with the route used
// @dying: set when egdev is going away, to skip further updates
// @egdev: egress device from the route lookup.  Holds a reference
// @dev_tracker: reference tracker entry for @egdev
// @ns_tracker: reference tracker entry for @ns
// @ref: counts encap actions referencing this entry
// @used: jiffies of last time traffic hit any encap action using this.
// When counter reads update this, a new neighbour event is sent to
// indicate that the neighbour entry is still in use.
// @users: list of &struct efx_tc_encap_action
// @linkage: entry in efx->neigh_ht (keys are @net, @dst_ip, @dst_ip6).
// @work: processes neighbour state changes, updates the encap actions
// @efx: owning NIC instance.
//
// Associates a neighbour entry with the encap actions that are
// interested in it, allowing the latter to be updated when the
// neighbour details change.
// Whichever of @dst_ip and @dst_ip6 is not in use will be all-zeroes,
// this distinguishes IPv4 from IPv6 entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_neigh_binder {
    pub net: *mut net,
    pub dst_ip: __be32,
    pub dst_ip6: in6_addr,
    pub ha: [c_char; ETH_ALEN],
    pub n_valid: bool,
    pub lock: rwlock_t,
    pub ttl: u8,
    pub dying: bool,
    pub egdev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub ns_tracker: netns_tracker,
    pub ref: refcount_t,
    pub used: c_ulong,
    pub users: list_head,
    pub linkage: rhash_head,
    pub work: work_struct,
    pub efx: *mut efx_nic,
}

// This limit is arbitrary; current hardware (SN1022) handles encap headers
// of up to 126 bytes, but that limit is not enshrined in the MCDI protocol.
//
pub const EFX_TC_MAX_ENCAP_HDR: c_int = 126;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efx_tc_encap_action {
    pub type: efx_encap_type,
    pub /: *mut *mut ip_tunnel_key key; / 52 bytes,
    pub /: *mut *mut u32 dest_mport; / is copied into struct efx_tc_action_set,
    pub encap_hdr_len: u8,
    pub n_valid: bool,
    pub encap_hdr: [u8; EFX_TC_MAX_ENCAP_HDR],
    pub neigh: *mut efx_neigh_binder,
    pub /: *mut *mut list_head list; / entry on neigh->users list,
    pub /: *mut *mut list_head users; / action sets using this encap_md,
    pub /: *mut *mut rhash_head linkage; / efx->tc_encap_ht,
    pub ref: refcount_t,
    pub /: *mut *mut u32 fw_id; / index of this entry in firmware encap table,
}

// create/uncreate/teardown hashtables
extern "C" {
    pub fn efx_tc_init_encap_actions(efx: *mut efx_nic) -> c_int;
}
extern "C" {
    pub fn efx_tc_destroy_encap_actions(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_tc_fini_encap_actions(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_tc_check_ready(efx: *mut efx_nic, rule: *mut efx_tc_flow_rule) -> bool;
}
extern "C" {
    pub fn efx_tc_unregister_egdev(efx: *mut efx_nic, net_dev: *mut net_device);
}

