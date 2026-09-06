//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/atalk.h
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

//
// AppleTalk networking structures
//
// The following are directly referenced from the University Of Michigan
// netatalk for compatibility reasons.
//
pub const ATPORT_FIRST: c_int = 1;
pub const ATPORT_RESERVED: c_int = 128;

pub const DDP_MAXSZ: c_int = 587;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atalk_addr {
    pub s_net: __be16,
    pub s_node: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_at {
    pub sat_family: __kernel_sa_family_t,
    pub sat_port: __u8,
    pub sat_addr: atalk_addr,
    pub sat_zero: [c_char; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atalk_netrange {
    pub nr_phase: __u8,
    pub nr_firstnet: __be16,
    pub nr_lastnet: __be16,
}
