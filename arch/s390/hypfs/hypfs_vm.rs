//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/hypfs/hypfs_vm.h
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
//
// Hypervisor filesystem for Linux on s390. z/VM implementation.
//
// Copyright IBM Corp. 2006
// Author(s): Michael Holzheu <holzheu@de.ibm.com>
//
pub const DIAG2FC_NAME_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag2fc_data {
    pub version: __u32,
    pub flags: __u32,
    pub used_cpu: __u64,
    pub el_time: __u64,
    pub mem_min_kb: __u64,
    pub mem_max_kb: __u64,
    pub mem_share_kb: __u64,
    pub mem_used_kb: __u64,
    pub pcpus: __u32,
    pub lcpus: __u32,
    pub vcpus: __u32,
    pub ocpus: __u32,
    pub cpu_max: __u32,
    pub cpu_shares: __u32,
    pub cpu_use_samp: __u32,
    pub cpu_delay_samp: __u32,
    pub page_wait_samp: __u32,
    pub idle_samp: __u32,
    pub other_samp: __u32,
    pub total_samp: __u32,
    pub guest_name: [c_char; DIAG2FC_NAME_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct diag2fc_parm_list {
    pub userid: [c_char; DIAG2FC_NAME_LEN],
    pub aci_grp: [c_char; DIAG2FC_NAME_LEN],
    pub addr: __u64,
    pub size: __u32,
    pub fmt: __u32,
}

extern "C" {
    pub fn diag2fc_free(data: *const c_void);
}
