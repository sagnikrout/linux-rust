//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lsm_hook_defs.h
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
// Linux Security Module Hook declarations.
//
// Copyright (C) 2001 WireX Communications, Inc <chris@wirex.com>
// Copyright (C) 2001 Greg Kroah-Hartman <greg@kroah.com>
// Copyright (C) 2001 Networks Associates Technology, Inc <ssmalley@nai.com>
// Copyright (C) 2001 James Morris <jmorris@intercode.com.au>
// Copyright (C) 2001 Silicon Graphics, Inc. (Trust Technology Group)
// Copyright (C) 2015 Intel Corporation.
// Copyright (C) 2015 Casey Schaufler <casey@schaufler-ca.com>
// Copyright (C) 2016 Mellanox Techonologies
// Copyright (C) 2020 Google LLC.
//
// The macro LSM_HOOK is used to define the data structures required by
// the LSM framework using the pattern:
//
// LSM_HOOK(<return_type>, <default_value>, <hook_name>, args...)
//
// struct security_hook_heads {
// #define LSM_HOOK(RET, DEFAULT, NAME, ...) struct hlist_head NAME;
// #include <linux/lsm_hook_defs.h>
// #undef LSM_HOOK
// };
//

// Needed for inode based security check

// key management security hooks

