//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/86xx/mpc86xx_smp.c
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
// Author: Xianghua Xiao <x.xiao@freescale.com>
// Zhang Wei <wei.zhang@freescale.com>
//
// Copyright 2006 Freescale Semiconductor Inc.
//

    extern void __secondary_start_mpc86xx(void);
pub const MCM_PORT_CONFIG_OFFSET: c_uint = 0x10;
// Offset from CCSRBAR

    static void __init
    smp_86xx_release_core(int nr)
    {
    __be32 __iomem *mcm_vaddr;
    unsigned long pcr;
    if (nr < 0 || nr >= NR_CPUS)
    return;
//
// Startup Core #nr.
//
    mcm_vaddr = ioremap(get_immrbase() + MPC86xx_MCM_OFFSET,
    MPC86xx_MCM_SIZE);
    pcr = in_be32(mcm_vaddr + (MCM_PORT_CONFIG_OFFSET >> 2));
    pcr |= 1 << (nr + 24);
    out_be32(mcm_vaddr + (MCM_PORT_CONFIG_OFFSET >> 2), pcr);
    iounmap(mcm_vaddr);
    }
    static int __init
    smp_86xx_kick_cpu(int nr)
    {
    unsigned int save_vector;
    unsigned long target, flags;
    let mut n: c_int = 0;
    unsigned int *vector = (unsigned int *)(KERNELBASE + 0x100);
    if (nr < 0 || nr >= NR_CPUS)
    return -ENOENT;
    pr_debug("smp_86xx_kick_cpu: kick CPU #%d\n", nr);
    local_irq_save(flags);
// Save reset vector
    save_vector = *vector;
// Setup fake reset vector to call __secondary_start_mpc86xx.
    target = (unsigned long) __secondary_start_mpc86xx;
    patch_branch(vector, target, BRANCH_SET_LINK);
// Kick that CPU
    smp_86xx_release_core(nr);
// Wait a bit for the CPU to take the exception.
    while ((__secondary_hold_acknowledge != nr) && (n++, n < 1000))
    mdelay(1);
// Restore the exception vector
    patch_instruction(vector, ppc_inst(save_vector));
    local_irq_restore(flags);
    pr_debug("wait CPU #%d for %d msecs.\n", nr, n);
    return 0;
    }
    static void __init
    smp_86xx_setup_cpu(int cpu_nr)
    {
    mpic_setup_this_cpu();
    }
    struct smp_ops_t smp_86xx_ops = {
    .cause_nmi_ipi = core::ptr::null_mut(),
    .message_pass = smp_mpic_message_pass,
    .probe = smp_mpic_probe,
    .kick_cpu = smp_86xx_kick_cpu,
    .setup_cpu = smp_86xx_setup_cpu,
    .take_timebase = smp_generic_take_timebase,
    .give_timebase = smp_generic_give_timebase,
    };
    void __init
    mpc86xx_smp_init(void)
    {
    smp_ops = &smp_86xx_ops;
    }
