//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/systbl.c
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
// This file contains the table of syscall-handling functions.
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Largely rewritten by Cort Dougan (cort@cs.nmt.edu)
// and Paul Mackerras.
//
// Adapted for iSeries by Mike Corrigan (mikejc@us.ibm.com)
// PPC64 updates by Dave Engebretsen (engebret@us.ibm.com)
//

//
// Coerce syscall handlers with arbitrary parameters to common type
// requires cast to void* to avoid -Wcast-function-type.
//

    const syscall_fn sys_call_table[] = {

    };

    const syscall_fn compat_sys_call_table[] = {

    };
