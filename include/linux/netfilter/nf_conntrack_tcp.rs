//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netfilter/nf_conntrack_tcp.h
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
pub struct ip_ct_tcp_state {
    pub /: *mut *mut u_int32_t td_end; / max of seq + len,
    pub /: *mut *mut u_int32_t td_maxend; / max of ack + max(win, 1),
    pub /: *mut *mut u_int32_t td_maxwin; / max(win),
    pub /: *mut *mut u_int32_t td_maxack; / max of ack,
    pub /: *mut *mut u_int8_t td_scale; / window scale factor,
    pub /: *mut *mut u_int8_t flags; / per direction options,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_ct_tcp {
    pub /: *mut *mut ip_ct_tcp_state seen[2]; / connection parameters per direction,
    pub /: *mut *mut u_int8_t state; / state of the connection (enum tcp_conntrack),
// For detecting stale connections
    pub /: *mut *mut u_int8_t last_dir; / Direction of the last packet (enum ip_conntrack_dir),
    pub /: *mut *mut u_int8_t retrans; / Number of retransmitted packets,
    pub /: *mut *mut u_int8_t last_index; / Index of the last packet,
    pub /: *mut *mut u_int32_t last_seq; / Last sequence number seen in dir,
    pub /: *mut *mut u_int32_t last_ack; / Last sequence number seen in opposite dir,
    pub /: *mut *mut u_int32_t last_end; / Last seq + len,
    pub /: *mut *mut u_int16_t last_win; / Last window advertisement seen in dir,
// For SYN packets while we may be out-of-sync
    pub /: *mut *mut u_int8_t last_wscale; / Last window scaling factor seen,
    pub /: *mut *mut u_int8_t last_flags; / Last flags set,
}
