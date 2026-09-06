//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/timer.h
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
// linux/include/linux/sunrpc/timer.h
//
// Declarations for the RPC transport timer.
//
// Copyright (C) 2002 Trond Myklebust <trond.myklebust@fys.uio.no>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_rtt {
    pub /: *mut *mut unsigned long timeo; / default timeout value,
    pub /: *mut *mut unsigned long srtt[5]; / smoothed round trip time << 3,
    pub /: *mut *mut unsigned long sdrtt[5]; / smoothed medium deviation of RTT,
    pub /: *mut *mut int ntimeouts[5]; / Number of timeouts for the last request,
}

extern "C" {
    pub fn rpc_init_rtt(rt: *mut rpc_rtt, timeo: c_ulong);
}
extern "C" {
    pub fn rpc_update_rtt(rt: *mut rpc_rtt, timer: unsigned, m: c_long);
}
extern "C" {
    pub fn rpc_calc_rto(rt: *mut rpc_rtt, timer: unsigned) -> c_ulong;
}
// t = ntimeo;
