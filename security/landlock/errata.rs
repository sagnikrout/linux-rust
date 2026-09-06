//! Automatically rewritten from C Header to Rust Module
//! Source: security/landlock/errata.h
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
// Landlock - Errata information
//
// Copyright © 2025 Microsoft Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct landlock_erratum {
    pub abi: c_int,
    pub number: u8,
}

// clang-format off

// clang-format on
//
// Some fixes may require user space to check if they are applied on the running
// kernel before using a specific feature.  For instance, this applies when a
// restriction was previously too restrictive and is now getting relaxed (for
// compatibility or semantic reasons).  However, non-visible changes for
// legitimate use (e.g. security fixes) do not require an erratum.
//
// Only Sparse may not implement __has_include.  If a compiler does not
// implement __has_include, a warning will be printed at boot time (see
// setup.c).
//

pub const LANDLOCK_ERRATA_ABI: c_int = 1;

pub const LANDLOCK_ERRATA_ABI: c_int = 2;

pub const LANDLOCK_ERRATA_ABI: c_int = 3;

pub const LANDLOCK_ERRATA_ABI: c_int = 4;

pub const LANDLOCK_ERRATA_ABI: c_int = 5;

pub const LANDLOCK_ERRATA_ABI: c_int = 6;

//
// For each new erratum, we need to include all the ABI files up to the impacted
// ABI to make all potential future intermediate errata easy to backport.
//
// If such change involves more than one ABI addition, then it must be in a
// dedicated commit with the same Fixes tag as used for the actual fix.
//
// Each commit creating a new security/landlock/errata/abi-*.h file must have a
// Depends-on tag to reference the commit that previously added the line to
// include this new file, except if the original Fixes tag is enough.
//
// Each erratum must be documented in its related ABI file, and a dedicated
// commit must update Documentation/userspace-api/landlock.rst to include this
// erratum.  This commit will not be backported.
//

