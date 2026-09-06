//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/processor_thermal.c
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
// processor_thermal.c - Passive cooling submodule of the ACPI processor driver
//
// Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
// Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
// Copyright (C) 2004       Dominik Brodowski <linux@brodo.de>
// Copyright (C) 2004  Anil S Keshavamurthy <anil.s.keshavamurthy@intel.com>
// - Added processor hotplug support
//

// If a passive cooling situation is detected, primarily CPUfreq is used, as it
// offers (in most cases) voltage scaling in addition to frequency scaling, and
// thus a cubic (instead of linear) reduction of energy. Also, we allow for
// _any_ cpufreq driver and not only the acpi-cpufreq driver.
//
pub const CPUFREQ_THERMAL_MIN_STEP: c_int = 0;
    let mut __read_mostly: static int cpufreq_thermal_max_step = 3;
//
// Minimum throttle percentage for processor_thermal cooling device.
// The processor_thermal driver uses it to calculate the percentage amount by
// which cpu frequency must be reduced for each cooling state. This is also used
// to calculate the maximum number of throttling steps or cooling states.
//
    let mut __read_mostly: static int cpufreq_thermal_reduction_pctg = 20;
    static DEFINE_PER_CPU(unsigned int, cpufreq_thermal_reduction_step);

    per_cpu(cpufreq_thermal_reduction_step, phys_package_first_cpu(cpu))
//
// Emulate "per package data" using per cpu data (which should really be
// provided elsewhere)
//
// Note we can lose a CPU on cpu hotunplug, in this case we forget the state
// temporarily. Fortunately that's not a big issue here (I hope)
//
#[no_mangle]
unsafe extern "C" fn phys_package_first_cpu(cpu: c_int) -> c_int {
    static int phys_package_first_cpu(int cpu)
    {
    int i;
    let mut id: c_int = topology_physical_package_id(cpu);
    for_each_online_cpu(i)
    if (topology_physical_package_id(i) == id)
    return i;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpu_has_cpufreq(cpu: c_uint) -> bool {
    static bool cpu_has_cpufreq(unsigned int cpu)
    {
    if (!acpi_processor_cpufreq_init)
    return 0;
    struct cpufreq_policy *policy __free(put_cpufreq_policy) = cpufreq_cpu_get(cpu);
    return policy != core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_get_max_state(cpu: c_uint) -> c_int {
    static int cpufreq_get_max_state(unsigned int cpu)
    {
    if (!cpu_has_cpufreq(cpu))
    return 0;
    return cpufreq_thermal_max_step;
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_get_cur_state(cpu: c_uint) -> c_int {
    static int cpufreq_get_cur_state(unsigned int cpu)
    {
    if (!cpu_has_cpufreq(cpu))
    return 0;
    return reduction_step(cpu);
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_update_thermal_limit(cpu: c_uint, pr: *mut acpi_processor) -> bool {
    static bool cpufreq_update_thermal_limit(unsigned int cpu, struct acpi_processor *pr)
    {
    unsigned long max_freq;
    int ret;
    struct cpufreq_policy *policy __free(put_cpufreq_policy) = cpufreq_cpu_get(cpu);
    if (!policy)
    return false;
    max_freq = (policy.cpuinfo.max_freq *
    (100 - reduction_step(cpu) * cpufreq_thermal_reduction_pctg)) / 100;
    ret = freq_qos_update_request(&pr.thermal_req, max_freq);
    if (ret < 0) {
    pr_warn("Failed to update thermal freq constraint: CPU%d (%d)\n",
    pr.id, ret);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_set_cur_state(cpu: c_uint, state: c_int) -> c_int {
    static int cpufreq_set_cur_state(unsigned int cpu, int state)
    {
    struct acpi_processor *pr;
    int i;
    if (!cpu_has_cpufreq(cpu))
    return 0;
    reduction_step(cpu) = state;
//
// Update all the CPUs in the same package because they all
// contribute to the temperature and often share the same
// frequency.
//
    for_each_online_cpu(i) {
    if (topology_physical_package_id(i) !=
    topology_physical_package_id(cpu))
    continue;
    pr = per_cpu(processors, i);
    if (unlikely(!freq_qos_request_active(&pr.thermal_req)))
    continue;
    if (!cpufreq_update_thermal_limit(i, pr))
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_thermal_cpufreq_config() {
    static void acpi_thermal_cpufreq_config(void)
    {
    let mut cpufreq_pctg: c_int = acpi_arch_thermal_cpufreq_pctg();
    if (!cpufreq_pctg)
    return;
    cpufreq_thermal_reduction_pctg = cpufreq_pctg;
//
// Derive the MAX_STEP from minimum throttle percentage so that the reduction
// percentage doesn't end up becoming negative. Also, cap the MAX_STEP so that
// the CPU performance doesn't become 0.
//
    cpufreq_thermal_max_step = (100 / cpufreq_pctg) - 2;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_thermal_cpufreq_init(policy: *mut cpufreq_policy) {
    void acpi_thermal_cpufreq_init(struct cpufreq_policy *policy)
    {
    unsigned int cpu;
    acpi_thermal_cpufreq_config();
    for_each_cpu(cpu, policy.related_cpus) {
    struct acpi_processor *pr = per_cpu(processors, cpu);
    int ret;
    if (!pr)
    continue;
    ret = freq_qos_add_request(&policy.constraints,
    &pr.thermal_req,
    FREQ_QOS_MAX, INT_MAX);
    if (ret < 0) {
    pr_err("Failed to add freq constraint for CPU%d (%d)\n",
    cpu, ret);
    continue;
    }
    thermal_cooling_device_update(pr.cdev);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_thermal_cpufreq_exit(policy: *mut cpufreq_policy) {
    void acpi_thermal_cpufreq_exit(struct cpufreq_policy *policy)
    {
    unsigned int cpu;
    for_each_cpu(cpu, policy.related_cpus) {
    struct acpi_processor *pr = per_cpu(processors, cpu);
    if (!pr)
    continue;
    freq_qos_remove_request(&pr.thermal_req);
    thermal_cooling_device_update(pr.cdev);
    }
    }

#[no_mangle]
unsafe extern "C" fn cpufreq_get_max_state(cpu: c_uint) -> c_int {
    static int cpufreq_get_max_state(unsigned int cpu)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_get_cur_state(cpu: c_uint) -> c_int {
    static int cpufreq_get_cur_state(unsigned int cpu)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpufreq_set_cur_state(cpu: c_uint, state: c_int) -> c_int {
    static int cpufreq_set_cur_state(unsigned int cpu, int state)
    {
    return 0;
    }

// thermal cooling device callbacks
#[no_mangle]
unsafe extern "C" fn acpi_processor_max_state(pr: *mut acpi_processor) -> c_int {
    static int acpi_processor_max_state(struct acpi_processor *pr)
    {
    let mut max_state: c_int = 0;
//
// There exists four states according to
// cpufreq_thermal_reduction_step. 0, 1, 2, 3
//
    max_state += cpufreq_get_max_state(pr.id);
    if (pr.flags.throttling)
    max_state += (pr.throttling.state_count -1);
    return max_state;
    }
    static int
    processor_get_max_state(struct thermal_cooling_device *cdev,
    unsigned long *state)
    {
    struct acpi_device *device = cdev.devdata;
    struct acpi_processor *pr;
    if (!device)
    return -EINVAL;
    pr = acpi_driver_data(device);
    if (!pr)
    return -EINVAL;
// state = acpi_processor_max_state(pr);
    return 0;
    }
    static int
    processor_get_cur_state(struct thermal_cooling_device *cdev,
    unsigned long *cur_state)
    {
    struct acpi_device *device = cdev.devdata;
    struct acpi_processor *pr;
    if (!device)
    return -EINVAL;
    pr = acpi_driver_data(device);
    if (!pr)
    return -EINVAL;
// cur_state = cpufreq_get_cur_state(pr->id);
    if (pr.flags.throttling)
// cur_state += pr->throttling.state;
    return 0;
    }
    static int
    processor_set_cur_state(struct thermal_cooling_device *cdev,
    unsigned long state)
    {
    struct acpi_device *device = cdev.devdata;
    struct acpi_processor *pr;
    let mut result: c_int = 0;
    int max_pstate;
    if (!device)
    return -EINVAL;
    pr = acpi_driver_data(device);
    if (!pr)
    return -EINVAL;
    max_pstate = cpufreq_get_max_state(pr.id);
    if (state > acpi_processor_max_state(pr))
    return -EINVAL;
    if (state <= max_pstate) {
    if (pr.flags.throttling && pr.throttling.state)
    result = acpi_processor_set_throttling(pr, 0, false);
    cpufreq_set_cur_state(pr.id, state);
    } else {
    cpufreq_set_cur_state(pr.id, max_pstate);
    result = acpi_processor_set_throttling(pr,
    state - max_pstate, false);
    }
    return result;
    }
    const struct thermal_cooling_device_ops processor_cooling_ops = {
    .get_max_state = processor_get_max_state,
    .get_cur_state = processor_get_cur_state,
    .set_cur_state = processor_set_cur_state,
    };
    int acpi_processor_thermal_init(struct acpi_processor *pr,
    struct acpi_device *device)
    {
    let mut result: c_int = 0;
    pr.cdev = thermal_cooling_device_register("Processor", device,
    &processor_cooling_ops);
    if (IS_ERR(pr.cdev)) {
    result = PTR_ERR(pr.cdev);
    return result;
    }
    dev_dbg(&device.dev, "registered as cooling_device%d\n",
    pr.cdev.id);
    result = sysfs_create_link(&device.dev.kobj,
    &pr.cdev.device.kobj,
    "thermal_cooling");
    if (result) {
    dev_err(&device.dev,
    "Failed to create sysfs link 'thermal_cooling'\n");
    goto err_thermal_unregister;
    }
    result = sysfs_create_link(&pr.cdev.device.kobj,
    &device.dev.kobj,
    "device");
    if (result) {
    dev_err(&pr.cdev.device,
    "Failed to create sysfs link 'device'\n");
    goto err_remove_sysfs_thermal;
    }
    return 0;
    err_remove_sysfs_thermal:
    sysfs_remove_link(&device.dev.kobj, "thermal_cooling");
    err_thermal_unregister:
    thermal_cooling_device_unregister(pr.cdev);
    return result;
    }
    void acpi_processor_thermal_exit(struct acpi_processor *pr,
    struct acpi_device *device)
    {
    if (pr.cdev) {
    sysfs_remove_link(&device.dev.kobj, "thermal_cooling");
    sysfs_remove_link(&pr.cdev.device.kobj, "device");
    thermal_cooling_device_unregister(pr.cdev);
    pr.cdev = core::ptr::null_mut();
    }
    }
