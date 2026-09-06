//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/intel_soc_dts_thermal.c
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
// intel_soc_dts_thermal.c
// Copyright (c) 2014, Intel Corporation.
//

pub const CRITICAL_OFFSET_FROM_TJ_MAX: c_int = 5000;
    let mut crit_offset: static int = CRITICAL_OFFSET_FROM_TJ_MAX;
    module_param(crit_offset, int, 0644);
    MODULE_PARM_DESC(crit_offset,
    "Critical Temperature offset from tj max in millidegree Celsius.");
// IRQ 86 is a fixed APIC interrupt for BYT DTS Aux threshold notifications
pub const BYT_SOC_DTS_APIC_IRQ: c_int = 86;
    static int soc_dts_thres_gsi;
    static int soc_dts_thres_irq;
    static struct intel_soc_dts_sensors *soc_dts;
#[no_mangle]
unsafe extern "C" fn soc_irq_thread_fn(irq: c_int, dev_data: *mut c_void) -> irqreturn_t {
    static irqreturn_t soc_irq_thread_fn(int irq, void *dev_data)
    {
    pr_debug("proc_thermal_interrupt\n");
    intel_soc_dts_iosf_interrupt_handler(soc_dts);
    return IRQ_HANDLED;
    }
    static const struct x86_cpu_id soc_thermal_ids[] = {
    X86_MATCH_VFM(INTEL_ATOM_SILVERMONT, BYT_SOC_DTS_APIC_IRQ),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, soc_thermal_ids);
#[no_mangle]
unsafe extern "C" fn intel_soc_thermal_init() -> int __init {
    static int __init intel_soc_thermal_init(void)
    {
    let mut err: c_int = 0;
    const struct x86_cpu_id *match_cpu;
    match_cpu = x86_match_cpu(soc_thermal_ids);
    if (!match_cpu)
    return -ENODEV;
// Create a zone with 2 trips with marked as read only
    soc_dts = intel_soc_dts_iosf_init(INTEL_SOC_DTS_INTERRUPT_APIC, true,
    crit_offset);
    if (IS_ERR(soc_dts)) {
    err = PTR_ERR(soc_dts);
    return err;
    }
    soc_dts_thres_gsi = (int)match_cpu.driver_data;
    if (soc_dts_thres_gsi) {
//
// Note the flags here MUST match the firmware defaults, rather
// then the request_irq flags, otherwise we get an EBUSY error.
//
    soc_dts_thres_irq = acpi_register_gsi(core::ptr::null_mut(), soc_dts_thres_gsi,
    ACPI_LEVEL_SENSITIVE,
    ACPI_ACTIVE_LOW);
    if (soc_dts_thres_irq < 0) {
    pr_warn("intel_soc_dts: Could not get IRQ for GSI %d, err %d\n",
    soc_dts_thres_gsi, soc_dts_thres_irq);
    soc_dts_thres_irq = 0;
    }
    }
    if (soc_dts_thres_irq) {
    err = request_threaded_irq(soc_dts_thres_irq, core::ptr::null_mut(),
    soc_irq_thread_fn,
    IRQF_TRIGGER_RISING | IRQF_ONESHOT,
    "soc_dts", soc_dts);
    if (err) {
//
// Do not just error out because the user space thermal
// daemon such as DPTF may use polling instead of being
// interrupt driven.
//
    pr_warn("request_threaded_irq ret %d\n", err);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_soc_thermal_exit() -> void __exit {
    static void __exit intel_soc_thermal_exit(void)
    {
    if (soc_dts_thres_irq) {
    free_irq(soc_dts_thres_irq, soc_dts);
    acpi_unregister_gsi(soc_dts_thres_gsi);
    }
    intel_soc_dts_iosf_exit(soc_dts);
    }
    module_init(intel_soc_thermal_init)
    module_exit(intel_soc_thermal_exit)
    MODULE_DESCRIPTION("Intel SoC DTS Thermal Driver");
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
