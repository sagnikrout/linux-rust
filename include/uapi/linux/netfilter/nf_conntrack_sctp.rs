//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nf_conntrack_sctp.h
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
// SCTP tracking.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sctp_conntrack {
    SCTP_CONNTRACK_NONE,
    SCTP_CONNTRACK_CLOSED,
    SCTP_CONNTRACK_COOKIE_WAIT,
    SCTP_CONNTRACK_COOKIE_ECHOED,
    SCTP_CONNTRACK_ESTABLISHED,
    SCTP_CONNTRACK_SHUTDOWN_SENT,
    SCTP_CONNTRACK_SHUTDOWN_RECD,
    SCTP_CONNTRACK_SHUTDOWN_ACK_SENT,
    SCTP_CONNTRACK_HEARTBEAT_SENT,
    SCTP_CONNTRACK_HEARTBEAT_ACKED,	/* no longer used */
    SCTP_CONNTRACK_MAX
}
