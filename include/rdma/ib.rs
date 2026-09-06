//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2010 Intel Corporation.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_addr {
    pub uib_addr8: [__u8; 16],
    pub uib_addr16: [__be16; 8],
    pub uib_addr32: [__be32; 4],
    pub uib_addr64: [__be64; 2],
    pub ib_u: },

}

extern "C" {
    pub fn memcmp(_arg: a1, _arg: a2, ib_addr): sizeof(struct) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_ib {
    pub /: *mut *mut unsigned short int sib_family; / AF_IB,
    pub sib_pkey: __be16,
    pub sib_flowinfo: __be32,
    pub sib_addr: ib_addr,
    pub sib_sid: __be64,
    pub sib_sid_mask: __be64,
    pub sib_scope_id: __u64,
}

//
// The IB interfaces that use write() as bi-directional ioctl() are
// fundamentally unsafe, since there are lots of ways to trigger "write()"
// calls from various contexts with elevated privileges. That includes the
// traditional suid executable error message writes, but also various kernel
// interfaces that can write to file descriptors.
//
// This function provides protection for the legacy API by restricting the
// calling context.
//
