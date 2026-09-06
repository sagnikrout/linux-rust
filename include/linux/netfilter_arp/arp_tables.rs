//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter_arp/arp_tables.h
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
// Format of an ARP firewall descriptor
//
// src, tgt, src_mask, tgt_mask, arpop, arpop_mask are always stored in
// network byte order.
// flags are stored in host byte order (of course).
//

// Standard entry.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpt_standard {
    pub entry: arpt_entry,
    pub target: xt_standard_target,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arpt_error {
    pub entry: arpt_entry,
    pub target: xt_error_target,
}

extern "C" {
    pub fn arpt_unregister_table(net: *mut net, name: *const c_char);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_arpt_entry {
    pub arp: arpt_arp,
    pub target_offset: __u16,
    pub next_offset: __u16,
    pub comefrom: compat_uint_t,
    pub counters: compat_xt_counters,
    pub elems: [c_uchar; ],
}

