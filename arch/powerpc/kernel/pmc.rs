//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/pmc.c
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
//
// arch/powerpc/kernel/pmc.c
//
// Copyright (C) 2004 David Gibson, IBM Corporation.
// Includes code formerly from arch/ppc/kernel/perfmon.c:
// Author: Andy Fleming
// Copyright (c) 2004 Freescale Semiconductor, Inc
//

pub const MMCR0_PMAO: c_int = 0;

#[no_mangle]
unsafe extern "C" fn dummy_perf(regs: *mut pt_regs) {
    static void dummy_perf(struct pt_regs *regs)
    {

    mtpmr(PMRN_PMGC0, mfpmr(PMRN_PMGC0) & ~PMGC0_PMIE);

    if (cur_cpu_spec.pmc_type == PPC_PMC_IBM)
    mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) & ~(MMCR0_PMXE|MMCR0_PMAO));

    mtspr(SPRN_MMCR0, mfspr(SPRN_MMCR0) & ~MMCR0_PMXE);

    }
    static DEFINE_RAW_SPINLOCK(pmc_owner_lock);
    static void *pmc_owner_caller; /* mostly for debugging */
    let mut perf_irq: perf_irq_t = dummy_perf;
#[no_mangle]
pub unsafe extern "C" fn reserve_pmc_hardware(new_perf_irq: perf_irq_t) -> c_int {
    int reserve_pmc_hardware(perf_irq_t new_perf_irq)
    {
    let mut err: c_int = 0;
    raw_spin_lock(&pmc_owner_lock);
    if (pmc_owner_caller) {
    printk(KERN_WARNING "reserve_pmc_hardware: "
    "PMC hardware busy (reserved by caller %p)\n",
    pmc_owner_caller);
    err = -EBUSY;
    goto out;
    }
    pmc_owner_caller = __builtin_return_address(0);
    perf_irq = new_perf_irq ? new_perf_irq : dummy_perf;
    out:
    raw_spin_unlock(&pmc_owner_lock);
    return err;
    }
    EXPORT_SYMBOL_GPL(reserve_pmc_hardware);
#[no_mangle]
pub unsafe extern "C" fn release_pmc_hardware() {
    void release_pmc_hardware(void)
    {
    raw_spin_lock(&pmc_owner_lock);
    WARN_ON(! pmc_owner_caller);
    pmc_owner_caller = core::ptr::null_mut();
    perf_irq = dummy_perf;
    raw_spin_unlock(&pmc_owner_lock);
    }
    EXPORT_SYMBOL_GPL(release_pmc_hardware);

#[no_mangle]
pub unsafe extern "C" fn power4_enable_pmcs() {
    void power4_enable_pmcs(void)
    {
    unsigned long hid0;
    hid0 = mfspr(SPRN_HID0);
    hid0 |= 1UL << (63 - 20);
// POWER4 requires the following sequence
    asm volatile(
    "sync\n"
    "mtspr     %1, %0\n"
    "mfspr     %0, %1\n"
    "mfspr     %0, %1\n"
    "mfspr     %0, %1\n"
    "mfspr     %0, %1\n"
    "mfspr     %0, %1\n"
    "mfspr     %0, %1\n"
    "isync" : "=&r" (hid0) : "i" (SPRN_HID0), "0" (hid0):
    "memory");
    }
