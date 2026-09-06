//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/centaur.c
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
unsafe extern "C" fn init_c3(c: *mut cpuinfo_x86) {
    static void init_c3(struct cpuinfo_x86 *c)
    {
    u64 msr;
// Test for Centaur Extended Feature Flags presence
    if (cpuid_eax(0xC0000000) >= 0xC0000001) {
    let mut tmp: u32 = cpuid_edx(0xC0000001);
// enable ACE unit, if present and disabled
    if ((tmp & (ACE_PRESENT | ACE_ENABLED)) == ACE_PRESENT) {
    rdmsrq(MSR_VIA_FCR, msr);
// enable ACE unit
    wrmsrq(MSR_VIA_FCR, msr | ACE_FCR);
    pr_info("CPU: Enabled ACE h/w crypto\n");
    }
// enable RNG unit, if present and disabled
    if ((tmp & (RNG_PRESENT | RNG_ENABLED)) == RNG_PRESENT) {
    rdmsrq(MSR_VIA_RNG, msr);
// enable RNG unit
    wrmsrq(MSR_VIA_RNG, msr | RNG_ENABLE);
    pr_info("CPU: Enabled h/w RNG\n");
    }
// store Centaur Extended Feature Flags as
// word 5 of the CPU capability bit array
//
    c.x86_capability[CPUID_C000_0001_EDX] = cpuid_edx(0xC0000001);
    }

// Cyrix III family needs CX8 & PGE explicitly enabled.
    if (c.x86_model >= 6 && c.x86_model <= 13) {
    rdmsrq(MSR_VIA_FCR, msr);
    wrmsrq(MSR_VIA_FCR, msr | (1 << 1 | 1 << 7));
    set_cpu_cap(c, X86_FEATURE_CX8);
    }
// Before Nehemiah, the C3's had 3dNOW!
    if (c.x86_model >= 6 && c.x86_model < 9)
    set_cpu_cap(c, X86_FEATURE_3DNOW);

    if (c.x86 == 0x6 && c.x86_model >= 0xf) {
    c.x86_cache_alignment = c.x86_clflush_size * 2;
    set_cpu_cap(c, X86_FEATURE_REP_GOOD);
    }
    if (c.x86 >= 7)
    set_cpu_cap(c, X86_FEATURE_REP_GOOD);
    }
    enum {
    ECX8		= 1<<1,
    EIERRINT	= 1<<2,
    DPM		= 1<<3,
    DMCE		= 1<<4,
    DSTPCLK		= 1<<5,
    ELINEAR		= 1<<6,
    DSMC		= 1<<7,
    DTLOCK		= 1<<8,
    EDCTLB		= 1<<8,
    EMMX		= 1<<9,
    DPDC		= 1<<11,
    EBRPRED		= 1<<12,
    DIC		= 1<<13,
    DDC		= 1<<14,
    DNA		= 1<<15,
    ERETSTK		= 1<<16,
    E2MMX		= 1<<19,
    EAMD3D		= 1<<20,
    };
#[no_mangle]
unsafe extern "C" fn early_init_centaur(c: *mut cpuinfo_x86) {
    static void early_init_centaur(struct cpuinfo_x86 *c)
    {

// Emulate MTRRs using Centaur's MCR.
    if (c.x86 == 5)
    set_cpu_cap(c, X86_FEATURE_CENTAUR_MCR);

    if ((c.x86 == 6 && c.x86_model >= 0xf) ||
    (c.x86 >= 7))
    set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
    if (c.x86_power & (1 << 8)) {
    set_cpu_cap(c, X86_FEATURE_CONSTANT_TSC);
    set_cpu_cap(c, X86_FEATURE_NONSTOP_TSC);
    }
    }
#[no_mangle]
unsafe extern "C" fn init_centaur(c: *mut cpuinfo_x86) {
    static void init_centaur(struct cpuinfo_x86 *c)
    {

    char *name;
    let mut fcr_set: u32 = 0;
    let mut fcr_clr: u32 = 0;
    u32  newlo;
    u32  aa, bb, cc, dd;
    struct msr val;

    early_init_centaur(c);
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

    if (c.x86 == 5) {
    switch (c.x86_model) {
    case 4:
    name = "C6";
    fcr_set = ECX8|DSMC|EDCTLB|EMMX|ERETSTK;
    fcr_clr = DPDC;
    pr_notice("Disabling bugged TSC.\n");
    clear_cpu_cap(c, X86_FEATURE_TSC);
    break;
    case 8:
    switch (c.x86_stepping) {
    default:
    name = "2";
    break;
    case 7 ... 9:
    name = "2A";
    break;
    case 10 ... 15:
    name = "2B";
    break;
    }
    fcr_set = ECX8|DSMC|DTLOCK|EMMX|EBRPRED|ERETSTK|
    E2MMX|EAMD3D;
    fcr_clr = DPDC;
    break;
    case 9:
    name = "3";
    fcr_set = ECX8|DSMC|DTLOCK|EMMX|EBRPRED|ERETSTK|
    E2MMX|EAMD3D;
    fcr_clr = DPDC;
    break;
    default:
    name = "??";
    }
    rdmsrq(MSR_IDT_FCR1, val.q);
    newlo = (val.l | fcr_set) & (~fcr_clr);
    if (newlo != val.l) {
    pr_info("Centaur FCR was 0x%X now 0x%X\n",
    val.l, newlo);
    val.l = newlo;
    wrmsrq(MSR_IDT_FCR1, val.q);
    } else {
    pr_info("Centaur FCR is 0x%X\n", val.l);
    }
// Emulate MTRRs using Centaur's MCR.
    set_cpu_cap(c, X86_FEATURE_CENTAUR_MCR);
// Report CX8
    set_cpu_cap(c, X86_FEATURE_CX8);
// Set 3DNow! on Winchip 2 and above.
    if (c.x86_model >= 8)
    set_cpu_cap(c, X86_FEATURE_3DNOW);
// See if we can find out some more.
    if (cpuid_eax(0x80000000) >= 0x80000005) {
// Yes, we can.
    cpuid(0x80000005, &aa, &bb, &cc, &dd);
// Add L1 data and code cache sizes.
    c.x86_cache_size = (cc>>24)+(dd>>24);
    }
    sprintf(c.x86_model_id, "WinChip %s", name);
    }

    if (c.x86 == 6 || c.x86 >= 7)
    init_c3(c);

    set_cpu_cap(c, X86_FEATURE_LFENCE_RDTSC);

    init_ia32_feat_ctl(c);
    }

    static unsigned int
    centaur_size_cache(struct cpuinfo_x86 *c, unsigned int size)
    {
// VIA C3 CPUs (670-68F) need further shifting.
    if ((c.x86 == 6) && ((c.x86_model == 7) || (c.x86_model == 8)))
    size >>= 8;
//
// There's also an erratum in Nehemiah stepping 1, which
// returns '65KB' instead of '64KB'
// - Note, it seems this may only be in engineering samples.
//
    if ((c.x86 == 6) && (c.x86_model == 9) &&
    (c.x86_stepping == 1) && (size == 65))
    size -= 1;
    return size;
    }

    static const struct cpu_dev centaur_cpu_dev = {
    .c_vendor	= "Centaur",
    .c_ident	= { "CentaurHauls" },
    .c_early_init	= early_init_centaur,
    .c_init		= init_centaur,

    .legacy_cache_size = centaur_size_cache,

    .c_x86_vendor	= X86_VENDOR_CENTAUR,
    };
    cpu_dev_register(centaur_cpu_dev);
