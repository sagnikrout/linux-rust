//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/af_rxrpc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// RxRPC kernel service interface definitions
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_interruptibility {
    RXRPC_INTERRUPTIBLE,	/* Call is interruptible */
    RXRPC_PREINTERRUPTIBLE,	/* Call can be cancelled whilst waiting for a slot */
    RXRPC_UNINTERRUPTIBLE,	/* Call should not be interruptible at all */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxrpc_oob_type {
    RXRPC_OOB_CHALLENGE,	/* Security challenge for a connection */
}

//
// Debug ID counter for tracing.
//
// Operations table for rxrpc to call out to a kernel application (e.g. kAFS).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxrpc_kernel_ops {
    pub user_call_ID): c_ulong,
    pub user_call_ID): *mut *mut *mut void (discard_new_call)(struct rxrpc_call call, unsigned long,
    pub user_call_ID): *mut *mut *mut void (user_attach_call)(struct rxrpc_call call, unsigned long,
    pub oob): *mut *mut *mut void (notify_oob)(struct sock sk, struct sk_buff,
}

extern "C" {
    pub fn rxrpc_kernel_shutdown_call(sock: *mut socket, call: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_kernel_put_call(sock: *mut socket, call: *mut rxrpc_call);
}
extern "C" {
    pub fn rxrpc_kernel_put_peer(peer: *mut rxrpc_peer);
}
extern "C" {
    pub fn rxrpc_kernel_set_peer_data(peer: *mut rxrpc_peer, app_data: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn rxrpc_kernel_get_peer_data(peer: *const rxrpc_peer) -> c_ulong;
}
extern "C" {
    pub fn rxrpc_kernel_get_srtt(: *const rxrpc_peer) -> c_uint;
}
extern "C" {
    pub fn rxrpc_kernel_set_tx_length(: *mut socket, : *mut rxrpc_call, _arg: i64);
}
extern "C" {
    pub fn rxrpc_kernel_check_life(: *const socket, : *const rxrpc_call) -> bool;
}
extern "C" {
    pub fn rxrpc_sock_set_min_security_level(sk: *mut sock, val: c_uint) -> c_int;
}
extern "C" {
    pub fn rxrpc_sock_set_security_keyring(: *mut sock, : *mut key) -> c_int;
}
extern "C" {
    pub fn rxrpc_sock_set_manage_response(sk: *mut sock, set: bool) -> c_int;
}
extern "C" {
    pub fn rxrpc_kernel_free_oob(oob: *mut sk_buff);
}
extern "C" {
    pub fn rxkad_kernel_respond_to_challenge(challenge: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn rxgk_kernel_query_challenge(challenge: *mut sk_buff) -> u32;
}
