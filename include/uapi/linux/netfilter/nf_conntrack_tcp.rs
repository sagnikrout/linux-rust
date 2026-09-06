//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nf_conntrack_tcp.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// TCP tracking.

// This is exposed to userspace (ctnetlink)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_conntrack {
    TCP_CONNTRACK_NONE,
    TCP_CONNTRACK_SYN_SENT,
    TCP_CONNTRACK_SYN_RECV,
    TCP_CONNTRACK_ESTABLISHED,
    TCP_CONNTRACK_FIN_WAIT,
    TCP_CONNTRACK_CLOSE_WAIT,
    TCP_CONNTRACK_LAST_ACK,
    TCP_CONNTRACK_TIME_WAIT,
    TCP_CONNTRACK_CLOSE,
    TCP_CONNTRACK_LISTEN,	/* obsolete */

    TCP_CONNTRACK_MAX,
    TCP_CONNTRACK_IGNORE,
    TCP_CONNTRACK_RETRANS,
    TCP_CONNTRACK_UNACK,
    TCP_CONNTRACK_TIMEOUT_MAX
}

// Window scaling is advertised by the sender
pub const IP_CT_TCP_FLAG_WINDOW_SCALE: c_uint = 0x01;
// SACK is permitted by the sender
pub const IP_CT_TCP_FLAG_SACK_PERM: c_uint = 0x02;
// This sender sent FIN first
pub const IP_CT_TCP_FLAG_CLOSE_INIT: c_uint = 0x04;
// Be liberal in window checking
pub const IP_CT_TCP_FLAG_BE_LIBERAL: c_uint = 0x08;
// Has unacknowledged data
pub const IP_CT_TCP_FLAG_DATA_UNACKNOWLEDGED: c_uint = 0x10;
// The field td_maxack has been set
pub const IP_CT_TCP_FLAG_MAXACK_SET: c_uint = 0x20;
// Marks possibility for expected RFC5961 challenge ACK
pub const IP_CT_EXP_CHALLENGE_ACK: c_uint = 0x40;
// Simultaneous open initialized
pub const IP_CT_TCP_SIMULTANEOUS_OPEN: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_tcp_flags {
    pub flags: __u8,
    pub mask: __u8,
}
