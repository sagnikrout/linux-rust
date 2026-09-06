//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/amd_freq_sensitivity.c
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
// amd_freq_sensitivity.c: AMD frequency sensitivity feedback powersave bias
// for the ondemand governor.
//
// Copyright (C) 2013 Advanced Micro Devices, Inc.
//
// Author: Jacob Shin <jacob.shin@amd.com>
//

pub const MSR_AMD64_FREQ_SENSITIVITY_ACTUAL: c_uint = 0xc0010080;
pub const MSR_AMD64_FREQ_SENSITIVITY_REFERENCE: c_uint = 0xc0010081;
pub const CLASS_CODE_SHIFT: c_int = 56;
pub const POWERSAVE_BIAS_MAX: c_int = 1000;
pub const POWERSAVE_BIAS_DEF: c_int = 400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_data_t {
    pub actual: u64,
    pub reference: u64,
    pub freq_prev: c_uint,
}

    static DEFINE_PER_CPU(struct cpu_data_t, cpu_data);
    static unsigned int amd_powersave_bias_target(struct cpufreq_policy *policy,
    unsigned int freq_next,
    unsigned int relation)
    {
    int sensitivity;
    long d_actual, d_reference;
    struct msr actual, reference;
    struct cpu_data_t *data = &per_cpu(cpu_data, policy.cpu);
    struct policy_dbs_info *policy_dbs = policy.governor_data;
    struct dbs_data *od_data = policy_dbs.dbs_data;
    struct od_dbs_tuners *od_tuners = od_data.tuners;
    if (!policy.freq_table)
    return freq_next;
    rdmsrq_on_cpu(policy.cpu, MSR_AMD64_FREQ_SENSITIVITY_ACTUAL, &actual.q);
    rdmsrq_on_cpu(policy.cpu, MSR_AMD64_FREQ_SENSITIVITY_REFERENCE, &reference.q);
    actual.h &= 0x00ffffff;
    reference.h &= 0x00ffffff;
// counter wrapped around, so stay on current frequency
    if (actual.q < data.actual || reference.q < data.reference) {
    freq_next = policy.cur;
    goto out;
    }
    d_actual = actual.q - data.actual;
    d_reference = reference.q - data.reference;
// divide by 0, so stay on current frequency as well
    if (d_reference == 0) {
    freq_next = policy.cur;
    goto out;
    }
    sensitivity = POWERSAVE_BIAS_MAX -
    (POWERSAVE_BIAS_MAX * (d_reference - d_actual) / d_reference);
    clamp(sensitivity, 0, POWERSAVE_BIAS_MAX);
// this workload is not CPU bound, so choose a lower freq
    if (sensitivity < od_tuners.powersave_bias) {
    if (data.freq_prev == policy.cur)
    freq_next = policy.cur;
    if (freq_next > policy.cur)
    freq_next = policy.cur;
#[no_mangle]
pub unsafe extern "C" fn if(policy->cur: freq_next <) -> else {
    else if (freq_next < policy.cur)
    freq_next = policy.min;
    else {
    unsigned int index;
    index = cpufreq_table_find_index_h(policy,
    policy.cur - 1,
    relation & CPUFREQ_RELATION_E);
    freq_next = policy.freq_table[index].frequency;
    }
    data.freq_prev = freq_next;
    } else
    data.freq_prev = 0;
    out:
    data.actual = actual.q;
    data.reference = reference.q;
    return freq_next;
    }
#[no_mangle]
unsafe extern "C" fn amd_freq_sensitivity_init() -> int __init {
    static int __init amd_freq_sensitivity_init(void)
    {
    u64 val;
    struct pci_dev *pcidev;
    unsigned int pci_vendor;
    if (boot_cpu_data.x86_vendor == X86_VENDOR_AMD)
    pci_vendor = PCI_VENDOR_ID_AMD;
#[no_mangle]
pub unsafe extern "C" fn if(X86_VENDOR_HYGON: boot_cpu_data.x86_vendor ==) -> else {
    else if (boot_cpu_data.x86_vendor == X86_VENDOR_HYGON)
    pci_vendor = PCI_VENDOR_ID_HYGON;
    else
    return -ENODEV;
    pcidev = pci_get_device(pci_vendor,
    PCI_DEVICE_ID_AMD_KERNCZ_SMBUS, core::ptr::null_mut());
    if (!pcidev) {
    if (!boot_cpu_has(X86_FEATURE_PROC_FEEDBACK))
    return -ENODEV;
    } else {
    pci_dev_put(pcidev);
    }
    if (rdmsrq_safe(MSR_AMD64_FREQ_SENSITIVITY_ACTUAL, &val))
    return -ENODEV;
    if (!(val >> CLASS_CODE_SHIFT))
    return -ENODEV;
    od_register_powersave_bias_handler(amd_powersave_bias_target,
    POWERSAVE_BIAS_DEF);
    return 0;
    }
    late_initcall(amd_freq_sensitivity_init);
#[no_mangle]
unsafe extern "C" fn amd_freq_sensitivity_exit() -> void __exit {
    static void __exit amd_freq_sensitivity_exit(void)
    {
    od_unregister_powersave_bias_handler();
    }
    module_exit(amd_freq_sensitivity_exit);
    static const struct x86_cpu_id __maybe_unused amd_freq_sensitivity_ids[] = {
    X86_MATCH_FEATURE(X86_FEATURE_PROC_FEEDBACK, core::ptr::null_mut()),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, amd_freq_sensitivity_ids);
    MODULE_AUTHOR("Jacob Shin <jacob.shin@amd.com>");
    MODULE_DESCRIPTION("AMD frequency sensitivity feedback powersave bias for "
    "the ondemand governor.");
    MODULE_LICENSE("GPL");
