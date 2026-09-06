//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/perf-regs-arch/perf_regs_loongarch.c
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

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_mask_loongarch(__maybe_unused: bool intr) -> u64 {
    uint64_t __perf_reg_mask_loongarch(bool intr __maybe_unused)
    {
    return PERF_REGS_MASK;
    }
    const char *__perf_reg_name_loongarch(int id)
    {
    switch (id) {
    case PERF_REG_LOONGARCH_PC:
    return "PC";
    case PERF_REG_LOONGARCH_R1:
    return "%r1";
    case PERF_REG_LOONGARCH_R2:
    return "%r2";
    case PERF_REG_LOONGARCH_R3:
    return "%r3";
    case PERF_REG_LOONGARCH_R4:
    return "%r4";
    case PERF_REG_LOONGARCH_R5:
    return "%r5";
    case PERF_REG_LOONGARCH_R6:
    return "%r6";
    case PERF_REG_LOONGARCH_R7:
    return "%r7";
    case PERF_REG_LOONGARCH_R8:
    return "%r8";
    case PERF_REG_LOONGARCH_R9:
    return "%r9";
    case PERF_REG_LOONGARCH_R10:
    return "%r10";
    case PERF_REG_LOONGARCH_R11:
    return "%r11";
    case PERF_REG_LOONGARCH_R12:
    return "%r12";
    case PERF_REG_LOONGARCH_R13:
    return "%r13";
    case PERF_REG_LOONGARCH_R14:
    return "%r14";
    case PERF_REG_LOONGARCH_R15:
    return "%r15";
    case PERF_REG_LOONGARCH_R16:
    return "%r16";
    case PERF_REG_LOONGARCH_R17:
    return "%r17";
    case PERF_REG_LOONGARCH_R18:
    return "%r18";
    case PERF_REG_LOONGARCH_R19:
    return "%r19";
    case PERF_REG_LOONGARCH_R20:
    return "%r20";
    case PERF_REG_LOONGARCH_R21:
    return "%r21";
    case PERF_REG_LOONGARCH_R22:
    return "%r22";
    case PERF_REG_LOONGARCH_R23:
    return "%r23";
    case PERF_REG_LOONGARCH_R24:
    return "%r24";
    case PERF_REG_LOONGARCH_R25:
    return "%r25";
    case PERF_REG_LOONGARCH_R26:
    return "%r26";
    case PERF_REG_LOONGARCH_R27:
    return "%r27";
    case PERF_REG_LOONGARCH_R28:
    return "%r28";
    case PERF_REG_LOONGARCH_R29:
    return "%r29";
    case PERF_REG_LOONGARCH_R30:
    return "%r30";
    case PERF_REG_LOONGARCH_R31:
    return "%r31";
    default:
    break;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_loongarch() -> u64 {
    uint64_t __perf_reg_ip_loongarch(void)
    {
    return PERF_REG_LOONGARCH_PC;
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_loongarch() -> u64 {
    uint64_t __perf_reg_sp_loongarch(void)
    {
    return PERF_REG_LOONGARCH_R3;
    }
