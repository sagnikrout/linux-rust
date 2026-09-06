//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/s390/include/uapi/asm/perf_regs.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_event_s390_regs {
    PERF_REG_S390_R0,
    PERF_REG_S390_R1,
    PERF_REG_S390_R2,
    PERF_REG_S390_R3,
    PERF_REG_S390_R4,
    PERF_REG_S390_R5,
    PERF_REG_S390_R6,
    PERF_REG_S390_R7,
    PERF_REG_S390_R8,
    PERF_REG_S390_R9,
    PERF_REG_S390_R10,
    PERF_REG_S390_R11,
    PERF_REG_S390_R12,
    PERF_REG_S390_R13,
    PERF_REG_S390_R14,
    PERF_REG_S390_R15,
    PERF_REG_S390_FP0,
    PERF_REG_S390_FP1,
    PERF_REG_S390_FP2,
    PERF_REG_S390_FP3,
    PERF_REG_S390_FP4,
    PERF_REG_S390_FP5,
    PERF_REG_S390_FP6,
    PERF_REG_S390_FP7,
    PERF_REG_S390_FP8,
    PERF_REG_S390_FP9,
    PERF_REG_S390_FP10,
    PERF_REG_S390_FP11,
    PERF_REG_S390_FP12,
    PERF_REG_S390_FP13,
    PERF_REG_S390_FP14,
    PERF_REG_S390_FP15,
    PERF_REG_S390_MASK,
    PERF_REG_S390_PC,

    PERF_REG_S390_MAX
}
