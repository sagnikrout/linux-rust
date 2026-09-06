//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/net/tuntap_helpers.h
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

pub const GENEVE_HLEN: c_int = 8;
pub const PKT_DATA: c_uint = 0xCB;
pub const TUNTAP_DEFAULT_TTL: c_int = 8;
pub const TUNTAP_DEFAULT_IPID: c_int = 1337;
extern "C" {
    pub fn if_nametoindex(ifname: *const c_char) -> c_uint;
}
extern "C" {
    pub fn finish_ip_csum(_arg: sum) -> return;
}
// ptr = htons(val);
extern "C" {
    pub fn sizeof(ipv6hdr: struct) -> return;
}
extern "C" {
    pub fn sizeof(_arg: *mut udph) -> return;
}
// No extension IPv4 and IPv6 headers addresses are the last fields
extern "C" {
    pub fn sizeof(virtio_net_hdr_v1_hash_tunnel: struct) -> return;
}
