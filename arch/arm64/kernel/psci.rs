//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/psci.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2013 ARM Limited
//
// Author: Will Deacon <will.deacon@arm.com>
//

#[no_mangle]
unsafe extern "C" fn cpu_psci_cpu_init(cpu: c_uint) -> int __init {
    static int __init cpu_psci_cpu_init(unsigned int cpu)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpu_psci_cpu_prepare(cpu: c_uint) -> int __init {
    static int __init cpu_psci_cpu_prepare(unsigned int cpu)
    {
    if (!psci_ops.cpu_on) {
    pr_err("no cpu_on method, not booting CPU%d\n", cpu);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpu_psci_cpu_boot(cpu: c_uint) -> c_int {
    static int cpu_psci_cpu_boot(unsigned int cpu)
    {
    let mut pa_secondary_entry: phys_addr_t = __pa_symbol(secondary_entry);
    let mut err: c_int = psci_ops.cpu_on(cpu_logical_map(cpu), pa_secondary_entry);
    if (err && err != -EPERM)
    pr_err("failed to boot CPU%d (%d)\n", cpu, err);
    return err;
    }

#[no_mangle]
unsafe extern "C" fn cpu_psci_cpu_can_disable(cpu: c_uint) -> bool {
    static bool cpu_psci_cpu_can_disable(unsigned int cpu)
    {
    return !psci_tos_resident_on(cpu);
    }
#[no_mangle]
unsafe extern "C" fn cpu_psci_cpu_disable(cpu: c_uint) -> c_int {
    static int cpu_psci_cpu_disable(unsigned int cpu)
    {
// Fail early if we don't have CPU_OFF support
    if (!psci_ops.cpu_off)
    return -EOPNOTSUPP;
// Trusted OS will deny CPU_OFF
    if (psci_tos_resident_on(cpu))
    return -EPERM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpu_psci_cpu_die(cpu: c_uint) {
    static void cpu_psci_cpu_die(unsigned int cpu)
    {
//
// There are no known implementations of PSCI actually using the
// power state field, pass a sensible default for now.
//
    u32 state = PSCI_POWER_STATE_TYPE_POWER_DOWN <<
    PSCI_0_2_POWER_STATE_TYPE_SHIFT;
    psci_ops.cpu_off(state);
    }
#[no_mangle]
unsafe extern "C" fn cpu_psci_cpu_kill(cpu: c_uint) -> c_int {
    static int cpu_psci_cpu_kill(unsigned int cpu)
    {
    int err;
    unsigned long start, end;
    if (!psci_ops.affinity_info)
    return 0;
//
// cpu_kill could race with cpu_die and we can
// potentially end up declaring this cpu undead
// while it is dying. So, try again a few times.
//
    start = jiffies;
    end = start + msecs_to_jiffies(100);
    do {
    err = psci_ops.affinity_info(cpu_logical_map(cpu), 0);
    if (err == PSCI_0_2_AFFINITY_LEVEL_OFF) {
    pr_info("CPU%d killed (polled %d ms)\n", cpu,
    jiffies_to_msecs(jiffies - start));
    return 0;
    }
    usleep_range(100, 1000);
    } while (time_before(jiffies, end));
    pr_warn("CPU%d may not have shut down cleanly (AFFINITY_INFO reports %d)\n",
    cpu, err);
    return -ETIMEDOUT;
    }

    const struct cpu_operations cpu_psci_ops = {
    .name		= "psci",
    .cpu_init	= cpu_psci_cpu_init,
    .cpu_prepare	= cpu_psci_cpu_prepare,
    .cpu_boot	= cpu_psci_cpu_boot,

    .cpu_can_disable = cpu_psci_cpu_can_disable,
    .cpu_disable	= cpu_psci_cpu_disable,
    .cpu_die	= cpu_psci_cpu_die,
    .cpu_kill	= cpu_psci_cpu_kill,

    };
