//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/smc.h
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
pub struct netns_smc {
// per cpu counters for SMC
    pub smc_stats: *mut smc_stats __percpu,
// protect fback_rsn
    pub mutex_fback_rsn: mutex,
    pub fback_rsn: *mut smc_stats_rsn,
    pub /: *mut *mut bool limit_smc_hs; / constraint on handshake,

    pub smc_hdr: *mut ctl_table_header,

    pub hs_ctrl: *mut smc_hs_ctrl __rcu,

    pub sysctl_autocorking_size: c_uint,
    pub sysctl_smcr_buf_type: c_uint,
    pub sysctl_smcr_testlink_time: c_int,
    pub sysctl_wmem: c_int,
    pub sysctl_rmem: c_int,
    pub sysctl_max_links_per_lgr: c_int,
    pub sysctl_max_conns_per_lgr: c_int,
    pub sysctl_smcr_max_send_wr: c_uint,
    pub sysctl_smcr_max_recv_wr: c_uint,
}
