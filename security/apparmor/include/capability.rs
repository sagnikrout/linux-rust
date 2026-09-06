//! Automatically rewritten from C Header to Rust Module
//! Source: security/apparmor/include/capability.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// AppArmor security module
//
// This file contains AppArmor capability mediation definitions.
//
// Copyright (C) 1998-2008 Novell/SUSE
// Copyright 2009-2013 Canonical Ltd.
//

// aa_caps - confinement data for capabilities
// @allowed: capabilities mask
// @audit: caps that are to be audited
// @denied: caps that are explicitly denied
// @quiet: caps that should not be audited
// @kill: caps that when requested will result in the task being killed
// @extended: caps that are subject finer grained mediation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aa_caps {
    pub allow: kernel_cap_t,
    pub audit: kernel_cap_t,
    pub denied: kernel_cap_t,
    pub quiet: kernel_cap_t,
    pub kill: kernel_cap_t,
    pub extended: kernel_cap_t,
}

extern "C" {
    pub fn aa_profile_capget(profile: *const aa_profile) -> kernel_cap_t;
}
// NOP
