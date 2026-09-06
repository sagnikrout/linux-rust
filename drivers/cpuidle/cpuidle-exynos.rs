//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-exynos.c
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
// Copyright (c) 2011-2014 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Coupled cpuidle support based on the work of:
// Colin Cross <ccross@android.com>
// Daniel Lezcano <daniel.lezcano@linaro.org>
//

    static atomic_t exynos_idle_barrier;
    static struct cpuidle_exynos_data *exynos_cpuidle_pdata;
    static void (*exynos_enter_aftr)(void);
    static int exynos_enter_coupled_lowpower(struct cpuidle_device *dev,
    struct cpuidle_driver *drv,
    int index)
    {
    int ret;
    exynos_cpuidle_pdata.pre_enter_aftr();
//
// Waiting all cpus to reach this point at the same moment
//
    cpuidle_coupled_parallel_barrier(dev, &exynos_idle_barrier);
//
// Both cpus will reach this point at the same time
//
    ret = dev.cpu ? exynos_cpuidle_pdata.cpu1_powerdown()
    : exynos_cpuidle_pdata.cpu0_enter_aftr();
    if (ret)
    index = ret;
//
// Waiting all cpus to finish the power sequence before going further
//
    cpuidle_coupled_parallel_barrier(dev, &exynos_idle_barrier);
    exynos_cpuidle_pdata.post_enter_aftr();
    return index;
    }
    static int exynos_enter_lowpower(struct cpuidle_device *dev,
    struct cpuidle_driver *drv,
    int index)
    {
    let mut new_index: c_int = index;
// AFTR can only be entered when cores other than CPU0 are offline
    if (num_online_cpus() > 1 || dev.cpu != 0)
    new_index = drv.safe_state_index;
    if (new_index == 0)
    return arm_cpuidle_simple_enter(dev, drv, new_index);
    exynos_enter_aftr();
    return new_index;
    }
    static struct cpuidle_driver exynos_idle_driver = {
    .name			= "exynos_idle",
    .owner			= THIS_MODULE,
    .states = {
    [0] = ARM_CPUIDLE_WFI_STATE,
    [1] = {
    .enter			= exynos_enter_lowpower,
    .exit_latency		= 300,
    .target_residency	= 10000,
    .name			= "C1",
    .desc			= "ARM power down",
    },
    },
    .state_count = 2,
    .safe_state_index = 0,
    };
    static struct cpuidle_driver exynos_coupled_idle_driver = {
    .name			= "exynos_coupled_idle",
    .owner			= THIS_MODULE,
    .states = {
    [0] = ARM_CPUIDLE_WFI_STATE,
    [1] = {
    .enter			= exynos_enter_coupled_lowpower,
    .exit_latency		= 5000,
    .target_residency	= 10000,
    .flags			= CPUIDLE_FLAG_COUPLED |
    CPUIDLE_FLAG_TIMER_STOP,
    .name			= "C1",
    .desc			= "ARM power down",
    },
    },
    .state_count = 2,
    .safe_state_index = 0,
    };
#[no_mangle]
unsafe extern "C" fn exynos_cpuidle_probe(pdev: *mut platform_device) -> c_int {
    static int exynos_cpuidle_probe(struct platform_device *pdev)
    {
    int ret;
    if (IS_ENABLED(CONFIG_SMP) &&
    (of_machine_is_compatible("samsung,exynos4210") ||
    of_machine_is_compatible("samsung,exynos3250"))) {
    exynos_cpuidle_pdata = pdev.dev.platform_data;
    ret = cpuidle_register(&exynos_coupled_idle_driver,
    cpu_possible_mask);
    } else {
    exynos_enter_aftr = (void *)(pdev.dev.platform_data);
    ret = cpuidle_register(&exynos_idle_driver, core::ptr::null_mut());
    }
    if (ret) {
    dev_err(&pdev.dev, "failed to register cpuidle driver\n");
    return ret;
    }
    return 0;
    }
    static struct platform_driver exynos_cpuidle_driver = {
    .probe	= exynos_cpuidle_probe,
    .driver = {
    .name = "exynos_cpuidle",
    },
    };
    builtin_platform_driver(exynos_cpuidle_driver);
