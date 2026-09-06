//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/uapi/asm/perf_regs.h
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
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_event_riscv_regs {
    PERF_REG_RISCV_PC,
    PERF_REG_RISCV_RA,
    PERF_REG_RISCV_SP,
    PERF_REG_RISCV_GP,
    PERF_REG_RISCV_TP,
    PERF_REG_RISCV_T0,
    PERF_REG_RISCV_T1,
    PERF_REG_RISCV_T2,
    PERF_REG_RISCV_S0,
    PERF_REG_RISCV_S1,
    PERF_REG_RISCV_A0,
    PERF_REG_RISCV_A1,
    PERF_REG_RISCV_A2,
    PERF_REG_RISCV_A3,
    PERF_REG_RISCV_A4,
    PERF_REG_RISCV_A5,
    PERF_REG_RISCV_A6,
    PERF_REG_RISCV_A7,
    PERF_REG_RISCV_S2,
    PERF_REG_RISCV_S3,
    PERF_REG_RISCV_S4,
    PERF_REG_RISCV_S5,
    PERF_REG_RISCV_S6,
    PERF_REG_RISCV_S7,
    PERF_REG_RISCV_S8,
    PERF_REG_RISCV_S9,
    PERF_REG_RISCV_S10,
    PERF_REG_RISCV_S11,
    PERF_REG_RISCV_T3,
    PERF_REG_RISCV_T4,
    PERF_REG_RISCV_T5,
    PERF_REG_RISCV_T6,
    PERF_REG_RISCV_MAX,
}
