//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/perf_regs.c
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
//
// Copyright (C) 2022 Loongson Technology Corporation Limited
//
// Derived from MIPS:
// Copyright (C) 2013 Cavium, Inc.
//

#[no_mangle]
pub unsafe extern "C" fn perf_reg_abi(tsk: *mut task_struct) -> u64 {
    u64 perf_reg_abi(struct task_struct *tsk)
    {
    return PERF_SAMPLE_REGS_ABI_32;
    }

#[no_mangle]
pub unsafe extern "C" fn perf_reg_abi(tsk: *mut task_struct) -> u64 {
    u64 perf_reg_abi(struct task_struct *tsk)
    {
    if (test_tsk_thread_flag(tsk, TIF_32BIT_REGS))
    return PERF_SAMPLE_REGS_ABI_32;
    else
    return PERF_SAMPLE_REGS_ABI_64;
    }

#[no_mangle]
pub unsafe extern "C" fn perf_reg_validate(mask: u64) -> c_int {
    int perf_reg_validate(u64 mask)
    {
    if (!mask)
    return -EINVAL;
    if (mask & ~((1ull << PERF_REG_LOONGARCH_MAX) - 1))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_reg_value(regs: *mut pt_regs, idx: c_int) -> u64 {
    u64 perf_reg_value(struct pt_regs *regs, int idx)
    {
    if (WARN_ON_ONCE((u32)idx >= PERF_REG_LOONGARCH_MAX))
    return 0;
    if ((u32)idx == PERF_REG_LOONGARCH_PC)
    return regs.csr_era;
    return regs.regs[idx];
    }
    void perf_get_regs_user(struct perf_regs *regs_user,
    struct pt_regs *regs)
    {
    regs_user.regs = task_pt_regs(current);
    regs_user.abi = perf_reg_abi(current);
    }
