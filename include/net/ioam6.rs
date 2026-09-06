//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ioam6.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// IPv6 IOAM implementation
//
// Author:
// Justin Iurman <justin.iurman@uliege.be>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioam6_namespace {
    pub head: rhash_head,
    pub rcu: rcu_head,
    pub schema: *mut ioam6_schema __rcu,
    pub id: __be16,
    pub data: __be32,
    pub data_wide: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioam6_schema {
    pub head: rhash_head,
    pub rcu: rcu_head,
    pub ns: *mut ioam6_namespace __rcu,
    pub id: u32,
    pub len: c_int,
    pub hdr: __be32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioam6_pernet_data {
    pub lock: mutex,
    pub namespaces: rhashtable,
    pub schemas: rhashtable,
}

extern "C" {
    pub fn ioam6_trace_compute_nodelen(trace_type: u32) -> u8;
}
extern "C" {
    pub fn ioam6_init() -> c_int;
}
extern "C" {
    pub fn ioam6_exit();
}
extern "C" {
    pub fn ioam6_iptunnel_init() -> c_int;
}
extern "C" {
    pub fn ioam6_iptunnel_exit();
}
