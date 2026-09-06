//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/loongarch/include/uapi/asm/perf_regs.h
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
pub enum perf_event_loongarch_regs {
    PERF_REG_LOONGARCH_PC,
    PERF_REG_LOONGARCH_R1,
    PERF_REG_LOONGARCH_R2,
    PERF_REG_LOONGARCH_R3,
    PERF_REG_LOONGARCH_R4,
    PERF_REG_LOONGARCH_R5,
    PERF_REG_LOONGARCH_R6,
    PERF_REG_LOONGARCH_R7,
    PERF_REG_LOONGARCH_R8,
    PERF_REG_LOONGARCH_R9,
    PERF_REG_LOONGARCH_R10,
    PERF_REG_LOONGARCH_R11,
    PERF_REG_LOONGARCH_R12,
    PERF_REG_LOONGARCH_R13,
    PERF_REG_LOONGARCH_R14,
    PERF_REG_LOONGARCH_R15,
    PERF_REG_LOONGARCH_R16,
    PERF_REG_LOONGARCH_R17,
    PERF_REG_LOONGARCH_R18,
    PERF_REG_LOONGARCH_R19,
    PERF_REG_LOONGARCH_R20,
    PERF_REG_LOONGARCH_R21,
    PERF_REG_LOONGARCH_R22,
    PERF_REG_LOONGARCH_R23,
    PERF_REG_LOONGARCH_R24,
    PERF_REG_LOONGARCH_R25,
    PERF_REG_LOONGARCH_R26,
    PERF_REG_LOONGARCH_R27,
    PERF_REG_LOONGARCH_R28,
    PERF_REG_LOONGARCH_R29,
    PERF_REG_LOONGARCH_R30,
    PERF_REG_LOONGARCH_R31,
    PERF_REG_LOONGARCH_MAX,
}
