//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/dexcr.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

#[no_mangle]
unsafe extern "C" fn init_task_dexcr() -> int __init {
    static int __init init_task_dexcr(void)
    {
    if (!early_cpu_has_feature(CPU_FTR_ARCH_31))
    return 0;
    current.thread.dexcr_onexec = mfspr(SPRN_DEXCR);
    return 0;
    }
    early_initcall(init_task_dexcr)
// Allow thread local configuration of these by default

    DEXCR_PR_IBRTPD | \
    DEXCR_PR_SRAPD | \
    DEXCR_PR_NPHIE)
#[no_mangle]
unsafe extern "C" fn prctl_to_aspect(which: c_ulong, aspect: *mut c_uint) -> c_int {
    static int prctl_to_aspect(unsigned long which, unsigned int *aspect)
    {
    switch (which) {
    case PR_PPC_DEXCR_SBHE:
// aspect = DEXCR_PR_SBHE;
    break;
    case PR_PPC_DEXCR_IBRTPD:
// aspect = DEXCR_PR_IBRTPD;
    break;
    case PR_PPC_DEXCR_SRAPD:
// aspect = DEXCR_PR_SRAPD;
    break;
    case PR_PPC_DEXCR_NPHIE:
// aspect = DEXCR_PR_NPHIE;
    break;
    default:
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_dexcr_prctl(task: *mut task_struct, which: c_ulong) -> c_int {
    int get_dexcr_prctl(struct task_struct *task, unsigned long which)
    {
    unsigned int aspect;
    int ret;
    ret = prctl_to_aspect(which, &aspect);
    if (ret)
    return ret;
    if (aspect & DEXCR_PRCTL_EDITABLE)
    ret |= PR_PPC_DEXCR_CTRL_EDITABLE;
    if (aspect & mfspr(SPRN_DEXCR))
    ret |= PR_PPC_DEXCR_CTRL_SET;
    else
    ret |= PR_PPC_DEXCR_CTRL_CLEAR;
    if (aspect & task.thread.dexcr_onexec)
    ret |= PR_PPC_DEXCR_CTRL_SET_ONEXEC;
    else
    ret |= PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn set_dexcr_prctl(task: *mut task_struct, which: c_ulong, ctrl: c_ulong) -> c_int {
    int set_dexcr_prctl(struct task_struct *task, unsigned long which, unsigned long ctrl)
    {
    unsigned long dexcr;
    unsigned int aspect;
    let mut err: c_int = 0;
    err = prctl_to_aspect(which, &aspect);
    if (err)
    return err;
    if (!(aspect & DEXCR_PRCTL_EDITABLE))
    return -EPERM;
    if (ctrl & ~PR_PPC_DEXCR_CTRL_MASK)
    return -EINVAL;
    if (ctrl & PR_PPC_DEXCR_CTRL_SET && ctrl & PR_PPC_DEXCR_CTRL_CLEAR)
    return -EINVAL;
    if (ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC && ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC)
    return -EINVAL;
//
// We do not want an unprivileged process being able to disable
// a setuid process's hash check instructions
//
    if (aspect == DEXCR_PR_NPHIE &&
    ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC &&
    !capable(CAP_SYS_ADMIN))
    return -EPERM;
    dexcr = mfspr(SPRN_DEXCR);
    if (ctrl & PR_PPC_DEXCR_CTRL_SET)
    dexcr |= aspect;
#[no_mangle]
pub unsafe extern "C" fn if(PR_PPC_DEXCR_CTRL_CLEAR: ctrl &) -> else {
    else if (ctrl & PR_PPC_DEXCR_CTRL_CLEAR)
    dexcr &= ~aspect;
    if (ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC)
    task.thread.dexcr_onexec |= aspect;
#[no_mangle]
pub unsafe extern "C" fn if(PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC: ctrl &) -> else {
    else if (ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC)
    task.thread.dexcr_onexec &= ~aspect;
    mtspr(SPRN_DEXCR, dexcr);
    return 0;
    }
