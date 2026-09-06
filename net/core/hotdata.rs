//! Automatically rewritten from C to Rust
//! Source: net/core/hotdata.c
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

    struct net_hotdata net_hotdata __cacheline_aligned = {
    .offload_base = LIST_HEAD_INIT(net_hotdata.offload_base),
    .gro_normal_batch = 8,
    .netdev_budget = 300,
// Must be at least 2 jiffes to guarantee 1 jiffy timeout
    .netdev_budget_usecs = 2 * USEC_PER_SEC / HZ,
    .tstamp_prequeue = 1,
    .max_backlog = 1000,
    .qdisc_max_burst = 1000,
    .dev_tx_weight = 64,
    .dev_rx_weight = 64,
    .sysctl_max_skb_frags = MAX_SKB_FRAGS,
    .sysctl_skb_defer_max = 128,
    .sysctl_mem_pcpu_rsv = SK_MEMORY_PCPU_RESERVE
    };
    EXPORT_SYMBOL(net_hotdata);
    struct net_aligned_data net_aligned_data;
