//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ovpn/crypto.h
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
// Author:	James Yonan <james@openvpn.net>
// Antonio Quartulli <antonio@openvpn.net>
//

// info needed for both encrypt and decrypt directions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_key_direction {
    pub cipher_key: *const u8,
    pub cipher_key_size: usize,
    pub /: *const *const *const u8 nonce_tail; / only needed for GCM modes,
    pub /: *mut *mut size_t nonce_tail_size; / only needed for GCM modes,
}

// all info for a particular symmetric key (primary or secondary)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_key_config {
    pub cipher_alg: ovpn_cipher_alg,
    pub key_id: u8,
    pub encrypt: ovpn_key_direction,
    pub decrypt: ovpn_key_direction,
}

// used to pass settings from netlink to the crypto engine
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_peer_key_reset {
    pub slot: ovpn_key_slot,
    pub key: ovpn_key_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_crypto_key_slot {
    pub key_id: u8,
    pub encrypt: *mut crypto_aead,
    pub decrypt: *mut crypto_aead,
    pub nonce_tail_xmit: [u8; OVPN_NONCE_TAIL_SIZE],
    pub nonce_tail_recv: [u8; OVPN_NONCE_TAIL_SIZE],
    pub ____cacheline_aligned_in_smp: ovpn_pktid_recv pid_recv,
    pub ____cacheline_aligned_in_smp: ovpn_pktid_xmit pid_xmit,
    pub free_work: rcu_work,
    pub refcount: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_crypto_state {
    pub slots: [*mut ovpn_crypto_key_slot __rcu; 2],
    pub primary_idx: u8,
// protects primary and secondary slots
    pub lock: spinlock_t,
}

extern "C" {
    pub fn kref_get_unless_zero(_arg: &ks->refcount) -> return;
}
// when both key slots are occupied but no matching key ID is found, ks
// has to be reset to NULL to avoid carrying a stale pointer
//
extern "C" {
    pub fn ovpn_crypto_key_slot_release(kref: *mut kref);
}
extern "C" {
    pub fn ovpn_crypto_state_release(cs: *mut ovpn_crypto_state);
}
extern "C" {
    pub fn ovpn_crypto_key_slots_swap(cs: *mut ovpn_crypto_state);
}
extern "C" {
    pub fn ovpn_crypto_kill_key(cs: *mut ovpn_crypto_state, key_id: u8) -> bool;
}
