//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/hwtimer.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: hwtimer.c - ACPI Power Management Timer Interface
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Macro flag: #define EXPORT_ACPI_INTERFACES

    ACPI_MODULE_NAME("hwtimer")

//
// FUNCTION:    acpi_get_timer_resolution
//
// PARAMETERS:  resolution          - Where the resolution is returned
//
// RETURN:      Status and timer resolution
//
// DESCRIPTION: Obtains resolution of the ACPI PM Timer (24 or 32 bits).
//
#[no_mangle]
pub unsafe extern "C" fn acpi_get_timer_resolution(resolution: *mut *mut u32) -> acpi_status {
    acpi_status acpi_get_timer_resolution(u32 * resolution)
    {
    ACPI_FUNCTION_TRACE(acpi_get_timer_resolution);
    if (!resolution) {
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
    if ((acpi_gbl_FADT.flags & ACPI_FADT_32BIT_TIMER) == 0) {
// resolution = 24;
    } else {
// resolution = 32;
    }
    return_ACPI_STATUS(AE_OK);
    }
    ACPI_EXPORT_SYMBOL(acpi_get_timer_resolution)
//
// FUNCTION:    acpi_get_timer
//
// PARAMETERS:  ticks               - Where the timer value is returned
//
// RETURN:      Status and current timer value (ticks)
//
// DESCRIPTION: Obtains current value of ACPI PM Timer (in ticks).
//
#[no_mangle]
pub unsafe extern "C" fn acpi_get_timer(ticks: *mut *mut u32) -> acpi_status {
    acpi_status acpi_get_timer(u32 * ticks)
    {
    acpi_status status;
    u64 timer_value;
    ACPI_FUNCTION_TRACE(acpi_get_timer);
    if (!ticks) {
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
// ACPI 5.0A: PM Timer is optional
    if (!acpi_gbl_FADT.xpm_timer_block.address) {
    return_ACPI_STATUS(AE_SUPPORT);
    }
    status = acpi_hw_read(&timer_value, &acpi_gbl_FADT.xpm_timer_block);
    if (ACPI_SUCCESS(status)) {
// ACPI PM Timer is defined to be 32 bits (PM_TMR_LEN)
// ticks = (u32)timer_value;
    }
    return_ACPI_STATUS(status);
    }
    ACPI_EXPORT_SYMBOL(acpi_get_timer)
//
// FUNCTION:    acpi_get_timer_duration
//
// PARAMETERS:  start_ticks         - Starting timestamp
// end_ticks           - End timestamp
// time_elapsed        - Where the elapsed time is returned
//
// RETURN:      Status and time_elapsed
//
// DESCRIPTION: Computes the time elapsed (in microseconds) between two
// PM Timer time stamps, taking into account the possibility of
// rollovers, the timer resolution, and timer frequency.
//
// The PM Timer's clock ticks at roughly 3.6 times per
// _microsecond_, and its clock continues through Cx state
// transitions (unlike many CPU timestamp counters) -- making it
// a versatile and accurate timer.
//
// Note that this function accommodates only a single timer
// rollover. Thus for 24-bit timers, this function should only
// be used for calculating durations less than ~4.6 seconds
// (~20 minutes for 32-bit timers) -- calculations below:
//
// 2**24 Ticks / 3,600,000 Ticks/Sec = 4.66 sec
// 2**32 Ticks / 3,600,000 Ticks/Sec = 1193 sec or 19.88 minutes
//
    acpi_status
    acpi_get_timer_duration(u32 start_ticks, u32 end_ticks, u32 *time_elapsed)
    {
    acpi_status status;
    u64 delta_ticks;
    u64 quotient;
    ACPI_FUNCTION_TRACE(acpi_get_timer_duration);
    if (!time_elapsed) {
    return_ACPI_STATUS(AE_BAD_PARAMETER);
    }
// ACPI 5.0A: PM Timer is optional
    if (!acpi_gbl_FADT.xpm_timer_block.address) {
    return_ACPI_STATUS(AE_SUPPORT);
    }
    if (start_ticks == end_ticks) {
// time_elapsed = 0;
    return_ACPI_STATUS(AE_OK);
    }
//
// Compute Tick Delta:
// Handle (max one) timer rollovers on 24-bit versus 32-bit timers.
//
    delta_ticks = end_ticks;
    if (start_ticks > end_ticks) {
    if ((acpi_gbl_FADT.flags & ACPI_FADT_32BIT_TIMER) == 0) {
// 24-bit Timer
    delta_ticks |= (u64)1 << 24;
    } else {
// 32-bit Timer
    delta_ticks |= (u64)1 << 32;
    }
    }
    delta_ticks -= start_ticks;
//
// Compute Duration (Requires a 64-bit multiply and divide):
//
// time_elapsed (microseconds) =
// (delta_ticks * ACPI_USEC_PER_SEC) / ACPI_PM_TIMER_FREQUENCY;
//
    status = acpi_ut_short_divide(delta_ticks * ACPI_USEC_PER_SEC,
    ACPI_PM_TIMER_FREQUENCY, &quotient, core::ptr::null_mut());
// time_elapsed = (u32)quotient;
    return_ACPI_STATUS(status);
    }
    ACPI_EXPORT_SYMBOL(acpi_get_timer_duration)
