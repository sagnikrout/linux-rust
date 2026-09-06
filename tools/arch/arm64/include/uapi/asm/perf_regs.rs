//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/arm64/include/uapi/asm/perf_regs.h
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
pub enum perf_event_arm_regs {
    PERF_REG_ARM64_X0,
    PERF_REG_ARM64_X1,
    PERF_REG_ARM64_X2,
    PERF_REG_ARM64_X3,
    PERF_REG_ARM64_X4,
    PERF_REG_ARM64_X5,
    PERF_REG_ARM64_X6,
    PERF_REG_ARM64_X7,
    PERF_REG_ARM64_X8,
    PERF_REG_ARM64_X9,
    PERF_REG_ARM64_X10,
    PERF_REG_ARM64_X11,
    PERF_REG_ARM64_X12,
    PERF_REG_ARM64_X13,
    PERF_REG_ARM64_X14,
    PERF_REG_ARM64_X15,
    PERF_REG_ARM64_X16,
    PERF_REG_ARM64_X17,
    PERF_REG_ARM64_X18,
    PERF_REG_ARM64_X19,
    PERF_REG_ARM64_X20,
    PERF_REG_ARM64_X21,
    PERF_REG_ARM64_X22,
    PERF_REG_ARM64_X23,
    PERF_REG_ARM64_X24,
    PERF_REG_ARM64_X25,
    PERF_REG_ARM64_X26,
    PERF_REG_ARM64_X27,
    PERF_REG_ARM64_X28,
    PERF_REG_ARM64_X29,
    PERF_REG_ARM64_LR,
    PERF_REG_ARM64_SP,
    PERF_REG_ARM64_PC,
    PERF_REG_ARM64_MAX,

// Extended/pseudo registers
    PERF_REG_ARM64_VG = 46,				/* SVE Vector Granule */
    PERF_REG_ARM64_EXTENDED_MAX
}

