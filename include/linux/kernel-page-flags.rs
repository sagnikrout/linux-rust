//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kernel-page-flags.h
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

// kernel hacking assistances
// WARNING: subject to change, never rely on them!
//
pub const KPF_RESERVED: c_int = 32;
pub const KPF_MLOCKED: c_int = 33;
pub const KPF_OWNER_2: c_int = 34;
pub const KPF_PRIVATE: c_int = 35;
pub const KPF_PRIVATE_2: c_int = 36;
pub const KPF_OWNER_PRIVATE: c_int = 37;
pub const KPF_ARCH: c_int = 38;
pub const KPF_SOFTDIRTY: c_int = 40;
pub const KPF_ARCH_2: c_int = 41;
pub const KPF_ARCH_3: c_int = 42;
