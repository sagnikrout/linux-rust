//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/handshake.h
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
//
// Generic netlink HANDSHAKE service.
//
// Author: Chuck Lever <chuck.lever@oracle.com>
//
// Copyright (c) 2023, Oracle and/or its affiliates.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_handshake_args {
    pub ta_sock: *mut socket,
    pub ta_done: tls_done_func_t,
    pub ta_data: *mut c_void,
    pub ta_peername: *const c_char,
    pub ta_timeout_ms: c_uint,
    pub ta_keyring: key_serial_t,
    pub ta_my_cert: key_serial_t,
    pub ta_my_privkey: key_serial_t,
    pub ta_num_peerids: c_uint,
    pub ta_my_peerids: [key_serial_t; 5],
}

extern "C" {
    pub fn tls_client_hello_anon(args: *const tls_handshake_args, flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn tls_client_hello_x509(args: *const tls_handshake_args, flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn tls_client_hello_psk(args: *const tls_handshake_args, flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn tls_server_hello_x509(args: *const tls_handshake_args, flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn tls_server_hello_psk(args: *const tls_handshake_args, flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn tls_handshake_cancel(sk: *mut sock) -> bool;
}
extern "C" {
    pub fn tls_handshake_close(sock: *mut socket);
}
extern "C" {
    pub fn tls_get_record_type(sk: *const sock, msg: *const cmsghdr) -> u8;
}
