//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/seg6_local.h
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
// SR-IPv6 implementation
//
// Authors:
// David Lebrun <david.lebrun@uclouvain.be>
// eBPF support: Mathieu Xhonneux <m.xhonneux@gmail.com>
//

extern "C" {
    pub fn seg6_bpf_has_valid_srh(skb: *mut sk_buff) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg6_bpf_srh_state {
    pub bh_lock: local_lock_t,
    pub srh: *mut ipv6_sr_hdr,
    pub hdrlen: u16,
    pub valid: bool,
}
