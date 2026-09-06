//! Automatically rewritten from C to Rust
//! Source: drivers/cpuidle/cpuidle-haltpoll.c
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
//
// cpuidle driver for haltpoll governor.
//
// Copyright 2019 Red Hat, Inc. and/or its affiliates.
//
// This work is licensed under the terms of the GNU GPL, version 2.  See
// the COPYING file in the top-level directory.
//
// Authors: Marcelo Tosatti <mtosatti@redhat.com>
//

    static bool force __read_mostly;
    module_param(force, bool, 0444);
    MODULE_PARM_DESC(force, "Load unconditionally");
    static struct cpuidle_device __percpu *haltpoll_cpuidle_devices;
    static enum cpuhp_state haltpoll_hp_state;
    static __cpuidle int default_enter_idle(struct cpuidle_device *dev,
    struct cpuidle_driver *drv, int index)
    {
    if (current_clr_polling_and_test())
    return index;
    arch_cpu_idle();
    return index;
    }
    static struct cpuidle_driver haltpoll_driver = {
    .name = "haltpoll",
    .governor = "haltpoll",
    .states = {
    { /* entry 0 is for polling */ },
    {
    .enter			= default_enter_idle,
    .exit_latency		= 1,
    .target_residency	= 1,
    .power_usage		= -1,
    .name			= "haltpoll idle",
    .desc			= "default architecture idle",
    },
    },
    .safe_state_index = 0,
    .state_count = 2,
    };
#[no_mangle]
unsafe extern "C" fn haltpoll_cpu_online(cpu: c_uint) -> c_int {
    static int haltpoll_cpu_online(unsigned int cpu)
    {
    struct cpuidle_device *dev;
    dev = per_cpu_ptr(haltpoll_cpuidle_devices, cpu);
    if (!dev.registered) {
    dev.cpu = cpu;
    if (cpuidle_register_device(dev)) {
    pr_notice("cpuidle_register_device %d failed!\n", cpu);
    return -EIO;
    }
    arch_haltpoll_enable(cpu);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn haltpoll_cpu_offline(cpu: c_uint) -> c_int {
    static int haltpoll_cpu_offline(unsigned int cpu)
    {
    struct cpuidle_device *dev;
    dev = per_cpu_ptr(haltpoll_cpuidle_devices, cpu);
    if (dev.registered) {
    arch_haltpoll_disable(cpu);
    cpuidle_unregister_device(dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn haltpoll_uninit() {
    static void haltpoll_uninit(void)
    {
    if (haltpoll_hp_state)
    cpuhp_remove_state(haltpoll_hp_state);
    cpuidle_unregister_driver(&haltpoll_driver);
    free_percpu(haltpoll_cpuidle_devices);
    haltpoll_cpuidle_devices = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn haltpoll_want() -> bool {
    static bool haltpoll_want(void)
    {
    return kvm_para_has_hint(KVM_HINTS_REALTIME) || force;
    }
#[no_mangle]
unsafe extern "C" fn haltpoll_init() -> int __init {
    static int __init haltpoll_init(void)
    {
    int ret;
    struct cpuidle_driver *drv = &haltpoll_driver;
// Do not load haltpoll if idle= is passed
    if (boot_option_idle_override != IDLE_NO_OVERRIDE)
    return -ENODEV;
    if (!kvm_para_available() || !haltpoll_want())
    return -ENODEV;
    cpuidle_poll_state_init(drv);
    ret = cpuidle_register_driver(drv);
    if (ret < 0)
    return ret;
    haltpoll_cpuidle_devices = alloc_percpu(struct cpuidle_device);
    if (haltpoll_cpuidle_devices == core::ptr::null_mut()) {
    cpuidle_unregister_driver(drv);
    return -ENOMEM;
    }
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "cpuidle/haltpoll:online",
    haltpoll_cpu_online, haltpoll_cpu_offline);
    if (ret < 0) {
    haltpoll_uninit();
    } else {
    haltpoll_hp_state = ret;
    ret = 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn haltpoll_exit() -> void __exit {
    static void __exit haltpoll_exit(void)
    {
    haltpoll_uninit();
    }
    module_init(haltpoll_init);
    module_exit(haltpoll_exit);
    MODULE_DESCRIPTION("cpuidle driver for haltpoll governor");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Marcelo Tosatti <mtosatti@redhat.com>");
