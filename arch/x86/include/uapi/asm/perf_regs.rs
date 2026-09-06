//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/perf_regs.h
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
pub enum perf_event_x86_regs {
    PERF_REG_X86_AX,
    PERF_REG_X86_BX,
    PERF_REG_X86_CX,
    PERF_REG_X86_DX,
    PERF_REG_X86_SI,
    PERF_REG_X86_DI,
    PERF_REG_X86_BP,
    PERF_REG_X86_SP,
    PERF_REG_X86_IP,
    PERF_REG_X86_FLAGS,
    PERF_REG_X86_CS,
    PERF_REG_X86_SS,
    PERF_REG_X86_DS,
    PERF_REG_X86_ES,
    PERF_REG_X86_FS,
    PERF_REG_X86_GS,
    PERF_REG_X86_R8,
    PERF_REG_X86_R9,
    PERF_REG_X86_R10,
    PERF_REG_X86_R11,
    PERF_REG_X86_R12,
    PERF_REG_X86_R13,
    PERF_REG_X86_R14,
    PERF_REG_X86_R15,
// These are the limits for the GPRs.
    PERF_REG_X86_32_MAX = PERF_REG_X86_GS + 1,
    PERF_REG_X86_64_MAX = PERF_REG_X86_R15 + 1,

// These all need two bits set because they are 128bit
    PERF_REG_X86_XMM0  = 32,
    PERF_REG_X86_XMM1  = 34,
    PERF_REG_X86_XMM2  = 36,
    PERF_REG_X86_XMM3  = 38,
    PERF_REG_X86_XMM4  = 40,
    PERF_REG_X86_XMM5  = 42,
    PERF_REG_X86_XMM6  = 44,
    PERF_REG_X86_XMM7  = 46,
    PERF_REG_X86_XMM8  = 48,
    PERF_REG_X86_XMM9  = 50,
    PERF_REG_X86_XMM10 = 52,
    PERF_REG_X86_XMM11 = 54,
    PERF_REG_X86_XMM12 = 56,
    PERF_REG_X86_XMM13 = 58,
    PERF_REG_X86_XMM14 = 60,
    PERF_REG_X86_XMM15 = 62,

// These include both GPRs and XMMX registers
    PERF_REG_X86_XMM_MAX = PERF_REG_X86_XMM15 + 2,
}

