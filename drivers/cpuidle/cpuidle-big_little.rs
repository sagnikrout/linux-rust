//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-big_little.c
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
// Copyright (c) 2013 ARM/Linaro
//
// Authors: Daniel Lezcano <daniel.lezcano@linaro.org>
// Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
// Nicolas Pitre <nicolas.pitre@linaro.org>
//
// Maintainer: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
// Maintainer: Daniel Lezcano <daniel.lezcano@linaro.org>
//

    static int bl_enter_powerdown(struct cpuidle_device *dev,
    struct cpuidle_driver *drv, int idx);
//
// NB: Owing to current menu governor behaviour big and LITTLE
// index 1 states have to define exit_latency and target_residency for
// cluster state since, when all CPUs in a cluster hit it, the cluster
// can be shutdown. This means that when a single CPU enters this state
// the exit_latency and target_residency values are somewhat overkill.
// There is no notion of cluster states in the menu governor, so CPUs
// have to define CPU states where possibly the cluster will be shutdown
// depending on the state of other CPUs. idle states entry and exit happen
// at random times; however the cluster state provides target_residency
// values as if all CPUs in a cluster enter the state at once; this is
// somewhat optimistic and behaviour should be fixed either in the governor
// or in the MCPM back-ends.
// To make this driver 100% generic the number of states and the exit_latency
// target_residency values must be obtained from device tree bindings.
//
// exit_latency: refers to the TC2 vexpress test chip and depends on the
// current cluster operating point. It is the time it takes to get the CPU
// up and running when the CPU is powered up on cluster wake-up from shutdown.
// Current values for big and LITTLE clusters are provided for clusters
// running at default operating points.
//
// target_residency: it is the minimum amount of time the cluster has
// to be down to break even in terms of power consumption. cluster
// shutdown has inherent dynamic power costs (L2 writebacks to DRAM
// being the main factor) that depend on the current operating points.
// The current values for both clusters are provided for a CPU whose half
// of L2 lines are dirty and require cleaning to DRAM, and takes into
// account leakage static power values related to the vexpress TC2 testchip.
//
    static struct cpuidle_driver bl_idle_little_driver = {
    .name = "little_idle",
    .owner = THIS_MODULE,
    .states[0] = ARM_CPUIDLE_WFI_STATE,
    .states[1] = {
    .enter			= bl_enter_powerdown,
    .exit_latency		= 700,
    .target_residency	= 2500,
    .flags			= CPUIDLE_FLAG_TIMER_STOP |
    CPUIDLE_FLAG_RCU_IDLE,
    .name			= "C1",
    .desc			= "ARM little-cluster power down",
    },
    .state_count = 2,
    };
    static const struct of_device_id bl_idle_state_match[] __initconst = {
    { .compatible = "arm,idle-state",
    .data = bl_enter_powerdown },
    { },
    };
    static struct cpuidle_driver bl_idle_big_driver = {
    .name = "big_idle",
    .owner = THIS_MODULE,
    .states[0] = ARM_CPUIDLE_WFI_STATE,
    .states[1] = {
    .enter			= bl_enter_powerdown,
    .exit_latency		= 500,
    .target_residency	= 2000,
    .flags			= CPUIDLE_FLAG_TIMER_STOP |
    CPUIDLE_FLAG_RCU_IDLE,
    .name			= "C1",
    .desc			= "ARM big-cluster power down",
    },
    .state_count = 2,
    };
//
// notrace prevents trace shims from getting inserted where they
// should not. Global jumps and ldrex/strex must not be inserted
// in power down sequences where caches and MMU may be turned off.
//
#[no_mangle]
unsafe extern "C" fn bl_powerdown_finisher(arg: c_ulong) -> int notrace {
    static int notrace bl_powerdown_finisher(unsigned long arg)
    {
// MCPM works with HW CPU identifiers
    let mut mpidr: c_uint = read_cpuid_mpidr();
    let mut cluster: c_uint = MPIDR_AFFINITY_LEVEL(mpidr, 1);
    let mut cpu: c_uint = MPIDR_AFFINITY_LEVEL(mpidr, 0);
    mcpm_set_entry_vector(cpu, cluster, cpu_resume);
    mcpm_cpu_suspend();
// return value != 0 means failure
    return 1;
    }
//
// bl_enter_powerdown - Programs CPU to enter the specified state
// @dev: cpuidle device
// @drv: The target state to be programmed
// @idx: state index
//
// Called from the CPUidle framework to program the device to the
// specified target state selected by the governor.
//
    static __cpuidle int bl_enter_powerdown(struct cpuidle_device *dev,
    struct cpuidle_driver *drv, int idx)
    {
    cpu_pm_enter();
    ct_cpuidle_enter();
    cpu_suspend(0, bl_powerdown_finisher);
// signals the MCPM core that CPU is out of low power state
    mcpm_cpu_powered_up();
    ct_cpuidle_exit();
    cpu_pm_exit();
    return idx;
    }
#[no_mangle]
unsafe extern "C" fn bl_idle_driver_init(drv: *mut cpuidle_driver, part_id: c_int) -> int __init {
    static int __init bl_idle_driver_init(struct cpuidle_driver *drv, int part_id)
    {
    struct cpumask *cpumask;
    int cpu;
    cpumask = kzalloc(cpumask_size(), GFP_KERNEL);
    if (!cpumask)
    return -ENOMEM;
    for_each_present_cpu(cpu)
    if (smp_cpuid_part(cpu) == part_id)
    cpumask_set_cpu(cpu, cpumask);
    drv.cpumask = cpumask;
    return 0;
    }
    static const struct of_device_id compatible_machine_match[] = {
    { .compatible = "arm,vexpress,v2p-ca15_a7" },
    { .compatible = "google,peach" },
    {},
    };
#[no_mangle]
unsafe extern "C" fn bl_idle_init() -> int __init {
    static int __init bl_idle_init(void)
    {
    int ret;
//
// Initialize the driver just for a compliant set of machines
//
    if (!of_machine_device_match(compatible_machine_match))
    return -ENODEV;
    if (!mcpm_is_available())
    return -EUNATCH;
//
// For now the differentiation between little and big cores
// is based on the part number. A7 cores are considered little
// cores, A15 are considered big cores. This distinction may
// evolve in the future with a more generic matching approach.
//
    ret = bl_idle_driver_init(&bl_idle_little_driver,
    ARM_CPU_PART_CORTEX_A7);
    if (ret)
    return ret;
    ret = bl_idle_driver_init(&bl_idle_big_driver, ARM_CPU_PART_CORTEX_A15);
    if (ret)
    goto out_uninit_little;
// Start at index 1, index 0 standard WFI
    ret = dt_init_idle_driver(&bl_idle_big_driver, bl_idle_state_match, 1);
    if (ret < 0)
    goto out_uninit_big;
// Start at index 1, index 0 standard WFI
    ret = dt_init_idle_driver(&bl_idle_little_driver,
    bl_idle_state_match, 1);
    if (ret < 0)
    goto out_uninit_big;
    ret = cpuidle_register(&bl_idle_little_driver, core::ptr::null_mut());
    if (ret)
    goto out_uninit_big;
    ret = cpuidle_register(&bl_idle_big_driver, core::ptr::null_mut());
    if (ret)
    goto out_unregister_little;
    return 0;
    out_unregister_little:
    cpuidle_unregister(&bl_idle_little_driver);
    out_uninit_big:
    kfree(bl_idle_big_driver.cpumask);
    out_uninit_little:
    kfree(bl_idle_little_driver.cpumask);
    return ret;
    }
    device_initcall(bl_idle_init);
