//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/transmeta.c
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
unsafe extern "C" fn early_init_transmeta(c: *mut cpuinfo_x86) {
    static void early_init_transmeta(struct cpuinfo_x86 *c)
    {
    u32 xlvl;
// Transmeta-defined flags: level 0x80860001
    xlvl = cpuid_eax(0x80860000);
    if ((xlvl & 0xffff0000) == 0x80860000) {
    if (xlvl >= 0x80860001)
    c.x86_capability[CPUID_8086_0001_EDX] = cpuid_edx(0x80860001);
    }
    }
#[no_mangle]
unsafe extern "C" fn init_transmeta(c: *mut cpuinfo_x86) {
    static void init_transmeta(struct cpuinfo_x86 *c)
    {
    u64 msr;
    unsigned int max, dummy;
    unsigned int cms_rev1, cms_rev2;
    unsigned int cpu_rev, cpu_freq = 0, cpu_flags, new_cpu_rev;
    char cpu_info[65];
    early_init_transmeta(c);
    cpu_detect_cache_sizes(c);
// Print CMS and CPU revision
    max = cpuid_eax(0x80860000);
    cpu_rev = 0;
    if (max >= 0x80860001) {
    cpuid(0x80860001, &dummy, &cpu_rev, &cpu_freq, &cpu_flags);
    if (cpu_rev != 0x02000000) {
    pr_info("CPU: Processor revision %u.%u.%u.%u, %u MHz\n",
    (cpu_rev >> 24) & 0xff,
    (cpu_rev >> 16) & 0xff,
    (cpu_rev >> 8) & 0xff,
    cpu_rev & 0xff,
    cpu_freq);
    }
    }
    if (max >= 0x80860002) {
    cpuid(0x80860002, &new_cpu_rev, &cms_rev1, &cms_rev2, &dummy);
    if (cpu_rev == 0x02000000) {
    pr_info("CPU: Processor revision %08X, %u MHz\n",
    new_cpu_rev, cpu_freq);
    }
    pr_info("CPU: Code Morphing Software revision %u.%u.%u-%u-%u\n",
    (cms_rev1 >> 24) & 0xff,
    (cms_rev1 >> 16) & 0xff,
    (cms_rev1 >> 8) & 0xff,
    cms_rev1 & 0xff,
    cms_rev2);
    }
    if (max >= 0x80860006) {
    cpuid(0x80860003,
    (void *)&cpu_info[0],
    (void *)&cpu_info[4],
    (void *)&cpu_info[8],
    (void *)&cpu_info[12]);
    cpuid(0x80860004,
    (void *)&cpu_info[16],
    (void *)&cpu_info[20],
    (void *)&cpu_info[24],
    (void *)&cpu_info[28]);
    cpuid(0x80860005,
    (void *)&cpu_info[32],
    (void *)&cpu_info[36],
    (void *)&cpu_info[40],
    (void *)&cpu_info[44]);
    cpuid(0x80860006,
    (void *)&cpu_info[48],
    (void *)&cpu_info[52],
    (void *)&cpu_info[56],
    (void *)&cpu_info[60]);
    cpu_info[64] = '\0';
    pr_info("CPU: %s\n", cpu_info);
    }
// Unhide possibly hidden capability flags
    rdmsrq(0x80860004, msr);
    wrmsrq(0x80860004, msr | ~0U);
    cpuid_refresh_leaf(c, 0x1);
    c.x86_capability[CPUID_1_EDX] = cpuid_edx(0x00000001);
    wrmsrq(0x80860004, msr);
// All Transmeta CPUs have a constant TSC
    set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);

//
// randomize_va_space slows us down enormously;
// it probably triggers retranslation of x86->native bytecode
//
    randomize_va_space = 0;

    }
    static const struct cpu_dev transmeta_cpu_dev = {
    .c_vendor	= "Transmeta",
    .c_ident	= { "GenuineTMx86", "TransmetaCPU" },
    .c_early_init	= early_init_transmeta,
    .c_init		= init_transmeta,
    .c_x86_vendor	= X86_VENDOR_TRANSMETA,
    };
    cpu_dev_register(transmeta_cpu_dev);
