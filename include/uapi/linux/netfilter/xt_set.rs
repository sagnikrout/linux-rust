//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_set.h
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

// Revision 0 interface: backward compatible with netfilter/iptables
//
// Option flags for kernel operations (xt_set_info_v0)
//
pub const IPSET_SRC: c_uint = 0x01	/* Source match/add */;
pub const IPSET_DST: c_uint = 0x02	/* Destination match/add */;
pub const IPSET_MATCH_INV: c_uint = 0x04	/* Inverse matching */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info_v0 {
    pub index: ip_set_id_t,
    pub 1]: __u32 flags[IPSET_DIM_MAX +,
    pub __flags: [__u32; IPSET_DIM_MAX],
    pub dim: __u8,
    pub flags: __u8,
    pub compat: },
    pub u: },
}

// match and target infos
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info_match_v0 {
    pub match_set: xt_set_info_v0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info_target_v0 {
    pub add_set: xt_set_info_v0,
    pub del_set: xt_set_info_v0,
}

// Revision 1  match and target
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info {
    pub index: ip_set_id_t,
    pub dim: __u8,
    pub flags: __u8,
}

// match and target infos
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info_match_v1 {
    pub match_set: xt_set_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info_target_v1 {
    pub add_set: xt_set_info,
    pub del_set: xt_set_info,
}

// Revision 2 target
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info_target_v2 {
    pub add_set: xt_set_info,
    pub del_set: xt_set_info,
    pub flags: __u32,
    pub timeout: __u32,
}

// Revision 3 match
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info_match_v3 {
    pub match_set: xt_set_info,
    pub packets: ip_set_counter_match0,
    pub bytes: ip_set_counter_match0,
    pub flags: __u32,
}

// Revision 3 target
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info_target_v3 {
    pub add_set: xt_set_info,
    pub del_set: xt_set_info,
    pub map_set: xt_set_info,
    pub flags: __u32,
    pub timeout: __u32,
}

// Revision 4 match
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_set_info_match_v4 {
    pub match_set: xt_set_info,
    pub packets: ip_set_counter_match,
    pub bytes: ip_set_counter_match,
    pub flags: __u32,
}
