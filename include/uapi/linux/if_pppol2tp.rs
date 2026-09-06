//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_pppol2tp.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Linux PPP over L2TP (PPPoL2TP) Socket Implementation (RFC 2661)
//
// This file supplies definitions required by the PPP over L2TP driver
// (l2tp_ppp.c).  All version information wrt this file is located in l2tp_ppp.c
//
// License:
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

// Structure used to connect() the socket to a particular tunnel UDP
// socket over IPv4.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppol2tp_addr {
    pub fd.: *mut *mut __kernel_pid_t pid; / pid that owns the,
// 0 => current
    pub /: *mut *mut int fd; / FD of UDP socket to use,
    pub /: *mut *mut sockaddr_in addr; / IP address and port to send to,
    pub /: *mut *mut __u16 s_tunnel, s_session; / For matching incoming packets,
    pub /: *mut *mut __u16 d_tunnel, d_session; / For sending outgoing packets,
}

// Structure used to connect() the socket to a particular tunnel UDP
// socket over IPv6.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppol2tpin6_addr {
    pub fd.: *mut *mut __kernel_pid_t pid; / pid that owns the,
// 0 => current
    pub /: *mut *mut int fd; / FD of UDP socket to use,
    pub /: *mut *mut __u16 s_tunnel, s_session; / For matching incoming packets,
    pub /: *mut *mut __u16 d_tunnel, d_session; / For sending outgoing packets,
    pub /: *mut *mut sockaddr_in6 addr; / IP address and port to send to,
}

// The L2TPv3 protocol changes tunnel and session ids from 16 to 32
// bits. So we need a different sockaddr structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppol2tpv3_addr {
    pub fd.: *mut *mut __kernel_pid_t pid; / pid that owns the,
// 0 => current
    pub /: *mut *mut int fd; / FD of UDP or IP socket to use,
    pub /: *mut *mut sockaddr_in addr; / IP address and port to send to,
    pub /: *mut *mut __u32 s_tunnel, s_session; / For matching incoming packets,
    pub /: *mut *mut __u32 d_tunnel, d_session; / For sending outgoing packets,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppol2tpv3in6_addr {
    pub fd.: *mut *mut __kernel_pid_t pid; / pid that owns the,
// 0 => current
    pub /: *mut *mut int fd; / FD of UDP or IP socket to use,
    pub /: *mut *mut __u32 s_tunnel, s_session; / For matching incoming packets,
    pub /: *mut *mut __u32 d_tunnel, d_session; / For sending outgoing packets,
    pub /: *mut *mut sockaddr_in6 addr; / IP address and port to send to,
}

// Socket options:
// DEBUG	- bitmask of debug message categories (not used)
// SENDSEQ	- 0 => don't send packets with sequence numbers
// 1 => send packets with sequence numbers
// RECVSEQ	- 0 => receive packet sequence numbers are optional
// 1 => drop receive packets without sequence numbers
// LNSMODE	- 0 => act as LAC.
// 1 => act as LNS.
// REORDERTO	- reorder timeout (in millisecs). If 0, don't try to reorder.
//
// Debug message categories for the DEBUG socket option (deprecated)
