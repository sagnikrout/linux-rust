//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/gre.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_base_hdr {
    pub flags: __be16,
    pub protocol: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_full_hdr {
    pub fixed_header: gre_base_hdr,
    pub csum: __be16,
    pub reserved1: __be16,
    pub key: __be32,
    pub seq: __be32,
    pub __packed: },
pub const GRE_HEADER_SECTION: c_int = 4;
pub const GREPROTO_CISCO: c_int = 0;
pub const GREPROTO_PPTP: c_int = 1;
pub const GREPROTO_MAX: c_int = 2;
pub const GRE_IP_PROTO_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_protocol {
    pub skb): *mut *mut int (handler)(struct sk_buff,
    pub info): *mut *mut *mut void (err_handler)(struct sk_buff skb, u32,
}

extern "C" {
    pub fn gre_add_protocol(proto: *const gre_protocol, version: u8) -> c_int;
}
extern "C" {
    pub fn gre_del_protocol(proto: *const gre_protocol, version: u8) -> c_int;
}
// ptr = seq;
// ptr = key;
// ptr = 0;
// (__sum16 *)ptr = csum_fold(lco_csum(skb));
