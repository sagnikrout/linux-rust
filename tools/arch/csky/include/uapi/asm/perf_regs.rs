//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/csky/include/uapi/asm/perf_regs.h
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
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.
// Index of struct pt_regs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_event_csky_regs {
    PERF_REG_CSKY_TLS,
    PERF_REG_CSKY_LR,
    PERF_REG_CSKY_PC,
    PERF_REG_CSKY_SR,
    PERF_REG_CSKY_SP,
    PERF_REG_CSKY_ORIG_A0,
    PERF_REG_CSKY_A0,
    PERF_REG_CSKY_A1,
    PERF_REG_CSKY_A2,
    PERF_REG_CSKY_A3,
    PERF_REG_CSKY_REGS0,
    PERF_REG_CSKY_REGS1,
    PERF_REG_CSKY_REGS2,
    PERF_REG_CSKY_REGS3,
    PERF_REG_CSKY_REGS4,
    PERF_REG_CSKY_REGS5,
    PERF_REG_CSKY_REGS6,
    PERF_REG_CSKY_REGS7,
    PERF_REG_CSKY_REGS8,
    PERF_REG_CSKY_REGS9,

    PERF_REG_CSKY_EXREGS0,
    PERF_REG_CSKY_EXREGS1,
    PERF_REG_CSKY_EXREGS2,
    PERF_REG_CSKY_EXREGS3,
    PERF_REG_CSKY_EXREGS4,
    PERF_REG_CSKY_EXREGS5,
    PERF_REG_CSKY_EXREGS6,
    PERF_REG_CSKY_EXREGS7,
    PERF_REG_CSKY_EXREGS8,
    PERF_REG_CSKY_EXREGS9,
    PERF_REG_CSKY_EXREGS10,
    PERF_REG_CSKY_EXREGS11,
    PERF_REG_CSKY_EXREGS12,
    PERF_REG_CSKY_EXREGS13,
    PERF_REG_CSKY_EXREGS14,
    PERF_REG_CSKY_HI,
    PERF_REG_CSKY_LO,
    PERF_REG_CSKY_DCSR,

    PERF_REG_CSKY_MAX,
}
