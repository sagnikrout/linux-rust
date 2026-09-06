//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/perf-regs-arch/perf_regs_mips.c
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
pub unsafe extern "C" fn __perf_reg_mask_mips(__maybe_unused: bool intr) -> u64 {
    uint64_t __perf_reg_mask_mips(bool intr __maybe_unused)
    {
    return PERF_REGS_MASK;
    }
    const char *__perf_reg_name_mips(int id)
    {
    switch (id) {
    case PERF_REG_MIPS_PC:
    return "PC";
    case PERF_REG_MIPS_R1:
    return "$1";
    case PERF_REG_MIPS_R2:
    return "$2";
    case PERF_REG_MIPS_R3:
    return "$3";
    case PERF_REG_MIPS_R4:
    return "$4";
    case PERF_REG_MIPS_R5:
    return "$5";
    case PERF_REG_MIPS_R6:
    return "$6";
    case PERF_REG_MIPS_R7:
    return "$7";
    case PERF_REG_MIPS_R8:
    return "$8";
    case PERF_REG_MIPS_R9:
    return "$9";
    case PERF_REG_MIPS_R10:
    return "$10";
    case PERF_REG_MIPS_R11:
    return "$11";
    case PERF_REG_MIPS_R12:
    return "$12";
    case PERF_REG_MIPS_R13:
    return "$13";
    case PERF_REG_MIPS_R14:
    return "$14";
    case PERF_REG_MIPS_R15:
    return "$15";
    case PERF_REG_MIPS_R16:
    return "$16";
    case PERF_REG_MIPS_R17:
    return "$17";
    case PERF_REG_MIPS_R18:
    return "$18";
    case PERF_REG_MIPS_R19:
    return "$19";
    case PERF_REG_MIPS_R20:
    return "$20";
    case PERF_REG_MIPS_R21:
    return "$21";
    case PERF_REG_MIPS_R22:
    return "$22";
    case PERF_REG_MIPS_R23:
    return "$23";
    case PERF_REG_MIPS_R24:
    return "$24";
    case PERF_REG_MIPS_R25:
    return "$25";
    case PERF_REG_MIPS_R28:
    return "$28";
    case PERF_REG_MIPS_R29:
    return "$29";
    case PERF_REG_MIPS_R30:
    return "$30";
    case PERF_REG_MIPS_R31:
    return "$31";
    default:
    break;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_mips() -> u64 {
    uint64_t __perf_reg_ip_mips(void)
    {
    return PERF_REG_MIPS_PC;
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_mips() -> u64 {
    uint64_t __perf_reg_sp_mips(void)
    {
    return PERF_REG_MIPS_R29;
    }
