//! Automatically rewritten from C Header to Rust Module
//! Source: net/tipc/crypto.h
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
// net/tipc/crypto.h: Include file for TIPC crypto
//
// Copyright (c) 2019, Ericsson AB
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//

pub const TIPC_EVERSION: c_int = 7;
// AEAD aes(gcm)
pub const TIPC_AES_GCM_KEY_SIZE_128: c_int = 16;
pub const TIPC_AES_GCM_KEY_SIZE_192: c_int = 24;
pub const TIPC_AES_GCM_KEY_SIZE_256: c_int = 32;
pub const TIPC_AES_GCM_SALT_SIZE: c_int = 4;
pub const TIPC_AES_GCM_IV_SIZE: c_int = 12;
pub const TIPC_AES_GCM_TAG_SIZE: c_int = 16;
//
// TIPC crypto modes:
// - CLUSTER_KEY:
// One single key is used for both TX & RX in all nodes in the cluster.
// - PER_NODE_KEY:
// Each nodes in the cluster has one TX key, for RX a node needs to know
// its peers' TX key for the decryption of messages from those nodes.
//
// TIPC encryption message format:
//
// 3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0
// 1 0 9 8 7 6 5 4|3 2 1 0 9 8 7 6|5 4 3 2 1 0 9 8|7 6 5 4 3 2 1 0
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// w0:|Ver=7| User  |D|TX |RX |K|M|N|             Rsvd                |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// w1:|                             Seqno                             |
// w2:|                           (8 octets)                          |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// w3:\                            Prevnode                           \
// /                        (4 or 16 octets)
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// \                                                               \
// /       Encrypted complete TIPC V2 header and user data
// \                                                               \
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                                                               |
// |                             AuthTag                           |
// |                           (16 octets)                         |
// |                                                               |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Word0:
// Ver	: = 7 i.e. TIPC encryption message version
// User	: = 7 (for LINK_PROTOCOL); = 13 (for LINK_CONFIG) or = 0
// D	: The destined bit i.e. the message's destination node is
// "known" or not at the message encryption
// TX	: TX key used for the message encryption
// RX	: Currently RX active key corresponding to the destination
// node's TX key (when the "D" bit is set)
// K	: Keep-alive bit (for RPS, LINK_PROTOCOL/STATE_MSG only)
// M       : Bit indicates if sender has master key
// N	: Bit indicates if sender has no RX keys corresponding to the
// receiver's TX (when the "D" bit is set)
// Rsvd	: Reserved bit, field
// Word1-2:
// Seqno	: The 64-bit sequence number of the encrypted message, also
// part of the nonce used for the message encryption/decryption
// Word3-:
// Prevnode: The source node address, or ID in case LINK_CONFIG only
// AuthTag	: The authentication tag for the message integrity checking
// generated by the message encryption
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tipc_ehdr {

    pub reserved_2: __be16,
    pub __packed: },
    pub w0: __be32,
}

extern "C" {
    pub fn tipc_crypto_stop(crypto: *mut tipc_crypto);
}
extern "C" {
    pub fn tipc_crypto_timeout(rx: *mut tipc_crypto);
}
extern "C" {
    pub fn tipc_crypto_key_flush(c: *mut tipc_crypto);
}
extern "C" {
    pub fn tipc_crypto_msg_rcv(net: *mut net, skb: *mut sk_buff);
}
extern "C" {
    pub fn tipc_aead_key_validate(ukey: *mut tipc_aead_key, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn tipc_ehdr_validate(skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 4, _arg: 16, _arg: 0xffff) -> return;
}
extern "C" {
    pub fn msg_bits(_arg: m, _arg: 4, _arg: 0, _arg: 0xf) -> return;
}

