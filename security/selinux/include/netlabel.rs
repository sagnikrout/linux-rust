//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/include/netlabel.h
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
//
// SELinux interface to the NetLabel subsystem
//
// Author: Paul Moore <paul@paul-moore.com>
//
// (c) Copyright Hewlett-Packard Development Company, L.P., 2006
//

extern "C" {
    pub fn selinux_netlbl_cache_invalidate();
}
extern "C" {
    pub fn selinux_netlbl_sk_security_free(sksec: *mut sk_security_struct);
}
extern "C" {
    pub fn selinux_netlbl_sk_security_reset(sksec: *mut sk_security_struct);
}
extern "C" {
    pub fn selinux_netlbl_skbuff_setsid(skb: *mut sk_buff, family: u16, sid: u32) -> c_int;
}
extern "C" {
    pub fn selinux_netlbl_inet_conn_request(req: *mut request_sock, family: u16) -> c_int;
}
extern "C" {
    pub fn selinux_netlbl_inet_csk_clone(sk: *mut sock, family: u16);
}
extern "C" {
    pub fn selinux_netlbl_sctp_sk_clone(sk: *mut sock, newsk: *mut sock);
}
extern "C" {
    pub fn selinux_netlbl_socket_post_create(sk: *mut sock, family: u16) -> c_int;
}
extern "C" {
    pub fn selinux_netlbl_socket_connect(sk: *mut sock, addr: *mut sockaddr) -> c_int;
}

// type = NETLBL_NLTYPE_NONE;
// sid = SECSID_NULL;

