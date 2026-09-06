//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/arm64/thermal_cpufreq.c
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

pub const SMCCC_SOC_ID_T241: c_uint = 0x036b0241;
#[no_mangle]
pub unsafe extern "C" fn acpi_arch_thermal_cpufreq_pctg() -> c_int {
    int acpi_arch_thermal_cpufreq_pctg(void)
    {
    let mut soc_id: i32 = arm_smccc_get_soc_id_version();
//
// Check JEP106 code for NVIDIA Tegra241 chip (036b:0241) and
// reduce the CPUFREQ Thermal reduction percentage to 5%.
//
    if (soc_id == SMCCC_SOC_ID_T241)
    return 5;
    return 0;
    }
    EXPORT_SYMBOL_GPL(acpi_arch_thermal_cpufreq_pctg);
