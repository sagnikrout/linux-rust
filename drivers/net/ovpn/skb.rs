//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ovpn/skb.h
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

//
// struct ovpn_cb - ovpn skb control block
// @peer: the peer this skb was received from/sent to
// @ks: the crypto key slot used to encrypt/decrypt this skb
// @crypto_tmp: pointer to temporary memory used for crypto operations
// containing the IV, the scatter gather list and the aead request
// @payload_offset: offset in the skb where the payload starts
// @nosignal: whether this skb should be sent with the MSG_NOSIGNAL flag (TCP)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_cb {
    pub peer: *mut ovpn_peer,
    pub ks: *mut ovpn_crypto_key_slot,
    pub crypto_tmp: *mut c_void,
    pub payload_offset: c_uint,
    pub nosignal: bool,
}

// Return IP protocol version from skb header.
// Return 0 if protocol is not IPv4/IPv6 or cannot be read.
//
// skb could be non-linear,
// make sure IP header is in non-fragmented part
//
