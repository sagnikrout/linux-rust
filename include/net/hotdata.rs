//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/hotdata.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_defer_node {
    pub defer_list: llist_head,
    pub defer_count: atomic_long_t,
    pub ____cacheline_aligned_in_smp: },
// Read mostly data used in network fast paths.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_hotdata {

    pub ip_packet_offload: packet_offload,
    pub tcpv4_offload: net_offload,
    pub tcp_protocol: net_protocol,
    pub udpv4_offload: net_offload,
    pub udp_protocol: net_protocol,
    pub ipv6_packet_offload: packet_offload,
    pub tcpv6_offload: net_offload,

    pub tcpv6_protocol: inet6_protocol,
    pub udpv6_protocol: inet6_protocol,

    pub udpv6_offload: net_offload,

    pub offload_base: list_head,
    pub skbuff_cache: *mut kmem_cache,
    pub skbuff_fclone_cache: *mut kmem_cache,
    pub skb_small_head_cache: *mut kmem_cache,

    pub rps_sock_flow_table: rps_tag_ptr,
    pub rps_cpu_mask: u32,

    pub skb_defer_nodes: *mut skb_defer_node __percpu,
    pub gro_normal_batch: c_int,
    pub netdev_budget: c_int,
    pub netdev_budget_usecs: c_int,
    pub tstamp_prequeue: c_int,
    pub max_backlog: c_int,
    pub qdisc_max_burst: c_int,
    pub dev_tx_weight: c_int,
    pub dev_rx_weight: c_int,
    pub sysctl_max_skb_frags: c_int,
    pub sysctl_skb_defer_max: c_int,
    pub sysctl_mem_pcpu_rsv: c_int,
}

