//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ovpn.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/ovpn.yaml
// YNL-GEN uapi header
// To regenerate run: tools/net/ynl/ynl-regen.sh

pub const OVPN_FAMILY_VERSION: c_int = 1;
pub const OVPN_NONCE_TAIL_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovpn_cipher_alg {
    OVPN_CIPHER_ALG_NONE,
    OVPN_CIPHER_ALG_AES_GCM,
    OVPN_CIPHER_ALG_CHACHA20_POLY1305,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovpn_del_peer_reason {
    OVPN_DEL_PEER_REASON_TEARDOWN,
    OVPN_DEL_PEER_REASON_USERSPACE,
    OVPN_DEL_PEER_REASON_EXPIRED,
    OVPN_DEL_PEER_REASON_TRANSPORT_ERROR,
    OVPN_DEL_PEER_REASON_TRANSPORT_DISCONNECT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ovpn_key_slot {
    OVPN_KEY_SLOT_PRIMARY,
    OVPN_KEY_SLOT_SECONDARY,
}

