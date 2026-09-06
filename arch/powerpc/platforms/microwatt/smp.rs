//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/microwatt/smp.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SMP support functions for Microwatt
// Copyright 2025 Paul Mackerras <paulus@ozlabs.org>
//

#[no_mangle]
unsafe extern "C" fn microwatt_smp_probe() -> void __init {
    static void __init microwatt_smp_probe(void)
    {
    xics_smp_probe();
    }
#[no_mangle]
unsafe extern "C" fn microwatt_smp_setup_cpu(cpu: c_int) {
    static void microwatt_smp_setup_cpu(int cpu)
    {
    if (cpu != 0)
    xics_setup_cpu();
    }
    static struct smp_ops_t microwatt_smp_ops = {
    .probe		= microwatt_smp_probe,
    .message_pass	= core::ptr::null_mut(),		/* Use smp_muxed_ipi_message_pass */
    .kick_cpu	= smp_generic_kick_cpu,
    .setup_cpu	= microwatt_smp_setup_cpu,
    };
// XXX get from device tree
pub const SYSCON_BASE: c_uint = 0xc0000000;
pub const SYSCON_LENGTH: c_uint = 0x100;
pub const SYSCON_CPU_CTRL: c_uint = 0x58;
#[no_mangle]
pub unsafe extern "C" fn microwatt_init_smp() -> void __init {
    void __init microwatt_init_smp(void)
    {
    volatile unsigned char __iomem *syscon;
    int ncpus;
    int timeout;
    syscon = early_ioremap(SYSCON_BASE, SYSCON_LENGTH);
    if (syscon == core::ptr::null_mut()) {
    pr_err("Failed to map SYSCON\n");
    return;
    }
    ncpus = (readl(syscon + SYSCON_CPU_CTRL) >> 8) & 0xff;
    if (ncpus < 2)
    goto out;
    smp_ops = &microwatt_smp_ops;
//
// Write two instructions at location 0:
// mfspr r3, PIR
// b __secondary_hold
//
// (unsigned int *)KERNELBASE = PPC_RAW_MFSPR(3, SPRN_PIR);
// (unsigned int *)(KERNELBASE+4) = PPC_RAW_BRANCH(&__secondary_hold - (char *)(KERNELBASE+4));
// enable the other CPUs, they start at location 0
    writel((1ul << ncpus) - 1, syscon + SYSCON_CPU_CTRL);
    timeout = 10000;
    while (!__secondary_hold_acknowledge) {
    if (--timeout == 0)
    break;
    barrier();
    }
    out:
    early_iounmap((void *)syscon, SYSCON_LENGTH);
    }
