//! Automatically rewritten from C Header to Rust Module
//! Source: net/ipv6/ila/ila.h
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
// Copyright (c) 2015 Tom Herbert <tom@herbertland.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ila_locator {
    pub v8: [__u8; 8],
    pub v16: [__be16; 4],
    pub v32: [__be32; 2],
    pub v64: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ila_identifier {

    pub __space:4: u8,
    pub csum_neutral:1: u8,
    pub type:3: u8,

    pub type:3: u8,
    pub csum_neutral:1: u8,
    pub __space:4: u8,
    pub __space2: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ila_addr {
    pub addr: in6_addr,
    pub loc: ila_locator,
    pub ident: ila_identifier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ila_params {
    pub locator: ila_locator,
    pub locator_match: ila_locator,
    pub csum_diff: __wsum,
    pub csum_mode: u8,
    pub ident_type: u8,
}

extern "C" {
    pub fn csum_partial(_arg: diff, _arg: sizeof(diff), _arg: 0) -> return;
}
extern "C" {
    pub fn ila_init_saved_csum(p: *mut ila_params);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ila_net {
    pub rhash_table: rhashtable,
    pub /: *mut *mut *mut spinlock_t locks; / Bucket locks for entry manipulation,
    pub locks_mask: c_uint,
    pub hooks_registered: bool,
    pub xlat: },
}

extern "C" {
    pub fn ila_lwt_init() -> c_int;
}
extern "C" {
    pub fn ila_lwt_fini();
}
extern "C" {
    pub fn ila_xlat_init_net(net: *mut net) -> c_int;
}
extern "C" {
    pub fn ila_xlat_pre_exit_net(net: *mut net);
}
extern "C" {
    pub fn ila_xlat_exit_net(net: *mut net);
}
extern "C" {
    pub fn ila_xlat_nl_cmd_add_mapping(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ila_xlat_nl_cmd_del_mapping(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ila_xlat_nl_cmd_get_mapping(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ila_xlat_nl_cmd_flush(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn ila_xlat_nl_dump_start(cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ila_xlat_nl_dump_done(cb: *mut netlink_callback) -> c_int;
}
extern "C" {
    pub fn ila_xlat_nl_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
