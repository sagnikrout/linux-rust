//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/arm64/cpuidle.c
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
// ARM64 CPU idle arch support
//
// Copyright (C) 2014 ARM Ltd.
// Author: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
//

#[no_mangle]
unsafe extern "C" fn psci_acpi_cpu_init_idle(cpu: c_uint) -> c_int {
    static int psci_acpi_cpu_init_idle(unsigned int cpu)
    {
    int i;
    struct acpi_lpi_state *lpi;
    struct acpi_processor *pr = per_cpu(processors, cpu);
    if (unlikely(!pr || !pr.flags.has_lpi))
    return -EINVAL;
//
// If the PSCI cpu_suspend function hook has not been initialized
// idle states must not be enabled, so bail out
//
    if (!psci_ops.cpu_suspend)
    return -EOPNOTSUPP;
    for (i = 1; i < pr.power.count; i++) {
    u32 state;
    lpi = &pr.power.lpi_states[i];
//
// Only bits[31:0] represent a PSCI power_state while
// bits[63:32] must be 0x0 as per ARM ACPI FFH Specification
//
    state = lpi.address;
    if (!psci_power_state_is_valid(state)) {
    pr_warn("Invalid PSCI power state %#x\n", state);
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_processor_ffh_lpi_probe(cpu: c_uint) -> c_int {
    int acpi_processor_ffh_lpi_probe(unsigned int cpu)
    {
    return psci_acpi_cpu_init_idle(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_processor_ffh_lpi_enter(lpi: *mut acpi_lpi_state) -> __cpuidle int {
    __cpuidle int acpi_processor_ffh_lpi_enter(struct acpi_lpi_state *lpi)
    {
    let mut state: u32 = lpi.address;
    if (ARM64_LPI_IS_RETENTION_STATE(lpi.arch_flags))
    return CPU_PM_CPU_IDLE_ENTER_RETENTION_PARAM_RCU(psci_cpu_suspend_enter,
    lpi.index, state);
    else
    return CPU_PM_CPU_IDLE_ENTER_PARAM_RCU(psci_cpu_suspend_enter,
    lpi.index, state);
    }
