//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/mailbox/tegra186-hsp.h
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
// This header provides constants for binding nvidia,tegra186-hsp.
//
// These define the type of mailbox that is to be used (doorbell, shared
// mailbox, shared semaphore or arbitrated semaphore).
//
pub const TEGRA_HSP_MBOX_TYPE_DB: c_uint = 0x0;
pub const TEGRA_HSP_MBOX_TYPE_SM: c_uint = 0x1;
pub const TEGRA_HSP_MBOX_TYPE_SS: c_uint = 0x2;
pub const TEGRA_HSP_MBOX_TYPE_AS: c_uint = 0x3;
//
// These define the types of shared mailbox supported based on data size.
//

//
// These defines represent the bit associated with the given master ID in the
// doorbell registers.
//
pub const TEGRA_HSP_DB_MASTER_CCPLEX: c_int = 17;
pub const TEGRA_HSP_DB_MASTER_BPMP: c_int = 19;
//
// Shared mailboxes are unidirectional, so the direction needs to be specified
// in the device tree.
//
pub const TEGRA_HSP_SM_MASK: c_uint = 0x00ffffff;

