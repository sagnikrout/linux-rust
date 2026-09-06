//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/riscv/cpuidle.c
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
// Copyright (C) 2024, Ventana Micro Systems Inc
// Author: Sunil V L <sunilvl@ventanamicro.com>
//

#[no_mangle]
unsafe extern "C" fn acpi_cpu_init_idle(cpu: c_uint) -> c_int {
    static int acpi_cpu_init_idle(unsigned int cpu)
    {
    int i;
    struct acpi_lpi_state *lpi;
    struct acpi_processor *pr = per_cpu(processors, cpu);
    if (unlikely(!pr || !pr.flags.has_lpi))
    return -EINVAL;
    if (!riscv_sbi_hsm_is_supported())
    return -ENODEV;
    if (pr.power.count <= 1)
    return -ENODEV;
    for (i = 1; i < pr.power.count; i++) {
    u32 state;
    lpi = &pr.power.lpi_states[i];
//
// Validate Entry Method as per FFH spec.
// bits[63:60] should be 0x1
// bits[59:32] should be 0x0
// bits[31:0] represent a SBI power_state
//
    if (((lpi.address & RISCV_FFH_LPI_TYPE_MASK) != RISCV_FFH_LPI_TYPE_SBI) ||
    (lpi.address & RISCV_FFH_LPI_RSVD_MASK)) {
    pr_warn("Invalid LPI entry method %#llx\n", lpi.address);
    return -EINVAL;
    }
    state = lpi.address;
    if (!riscv_sbi_suspend_state_is_valid(state)) {
    pr_warn("Invalid SBI power state %#x\n", state);
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_processor_ffh_lpi_probe(cpu: c_uint) -> c_int {
    int acpi_processor_ffh_lpi_probe(unsigned int cpu)
    {
    return acpi_cpu_init_idle(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_processor_ffh_lpi_enter(lpi: *mut acpi_lpi_state) -> int __cpuidle {
    int __cpuidle acpi_processor_ffh_lpi_enter(struct acpi_lpi_state *lpi)
    {
    let mut state: u32 = lpi.address;
    if (state & SBI_HSM_SUSP_NON_RET_BIT)
    return CPU_PM_CPU_IDLE_ENTER_PARAM(riscv_sbi_hart_suspend,
    lpi.index,
    state);
    else
    return CPU_PM_CPU_IDLE_ENTER_RETENTION_PARAM(riscv_sbi_hart_suspend,
    lpi.index,
    state);
    }
