//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/perf-regs-arch/perf_regs_csky.c
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
pub unsafe extern "C" fn __perf_reg_mask_csky(__maybe_unused: bool intr) -> u64 {
    uint64_t __perf_reg_mask_csky(bool intr __maybe_unused)
    {
    return PERF_REGS_MASK;
    }
    const char *__perf_reg_name_csky(int id, uint32_t e_flags)
    {
    if (id >= PERF_REG_CSKY_EXREGS0 && (e_flags & EF_CSKY_ABIMASK) == EF_CSKY_ABIV2)
    return core::ptr::null_mut();
    switch (id) {
    case PERF_REG_CSKY_A0:
    return "a0";
    case PERF_REG_CSKY_A1:
    return "a1";
    case PERF_REG_CSKY_A2:
    return "a2";
    case PERF_REG_CSKY_A3:
    return "a3";
    case PERF_REG_CSKY_REGS0:
    return "regs0";
    case PERF_REG_CSKY_REGS1:
    return "regs1";
    case PERF_REG_CSKY_REGS2:
    return "regs2";
    case PERF_REG_CSKY_REGS3:
    return "regs3";
    case PERF_REG_CSKY_REGS4:
    return "regs4";
    case PERF_REG_CSKY_REGS5:
    return "regs5";
    case PERF_REG_CSKY_REGS6:
    return "regs6";
    case PERF_REG_CSKY_REGS7:
    return "regs7";
    case PERF_REG_CSKY_REGS8:
    return "regs8";
    case PERF_REG_CSKY_REGS9:
    return "regs9";
    case PERF_REG_CSKY_SP:
    return "sp";
    case PERF_REG_CSKY_LR:
    return "lr";
    case PERF_REG_CSKY_PC:
    return "pc";
    case PERF_REG_CSKY_EXREGS0:
    return "exregs0";
    case PERF_REG_CSKY_EXREGS1:
    return "exregs1";
    case PERF_REG_CSKY_EXREGS2:
    return "exregs2";
    case PERF_REG_CSKY_EXREGS3:
    return "exregs3";
    case PERF_REG_CSKY_EXREGS4:
    return "exregs4";
    case PERF_REG_CSKY_EXREGS5:
    return "exregs5";
    case PERF_REG_CSKY_EXREGS6:
    return "exregs6";
    case PERF_REG_CSKY_EXREGS7:
    return "exregs7";
    case PERF_REG_CSKY_EXREGS8:
    return "exregs8";
    case PERF_REG_CSKY_EXREGS9:
    return "exregs9";
    case PERF_REG_CSKY_EXREGS10:
    return "exregs10";
    case PERF_REG_CSKY_EXREGS11:
    return "exregs11";
    case PERF_REG_CSKY_EXREGS12:
    return "exregs12";
    case PERF_REG_CSKY_EXREGS13:
    return "exregs13";
    case PERF_REG_CSKY_EXREGS14:
    return "exregs14";
    case PERF_REG_CSKY_TLS:
    return "tls";
    case PERF_REG_CSKY_HI:
    return "hi";
    case PERF_REG_CSKY_LO:
    return "lo";
    default:
    return core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_csky() -> u64 {
    uint64_t __perf_reg_ip_csky(void)
    {
    return PERF_REG_CSKY_PC;
    }
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_csky() -> u64 {
    uint64_t __perf_reg_sp_csky(void)
    {
    return PERF_REG_CSKY_SP;
    }
