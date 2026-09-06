//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/perf-regs-arch/perf_regs_arm.c
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
pub unsafe extern "C" fn __perf_reg_mask_arm(__maybe_unused: bool intr) -> u64 {
    uint64_t __perf_reg_mask_arm(bool intr __maybe_unused)
    {
    return PERF_REGS_MASK;
    }
    const char *__perf_reg_name_arm(int id)
    {
    switch (id) {
    case PERF_REG_ARM_R0:
    return "r0";
    case PERF_REG_ARM_R1:
    return "r1";
    case PERF_REG_ARM_R2:
    return "r2";
    case PERF_REG_ARM_R3:
    return "r3";
    case PERF_REG_ARM_R4:
    return "r4";
    case PERF_REG_ARM_R5:
    return "r5";
    case PERF_REG_ARM_R6:
    return "r6";
    case PERF_REG_ARM_R7:
    return "r7";
    case PERF_REG_ARM_R8:
    return "r8";
    case PERF_REG_ARM_R9:
    return "r9";
    case PERF_REG_ARM_R10:
    return "r10";
    case PERF_REG_ARM_FP:
    return "fp";
    case PERF_REG_ARM_IP:
    return "ip";
    case PERF_REG_ARM_SP:
    return "sp";
    case PERF_REG_ARM_LR:
    return "lr";
    case PERF_REG_ARM_PC:
    return "pc";
    default:
    return core::ptr::null_mut();
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_arm() -> u64 {
    uint64_t __perf_reg_ip_arm(void)
    {
    return PERF_REG_ARM_PC;
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_arm() -> u64 {
    uint64_t __perf_reg_sp_arm(void)
    {
    return PERF_REG_ARM_SP;
    }
