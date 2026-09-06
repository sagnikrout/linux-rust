//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ovpn/tcp.h
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
// Copyright (C) 2019-2025 OpenVPN, Inc.
//
// Author:	Antonio Quartulli <antonio@openvpn.net>
//

extern "C" {
    pub fn ovpn_tcp_init() -> void __init;
}
extern "C" {
    pub fn ovpn_tcp_socket_detach(ovpn_sock: *mut ovpn_socket);
}
extern "C" {
    pub fn ovpn_tcp_socket_wait_finish(sock: *mut ovpn_socket);
}
// Prepare skb and enqueue it for sending to peer.
//
// Preparation consist in prepending the skb payload with its size.
// Required by the OpenVPN protocol in order to extract packets from
// the TCP stream on the receiver side.
//
extern "C" {
    pub fn ovpn_tcp_tx_work(work: *mut work_struct);
}
