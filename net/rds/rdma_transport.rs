//! Automatically rewritten from C Header to Rust Module
//! Source: net/rds/rdma_transport.h
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

// RDMA_CM also uses 16385 as the listener port.
pub const RDS_CM_PORT: c_int = 16385;
pub const RDS_RDMA_RESOLVE_TIMEOUT_MS: c_int = 5000;
// Below reject reason is for legacy interoperability issue with non-linux
// RDS endpoints where older version incompatibility is conveyed via value 1.
// For future version(s), proper encoded reject reason should be used.
//
pub const RDS_RDMA_REJ_INCOMPAT: c_int = 1;
// from ib.c
extern "C" {
    pub fn rds_ib_init() -> c_int;
}
extern "C" {
    pub fn rds_ib_exit();
}
