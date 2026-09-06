//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ovpn/pktid.h
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
// Copyright (C) 2020-2025 OpenVPN, Inc.
//
// Author:	Antonio Quartulli <antonio@openvpn.net>
// James Yonan <james@openvpn.net>
//

// If no packets received for this length of time, set a backtrack floor
// at highest received packet ID thus far.
//

// Packet-ID state for transmitter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_pktid_xmit {
    pub seq_num: core::sync::atomic::AtomicI32,
}

// replay window sizing in bytes = 2^REPLAY_WINDOW_ORDER
pub const REPLAY_WINDOW_ORDER: c_int = 8;

// Packet-ID state for receiver.
// Other than lock member, can be zeroed to initialize.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_pktid_recv {
// "sliding window" bitmask of recent packet IDs received
    pub REPLAY_WINDOW_SIZE): DECLARE_BITMAP(history,,
// bit position of deque base in history
    pub base: c_uint,
// extent (in bits) of deque in history
    pub extent: c_uint,
// expiration of history in jiffies
    pub expire: c_ulong,
// highest sequence number received
    pub id: u32,
// highest time stamp received
    pub time: u32,
// we will only accept backtrack IDs > id_floor
    pub id_floor: u32,
    pub max_backtrack: c_uint,
// protects entire pktd ID state
    pub lock: spinlock_t,
}

// Get the next packet ID for xmit
// when the 32bit space is over, we return an error because the packet
// ID is used to create the cipher IV and we do not want to reuse the
// same value more than once
//
// pktid = seq_num;
// Write 12-byte AEAD IV to dest
// ( __be32 *)(dest) = htonl(pktid);
extern "C" {
    pub fn ovpn_pktid_xmit_init(pid: *mut ovpn_pktid_xmit);
}
extern "C" {
    pub fn ovpn_pktid_recv_init(pr: *mut ovpn_pktid_recv);
}
extern "C" {
    pub fn ovpn_pktid_recv(pr: *mut ovpn_pktid_recv, pkt_id: u32, pkt_time: u32) -> c_int;
}
