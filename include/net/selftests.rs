//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/selftests.h
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
pub struct net_packet_attrs {
    pub src: *const c_uchar,
    pub dst: *const c_uchar,
    pub ip_src: u32,
    pub ip_dst: u32,
    pub tcp: bool,
    pub sport: u16,
    pub dport: u16,
    pub timeout: c_int,
    pub size: c_int,
    pub max_size: c_int,
    pub id: u8,
    pub queue_mapping: u16,
    pub bad_csum: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_test_priv {
    pub packet: *mut net_packet_attrs,
    pub pt: packet_type,
    pub comp: completion,
    pub double_vlan: c_int,
    pub vlan_id: c_int,
    pub ok: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netsfhdr {
    pub version: __be32,
    pub magic: __be64,
    pub id: u8,
    pub __packed: },

pub const NET_TEST_PKT_MAGIC: c_uint = 0xdeadcafecafedeadULL;

    pub attr): *mut net_packet_attrs,
    pub buf): *mut u64,
    pub net_selftest_get_count(void): c_int,
    pub data): *mut void net_selftest_get_strings(u8,

    pub NULL: return,
    pub 0: return,

