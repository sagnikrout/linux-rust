//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/powerpc/include/uapi/asm/perf_regs.h
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
pub enum perf_event_powerpc_regs {
    PERF_REG_POWERPC_R0,
    PERF_REG_POWERPC_R1,
    PERF_REG_POWERPC_R2,
    PERF_REG_POWERPC_R3,
    PERF_REG_POWERPC_R4,
    PERF_REG_POWERPC_R5,
    PERF_REG_POWERPC_R6,
    PERF_REG_POWERPC_R7,
    PERF_REG_POWERPC_R8,
    PERF_REG_POWERPC_R9,
    PERF_REG_POWERPC_R10,
    PERF_REG_POWERPC_R11,
    PERF_REG_POWERPC_R12,
    PERF_REG_POWERPC_R13,
    PERF_REG_POWERPC_R14,
    PERF_REG_POWERPC_R15,
    PERF_REG_POWERPC_R16,
    PERF_REG_POWERPC_R17,
    PERF_REG_POWERPC_R18,
    PERF_REG_POWERPC_R19,
    PERF_REG_POWERPC_R20,
    PERF_REG_POWERPC_R21,
    PERF_REG_POWERPC_R22,
    PERF_REG_POWERPC_R23,
    PERF_REG_POWERPC_R24,
    PERF_REG_POWERPC_R25,
    PERF_REG_POWERPC_R26,
    PERF_REG_POWERPC_R27,
    PERF_REG_POWERPC_R28,
    PERF_REG_POWERPC_R29,
    PERF_REG_POWERPC_R30,
    PERF_REG_POWERPC_R31,
    PERF_REG_POWERPC_NIP,
    PERF_REG_POWERPC_MSR,
    PERF_REG_POWERPC_ORIG_R3,
    PERF_REG_POWERPC_CTR,
    PERF_REG_POWERPC_LINK,
    PERF_REG_POWERPC_XER,
    PERF_REG_POWERPC_CCR,
    PERF_REG_POWERPC_SOFTE,
    PERF_REG_POWERPC_TRAP,
    PERF_REG_POWERPC_DAR,
    PERF_REG_POWERPC_DSISR,
    PERF_REG_POWERPC_SIER,
    PERF_REG_POWERPC_MMCRA,
// Extended registers
    PERF_REG_POWERPC_MMCR0,
    PERF_REG_POWERPC_MMCR1,
    PERF_REG_POWERPC_MMCR2,
    PERF_REG_POWERPC_MMCR3,
    PERF_REG_POWERPC_SIER2,
    PERF_REG_POWERPC_SIER3,
    PERF_REG_POWERPC_PMC1,
    PERF_REG_POWERPC_PMC2,
    PERF_REG_POWERPC_PMC3,
    PERF_REG_POWERPC_PMC4,
    PERF_REG_POWERPC_PMC5,
    PERF_REG_POWERPC_PMC6,
    PERF_REG_POWERPC_SDAR,
    PERF_REG_POWERPC_SIAR,
// Max mask value for interrupt regs w/o extended regs
    PERF_REG_POWERPC_MAX = PERF_REG_POWERPC_MMCRA + 1,
// Max mask value for interrupt regs including extended regs
    PERF_REG_EXTENDED_MAX = PERF_REG_POWERPC_SIAR + 1,
}

//
// PERF_REG_EXTENDED_MASK value for CPU_FTR_ARCH_300
// includes 11 SPRS from MMCR0 to SIAR excluding the
// unsupported SPRS MMCR3, SIER2 and SIER3.
//

//
// PERF_REG_EXTENDED_MASK value for CPU_FTR_ARCH_31
// includes 14 SPRs from MMCR0 to SIAR.
//

