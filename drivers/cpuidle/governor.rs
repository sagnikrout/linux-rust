//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/governor.c
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


//
// governor.c - governor support
//
// (C) 2006-2007 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>
// Shaohua Li <shaohua.li@intel.com>
// Adam Belay <abelay@novell.com>
//
// This code is licenced under the GPL.
//

    char param_governor[CPUIDLE_NAME_LEN];
    LIST_HEAD(cpuidle_governors);
    struct cpuidle_governor *cpuidle_curr_governor;
    struct cpuidle_governor *cpuidle_prev_governor;
//
// cpuidle_find_governor - finds a governor of the specified name
// @str: the name
//
// Must be called with cpuidle_lock acquired.
//
    struct cpuidle_governor *cpuidle_find_governor(const char *str)
    {
    struct cpuidle_governor *gov;
    list_for_each_entry(gov, &cpuidle_governors, governor_list)
    if (!strncasecmp(str, gov.name, CPUIDLE_NAME_LEN))
    return gov;
    return core::ptr::null_mut();
    }
//
// cpuidle_switch_governor - changes the governor
// @gov: the new target governor
// Must be called with cpuidle_lock acquired.
//
#[no_mangle]
pub unsafe extern "C" fn cpuidle_switch_governor(gov: *mut cpuidle_governor) -> c_int {
    int cpuidle_switch_governor(struct cpuidle_governor *gov)
    {
    struct cpuidle_device *dev;
    if (!gov)
    return -EINVAL;
    if (gov == cpuidle_curr_governor)
    return 0;
    cpuidle_uninstall_idle_handler();
    if (cpuidle_curr_governor) {
    list_for_each_entry(dev, &cpuidle_detected_devices, device_list)
    cpuidle_disable_device(dev);
    }
    cpuidle_curr_governor = gov;
    list_for_each_entry(dev, &cpuidle_detected_devices, device_list)
    cpuidle_enable_device(dev);
    cpuidle_install_idle_handler();
    pr_info("cpuidle: using governor %s\n", gov.name);
    return 0;
    }
//
// cpuidle_register_governor - registers a governor
// @gov: the governor
//
#[no_mangle]
pub unsafe extern "C" fn cpuidle_register_governor(gov: *mut cpuidle_governor) -> c_int {
    int cpuidle_register_governor(struct cpuidle_governor *gov)
    {
    let mut ret: c_int = -EEXIST;
    if (!gov || !gov.select)
    return -EINVAL;
    if (cpuidle_disabled())
    return -ENODEV;
    mutex_lock(&cpuidle_lock);
    if (cpuidle_find_governor(gov.name) == core::ptr::null_mut()) {
    ret = 0;
    list_add_tail(&gov.governor_list, &cpuidle_governors);
    if (!cpuidle_curr_governor ||
    !strncasecmp(param_governor, gov.name, CPUIDLE_NAME_LEN) ||
    (cpuidle_curr_governor.rating < gov.rating &&
    strncasecmp(param_governor, cpuidle_curr_governor.name,
    CPUIDLE_NAME_LEN)))
    cpuidle_switch_governor(gov);
    }
    mutex_unlock(&cpuidle_lock);
    return ret;
    }
//
// cpuidle_governor_latency_req - Compute a latency constraint for CPU
// @cpu: Target CPU
//
#[no_mangle]
pub unsafe extern "C" fn cpuidle_governor_latency_req(cpu: c_uint) -> i64 {
    s64 cpuidle_governor_latency_req(unsigned int cpu)
    {
    struct device *device = get_cpu_device(cpu);
    let mut device_req: c_int = dev_pm_qos_raw_resume_latency(device);
    let mut global_req: c_int = cpu_latency_qos_limit();
    let mut global_wake_req: c_int = cpu_wakeup_latency_qos_limit();
    if (global_req > global_wake_req)
    global_req = global_wake_req;
    if (device_req > global_req)
    device_req = global_req;
    return (s64)device_req * NSEC_PER_USEC;
    }
