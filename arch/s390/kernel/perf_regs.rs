//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/perf_regs.c
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
pub unsafe extern "C" fn perf_reg_value(regs: *mut pt_regs, idx: c_int) -> u64 {
    u64 perf_reg_value(struct pt_regs *regs, int idx)
    {
    freg_t fp;
    if (idx >= PERF_REG_S390_R0 && idx <= PERF_REG_S390_R15)
    return regs.gprs[idx];
    if (idx >= PERF_REG_S390_FP0 && idx <= PERF_REG_S390_FP15) {
    if (!user_mode(regs))
    return 0;
    idx -= PERF_REG_S390_FP0;
    fp = *(freg_t *)(current.thread.ufpu.vxrs + idx);
    return fp.ui;
    }
    if (idx == PERF_REG_S390_MASK)
    return regs.psw.mask;
    if (idx == PERF_REG_S390_PC)
    return regs.psw.addr;
    WARN_ON_ONCE((u32)idx >= PERF_REG_S390_MAX);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn perf_reg_validate(mask: u64) -> c_int {
    int perf_reg_validate(u64 mask)
    {
    if (!mask || mask & REG_RESERVED)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_reg_abi(task: *mut task_struct) -> u64 {
    u64 perf_reg_abi(struct task_struct *task)
    {
    return PERF_SAMPLE_REGS_ABI_64;
    }
    void perf_get_regs_user(struct perf_regs *regs_user,
    struct pt_regs *regs)
    {
//
// Use the regs from the first interruption and let
// perf_sample_regs_intr() handle interrupts (regs == get_irq_regs()).
//
// Also save FPU registers for user-space tasks only.
//
    regs_user.regs = task_pt_regs(current);
    if (user_mode(regs_user.regs))
    save_user_fpu_regs();
    regs_user.abi = perf_reg_abi(current);
    }
