//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/netcnt_common.h
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

pub const MAX_PERCPU_PACKETS: c_int = 32;
// sizeof(struct bpf_local_storage_elem):
//
// It is about 128 bytes on x86_64 and 512 bytes on s390x, but allocate more to
// account for possible layout changes, different architectures, etc.
// The kernel will wrap up to PAGE_SIZE internally anyway.
//
pub const SIZEOF_BPF_LOCAL_STORAGE_ELEM: c_int = 768;
// Try to estimate kernel's BPF_LOCAL_STORAGE_MAX_VALUE_SIZE:

pub const PCPU_MIN_UNIT_SIZE: c_int = 32768;
#[repr(C)]
#[derive(Copy, Clone)]
pub union percpu_net_cnt {
    pub packets: __u64,
    pub bytes: __u64,
    pub prev_ts: __u64,
    pub prev_packets: __u64,
    pub prev_bytes: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union net_cnt {
    pub packets: __u64,
    pub bytes: __u64,
}
