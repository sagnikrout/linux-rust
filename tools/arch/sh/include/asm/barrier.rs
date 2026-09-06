//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/sh/include/asm/barrier.h
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
// Copied from the kernel sources:
//
// Copyright (C) 1999, 2000  Niibe Yutaka  &  Kaz Kojima
// Copyright (C) 2002 Paul Mundt
//
// A brief note on ctrl_barrier(), the control register write barrier.
//
// Legacy SH cores typically require a sequence of 8 nops after
// modification of a control register in order for the changes to take
// effect. On newer cores (like the sh4a and sh5) this is accomplished
// with icbi.
//
// Also note that on sh4a in the icbi case we can forego a synco for the
// write barrier, as it's not necessary for control registers.
//
// Historically we have only done this type of barrier for the MMUCR, but
// it's also necessary for the CCR, so we make it generic here instead.
//

