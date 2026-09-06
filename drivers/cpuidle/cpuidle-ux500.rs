//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-ux500.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2012 Linaro : Daniel Lezcano <daniel.lezcano@linaro.org> (IBM)
//
// Based on the work of Rickard Andersson <rickard.andersson@stericsson.com>
// and Jonas Aaberg <jonas.aberg@stericsson.com>.
//

    let mut master: static atomic_t = ATOMIC_INIT(0);
    static DEFINE_SPINLOCK(master_lock);
    static inline int ux500_enter_idle(struct cpuidle_device *dev,
    struct cpuidle_driver *drv, int index)
    {
    let mut this_cpu: c_int = smp_processor_id();
    let mut recouple: bool = false;
    if (atomic_inc_return(&master) == num_online_cpus()) {
// With this lock, we prevent the other cpu to exit and enter
// this function again and become the master
    if (!spin_trylock(&master_lock))
    goto wfi;
// decouple the gic from the A9 cores
    if (prcmu_gic_decouple()) {
    spin_unlock(&master_lock);
    goto out;
    }
// If an error occur, we will have to recouple the gic
// manually
    recouple = true;
// At this state, as the gic is decoupled, if the other
// cpu is in WFI, we have the guarantee it won't be wake
// up, so we can safely go to retention
    if (!prcmu_is_cpu_in_wfi(this_cpu ? 0 : 1))
    goto out;
// The prcmu will be in charge of watching the interrupts
// and wake up the cpus
    if (prcmu_copy_gic_settings())
    goto out;
// Check in the meantime an interrupt did
// not occur on the gic ...
    if (prcmu_gic_pending_irq())
    goto out;
// ... and the prcmu
    if (prcmu_pending_irq())
    goto out;
// Go to the retention state, the prcmu will wait for the
// cpu to go WFI and this is what happens after exiting this
// 'master' critical section
    if (db8500_prcmu_set_power_state(PRCMU_AP_IDLE, true, true))
    goto out;
// When we switch to retention, the prcmu is in charge
// of recoupling the gic automatically
    recouple = false;
    spin_unlock(&master_lock);
    }
    wfi:
    cpu_do_idle();
    out:
    atomic_dec(&master);
    if (recouple) {
    prcmu_gic_recouple();
    spin_unlock(&master_lock);
    }
    return index;
    }
    static struct cpuidle_driver ux500_idle_driver = {
    .name = "ux500_idle",
    .owner = THIS_MODULE,
    .states = {
    ARM_CPUIDLE_WFI_STATE,
    {
    .enter		  = ux500_enter_idle,
    .exit_latency	  = 70,
    .target_residency = 260,
    .flags		  = CPUIDLE_FLAG_TIMER_STOP,
    .name		  = "ApIdle",
    .desc		  = "ARM Retention",
    },
    },
    .safe_state_index = 0,
    .state_count = 2,
    };
#[no_mangle]
unsafe extern "C" fn dbx500_cpuidle_probe(pdev: *mut platform_device) -> c_int {
    static int dbx500_cpuidle_probe(struct platform_device *pdev)
    {
// Configure wake up reasons
    db8500_prcmu_enable_wakeups(PRCMU_WAKEUP(ARM) | PRCMU_WAKEUP(RTC) |
    PRCMU_WAKEUP(ABB));
    return cpuidle_register(&ux500_idle_driver, core::ptr::null_mut());
    }
    static struct platform_driver dbx500_cpuidle_plat_driver = {
    .driver = {
    .name = "db8500-cpuidle",
    },
    .probe = dbx500_cpuidle_probe,
    };
    builtin_platform_driver(dbx500_cpuidle_plat_driver);
