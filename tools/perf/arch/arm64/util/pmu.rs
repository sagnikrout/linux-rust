//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/arm64/util/pmu.c
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
pub unsafe extern "C" fn tool_pmu__cpu_slots_per_cycle() -> u64 {
    u64 tool_pmu__cpu_slots_per_cycle(void)
    {
    char path[PATH_MAX];
    let mut slots: c_ulonglong = 0;
    struct perf_pmu *pmu = perf_pmus__find_core_pmu();
    if (pmu) {
    perf_pmu__pathname_scnprintf(path, sizeof(path),
    pmu.name, "caps/slots");
//
// The value of slots is not greater than 32 bits, but
// filename__read_int can't read value with 0x prefix,
// so use filename__read_ull instead.
//
    filename__read_ull(path, &slots);
    }
    return slots;
    }
