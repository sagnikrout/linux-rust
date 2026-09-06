//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/mpc85xx_pm_ops.c
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
// MPC85xx PM operators
//
// Copyright 2015 Freescale Semiconductor Inc.
//

    static struct ccsr_guts __iomem *guts;

#[no_mangle]
unsafe extern "C" fn mpc85xx_irq_mask(cpu: c_int) {
    static void mpc85xx_irq_mask(int cpu)
    {
    }
#[no_mangle]
unsafe extern "C" fn mpc85xx_irq_unmask(cpu: c_int) {
    static void mpc85xx_irq_unmask(int cpu)
    {
    }
#[no_mangle]
unsafe extern "C" fn mpc85xx_cpu_die(cpu: c_int) {
    static void mpc85xx_cpu_die(int cpu)
    {
    u32 tmp;
    tmp = (mfspr(SPRN_HID0) & ~(HID0_DOZE|HID0_SLEEP)) | HID0_NAP;
    mtspr(SPRN_HID0, tmp);
// Enter NAP mode.
    tmp = mfmsr();
    tmp |= MSR_WE;
    asm volatile(
    "msync\n"
    "mtmsr %0\n"
    "isync\n"
    :
    : "r" (tmp));
    }
#[no_mangle]
unsafe extern "C" fn mpc85xx_cpu_up_prepare(cpu: c_int) {
    static void mpc85xx_cpu_up_prepare(int cpu)
    {
    }

#[no_mangle]
unsafe extern "C" fn mpc85xx_freeze_time_base(freeze: bool) {
    static void mpc85xx_freeze_time_base(bool freeze)
    {
    uint32_t mask;
    mask = CCSR_GUTS_DEVDISR_TB0 | CCSR_GUTS_DEVDISR_TB1;
    if (freeze)
    setbits32(&guts.devdisr, mask);
    else
    clrbits32(&guts.devdisr, mask);
    in_be32(&guts.devdisr);
    }
    static const struct of_device_id mpc85xx_smp_guts_ids[] = {
    { .compatible = "fsl,mpc8572-guts", },
    { .compatible = "fsl,p1020-guts", },
    { .compatible = "fsl,p1021-guts", },
    { .compatible = "fsl,p1022-guts", },
    { .compatible = "fsl,p1023-guts", },
    { .compatible = "fsl,p2020-guts", },
    { .compatible = "fsl,bsc9132-guts", },
    {},
    };
    static const struct fsl_pm_ops mpc85xx_pm_ops = {
    .freeze_time_base = mpc85xx_freeze_time_base,

    .irq_mask = mpc85xx_irq_mask,
    .irq_unmask = mpc85xx_irq_unmask,
    .cpu_die = mpc85xx_cpu_die,
    .cpu_up_prepare = mpc85xx_cpu_up_prepare,

    };
#[no_mangle]
pub unsafe extern "C" fn mpc85xx_setup_pmc() -> int __init {
    int __init mpc85xx_setup_pmc(void)
    {
    struct device_node *np;
    np = of_find_matching_node(core::ptr::null_mut(), mpc85xx_smp_guts_ids);
    if (np) {
    guts = of_iomap(np, 0);
    of_node_put(np);
    if (!guts) {
    pr_err("Could not map guts node address\n");
    return -ENOMEM;
    }
    qoriq_pm_ops = &mpc85xx_pm_ops;
    }
    return 0;
    }
