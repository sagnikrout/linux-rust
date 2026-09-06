//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/dst_ops.h
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
pub struct dst_ops {
    pub family: c_ushort,
    pub gc_thresh: c_uint,
    pub ops): *mut *mut void (gc)(struct dst_ops,
    pub cookie): *mut *mut *mut *mut dst_entry  (check)(dst_entry , __u32,
    pub ): *const *const unsigned int (default_advmss)(struct dst_entry,
    pub ): *const *const unsigned int (mtu)(struct dst_entry,
    pub long): *mut *mut *mut *mut u32  (cow_metrics)(struct dst_entry , unsigned,
    pub ): *mut *mut void (destroy)(struct dst_entry,
    pub dev): *mut net_device,
    pub ): *mut *mut *mut void (negative_advice)(struct sock sk, struct dst_entry,
    pub ): *mut *mut void (link_failure)(struct sk_buff,
    pub confirm_neigh): bool,
    pub skb): *mut sk_buff,
    pub skb): *mut *mut *mut *mut int (local_out)(struct net net, struct sock sk, struct sk_buff,
    pub daddr): *const c_void,
    pub daddr): *const c_void,
    pub kmem_cachep: *mut kmem_cache,
    pub ____cacheline_aligned_in_smp: percpu_counter pcpuc_entries,
}

extern "C" {
    pub fn percpu_counter_read_positive(_arg: &dst->pcpuc_entries) -> return;
}
extern "C" {
    pub fn percpu_counter_sum_positive(_arg: &dst->pcpuc_entries) -> return;
}
pub const DST_PERCPU_COUNTER_BATCH: c_int = 32;
extern "C" {
    pub fn percpu_counter_init(_arg: &dst->pcpuc_entries, _arg: 0, _arg: GFP_KERNEL) -> return;
}
