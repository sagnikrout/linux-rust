//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/int340x_thermal/processor_thermal_soc_slider.c
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
// Processor Thermal Device Interface for Reading and Writing
// SoC Power Slider Values from User Space.
//
// Operation:
// The SOC_EFFICIENCY_SLIDER_0_0_0_MCHBAR register is accessed
// using the MMIO (Memory-Mapped I/O) interface with an MMIO offset of 0x5B38.
// Although this register is 64 bits wide, only bits 7:0 are used,
// and the other bits remain unchanged.
//
// Bit definitions
//
// Bits 2:0 (Slider value):
// The SoC optimizer slider value indicates the system wide energy performance
// hint. The slider has no specific units and ranges from 0 (highest
// performance) to 6 (highest energy efficiency). Value of 7 is reserved.
// Bits 3 : Reserved
// Bits 6:4 (Offset)
// Offset allows the SoC to automatically switch slider position in range
// [slider value (bits 2:0) + offset] to improve power efficiency based on
// internal SoC algorithms.
// Bit 7 (Enable):
// If this bit is set, the SoC Optimization sliders will be processed by the
// SoC firmware.
//
// Copyright (c) 2025, Intel Corporation.
//

pub const SOC_POWER_SLIDER_OFFSET: c_uint = 0x5B38;
    enum power_slider_preference {
    SOC_POWER_SLIDER_PERFORMANCE,
    SOC_POWER_SLIDER_BALANCE,
    SOC_POWER_SLIDER_POWERSAVE,
    };
pub const SOC_SLIDER_VALUE_MINIMUM: c_uint = 0x00;
pub const SOC_SLIDER_VALUE_BALANCE: c_uint = 0x03;
pub const SOC_SLIDER_VALUE_MAXIMUM: c_uint = 0x06;

pub const SLIDER_ENABLE_BIT: c_int = 7;
    static u8 slider_values[] = {
    [SOC_POWER_SLIDER_PERFORMANCE] = SOC_SLIDER_VALUE_MINIMUM,
    [SOC_POWER_SLIDER_BALANCE] = SOC_SLIDER_VALUE_BALANCE,
    [SOC_POWER_SLIDER_POWERSAVE] = SOC_SLIDER_VALUE_MAXIMUM,
    };
// Lock to protect module param updates
    static DEFINE_MUTEX(slider_param_lock);
    let mut slider_balanced_param: static int = SOC_SLIDER_VALUE_BALANCE;
#[no_mangle]
unsafe extern "C" fn slider_def_balance_set(arg: *const c_char, kp: *const kernel_param) -> c_int {
    static int slider_def_balance_set(const char *arg, const struct kernel_param *kp)
    {
    u8 slider_val;
    int ret;
    guard(mutex)(&slider_param_lock);
    ret = kstrtou8(arg, 16, &slider_val);
    if (!ret) {
    if (slider_val <= slider_values[SOC_POWER_SLIDER_PERFORMANCE] ||
    slider_val >= slider_values[SOC_POWER_SLIDER_POWERSAVE])
    return -EINVAL;
    slider_balanced_param = slider_val;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn slider_def_balance_get(buf: *mut c_char, kp: *const kernel_param) -> c_int {
    static int slider_def_balance_get(char *buf, const struct kernel_param *kp)
    {
    guard(mutex)(&slider_param_lock);
    return sysfs_emit(buf, "%02x\n", slider_values[SOC_POWER_SLIDER_BALANCE]);
    }
    static const struct kernel_param_ops slider_def_balance_ops = {
    .set = slider_def_balance_set,
    .get = slider_def_balance_get,
    };
    module_param_cb(slider_balance, &slider_def_balance_ops, core::ptr::null_mut(), 0644);
    MODULE_PARM_DESC(slider_balance, "Set slider default value for balance");
    static u8 slider_offset;
#[no_mangle]
unsafe extern "C" fn slider_def_offset_set(arg: *const c_char, kp: *const kernel_param) -> c_int {
    static int slider_def_offset_set(const char *arg, const struct kernel_param *kp)
    {
    u8 offset;
    int ret;
    guard(mutex)(&slider_param_lock);
    ret = kstrtou8(arg, 16, &offset);
    if (!ret) {
    if (offset > SOC_SLIDER_VALUE_MAXIMUM)
    return -EINVAL;
    slider_offset = offset;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn slider_def_offset_get(buf: *mut c_char, kp: *const kernel_param) -> c_int {
    static int slider_def_offset_get(char *buf, const struct kernel_param *kp)
    {
    guard(mutex)(&slider_param_lock);
    return sysfs_emit(buf, "%02x\n", slider_offset);
    }
    static const struct kernel_param_ops slider_offset_ops = {
    .set = slider_def_offset_set,
    .get = slider_def_offset_get,
    };
//
// To enhance power efficiency dynamically, the firmware can optionally
// auto-adjust the slider value based on the current workload. This
// adjustment is controlled by the "slider_offset" module parameter.
// This offset permits the firmware to increase the slider value
// up to and including "SoC slider + slider offset,".
//
    module_param_cb(slider_offset, &slider_offset_ops, core::ptr::null_mut(), 0644);
    MODULE_PARM_DESC(slider_offset, "Set slider offset");
// Convert from platform power profile option to SoC slider value
#[no_mangle]
unsafe extern "C" fn convert_profile_to_power_slider(profile: enum platform_profile_option) -> c_int {
    static int convert_profile_to_power_slider(enum platform_profile_option profile)
    {
    switch (profile) {
    case PLATFORM_PROFILE_LOW_POWER:
    return slider_values[SOC_POWER_SLIDER_POWERSAVE];
    case PLATFORM_PROFILE_BALANCED:
    return slider_values[SOC_POWER_SLIDER_BALANCE];
    case PLATFORM_PROFILE_PERFORMANCE:
    return slider_values[SOC_POWER_SLIDER_PERFORMANCE];
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
// Convert to platform power profile option from SoC slider values
#[no_mangle]
unsafe extern "C" fn convert_power_slider_to_profile(slider: u8) -> c_int {
    static int convert_power_slider_to_profile(u8 slider)
    {
    if (slider == slider_values[SOC_POWER_SLIDER_PERFORMANCE])
    return PLATFORM_PROFILE_PERFORMANCE;
    if (slider == slider_values[SOC_POWER_SLIDER_BALANCE])
    return PLATFORM_PROFILE_BALANCED;
    if (slider == slider_values[SOC_POWER_SLIDER_POWERSAVE])
    return PLATFORM_PROFILE_LOW_POWER;
    return -EOPNOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn read_soc_slider(proc_priv: *mut proc_thermal_device) -> u64 {
    static inline u64 read_soc_slider(struct proc_thermal_device *proc_priv)
    {
    return readq(proc_priv.mmio_base + SOC_POWER_SLIDER_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn write_soc_slider(proc_priv: *mut proc_thermal_device, val: u64) {
    static inline void write_soc_slider(struct proc_thermal_device *proc_priv, u64 val)
    {
    writeq(val, proc_priv.mmio_base + SOC_POWER_SLIDER_OFFSET);
    }

#[no_mangle]
unsafe extern "C" fn set_soc_power_profile(proc_priv: *mut proc_thermal_device, slider: c_int) {
    static void set_soc_power_profile(struct proc_thermal_device *proc_priv, int slider)
    {
    u8 offset;
    u64 val;
    val = read_soc_slider(proc_priv);
    val &= ~SLIDER_MASK;
    val |= FIELD_PREP(SLIDER_MASK, slider) | BIT(SLIDER_ENABLE_BIT);
    if (slider == SOC_SLIDER_VALUE_MINIMUM || slider == SOC_SLIDER_VALUE_MAXIMUM)
    offset = 0;
    else
    offset = slider_offset;
// Set the slider offset from module params
    val &= ~SLIDER_OFFSET_MASK;
    val |= FIELD_PREP(SLIDER_OFFSET_MASK, offset);
    write_soc_slider(proc_priv, val);
    }
// profile get/set callbacks are called with a profile lock, so no need for local locks
    static int power_slider_platform_profile_set(struct device *dev,
    enum platform_profile_option profile)
    {
    struct proc_thermal_device *proc_priv;
    int slider;
    proc_priv = dev_get_drvdata(dev);
    if (!proc_priv)
    return -EOPNOTSUPP;
    guard(mutex)(&slider_param_lock);
    slider_values[SOC_POWER_SLIDER_BALANCE] = slider_balanced_param;
    slider = convert_profile_to_power_slider(profile);
    if (slider < 0)
    return slider;
    set_soc_power_profile(proc_priv, slider);
    return 0;
    }
    static int power_slider_platform_profile_get(struct device *dev,
    enum platform_profile_option *profile)
    {
    struct proc_thermal_device *proc_priv;
    int slider, ret;
    u64 val;
    proc_priv = dev_get_drvdata(dev);
    if (!proc_priv)
    return -EOPNOTSUPP;
    val = read_soc_slider(proc_priv);
    slider = FIELD_GET(SLIDER_MASK, val);
    ret = convert_power_slider_to_profile(slider);
    if (ret < 0)
    return ret;
// profile = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn power_slider_platform_profile_probe(drvdata: *mut c_void, choices: *mut c_ulong) -> c_int {
    static int power_slider_platform_profile_probe(void *drvdata, unsigned long *choices)
    {
    set_bit(PLATFORM_PROFILE_LOW_POWER, choices);
    set_bit(PLATFORM_PROFILE_BALANCED, choices);
    set_bit(PLATFORM_PROFILE_PERFORMANCE, choices);
    return 0;
    }
    static const struct platform_profile_ops power_slider_platform_profile_ops = {
    .probe = power_slider_platform_profile_probe,
    .profile_get = power_slider_platform_profile_get,
    .profile_set = power_slider_platform_profile_set,
    };
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_soc_power_slider_add(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device) -> c_int {
    int proc_thermal_soc_power_slider_add(struct pci_dev *pdev, struct proc_thermal_device *proc_priv)
    {
    struct device *ppdev;
    set_soc_power_profile(proc_priv, slider_values[SOC_POWER_SLIDER_BALANCE]);
    ppdev = devm_platform_profile_register(&pdev.dev, "SoC Power Slider", proc_priv,
    &power_slider_platform_profile_ops);
    return PTR_ERR_OR_ZERO(ppdev);
    }
    EXPORT_SYMBOL_NS_GPL(proc_thermal_soc_power_slider_add, "INT340X_THERMAL");
    static u64 soc_slider_save;
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_soc_power_slider_suspend(proc_priv: *mut proc_thermal_device) {
    void proc_thermal_soc_power_slider_suspend(struct proc_thermal_device *proc_priv)
    {
    soc_slider_save = read_soc_slider(proc_priv);
    }
    EXPORT_SYMBOL_NS_GPL(proc_thermal_soc_power_slider_suspend, "INT340X_THERMAL");
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_soc_power_slider_resume(proc_priv: *mut proc_thermal_device) {
    void proc_thermal_soc_power_slider_resume(struct proc_thermal_device *proc_priv)
    {
    write_soc_slider(proc_priv, soc_slider_save);
    }
    EXPORT_SYMBOL_NS_GPL(proc_thermal_soc_power_slider_resume, "INT340X_THERMAL");
    MODULE_IMPORT_NS("INT340X_THERMAL");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Processor Thermal Power Slider Interface");
