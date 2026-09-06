//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireguard/noise.h
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
pub struct noise_replay_counter {
    pub counter: u64,
    pub lock: spinlock_t,
    pub BITS_PER_LONG]: unsigned long backtrack[COUNTER_BITS_TOTAL /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct noise_symmetric_key {
    pub key: [u8; NOISE_SYMMETRIC_KEY_LEN],
    pub birthdate: u64,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct noise_keypair {
    pub entry: index_hashtable_entry,
    pub sending: noise_symmetric_key,
    pub sending_counter: core::sync::atomic::AtomicI64,
    pub receiving: noise_symmetric_key,
    pub receiving_counter: noise_replay_counter,
    pub remote_index: __le32,
    pub i_am_the_initiator: bool,
    pub refcount: kref,
    pub rcu: rcu_head,
    pub internal_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct noise_keypairs {
    pub current_keypair: *mut noise_keypair __rcu,
    pub previous_keypair: *mut noise_keypair __rcu,
    pub next_keypair: *mut noise_keypair __rcu,
    pub keypair_update_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct noise_static_identity {
    pub static_public: [u8; NOISE_PUBLIC_KEY_LEN],
    pub static_private: [u8; NOISE_PUBLIC_KEY_LEN],
    pub lock: rw_semaphore,
    pub has_identity: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum noise_handshake_state {
    HANDSHAKE_ZEROED,
    HANDSHAKE_CREATED_INITIATION,
    HANDSHAKE_CONSUMED_INITIATION,
    HANDSHAKE_CREATED_RESPONSE,
    HANDSHAKE_CONSUMED_RESPONSE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct noise_handshake {
    pub entry: index_hashtable_entry,
    pub state: noise_handshake_state,
    pub last_initiation_consumption: u64,
    pub static_identity: *mut noise_static_identity,
    pub ephemeral_private: [u8; NOISE_PUBLIC_KEY_LEN],
    pub remote_static: [u8; NOISE_PUBLIC_KEY_LEN],
    pub remote_ephemeral: [u8; NOISE_PUBLIC_KEY_LEN],
    pub precomputed_static_static: [u8; NOISE_PUBLIC_KEY_LEN],
    pub preshared_key: [u8; NOISE_SYMMETRIC_KEY_LEN],
    pub hash: [u8; NOISE_HASH_LEN],
    pub chaining_key: [u8; NOISE_HASH_LEN],
    pub latest_timestamp: [u8; NOISE_TIMESTAMP_LEN],
    pub remote_index: __le32,
// Protects all members except the immutable (after noise_handshake_
// init): remote_static, precomputed_static_static, static_identity.
//
    pub lock: rw_semaphore,
}

extern "C" {
    pub fn wg_noise_init();
}
extern "C" {
    pub fn wg_noise_handshake_clear(handshake: *mut noise_handshake);
}
extern "C" {
    pub fn wg_noise_keypair_put(keypair: *mut noise_keypair, unreference_now: bool);
}
extern "C" {
    pub fn wg_noise_keypairs_clear(keypairs: *mut noise_keypairs);
}
extern "C" {
    pub fn wg_noise_expire_current_peer_keypairs(peer: *mut wg_peer);
}
extern "C" {
    pub fn wg_noise_precompute_static_static(peer: *mut wg_peer);
}
