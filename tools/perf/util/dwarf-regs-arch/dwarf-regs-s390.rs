//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/dwarf-regs-arch/dwarf-regs-s390.c
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
pub unsafe extern "C" fn __get_dwarf_regnum_for_perf_regnum_s390(perf_regnum: c_int) -> c_int {
    int __get_dwarf_regnum_for_perf_regnum_s390(int perf_regnum)
    {
    static const int dwarf_s390_regnums[] = {
    [PERF_REG_S390_R0] = 0,
    [PERF_REG_S390_R1] = 1,
    [PERF_REG_S390_R2] = 2,
    [PERF_REG_S390_R3] = 3,
    [PERF_REG_S390_R4] = 4,
    [PERF_REG_S390_R5] = 5,
    [PERF_REG_S390_R6] = 6,
    [PERF_REG_S390_R7] = 7,
    [PERF_REG_S390_R8] = 8,
    [PERF_REG_S390_R9] = 9,
    [PERF_REG_S390_R10] = 10,
    [PERF_REG_S390_R11] = 11,
    [PERF_REG_S390_R12] = 12,
    [PERF_REG_S390_R13] = 13,
    [PERF_REG_S390_R14] = 14,
    [PERF_REG_S390_R15] = 15,
    [PERF_REG_S390_FP0] = 16,
    [PERF_REG_S390_FP1] = 20,
    [PERF_REG_S390_FP2] = 17,
    [PERF_REG_S390_FP3] = 21,
    [PERF_REG_S390_FP4] = 18,
    [PERF_REG_S390_FP5] = 22,
    [PERF_REG_S390_FP6] = 19,
    [PERF_REG_S390_FP7] = 23,
    [PERF_REG_S390_FP8] = 24,
    [PERF_REG_S390_FP9] = 28,
    [PERF_REG_S390_FP10] = 25,
    [PERF_REG_S390_FP11] = 29,
    [PERF_REG_S390_FP12] = 26,
    [PERF_REG_S390_FP13] = 30,
    [PERF_REG_S390_FP14] = 27,
    [PERF_REG_S390_FP15] = 31,
    [PERF_REG_S390_MASK] = 64,
    [PERF_REG_S390_PC] = 65,
    };
    if (perf_regnum == 0)
    return 0;
    if (perf_regnum <  0 || perf_regnum > (int)ARRAY_SIZE(dwarf_s390_regnums) ||
    dwarf_s390_regnums[perf_regnum] == 0)
    return -ENOENT;
    return dwarf_s390_regnums[perf_regnum];
    }
