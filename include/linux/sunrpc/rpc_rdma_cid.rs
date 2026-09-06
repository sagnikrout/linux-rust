//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/rpc_rdma_cid.h
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
// * Copyright (c) 2020, Oracle and/or its affiliates.
//
// The rpc_rdma_cid struct records completion ID information. A
// completion ID matches an incoming Send or Receive completion
// to a Completion Queue and to a previous ib_post_*(). The ID
// can then be displayed in an error message or recorded in a
// trace record.
//
// This struct is shared between the server and client RPC/RDMA
// transport implementations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_rdma_cid {
    pub ci_queue_id: u32,
    pub ci_completion_id: c_int,
}
