//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/in6.h
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
//
// Types and definitions for AF_INET6
// Linux INET6 implementation
//
// Authors:
// Pedro Roque		<roque@di.fc.ul.pt>
//
// Sources:
// IPv6 Program Interfaces for BSD Systems
// <draft-ietf-ipngwg-bsd-api-05.txt>
//
// Advanced Sockets API for IPv6
// <draft-stevens-advanced-api-00.txt>
//

// Large enough to hold both sockaddr_in and sockaddr_in6.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_inet {
    pub sa_family: c_ushort,
    pub short)]: sizeof(unsigned,
}

// IPv6 Wildcard Address (::) and Loopback Address (::1) defined in RFC2553
// NOTE: Be aware the IN6ADDR_* constants and in6addr_* externals are defined
// in network byte order, not in host byte order as are the IPv4 equivalents
//

