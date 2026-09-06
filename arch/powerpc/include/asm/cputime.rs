//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cputime.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Definitions for measuring cputime on powerpc machines.
//
// Copyright (C) 2006 Paul Mackerras, IBM Corp.
//
// If we have CONFIG_VIRT_CPU_ACCOUNTING_NATIVE, we measure cpu time in
// the same units as the timebase.  Otherwise we measure cpu time
// in jiffies using the generic definitions.
//

//
// PPC64 uses PACA which is task independent for storing accounting data while
// PPC32 uses struct thread_info, therefore at task switch the accounting data
// has to be populated in the new task
//

//
// account_cpu_user_entry/exit runs "unreconciled", so can't trace,
// can't use get_paca()
//

