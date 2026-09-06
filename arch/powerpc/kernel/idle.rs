//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/idle.c
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
// Idle daemon for PowerPC.  Idle daemon will handle any action
// that needs to be taken when the system becomes idle.
//
// Originally written by Cort Dougan (cort@cs.nmt.edu).
// Subsequent 32-bit hacking by Tom Rini, Armin Kuster,
// Paul Mackerras and others.
//
// iSeries supported added by Mike Corrigan <mikejc@us.ibm.com>
//
// Additional shared processor, SMT, and firmware support
// Copyright (c) 2003 Dave Engebretsen <engebret@us.ibm.com>
//
// 32-bit and 64-bit versions merged by Paul Mackerras <paulus@samba.org>
//

    let mut cpuidle_disable: c_ulong = IDLE_NO_OVERRIDE;
    EXPORT_SYMBOL(cpuidle_disable);
#[no_mangle]
unsafe extern "C" fn powersave_off(arg: *mut c_char) -> int __init {
    static int __init powersave_off(char *arg)
    {
    ppc_md.power_save = core::ptr::null_mut();
    cpuidle_disable = IDLE_POWERSAVE_OFF;
    return 1;
    }
    __setup("powersave=off", powersave_off);
#[no_mangle]
pub unsafe extern "C" fn arch_cpu_idle() {
    void arch_cpu_idle(void)
    {
    ppc64_runlatch_off();
    if (ppc_md.power_save) {
    ppc_md.power_save();
//
// Some power_save functions return with
// interrupts enabled, some don't.
//
    if (!irqs_disabled())
    raw_local_irq_disable();
    } else {
//
// Go into low thread priority and possibly
// low power mode.
//
    HMT_low();
    HMT_very_low();
    }
    HMT_medium();
    ppc64_runlatch_on();
    }
    int powersave_nap;

#[no_mangle]
pub unsafe extern "C" fn power4_idle() {
    void power4_idle(void)
    {
    if (!cpu_has_feature(CPU_FTR_CAN_NAP))
    return;
    if (!powersave_nap)
    return;
    if (!prep_irq_for_idle())
    return;
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    asm volatile(PPC_DSSALL " ; sync" ::: "memory");
    power4_idle_nap();
//
// power4_idle_nap returns with interrupts enabled (soft and hard).
// to our caller with interrupts enabled (soft and hard). Our caller
// can cope with either interrupts disabled or enabled upon return.
//
    }

//
// Register the sysctl to set/clear powersave_nap.
//
    static const struct ctl_table powersave_nap_ctl_table[] = {
    {
    .procname	= "powersave-nap",
    .data		= &powersave_nap,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    };
    static int __init
    register_powersave_nap_sysctl(void)
    {
    register_sysctl("kernel", powersave_nap_ctl_table);
    return 0;
    }
    __initcall(register_powersave_nap_sysctl);
