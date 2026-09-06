//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ovpn/proto.h
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

// When the OpenVPN protocol is ran in AEAD mode, use
// the OpenVPN packet ID as the AEAD nonce:
//
// 00000005 521c3b01 4308c041
// [seq # ] [  nonce_tail   ]
// [     12-byte full IV    ] -> OVPN_NONCE_SIZE
// [4-bytes                   -> OVPN_NONCE_WIRE_SIZE
// on wire]
//
// nonce size (96bits) as required by AEAD ciphers
pub const OVPN_NONCE_SIZE: c_int = 12;
// last 8 bytes of AEAD nonce: provided by userspace and usually derived
// from key material generated during TLS handshake
//
pub const OVPN_NONCE_TAIL_SIZE: c_int = 8;
// OpenVPN nonce size reduced by 8-byte nonce tail -- this is the
// size of the AEAD Associated Data (AD) sent over the wire
// and is normally the head of the IV
//

pub const OVPN_OPCODE_KEYID_MASK: c_uint = 0x07000000;
pub const OVPN_OPCODE_PKTTYPE_MASK: c_uint = 0xF8000000;
pub const OVPN_OPCODE_PEERID_MASK: c_uint = 0x00FFFFFF;
// packet opcodes of interest to us

pub const OVPN_PEER_ID_UNDEF: c_uint = 0x00FFFFFF;
//
// ovpn_opcode_from_skb - extract OP code from skb at specified offset
// @skb: the packet to extract the OP code from
// @offset: the offset in the data buffer where the OP code is located
//
// Note: this function assumes that the skb head was pulled enough
// to access the first 4 bytes.
//
// Return: the OP code
//
extern "C" {
    pub fn FIELD_GET(_arg: OVPN_OPCODE_PKTTYPE_MASK, _arg: opcode) -> return;
}
//
// ovpn_peer_id_from_skb - extract peer ID from skb at specified offset
// @skb: the packet to extract the OP code from
// @offset: the offset in the data buffer where the OP code is located
//
// Note: this function assumes that the skb head was pulled enough
// to access the first 4 bytes.
//
// Return: the peer ID
//
extern "C" {
    pub fn FIELD_GET(_arg: OVPN_OPCODE_PEERID_MASK, _arg: opcode) -> return;
}
//
// ovpn_key_id_from_skb - extract key ID from the skb head
// @skb: the packet to extract the key ID code from
//
// Note: this function assumes that the skb head was pulled enough
// to access the first 4 bytes.
//
// Return: the key ID
//
extern "C" {
    pub fn FIELD_GET(_arg: OVPN_OPCODE_KEYID_MASK, _arg: opcode) -> return;
}
//
// ovpn_opcode_compose - combine OP code, key ID and peer ID to wire format
// @opcode: the OP code
// @key_id: the key ID
// @peer_id: the peer ID
//
// Return: a 4 bytes integer obtained combining all input values following the
// OpenVPN wire format. This integer can then be written to the packet header.
//
