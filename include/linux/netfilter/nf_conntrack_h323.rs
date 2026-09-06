//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/nf_conntrack_h323.h
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

// This structure exists only once per master
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_h323_master {
// Original and NATed Q.931 or H.245 signal ports
    pub sig_port: [__be16; IP_CT_DIR_MAX],
// Original and NATed RTP ports
    pub rtp_port: [__be16; H323_RTP_CHANNEL_MAX][IP_CT_DIR_MAX],
// RAS connection timeout
    pub timeout: u_int32_t,
// Next TPKT length (for separate TPKT header and data)
    pub tpkt_len: [u_int16_t; IP_CT_DIR_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfct_h323_nat_hooks {
    pub port): *mut *mut nf_inet_addr addr, __be16,
    pub port): *mut *mut nf_inet_addr addr, __be16,
    pub count): *mut *mut TransportAddress taddr, int,
    pub count): *mut *mut TransportAddress taddr, int,
    pub rtcp_exp): *mut nf_conntrack_expect,
    pub exp): *mut nf_conntrack_expect,
    pub exp): *mut nf_conntrack_expect,
    pub exp): *mut nf_conntrack_expect,
    pub exp): *mut __be16 port, struct nf_conntrack_expect,
}
