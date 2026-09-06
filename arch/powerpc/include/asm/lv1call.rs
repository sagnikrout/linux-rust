//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/lv1call.h
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
// PS3 hvcall interface.
//
// Copyright (C) 2006 Sony Computer Entertainment Inc.
// Copyright 2006 Sony Corp.
// Copyright 2003, 2004 (c) MontaVista Software, Inc.
//

// lv1 call declaration macros

// Macro flag: #define LV1_0_IN_0_OUT_ARGS

//
// This LV1_CALL() macro is for use by callers.  It expands into an
// inline call wrapper and an underscored HV call declaration.  The
// wrapper can be used to instrument the lv1 call interface.  The
// file lv1call.S defines its own LV1_CALL() macro to expand into
// the actual underscored call definition.
//

// lv1 call table
