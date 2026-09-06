//! Automatically rewritten from C Header to Rust Module
//! Source: net/ceph/auth_x.h
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
// Handle ticket for a single service.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_ticket_handler {
    pub node: rb_node,
    pub service: c_uint,
    pub session_key: ceph_crypto_key,
    pub have_key: bool,
    pub secret_id: u64,
    pub ticket_blob: *mut ceph_buffer,
    pub expires: time64_t renew_after,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_authorizer {
    pub base: ceph_authorizer,
    pub session_key: ceph_crypto_key,
    pub buf: *mut ceph_buffer,
    pub service: c_uint,
    pub nonce: u64,
    pub secret_id: u64,
    pub __aligned(8): char enc_buf[CEPHX_AU_ENC_BUF_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_x_info {
    pub secret: ceph_crypto_key,
    pub starting: bool,
    pub server_challenge: u64,
    pub have_keys: c_uint,
    pub ticket_handlers: rb_root,
    pub auth_authorizer: ceph_x_authorizer,
}

extern "C" {
    pub fn ceph_x_init(ac: *mut ceph_auth_client) -> c_int;
}
