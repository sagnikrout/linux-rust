//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/intel_tcc_cooling.c
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
// cooling device driver that activates the processor throttling by
// programming the TCC Offset register.
// Copyright (c) 2021, Intel Corporation.
//

    static struct thermal_cooling_device *tcc_cdev;
    static int tcc_get_max_state(struct thermal_cooling_device *cdev, unsigned long
// state)
    {
// state = intel_tcc_get_offset_mask();
    return 0;
    }
    static int tcc_get_cur_state(struct thermal_cooling_device *cdev, unsigned long
// state)
    {
    let mut offset: c_int = intel_tcc_get_offset(-1);
    if (offset < 0)
    return offset;
// state = offset;
    return 0;
    }
    static int tcc_set_cur_state(struct thermal_cooling_device *cdev, unsigned long
    state)
    {
    return intel_tcc_set_offset(-1, (int)state);
    }
    static const struct thermal_cooling_device_ops tcc_cooling_ops = {
    .get_max_state = tcc_get_max_state,
    .get_cur_state = tcc_get_cur_state,
    .set_cur_state = tcc_set_cur_state,
    };
    static const struct x86_cpu_id tcc_ids[] __initconst = {
    X86_MATCH_VFM(INTEL_SKYLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_SKYLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_KABYLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_KABYLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ICELAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ICELAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_TIGERLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_TIGERLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_COMETLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ALDERLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ALDERLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ATOM_GRACEMONT, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_RAPTORLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_RAPTORLAKE_P, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_RAPTORLAKE_S, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ARROWLAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ARROWLAKE_U, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_ARROWLAKE_H, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_PANTHERLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_WILDCATLAKE_L, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_NOVALAKE, core::ptr::null_mut()),
    X86_MATCH_VFM(INTEL_NOVALAKE_L, core::ptr::null_mut()),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, tcc_ids);
#[no_mangle]
unsafe extern "C" fn tcc_cooling_init() -> int __init {
    static int __init tcc_cooling_init(void)
    {
    u64 val;
    const struct x86_cpu_id *id;
    int err;
    id = x86_match_cpu(tcc_ids);
    if (!id)
    return -ENODEV;
    err = rdmsrq_safe(MSR_PLATFORM_INFO, &val);
    if (err)
    return err;
    if (!(val & TCC_PROGRAMMABLE))
    return -ENODEV;
    err = rdmsrq_safe(MSR_IA32_TEMPERATURE_TARGET, &val);
    if (err)
    return err;
    if (val & TCC_LOCKED) {
    pr_info("TCC Offset locked\n");
    return -ENODEV;
    }
    pr_info("Programmable TCC Offset detected\n");
    tcc_cdev =
    thermal_cooling_device_register("TCC Offset", core::ptr::null_mut(),
    &tcc_cooling_ops);
    if (IS_ERR(tcc_cdev))
    return PTR_ERR(tcc_cdev);
    return 0;
    }
    module_init(tcc_cooling_init)
#[no_mangle]
unsafe extern "C" fn tcc_cooling_exit() -> void __exit {
    static void __exit tcc_cooling_exit(void)
    {
    thermal_cooling_device_unregister(tcc_cdev);
    }
    module_exit(tcc_cooling_exit)
    MODULE_IMPORT_NS("INTEL_TCC");
    MODULE_DESCRIPTION("TCC offset cooling device Driver");
    MODULE_AUTHOR("Zhang Rui <rui.zhang@intel.com>");
    MODULE_LICENSE("GPL v2");
