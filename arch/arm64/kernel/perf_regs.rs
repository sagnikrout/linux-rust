//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/perf_regs.c
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
unsafe extern "C" fn perf_ext_regs_value(idx: c_int) -> u64 {
    static u64 perf_ext_regs_value(int idx)
    {
    switch (idx) {
    case PERF_REG_ARM64_VG:
    if (WARN_ON_ONCE(!system_supports_sve()))
    return 0;
//
// Vector granule is current length in bits of SVE registers
// divided by 64.
//
    return (task_get_sve_vl(current) * 8) / 64;
    default:
    WARN_ON_ONCE(true);
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn perf_reg_value(regs: *mut pt_regs, idx: c_int) -> u64 {
    u64 perf_reg_value(struct pt_regs *regs, int idx)
    {
    if (WARN_ON_ONCE((u32)idx >= PERF_REG_ARM64_EXTENDED_MAX))
    return 0;
//
// Our handling of compat tasks (PERF_SAMPLE_REGS_ABI_32) is weird, but
// we're stuck with it for ABI compatibility reasons.
//
// For a 32-bit consumer inspecting a 32-bit task, then it will look at
// the first 16 registers (see arch/arm/include/uapi/asm/perf_regs.h).
// These correspond directly to a prefix of the registers saved in our
// 'struct pt_regs', with the exception of the PC, so we copy that down
// (x15 corresponds to SP_hyp in the architecture).
//
// So far, so good.
//
// The oddity arises when a 64-bit consumer looks at a 32-bit task and
// asks for registers beyond PERF_REG_ARM_MAX. In this case, we return
// SP_usr, LR_usr and PC in the positions where the AArch64 SP, LR and
// PC registers would normally live. The initial idea was to allow a
// 64-bit unwinder to unwind a 32-bit task and, although it's not clear
// how well that works in practice, somebody might be relying on it.
//
// At the time we make a sample, we don't know whether the consumer is
// 32-bit or 64-bit, so we have to cater for both possibilities.
//
    if (compat_user_mode(regs)) {
    if ((u32)idx == PERF_REG_ARM64_SP)
    return regs.compat_sp;
    if ((u32)idx == PERF_REG_ARM64_LR)
    return regs.compat_lr;
    if (idx == 15)
    return regs.pc;
    }
    if ((u32)idx == PERF_REG_ARM64_SP)
    return regs.sp;
    if ((u32)idx == PERF_REG_ARM64_PC)
    return regs.pc;
    if ((u32)idx >= PERF_REG_ARM64_MAX)
    return perf_ext_regs_value(idx);
    return regs.regs[idx];
    }

#[no_mangle]
pub unsafe extern "C" fn perf_reg_validate(mask: u64) -> c_int {
    int perf_reg_validate(u64 mask)
    {
    let mut reserved_mask: u64 = REG_RESERVED;
    if (system_supports_sve())
    reserved_mask &= ~(1ULL << PERF_REG_ARM64_VG);
    if (!mask || mask & reserved_mask)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_reg_abi(task: *mut task_struct) -> u64 {
    u64 perf_reg_abi(struct task_struct *task)
    {
    if (is_compat_thread(task_thread_info(task)))
    return PERF_SAMPLE_REGS_ABI_32;
    else
    return PERF_SAMPLE_REGS_ABI_64;
    }
    void perf_get_regs_user(struct perf_regs *regs_user,
    struct pt_regs *regs)
    {
    regs_user.regs = task_pt_regs(current);
    regs_user.abi = perf_reg_abi(current);
    }
