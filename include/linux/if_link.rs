//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/if_link.h
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

// We don't want this structure exposed to user space
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_stats {
    pub rx_packets: __u64,
    pub tx_packets: __u64,
    pub rx_bytes: __u64,
    pub tx_bytes: __u64,
    pub broadcast: __u64,
    pub multicast: __u64,
    pub rx_dropped: __u64,
    pub tx_dropped: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifla_vf_info {
    pub vf: __u32,
    pub mac: [__u8; 32],
    pub vlan: __u32,
    pub qos: __u32,
    pub spoofchk: __u32,
    pub linkstate: __u32,
    pub min_tx_rate: __u32,
    pub max_tx_rate: __u32,
    pub rss_query_en: __u32,
    pub trusted: __u32,
    pub vlan_proto: __be16,
}
