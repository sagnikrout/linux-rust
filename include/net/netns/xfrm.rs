//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/xfrm.h
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
pub struct xfrm_policy_hash {
    pub table: *mut hlist_head __rcu,
    pub hmask: c_uint,
    pub dbits4: u8,
    pub sbits4: u8,
    pub dbits6: u8,
    pub sbits6: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_policy_hthresh {
    pub work: work_struct,
    pub lock: seqlock_t,
    pub lbits4: u8,
    pub rbits4: u8,
    pub lbits6: u8,
    pub rbits6: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_xfrm {
    pub state_all: list_head,
//
// Hash table to find appropriate SA towards given target (endpoint of
// tunnel or destination of transport mode) allowed by selector.
//
// Main use is finding SA after policy selected tunnel or transport
// mode. Also, it can be used by ah/esp icmp error handler to find
// offending SA.
//
    pub state_bydst: *mut hlist_head __rcu,
    pub state_bysrc: *mut hlist_head __rcu,
    pub state_byspi: *mut hlist_head __rcu,
    pub state_byseq: *mut hlist_head __rcu,
    pub state_cache_input: *mut hlist_head __percpu,
    pub state_hmask: c_uint,
    pub state_num: c_uint,
    pub state_hash_work: work_struct,
    pub policy_all: list_head,
    pub policy_byidx: *mut hlist_head,
    pub policy_idx_hmask: c_uint,
    pub idx_generator: c_uint,
    pub policy_bydst: [xfrm_policy_hash; XFRM_POLICY_MAX],
    pub 2]: *mut *mut unsigned int policy_count[XFRM_POLICY_MAX,
    pub policy_hash_work: work_struct,
    pub policy_hthresh: xfrm_policy_hthresh,
    pub inexact_bins: list_head,
    pub nlsk: *mut sock __rcu,
    pub nlsk_stash: *mut sock,
    pub sysctl_aevent_etime: u32,
    pub sysctl_aevent_rseqth: u32,
    pub sysctl_larval_drop: c_int,
    pub sysctl_acq_expires: u32,
    pub policy_default: [u8; XFRM_POLICY_MAX],
    pub sysctl_hdr: *mut ctl_table_header,

    pub xfrm4_dst_ops: dst_ops,

    pub xfrm6_dst_ops: dst_ops,

    pub xfrm_state_lock: spinlock_t,
    pub xfrm_state_hash_generation: seqcount_spinlock_t,
    pub xfrm_policy_hash_generation: seqcount_spinlock_t,
    pub xfrm_policy_lock: spinlock_t,
    pub xfrm_cfg_mutex: mutex,
    pub nat_keepalive_work: delayed_work,
}
