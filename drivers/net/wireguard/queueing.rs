//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireguard/queueing.h
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

// queueing.c APIs:
extern "C" {
    pub fn wg_packet_queue_free(queue: *mut crypt_queue, purge: bool);
}
// receive.c APIs:
extern "C" {
    pub fn wg_packet_receive(wg: *mut wg_device, skb: *mut sk_buff);
}
extern "C" {
    pub fn wg_packet_handshake_receive_worker(work: *mut work_struct);
}
// NAPI poll function:
extern "C" {
    pub fn wg_packet_rx_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
// Workqueue worker:
extern "C" {
    pub fn wg_packet_decrypt_worker(work: *mut work_struct);
}
// send.c APIs:
extern "C" {
    pub fn wg_packet_send_handshake_response(peer: *mut wg_peer);
}
extern "C" {
    pub fn wg_packet_send_keepalive(peer: *mut wg_peer);
}
extern "C" {
    pub fn wg_packet_purge_staged_packets(peer: *mut wg_peer);
}
extern "C" {
    pub fn wg_packet_send_staged_packets(peer: *mut wg_peer);
}
// Workqueue workers:
extern "C" {
    pub fn wg_packet_handshake_send_worker(work: *mut work_struct);
}
extern "C" {
    pub fn wg_packet_tx_worker(work: *mut work_struct);
}
extern "C" {
    pub fn wg_packet_encrypt_worker(work: *mut work_struct);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum packet_state {
    PACKET_STATE_UNCRYPTED,
    PACKET_STATE_CRYPTED,
    PACKET_STATE_DEAD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_cb {
    pub nonce: u64,
    pub keypair: *mut noise_keypair,
    pub state: core::sync::atomic::AtomicI32,
    pub mtu: u32,
    pub ds: u8,
}

// This function is racy, in the sense that it's called while last_cpu is
// unlocked, so it could return the same CPU twice. Adding locking or using
// atomic sequence numbers is slower though, and the consequences of racing are
// harmless, so live with it.
//
extern "C" {
    pub fn wg_prev_queue_init(queue: *mut prev_queue);
}
// Multi producer
extern "C" {
    pub fn wg_prev_queue_enqueue(queue: *mut prev_queue, skb: *mut sk_buff) -> bool;
}
// Single consumer
// We first queue this up for the peer ingestion, but the consumer
// will wait for the state to change to CRYPTED or DEAD before.
//
// Then we queue it up in the device queue, which consumes the
// packet as soon as it can.
//
// We take a reference, because as soon as we call atomic_set, the
// peer can be freed from below us.
//
// We take a reference, because as soon as we call atomic_set, the
// peer can be freed from below us.
//

extern "C" {
    pub fn wg_packet_counter_selftest() -> bool;
}

