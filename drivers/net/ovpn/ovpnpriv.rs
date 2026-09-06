//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ovpn/ovpnpriv.h
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
// OpenVPN data channel offload
//
// Copyright (C) 2019-2025 OpenVPN, Inc.
//
// Author:	James Yonan <james@openvpn.net>
// Antonio Quartulli <antonio@openvpn.net>
//

//
// struct ovpn_peer_collection - container of peers for MultiPeer mode
// @by_id: table of peers index by ID
// @by_vpn_addr4: table of peers indexed by VPN IPv4 address (items can be
// rehashed on the fly due to peer IP change)
// @by_vpn_addr6: table of peers indexed by VPN IPv6 address (items can be
// rehashed on the fly due to peer IP change)
// @by_transp_addr: table of peers indexed by transport address (items can be
// rehashed on the fly due to peer IP change)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_peer_collection {
    pub 12): DECLARE_HASHTABLE(by_id,,
    pub 12]: hlist_nulls_head by_vpn_addr4[1 <<,
    pub 12]: hlist_nulls_head by_vpn_addr6[1 <<,
    pub 12]: hlist_nulls_head by_transp_addr[1 <<,
}

//
// struct ovpn_priv - per ovpn interface state
// @dev: the actual netdev representing the tunnel
// @mode: device operation mode (i.e. p2p, mp, ..)
// @lock: protect this object
// @peers: data structures holding multi-peer references
// @peer: in P2P mode, this is the only remote peer
// @gro_cells: pointer to the Generic Receive Offload cell
// @keepalive_work: struct used to schedule keepalive periodic job
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_priv {
    pub dev: *mut net_device,
    pub mode: ovpn_mode,
    pub /: *mut *mut spinlock_t lock; / protect writing to the ovpn_priv object,
    pub peers: *mut ovpn_peer_collection,
    pub peer: *mut ovpn_peer __rcu,
    pub gro_cells: gro_cells,
    pub keepalive_work: delayed_work,
}
