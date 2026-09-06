//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/auth.h
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
// Abstract interface for communicating with the authenticate module.
// There is some handshake that takes place between us and the monitor
// to acquire the necessary keys.  These are used to generate an
// 'authorizer' that we use when connecting to a service (mds, osd).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_authorizer {
    pub ): *mut *mut void (destroy)(struct ceph_authorizer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_auth_handshake {
    pub authorizer: *mut ceph_authorizer,
    pub authorizer_buf: *mut c_void,
    pub authorizer_buf_len: usize,
    pub authorizer_reply_buf: *mut c_void,
    pub authorizer_reply_buf_len: usize,
    pub msg): *mut ceph_msg,
    pub msg): *mut ceph_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_auth_client_ops {
//
// true if we are authenticated and can connect to
// services.
//
    pub ac): *mut *mut int (is_authenticated)(struct ceph_auth_client,
//
// true if we should (re)authenticate, e.g., when our tickets
// are getting old and crusty.
//
    pub ac): *mut *mut int (should_authenticate)(struct ceph_auth_client,
//
// build requests and process replies during monitor
// handshake.  if handle_reply returns -EAGAIN, we build
// another request.
//
    pub end): *mut *mut *mut *mut int (build_request)(struct ceph_auth_client ac, void buf, void,
    pub con_secret_len): *mut c_int,
//
// Create authorizer for connecting to a service, and verify
// the response to authenticate the service.
//
    pub auth): *mut ceph_auth_handshake,
// ensure that an existing authorizer is up to date
    pub auth): *mut ceph_auth_handshake,
    pub challenge_buf_len): c_int,
    pub con_secret_len): *mut *mut u8 con_secret, int,
    pub peer_type): c_int,
// reset when we (re)connect to a monitor
    pub ac): *mut *mut void (reset)(struct ceph_auth_client,
    pub ac): *mut *mut void (destroy)(struct ceph_auth_client,
    pub msg): *mut ceph_msg,
    pub msg): *mut ceph_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_auth_client {
    pub /: *mut *mut *mut u32 protocol; / CEPH_AUTH_,
    pub /: *mut *mut *mut void private; / for use by protocol implementation,
    pub /: *const *const *const ceph_auth_client_ops ops; / null iff protocol==0,
    pub /: *mut *mut bool negotiating; / true if negotiating protocol,
    pub /: *const *const *const char name; / entity name,
    pub /: *mut *mut u64 global_id; / our unique id in system,
    pub /: *const *const *const ceph_crypto_key key; / our secret key,
    pub /: *mut *mut unsigned want_keys; / which services we want,
    pub /: *mut *mut *mut int preferred_mode; / CEPH_CON_MODE_,
    pub /: *mut *mut int fallback_mode; / ditto,
    pub mutex: mutex,
}

extern "C" {
    pub fn ceph_auth_set_global_id(ac: *mut ceph_auth_client, global_id: u64);
}
extern "C" {
    pub fn ceph_auth_destroy(ac: *mut ceph_auth_client);
}
extern "C" {
    pub fn ceph_auth_reset(ac: *mut ceph_auth_client);
}
extern "C" {
    pub fn ceph_auth_entity_name_encode(name: *const c_char, p: *mut c_void, end: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ceph_auth_is_authenticated(ac: *mut ceph_auth_client) -> c_int;
}
extern "C" {
    pub fn ceph_auth_destroy_authorizer(a: *mut ceph_authorizer);
}
extern "C" {
    pub fn ceph_auth_get_request(ac: *mut ceph_auth_client, buf: *mut c_void, buf_len: c_int) -> c_int;
}
