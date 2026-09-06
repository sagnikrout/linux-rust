//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireguard/allowedips.h
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
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct allowedips_node {
    pub peer: *mut wg_peer __rcu,
    pub bit: [*mut allowedips_node __rcu; 2],
    pub bitlen: u8 cidr, bit_at_a, bit_at_b,,
    pub __aligned(__alignof(u64)): u8 bits[16],
// Keep rarely used members at bottom to be beyond cache line.
    pub parent_bit_packed: c_ulong,
    pub peer_list: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct allowedips {
    pub root4: *mut allowedips_node __rcu,
    pub root6: *mut allowedips_node __rcu,
    pub seq: u64,
    pub /: *mut *mut } __aligned(4); / We pack the lower 2 bits of &root, but m68k only gives 16-bit alignment.,
    pub table): *mut void wg_allowedips_init(struct allowedips,
    pub mutex): *mut *mut void wg_allowedips_free(struct allowedips table, struct mutex,
    pub lock): *mut *mut u8 cidr, struct wg_peer peer, struct mutex,
    pub lock): *mut *mut u8 cidr, struct wg_peer peer, struct mutex,
    pub lock): *mut *mut u8 cidr, struct wg_peer peer, struct mutex,
    pub lock): *mut *mut u8 cidr, struct wg_peer peer, struct mutex,
    pub lock): *mut *mut wg_peer peer, mutex,
// The ip input pointer should be __aligned(__alignof(u64)))
    pub cidr): *mut *mut int wg_allowedips_read_node(struct allowedips_node node, u8 ip[16], u8,
// These return a strong reference to a peer:
    pub skb): *mut sk_buff,
    pub skb): *mut sk_buff,

    pub wg_allowedips_selftest(void): bool,

    pub wg_allowedips_slab_init(void): c_int,
    pub wg_allowedips_slab_uninit(void): c_void,
