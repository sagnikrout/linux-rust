//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ovpn/socket.h
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

//
// struct ovpn_socket - a kernel socket referenced in the ovpn code
// @ovpn: ovpn instance owning this socket (UDP only)
// @dev_tracker: reference tracker for associated dev (UDP only)
// @peer: unique peer transmitting over this socket (TCP only)
// @sk: the low level sock object
// @refcount: amount of contexts currently referencing this object
// @work: member used to schedule release routine (it may block)
// @tcp_tx_work: work for deferring outgoing packet processing (TCP only)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovpn_socket {
    pub ovpn: *mut ovpn_priv,
    pub dev_tracker: netdevice_tracker,
}

extern "C" {
    pub fn ovpn_socket_release(peer: *mut ovpn_peer);
}
