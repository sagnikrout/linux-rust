//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/resctrl/ctrlmondata.c
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
// Resource Director Technology(RDT)
// - Cache Allocation code.
//
// Copyright (C) 2016 Intel Corporation
//
// Authors:
// Fenghua Yu <fenghua.yu@intel.com>
// Tony Luck <tony.luck@intel.com>
//
// More information about RDT be found in the Intel (R) x86 Architecture
// Software Developer Manual June 2016, volume 3, section 17.17.
//

    int resctrl_arch_update_one(struct rdt_resource *r, struct rdt_ctrl_domain *d,
    u32 closid, enum resctrl_conf_type t, u32 cfg_val)
    {
    struct rdt_hw_ctrl_domain *hw_dom = resctrl_to_arch_ctrl_dom(d);
    struct rdt_hw_resource *hw_res = resctrl_to_arch_res(r);
    let mut idx: u32 = resctrl_get_config_index(closid, t);
    struct msr_param msr_param;
    if (!cpumask_test_cpu(smp_processor_id(), &d.hdr.cpu_mask))
    return -EINVAL;
    hw_dom.ctrl_val[idx] = cfg_val;
    msr_param.res = r;
    msr_param.dom = d;
    msr_param.low = idx;
    msr_param.high = idx + 1;
    hw_res.msr_update(&msr_param);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn resctrl_arch_update_domains(r: *mut rdt_resource, closid: u32) -> c_int {
    int resctrl_arch_update_domains(struct rdt_resource *r, u32 closid)
    {
    struct resctrl_staged_config *cfg;
    struct rdt_hw_ctrl_domain *hw_dom;
    struct msr_param msr_param;
    struct rdt_ctrl_domain *d;
    enum resctrl_conf_type t;
    u32 idx;
// Walking r->domains, ensure it can't race with cpuhp
    lockdep_assert_cpus_held();
    list_for_each_entry_rcu(d, &r.ctrl_domains, hdr.list, lockdep_is_cpus_held()) {
    hw_dom = resctrl_to_arch_ctrl_dom(d);
    msr_param.res = core::ptr::null_mut();
    for (t = 0; t < CDP_NUM_TYPES; t++) {
    cfg = &hw_dom.d_resctrl.staged_config[t];
    if (!cfg.have_new_ctrl)
    continue;
    idx = resctrl_get_config_index(closid, t);
    if (cfg.new_ctrl == hw_dom.ctrl_val[idx])
    continue;
    hw_dom.ctrl_val[idx] = cfg.new_ctrl;
    if (!msr_param.res) {
    msr_param.low = idx;
    msr_param.high = msr_param.low + 1;
    msr_param.res = r;
    msr_param.dom = d;
    } else {
    msr_param.low = min(msr_param.low, idx);
    msr_param.high = max(msr_param.high, idx + 1);
    }
    }
    if (msr_param.res)
    smp_call_function_any(&d.hdr.cpu_mask, rdt_ctrl_update, &msr_param, 1);
    }
    return 0;
    }
    u32 resctrl_arch_get_config(struct rdt_resource *r, struct rdt_ctrl_domain *d,
    u32 closid, enum resctrl_conf_type type)
    {
    struct rdt_hw_ctrl_domain *hw_dom = resctrl_to_arch_ctrl_dom(d);
    let mut idx: u32 = resctrl_get_config_index(closid, type);
    return hw_dom.ctrl_val[idx];
    }
#[no_mangle]
pub unsafe extern "C" fn resctrl_arch_get_io_alloc_enabled(r: *mut rdt_resource) -> bool {
    bool resctrl_arch_get_io_alloc_enabled(struct rdt_resource *r)
    {
    return resctrl_to_arch_res(r).sdciae_enabled;
    }
#[no_mangle]
unsafe extern "C" fn resctrl_sdciae_set_one_amd(arg: *mut c_void) {
    static void resctrl_sdciae_set_one_amd(void *arg)
    {
    bool *enable = arg;
    if (*enable)
    msr_set_bit(MSR_IA32_L3_QOS_EXT_CFG, SDCIAE_ENABLE_BIT);
    else
    msr_clear_bit(MSR_IA32_L3_QOS_EXT_CFG, SDCIAE_ENABLE_BIT);
    }
#[no_mangle]
unsafe extern "C" fn _resctrl_sdciae_enable(r: *mut rdt_resource, enable: bool) {
    static void _resctrl_sdciae_enable(struct rdt_resource *r, bool enable)
    {
    struct rdt_ctrl_domain *d;
// Walking r->ctrl_domains, ensure it can't race with cpuhp
    lockdep_assert_cpus_held();
// Update MSR_IA32_L3_QOS_EXT_CFG MSR on all the CPUs in all domains
    list_for_each_entry_rcu(d, &r.ctrl_domains, hdr.list, lockdep_is_cpus_held())
    on_each_cpu_mask(&d.hdr.cpu_mask, resctrl_sdciae_set_one_amd, &enable, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn resctrl_arch_io_alloc_enable(r: *mut rdt_resource, enable: bool) -> c_int {
    int resctrl_arch_io_alloc_enable(struct rdt_resource *r, bool enable)
    {
    struct rdt_hw_resource *hw_res = resctrl_to_arch_res(r);
    if (hw_res.r_resctrl.cache.io_alloc_capable &&
    hw_res.sdciae_enabled != enable) {
    _resctrl_sdciae_enable(r, enable);
    hw_res.sdciae_enabled = enable;
    }
    return 0;
    }
