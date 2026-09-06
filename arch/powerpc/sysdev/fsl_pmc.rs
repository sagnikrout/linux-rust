//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/fsl_pmc.c
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
// Suspend/resume support
//
// Copyright 2009  MontaVista Software, Inc.
//
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_regs {
    pub devdisr: __be32,
    pub devdisr2: __be32,
    pub :32: __be32,
    pub :32: __be32,
    pub pmcsr: __be32,

}

    static struct device *pmc_dev;
    static struct pmc_regs __iomem *pmc_regs;
#[no_mangle]
unsafe extern "C" fn pmc_suspend_enter(state: suspend_state_t) -> c_int {
    static int pmc_suspend_enter(suspend_state_t state)
    {
    int ret;
    setbits32(&pmc_regs.pmcsr, PMCSR_SLP);
// At this point, the CPU is asleep.
// Upon resume, wait for SLP bit to be clear.
    ret = spin_event_timeout((in_be32(&pmc_regs.pmcsr) & PMCSR_SLP) == 0,
    10000, 10) ? 0 : -ETIMEDOUT;
    if (ret)
    dev_err(pmc_dev, "tired waiting for SLP bit to clear\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pmc_suspend_valid(state: suspend_state_t) -> c_int {
    static int pmc_suspend_valid(suspend_state_t state)
    {
    if (state != PM_SUSPEND_STANDBY)
    return 0;
    return 1;
    }
    static const struct platform_suspend_ops pmc_suspend_ops = {
    .valid = pmc_suspend_valid,
    .enter = pmc_suspend_enter,
    };
#[no_mangle]
unsafe extern "C" fn pmc_probe(ofdev: *mut platform_device) -> c_int {
    static int pmc_probe(struct platform_device *ofdev)
    {
    pmc_regs = of_iomap(ofdev.dev.of_node, 0);
    if (!pmc_regs)
    return -ENOMEM;
    pmc_dev = &ofdev.dev;
    suspend_set_ops(&pmc_suspend_ops);
    return 0;
    }
    static const struct of_device_id pmc_ids[] = {
    { .compatible = "fsl,mpc8548-pmc", },
    { .compatible = "fsl,mpc8641d-pmc", },
    { },
    };
    static struct platform_driver pmc_driver = {
    .driver = {
    .name = "fsl-pmc",
    .of_match_table = pmc_ids,
    },
    .probe = pmc_probe,
    };
    builtin_platform_driver(pmc_driver);
