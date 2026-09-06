//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireguard/messages.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum noise_lengths {
    NOISE_PUBLIC_KEY_LEN = CURVE25519_KEY_SIZE,
    NOISE_SYMMETRIC_KEY_LEN = CHACHA20POLY1305_KEY_SIZE,
    NOISE_TIMESTAMP_LEN = sizeof(u64) + sizeof(u32),
    NOISE_AUTHTAG_LEN = CHACHA20POLY1305_AUTHTAG_SIZE,
    NOISE_HASH_LEN = BLAKE2S_HASH_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cookie_values {
    COOKIE_SECRET_MAX_AGE = 2 * 60,
    COOKIE_SECRET_LATENCY = 5,
    COOKIE_NONCE_LEN = XCHACHA20POLY1305_NONCE_SIZE,
    COOKIE_LEN = 16
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum counter_values {
    COUNTER_BITS_TOTAL = 8192,
    COUNTER_REDUNDANT_BITS = BITS_PER_LONG,
    COUNTER_WINDOW_SIZE = COUNTER_BITS_TOTAL - COUNTER_REDUNDANT_BITS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum limits {
    REKEY_AFTER_MESSAGES = 1ULL << 60,
    REJECT_AFTER_MESSAGES = U64_MAX - COUNTER_WINDOW_SIZE - 1,
    REKEY_TIMEOUT = 5,
    REKEY_TIMEOUT_JITTER_MAX_JIFFIES = HZ / 3,
    REKEY_AFTER_TIME = 120,
    REJECT_AFTER_TIME = 180,
    INITIATIONS_PER_SECOND = 50,
    MAX_PEERS_PER_DEVICE = 1U << 20,
    KEEPALIVE_TIMEOUT = 10,
    MAX_TIMER_HANDSHAKES = 90 / REKEY_TIMEOUT,
    MAX_QUEUED_INCOMING_HANDSHAKES = 4096, /* TODO: replace this with DQL */
    MAX_STAGED_PACKETS = 128,
    MAX_QUEUED_PACKETS = 1024 /* TODO: replace this with DQL */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum message_type {
    MESSAGE_INVALID = 0,
    MESSAGE_HANDSHAKE_INITIATION = 1,
    MESSAGE_HANDSHAKE_RESPONSE = 2,
    MESSAGE_HANDSHAKE_COOKIE = 3,
    MESSAGE_DATA = 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct message_header {
// The actual layout of this that we want is:
// u8 type
// u8 reserved_zero[3]
//
// But it turns out that by encoding this as little endian,
// we achieve the same thing, and it makes checking faster.
//
    pub type: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct message_macs {
    pub mac1: [u8; COOKIE_LEN],
    pub mac2: [u8; COOKIE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct message_handshake_initiation {
    pub header: message_header,
    pub sender_index: __le32,
    pub unencrypted_ephemeral: [u8; NOISE_PUBLIC_KEY_LEN],
    pub encrypted_static: [u8; noise_encrypted_len(NOISE_PUBLIC_KEY_LEN)],
    pub encrypted_timestamp: [u8; noise_encrypted_len(NOISE_TIMESTAMP_LEN)],
    pub macs: message_macs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct message_handshake_response {
    pub header: message_header,
    pub sender_index: __le32,
    pub receiver_index: __le32,
    pub unencrypted_ephemeral: [u8; NOISE_PUBLIC_KEY_LEN],
    pub encrypted_nothing: [u8; noise_encrypted_len(0)],
    pub macs: message_macs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct message_handshake_cookie {
    pub header: message_header,
    pub receiver_index: __le32,
    pub nonce: [u8; COOKIE_NONCE_LEN],
    pub encrypted_cookie: [u8; noise_encrypted_len(COOKIE_LEN)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct message_data {
    pub header: message_header,
    pub key_idx: __le32,
    pub counter: __le64,
    pub encrypted_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum message_alignments {
    MESSAGE_PADDING_MULTIPLE = 16,
    MESSAGE_MINIMUM_LENGTH = message_data_len(0)
}

