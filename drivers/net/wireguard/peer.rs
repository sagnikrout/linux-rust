//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireguard/peer.h
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
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct endpoint {
    pub /: *mut *mut sockaddr_inet addr; / Large enough for both address families,
    pub addr4: sockaddr_in,
    pub addr6: sockaddr_in6,
}

// Essentially the same as addr6->scope_id
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wg_peer {
    pub device: *mut wg_device,
    pub rx_queue: prev_queue tx_queue,,
    pub staged_packet_queue: sk_buff_head,
    pub serial_work_cpu: c_int,
    pub is_dead: bool,
    pub keypairs: noise_keypairs,
    pub endpoint: endpoint,
    pub endpoint_cache: dst_cache,
    pub endpoint_lock: rwlock_t,
    pub handshake: noise_handshake,
    pub last_sent_handshake: core::sync::atomic::AtomicI64,
    pub transmit_packet_work: work_transmit_handshake_work, clear_peer_work,,
    pub latest_cookie: cookie,
    pub pubkey_hash: hlist_node,
    pub tx_bytes: u64 rx_bytes,,
    pub timer_send_keepalive: timer_list timer_retransmit_handshake,,
    pub timer_zero_key_material: timer_list timer_new_handshake,,
    pub timer_persistent_keepalive: timer_list,
    pub timer_handshake_attempts: c_uint,
    pub persistent_keepalive_interval: u16,
    pub timer_need_another_keepalive: bool,
    pub sent_lastminute_handshake: bool,
    pub walltime_last_handshake: timespec64,
    pub refcount: kref,
    pub rcu: rcu_head,
    pub peer_list: list_head,
    pub allowedips_list: list_head,
    pub napi: napi_struct,
    pub internal_id: u64,
}

extern "C" {
    pub fn wg_peer_get_maybe_zero(peer: *mut wg_peer) -> *mut wg_peer __must_check;
}
extern "C" {
    pub fn wg_peer_put(peer: *mut wg_peer);
}
extern "C" {
    pub fn wg_peer_remove(peer: *mut wg_peer);
}
extern "C" {
    pub fn wg_peer_remove_all(wg: *mut wg_device);
}
extern "C" {
    pub fn wg_peer_init() -> c_int;
}
extern "C" {
    pub fn wg_peer_uninit();
}
