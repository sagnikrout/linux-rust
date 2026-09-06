//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nf_nat.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_ipv4_range {
    pub flags: c_uint,
    pub min_ip: __be32,
    pub max_ip: __be32,
    pub min: nf_conntrack_man_proto,
    pub max: nf_conntrack_man_proto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_ipv4_multi_range_compat {
    pub rangesize: c_uint,
    pub range: [nf_nat_ipv4_range; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_range {
    pub flags: c_uint,
    pub min_addr: nf_inet_addr,
    pub max_addr: nf_inet_addr,
    pub min_proto: nf_conntrack_man_proto,
    pub max_proto: nf_conntrack_man_proto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_range2 {
    pub flags: c_uint,
    pub min_addr: nf_inet_addr,
    pub max_addr: nf_inet_addr,
    pub min_proto: nf_conntrack_man_proto,
    pub max_proto: nf_conntrack_man_proto,
    pub base_proto: nf_conntrack_man_proto,
}
