//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/paravirt.c
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
// Copyright (C) 2013 Citrix Systems
//
// Author: Stefano Stabellini <stefano.stabellini@eu.citrix.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_time_stolen_time_region {
    pub kaddr: *mut pvclock_vcpu_stolen_time __rcu,
}

    static DEFINE_PER_CPU(struct pv_time_stolen_time_region, stolen_time_region);
    let mut steal_acc: static bool = true;
#[no_mangle]
unsafe extern "C" fn parse_no_stealacc(arg: *mut c_char) -> int __init {
    static int __init parse_no_stealacc(char *arg)
    {
    steal_acc = false;
    return 0;
    }
    early_param("no-steal-acc", parse_no_stealacc);
// return stolen time in ns by asking the hypervisor
#[no_mangle]
unsafe extern "C" fn para_steal_clock(cpu: c_int) -> u64 {
    static u64 para_steal_clock(int cpu)
    {
    struct pvclock_vcpu_stolen_time *kaddr = core::ptr::null_mut();
    struct pv_time_stolen_time_region *reg;
    let mut ret: u64 = 0;
    reg = per_cpu_ptr(&stolen_time_region, cpu);
//
// paravirt_steal_clock() may be called before the CPU
// online notification callback runs. Until the callback
// has run we just return zero.
//
    rcu_read_lock();
    kaddr = rcu_dereference(reg.kaddr);
    if (!kaddr) {
    rcu_read_unlock();
    return 0;
    }
    ret = le64_to_cpu(READ_ONCE(kaddr.stolen_time));
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stolen_time_cpu_down_prepare(cpu: c_uint) -> c_int {
    static int stolen_time_cpu_down_prepare(unsigned int cpu)
    {
    struct pvclock_vcpu_stolen_time *kaddr = core::ptr::null_mut();
    struct pv_time_stolen_time_region *reg;
    reg = this_cpu_ptr(&stolen_time_region);
    if (!reg.kaddr)
    return 0;
    kaddr = rcu_replace_pointer(reg.kaddr, core::ptr::null_mut(), true);
    synchronize_rcu();
    memunmap(kaddr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stolen_time_cpu_online(cpu: c_uint) -> c_int {
    static int stolen_time_cpu_online(unsigned int cpu)
    {
    struct pvclock_vcpu_stolen_time *kaddr = core::ptr::null_mut();
    struct pv_time_stolen_time_region *reg;
    struct arm_smccc_res res;
    reg = this_cpu_ptr(&stolen_time_region);
    arm_smccc_1_1_invoke(ARM_SMCCC_HV_PV_TIME_ST, &res);
    if (res.a0 == SMCCC_RET_NOT_SUPPORTED)
    return -EINVAL;
    kaddr = memremap(res.a0,
    sizeof(struct pvclock_vcpu_stolen_time),
    MEMREMAP_WB);
    rcu_assign_pointer(reg.kaddr, kaddr);
    if (!reg.kaddr) {
    pr_warn("Failed to map stolen time data structure\n");
    return -ENOMEM;
    }
    if (le32_to_cpu(kaddr.revision) != 0 ||
    le32_to_cpu(kaddr.attributes) != 0) {
    pr_warn_once("Unexpected revision or attributes in stolen time data\n");
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pv_time_init_stolen_time() -> int __init {
    static int __init pv_time_init_stolen_time(void)
    {
    int ret;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN,
    "hypervisor/arm/pvtime:online",
    stolen_time_cpu_online,
    stolen_time_cpu_down_prepare);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn has_pv_steal_clock() -> bool __init {
    static bool __init has_pv_steal_clock(void)
    {
    struct arm_smccc_res res;
    arm_smccc_1_1_invoke(ARM_SMCCC_ARCH_FEATURES_FUNC_ID,
    ARM_SMCCC_HV_PV_TIME_FEATURES, &res);
    if (res.a0 != SMCCC_RET_SUCCESS)
    return false;
    arm_smccc_1_1_invoke(ARM_SMCCC_HV_PV_TIME_FEATURES,
    ARM_SMCCC_HV_PV_TIME_ST, &res);
    return (res.a0 == SMCCC_RET_SUCCESS);
    }
#[no_mangle]
pub unsafe extern "C" fn pv_time_init() -> int __init {
    int __init pv_time_init(void)
    {
    int ret;
    if (!has_pv_steal_clock())
    return 0;
    ret = pv_time_init_stolen_time();
    if (ret)
    return ret;
    static_call_update(pv_steal_clock, para_steal_clock);
    static_key_slow_inc(&paravirt_steal_enabled);
    if (steal_acc)
    static_key_slow_inc(&paravirt_steal_rq_enabled);
    pr_info("using stolen time PV\n");
    return 0;
    }
