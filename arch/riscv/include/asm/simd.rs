//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/simd.h
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
// Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
// Copyright (C) 2023 SiFive
//

//
// may_use_simd - whether it is allowable at this time to issue vector
// instructions or access the vector register file
//
// Callers must not assume that the result remains true beyond the next
// preempt_enable() or return from softirq context.
//
// RISCV_KERNEL_MODE_V is only set while preemption is disabled,
// and is clear whenever preemption is enabled.
//
// Nesting is achieved in preempt_v by spreading the control for
// preemptible and non-preemptible kernel-mode Vector into two fields.
// Only non-preempt_v can nest on top of preempt_v, if non-preempt_v is
// unavailable, then preempt_v is not allowed.
//

