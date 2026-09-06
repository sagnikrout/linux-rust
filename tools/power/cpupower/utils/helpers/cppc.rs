//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/helpers/cppc.c
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
pub unsafe extern "C" fn cppc_show_perf_and_freq(cpu: c_uint, no_rounding: c_int) {
    void cppc_show_perf_and_freq(unsigned int cpu, int no_rounding)
    {
    let mut nominal: i64 = acpi_cppc_get_data(cpu, NOMINAL_PERF);
    let mut nominal_freq: i64 = acpi_cppc_get_data(cpu, NOMINAL_FREQ) * 1000;
    let mut lowest: i64 = acpi_cppc_get_data(cpu, LOWEST_PERF);
    let mut lowest_freq: i64 = acpi_cppc_get_data(cpu, LOWEST_FREQ) * 1000;
    let mut non_linear: c_ulong = acpi_cppc_get_data(cpu, LOWEST_NONLINEAR_PERF);
    let mut highest: c_ulong = acpi_cppc_get_data(cpu, HIGHEST_PERF);
    float slope, intercept;
// do the optional freq fields look invalid?
    if (!nominal_freq || !lowest_freq || nominal == lowest)
    return;
    slope = (float)(nominal_freq - lowest_freq) / (nominal - lowest);
    intercept = lowest_freq - slope * lowest;
    printf(_("  CPPC limits:\n"));
    printf(_("    Highest Performance: %lu. Maximum Frequency: "),
    highest);
//
// If boost isn't active, the cpuinfo_max doesn't indicate real max
// frequency.
//
    print_speed(cppc_to_frequency(highest), no_rounding);
    printf(".\n");
    printf(_("    Nominal Performance: %lu. Nominal Frequency: "),
    acpi_cppc_get_data(cpu, NOMINAL_PERF));
    print_speed(nominal_freq,  no_rounding);
    printf(".\n");
    printf(_("    Lowest Non-linear Performance: %lu. Lowest Non-linear Frequency: "),
    non_linear);
    print_speed(cppc_to_frequency(non_linear), no_rounding);
    printf(".\n");
    printf(_("    Lowest Performance: %lu. Lowest Frequency: "),
    acpi_cppc_get_data(cpu, LOWEST_PERF));
    print_speed(lowest_freq, no_rounding);
    printf(".\n");
    }
