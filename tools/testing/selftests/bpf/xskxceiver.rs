//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/xskxceiver.h
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
// Copyright(c) 2020 Intel Corporation.
//

pub const SOL_XDP: c_int = 283;

pub const AF_XDP: c_int = 44;

pub const MAX_TEARDOWN_ITER: c_int = 10;
pub const MAX_ETH_JUMBO_SIZE: c_int = 9000;
pub const SOCK_RECONF_CTR: c_int = 10;
pub const RX_FULL_RXQSIZE: c_int = 32;
pub const UMEM_HEADROOM_TEST_SIZE: c_int = 128;

pub const NUM_MAC_ADDRESSES: c_int = 4;
