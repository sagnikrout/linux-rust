//! Automatically rewritten from C Header to Rust Module
//! Source: net/mpls/internal.h
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

// put a reasonable limit on the number of labels
// we will accept from userspace
//
pub const MAX_NEW_LABELS: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpls_entry_decoded {
    pub label: u32,
    pub ttl: u8,
    pub tc: u8,
    pub bos: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpls_pcpu_stats {
    pub stats: mpls_link_stats,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpls_dev {
    pub input_enabled: c_int,
    pub dev: *mut net_device,
    pub stats: *mut mpls_pcpu_stats __percpu,
    pub sysctl: *mut ctl_table_header,
    pub rcu: rcu_head,
}

// This maximum ha length copied from the definition of struct neighbour

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpls_payload_type {
    MPT_UNSPEC, /* IPv4 or IPv6 */
    MPT_IPV4 = 4,
    MPT_IPV6 = 6,

// Other types not implemented:
// - Pseudo-wire with or without control word (RFC4385)
// - GAL (RFC5586)
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpls_nh {
    pub nh_dev: *mut net_device,
    pub nh_dev_tracker: netdevice_tracker,
// nh_flags is accessed under RCU in the packet path; it is
// modified handling netdev events with rtnl lock held
//
    pub nh_flags: c_uint,
    pub nh_labels: u8,
    pub nh_via_alen: u8,
    pub nh_via_table: u8,
    pub nh_reserved1: u8,
    pub nh_label: [u32; ],
}

// offset of via from beginning of mpls_nh

// all nexthops within a route have the same size based on the
// max number of labels and max via length across all nexthops
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpls_ttl_propagation {
    MPLS_TTL_PROP_DEFAULT,
    MPLS_TTL_PROP_ENABLED,
    MPLS_TTL_PROP_DISABLED,
}

// The route, nexthops and vias are stored together in the same memory
// block:
//
// +----------------------+
// | mpls_route           |
// +----------------------+
// | mpls_nh 0            |
// +----------------------+
// | alignment padding    |   4 bytes for odd number of labels
// +----------------------+
// | via[rt_max_alen] 0   |
// +----------------------+
// | alignment padding    |   via's aligned on sizeof(unsigned long)
// +----------------------+
// | ...                  |
// +----------------------+
// | mpls_nh n-1          |
// +----------------------+
// | via[rt_max_alen] n-1 |
// +----------------------+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpls_route {
    pub rt_rcu: rcu_head,
    pub rt_protocol: u8,
    pub rt_payload_type: u8,
    pub rt_max_alen: u8,
    pub rt_ttl_propagate: u8,
    pub rt_nhn: u8,
// rt_nhn_alive is accessed under RCU in the packet path; it
// is modified handling netdev events with rtnl lock held
//
    pub rt_nhn_alive: u8,
    pub rt_nh_size: u8,
    pub rt_via_offset: u8,
    pub rt_reserved1: u8,
    pub rt_nh: [mpls_nh; ],
}

extern "C" {
    pub fn rcu_dereference(_arg: dev->mpls_ptr) -> return;
}
extern "C" {
    pub fn mpls_dereference(_arg: net, _arg: dev->mpls_ptr) -> return;
}
extern "C" {
    pub fn mpls_output_possible(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn mpls_dev_mtu(dev: *const net_device) -> c_uint;
}
extern "C" {
    pub fn mpls_pkt_too_big(skb: *const sk_buff, mtu: c_uint) -> bool;
}
