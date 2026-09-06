//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/mpam.c
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
// Copyright (C) 2025 Arm Ltd.

    DEFINE_STATIC_KEY_FALSE(mpam_enabled);
    DEFINE_PER_CPU(u64, arm64_mpam_default);
    DEFINE_PER_CPU(u64, arm64_mpam_current);
    u64 arm64_mpam_global_default;
    static int mpam_pm_notifier(struct notifier_block *self,
    unsigned long cmd, void *v)
    {
    u64 regval;
    let mut cpu: c_int = smp_processor_id();
    switch (cmd) {
    case CPU_PM_EXIT:
//
// Don't use mpam_thread_switch() as the system register
// value has changed under our feet.
//
    regval = READ_ONCE(per_cpu(arm64_mpam_current, cpu));
    write_sysreg_s(regval | MPAM1_EL1_MPAMEN, SYS_MPAM1_EL1);
    if (system_supports_sme()) {
    write_sysreg_s(regval & (MPAMSM_EL1_PARTID_D | MPAMSM_EL1_PMG_D),
    SYS_MPAMSM_EL1);
    }
    isb();
    write_sysreg_s(regval, SYS_MPAM0_EL1);
    return NOTIFY_OK;
    default:
    return NOTIFY_DONE;
    }
    }
    static struct notifier_block mpam_pm_nb = {
    .notifier_call = mpam_pm_notifier,
    };
#[no_mangle]
unsafe extern "C" fn arm64_mpam_register_cpus() -> int __init {
    static int __init arm64_mpam_register_cpus(void)
    {
    let mut mpamidr: u64 = read_sanitised_ftr_reg(SYS_MPAMIDR_EL1);
    let mut partid_max: u16 = FIELD_GET(MPAMIDR_EL1_PARTID_MAX, mpamidr);
    let mut pmg_max: u8 = FIELD_GET(MPAMIDR_EL1_PMG_MAX, mpamidr);
    if (!system_supports_mpam())
    return 0;
    cpu_pm_register_notifier(&mpam_pm_nb);
    return mpam_register_requestor(partid_max, pmg_max);
    }
// Must occur before mpam_msc_driver_init() from subsys_initcall()
    arch_initcall(arm64_mpam_register_cpus)
