//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lsm_count.h
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
// Copyright (C) 2023 Google LLC.
//

//
// Macros to count the number of LSMs enabled in the kernel at compile time.
//
// Capabilities is enabled when CONFIG_SECURITY is enabled.
//

// Macro flag: #define CAPABILITIES_ENABLED

// Macro flag: #define SELINUX_ENABLED

// Macro flag: #define SMACK_ENABLED

// Macro flag: #define APPARMOR_ENABLED

// Macro flag: #define TOMOYO_ENABLED

// Macro flag: #define YAMA_ENABLED

// Macro flag: #define LOADPIN_ENABLED

// Macro flag: #define LOCKDOWN_ENABLED

// Macro flag: #define SAFESETID_ENABLED

// Macro flag: #define BPF_LSM_ENABLED

// Macro flag: #define LANDLOCK_ENABLED

// Macro flag: #define IMA_ENABLED

// Macro flag: #define EVM_ENABLED

// Macro flag: #define IPE_ENABLED

//
// There is a trailing comma that we need to be accounted for. This is done by
// using a skipped argument in __COUNT_LSMS
//

pub const MAX_LSM_COUNT: c_int = 0;

