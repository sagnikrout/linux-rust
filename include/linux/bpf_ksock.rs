//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bpf_ksock.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2026 Isovalent

//
// struct bpf_ksock_create_opts - BPF kernel socket creation parameters
// @family:	Address family: AF_INET or AF_INET6.
// @type:	Socket type: only SOCK_DGRAM supported for now.
// @protocol:	Protocol number (e.g. IPPROTO_UDP), or 0 for the default protocol
// of the given type.
// @reserved:	Must be zero. Reserved for future use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_ksock_create_opts {
    pub family: __u8,
    pub type: __u8,
    pub protocol: __u8,
    pub reserved: __u8,
}

//
// union bpf_ksock_addr - IPv4 or IPv6 socket address
// @sin: IPv4 socket address.
// @sin6: IPv6 socket address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union bpf_ksock_addr {
    pub sin: sockaddr_in,
    pub sin6: sockaddr_in6,
}
