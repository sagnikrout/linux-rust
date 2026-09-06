//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/zhaoxin.c
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

pub const MSR_ZHAOXIN_FCR57: c_uint = 0x00001257;

#[no_mangle]
unsafe extern "C" fn init_zhaoxin_cap(c: *mut cpuinfo_x86) {
    static void init_zhaoxin_cap(struct cpuinfo_x86 *c)
    {
    u64 msr;
// Test for Extended Feature Flags presence
    if (cpuid_eax(0xC0000000) >= 0xC0000001) {
    let mut tmp: u32 = cpuid_edx(0xC0000001);
// Enable ACE unit, if present and disabled
    if ((tmp & (ACE_PRESENT | ACE_ENABLED)) == ACE_PRESENT) {
    rdmsrq(MSR_ZHAOXIN_FCR57, msr);
// Enable ACE unit
    wrmsrq(MSR_ZHAOXIN_FCR57, msr | ACE_FCR);
    pr_info("CPU: Enabled ACE h/w crypto\n");
    }
// Enable RNG unit, if present and disabled
    if ((tmp & (RNG_PRESENT | RNG_ENABLED)) == RNG_PRESENT) {
    rdmsrq(MSR_ZHAOXIN_FCR57, msr);
// Enable RNG unit
    wrmsrq(MSR_ZHAOXIN_FCR57, msr | RNG_ENABLE);
    pr_info("CPU: Enabled h/w RNG\n");
    }
//
// Store Extended Feature Flags as word 5 of the CPU
// capability bit array
//
    c.x86_capability[CPUID_C000_0001_EDX] = cpuid_edx(0xC0000001);
    }
    if (c.x86 >= 0x6)
    set_cpu_cap(c, X86_FEATURE_REP_GOOD);
    }
#[no_mangle]
unsafe extern "C" fn early_init_zhaoxin(c: *mut cpuinfo_x86) {
    static void early_init_zhaoxin(struct cpuinfo_x86 *c)
    {
    if (c.x86 >= 0x6)
    set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
    if (c.x86_power & (1 << 8)) {
    set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
    set_cpu_cap(c, X86_FEATURE_NONSTOP_TSC);
    }
    }
#[no_mangle]
unsafe extern "C" fn init_zhaoxin(c: *mut cpuinfo_x86) {
    static void init_zhaoxin(struct cpuinfo_x86 *c)
    {
    early_init_zhaoxin(c);
    init_intel_cacheinfo(c);
    if (c.cpuid_level > 9) {
    let mut eax: c_uint = cpuid_eax(10);
//
// Check for version and the number of counters
// Version(eax[7:0]) can't be 0;
// Counters(eax[15:8]) should be greater than 1;
//
    if ((eax & 0xff) && (((eax >> 8) & 0xff) > 1))
    set_cpu_cap(c, X86_FEATURE_ARCH_PERFMON);
    }
    if (c.x86 >= 0x6)
    init_zhaoxin_cap(c);

    set_cpu_cap(c, X86_FEATURE_LFENCE_RDTSC);

    init_ia32_feat_ctl(c);
    }

    static unsigned int
    zhaoxin_size_cache(struct cpuinfo_x86 *c, unsigned int size)
    {
    return size;
    }

    static const struct cpu_dev zhaoxin_cpu_dev = {
    .c_vendor	= "zhaoxin",
    .c_ident	= { "  Shanghai  " },
    .c_early_init	= early_init_zhaoxin,
    .c_init		= init_zhaoxin,

    .legacy_cache_size = zhaoxin_size_cache,

    .c_x86_vendor	= X86_VENDOR_ZHAOXIN,
    };
    cpu_dev_register(zhaoxin_cpu_dev);
