//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/util/tsc.c
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
pub unsafe extern "C" fn rdtsc() -> u64 {
    u64 rdtsc(void)
    {
    unsigned int low, high;
    asm volatile("rdtsc" : "=a" (low), "=d" (high));
    return low | ((u64)high) << 32;
    }
//
// Derive the TSC frequency in Hz from the /proc/cpuinfo, for example:
// ...
// model name      : Intel(R) Xeon(R) Gold 6154 CPU @ 3.00GHz
// ...
// will return 3000000000.
//
#[no_mangle]
unsafe extern "C" fn cpuinfo_tsc_freq() -> u64 {
    static u64 cpuinfo_tsc_freq(void)
    {
    let mut result: u64 = 0;
    FILE *cpuinfo;
    char *line = core::ptr::null_mut();
    let mut len: usize = 0;
    cpuinfo = fopen("/proc/cpuinfo", "r");
    if (!cpuinfo) {
    pr_err("Failed to read /proc/cpuinfo for TSC frequency\n");
    return 0;
    }
    while (getline(&line, &len, cpuinfo) > 0) {
    if (!strncmp(line, "model name", 10)) {
    char *pos = strstr(line + 11, " @ ");
    double float_result;
    if (pos && sscanf(pos, " @ %lfGHz", &float_result) == 1) {
    float_result *= 1000000000;
    result = (u64)float_result;
    goto out;
    }
    }
    }
    out:
    if (result == 0)
    pr_err("Failed to find TSC frequency in /proc/cpuinfo\n");
    free(line);
    fclose(cpuinfo);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_get_tsc_freq() -> u64 {
    u64 arch_get_tsc_freq(void)
    {
    unsigned int a, b, c, d, lvl;
    static bool cached;
    static double tsc;
    char vendor[16];
    if (cached)
    return tsc;
    cached = true;
    get_cpuid_0(vendor, &lvl);
    if (!strstr(vendor, "Intel"))
    return 0;
//
// Don't support Time Stamp Counter and
// Nominal Core Crystal Clock Information Leaf.
//
    if (lvl < 0x15) {
    tsc = cpuinfo_tsc_freq();
    return tsc;
    }
    cpuid(0x15, 0, &a, &b, &c, &d);
// TSC frequency is not enumerated
    if (!a || !b || !c) {
    tsc = cpuinfo_tsc_freq();
    return tsc;
    }
    tsc = (u64)c * (u64)b / (u64)a;
    return tsc;
    }
