//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/core.h
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
pub struct netns_core {
// core sysctls
    pub sysctl_hdr: *mut ctl_table_header,
    pub sysctl_somaxconn: c_int,
    pub sysctl_txq_reselection: c_int,
    pub sysctl_optmem_max: c_int,
    pub sysctl_txrehash: u8,
    pub sysctl_tstamp_allow_data: u8,
    pub sysctl_bypass_prot_mem: u8,

    pub prot_inuse: *mut prot_inuse __percpu,

    pub rps_default_mask: *mut cpumask,

}
