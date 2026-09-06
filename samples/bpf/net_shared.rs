//! Automatically rewritten from C Header to Rust Module
//! Source: samples/bpf/net_shared.h
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
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const ETH_ALEN: c_int = 6;
pub const ETH_P_802_3_MIN: c_uint = 0x0600;
pub const ETH_P_8021Q: c_uint = 0x8100;
pub const ETH_P_8021AD: c_uint = 0x88A8;
pub const ETH_P_IP: c_uint = 0x0800;
pub const ETH_P_IPV6: c_uint = 0x86DD;
pub const ETH_P_ARP: c_uint = 0x0806;
pub const IPPROTO_ICMPV6: c_int = 58;
pub const TC_ACT_OK: c_int = 0;
pub const TC_ACT_SHOT: c_int = 2;
pub const IFNAMSIZ: c_int = 16;

