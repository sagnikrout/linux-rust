//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/dptf/dptf_power.c
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
// dptf_power:  DPTF platform power driver
// Copyright (c) 2016, Intel Corporation.
//

//
// Presentation of attributes which are defined for INT3407 and INT3532.
// They are:
// PMAX : Maximum platform power
// PSRC : Platform power source
// ARTG : Adapter rating
// CTYP : Charger type
// PROP : Rest of worst case platform Power
// PBSS : Power Battery Steady State
// RBHF : High Frequency Impedance
// VBNL : Instantaneous No-Load Voltage
// CMPP : Current Discharge Capability
//

    static ssize_t name##_show(struct device *dev,\
    struct device_attribute *attr,\
    char *buf)\
    {\
    struct acpi_device *acpi_dev = dev_get_drvdata(dev);\
    unsigned long long val;\
    acpi_status status;\
    \
    status = acpi_evaluate_integer(acpi_dev.handle, #object,\
    core::ptr::null_mut(), &val);\
    if (ACPI_SUCCESS(status))\
    return sprintf(buf, "%d\n", (int)val);\
    else \
    return -EINVAL;\
    }
    DPTF_POWER_SHOW(max_platform_power_mw, PMAX)
    DPTF_POWER_SHOW(platform_power_source, PSRC)
    DPTF_POWER_SHOW(adapter_rating_mw, ARTG)
    DPTF_POWER_SHOW(battery_steady_power_mw, PBSS)
    DPTF_POWER_SHOW(charger_type, CTYP)
    DPTF_POWER_SHOW(rest_of_platform_power_mw, PROP)
    DPTF_POWER_SHOW(max_steady_state_power_mw, PBSS)
    DPTF_POWER_SHOW(high_freq_impedance_mohm, RBHF)
    DPTF_POWER_SHOW(no_load_voltage_mv, VBNL)
    DPTF_POWER_SHOW(current_discharge_capbility_ma, CMPP);
    static DEVICE_ATTR_RO(max_platform_power_mw);
    static DEVICE_ATTR_RO(platform_power_source);
    static DEVICE_ATTR_RO(adapter_rating_mw);
    static DEVICE_ATTR_RO(battery_steady_power_mw);
    static DEVICE_ATTR_RO(charger_type);
    static DEVICE_ATTR_RO(rest_of_platform_power_mw);
    static DEVICE_ATTR_RO(max_steady_state_power_mw);
    static DEVICE_ATTR_RO(high_freq_impedance_mohm);
    static DEVICE_ATTR_RO(no_load_voltage_mv);
    static DEVICE_ATTR_RO(current_discharge_capbility_ma);
    static ssize_t prochot_confirm_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_device *acpi_dev = dev_get_drvdata(dev);
    acpi_status status;
    int seq_no;
    if (kstrtouint(buf, 0, &seq_no) < 0)
    return -EINVAL;
    status = acpi_execute_simple_method(acpi_dev.handle, "PBOK", seq_no);
    if (ACPI_SUCCESS(status))
    return count;
    return -EINVAL;
    }
    static DEVICE_ATTR_WO(prochot_confirm);
    static struct attribute *dptf_power_attrs[] = {
    &dev_attr_max_platform_power_mw.attr,
    &dev_attr_platform_power_source.attr,
    &dev_attr_adapter_rating_mw.attr,
    &dev_attr_battery_steady_power_mw.attr,
    &dev_attr_charger_type.attr,
    &dev_attr_rest_of_platform_power_mw.attr,
    &dev_attr_prochot_confirm.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group dptf_power_attribute_group = {
    .attrs = dptf_power_attrs,
    .name = "dptf_power"
    };
    static struct attribute *dptf_battery_attrs[] = {
    &dev_attr_max_platform_power_mw.attr,
    &dev_attr_max_steady_state_power_mw.attr,
    &dev_attr_high_freq_impedance_mohm.attr,
    &dev_attr_no_load_voltage_mv.attr,
    &dev_attr_current_discharge_capbility_ma.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group dptf_battery_attribute_group = {
    .attrs = dptf_battery_attrs,
    .name = "dptf_battery"
    };
pub const MAX_POWER_CHANGED: c_uint = 0x80;
pub const POWER_STATE_CHANGED: c_uint = 0x81;
pub const STEADY_STATE_POWER_CHANGED: c_uint = 0x83;
pub const POWER_PROP_CHANGE_EVENT: c_uint = 0x84;
pub const IMPEDANCE_CHANGED: c_uint = 0x85;
pub const VOLTAGE_CURRENT_CHANGED: c_uint = 0x86;
#[no_mangle]
unsafe extern "C" fn dptf_participant_type(handle: acpi_handle) -> c_longlong {
    static long long dptf_participant_type(acpi_handle handle)
    {
    unsigned long long ptype;
    acpi_status status;
    status = acpi_evaluate_integer(handle, "PTYP", core::ptr::null_mut(), &ptype);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    return ptype;
    }
#[no_mangle]
unsafe extern "C" fn dptf_power_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void dptf_power_notify(acpi_handle handle, u32 event, void *data)
    {
    struct platform_device *pdev = data;
    char *attr;
    switch (event) {
    case POWER_STATE_CHANGED:
    attr = "platform_power_source";
    break;
    case POWER_PROP_CHANGE_EVENT:
    attr = "rest_of_platform_power_mw";
    break;
    case MAX_POWER_CHANGED:
    attr = "max_platform_power_mw";
    break;
    case STEADY_STATE_POWER_CHANGED:
    attr = "max_steady_state_power_mw";
    break;
    case IMPEDANCE_CHANGED:
    attr = "high_freq_impedance_mohm";
    break;
    case VOLTAGE_CURRENT_CHANGED:
    attr = "no_load_voltage_mv";
    break;
    default:
    dev_err(&pdev.dev, "Unsupported event [0x%x]\n", event);
    return;
    }
//
// Notify that an attribute is changed, so that user space can read
// again.
//
    if (dptf_participant_type(handle) == 0x0CULL)
    sysfs_notify(&pdev.dev.kobj, "dptf_battery", attr);
    else
    sysfs_notify(&pdev.dev.kobj, "dptf_power", attr);
    }
#[no_mangle]
unsafe extern "C" fn dptf_power_add(pdev: *mut platform_device) -> c_int {
    static int dptf_power_add(struct platform_device *pdev)
    {
    const struct attribute_group *attr_group;
    struct acpi_device *acpi_dev;
    unsigned long long ptype;
    int result;
    acpi_dev = ACPI_COMPANION(&(pdev.dev));
    if (!acpi_dev)
    return -ENODEV;
    ptype = dptf_participant_type(acpi_dev.handle);
    if (ptype == 0x11)
    attr_group = &dptf_power_attribute_group;
#[no_mangle]
pub unsafe extern "C" fn if(0x0C: ptype ==) -> else {
    else if (ptype == 0x0C)
    attr_group = &dptf_battery_attribute_group;
    else
    return -ENODEV;
    result = acpi_install_notify_handler(acpi_dev.handle,
    ACPI_DEVICE_NOTIFY,
    dptf_power_notify,
    (void *)pdev);
    if (result)
    return result;
    result = sysfs_create_group(&pdev.dev.kobj,
    attr_group);
    if (result) {
    acpi_remove_notify_handler(acpi_dev.handle,
    ACPI_DEVICE_NOTIFY,
    dptf_power_notify);
    return result;
    }
    platform_set_drvdata(pdev, acpi_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dptf_power_remove(pdev: *mut platform_device) {
    static void dptf_power_remove(struct platform_device *pdev)
    {
    struct acpi_device *acpi_dev = platform_get_drvdata(pdev);
    acpi_remove_notify_handler(acpi_dev.handle,
    ACPI_DEVICE_NOTIFY,
    dptf_power_notify);
    if (dptf_participant_type(acpi_dev.handle) == 0x0CULL)
    sysfs_remove_group(&pdev.dev.kobj, &dptf_battery_attribute_group);
    else
    sysfs_remove_group(&pdev.dev.kobj, &dptf_power_attribute_group);
    }
    static const struct acpi_device_id int3407_device_ids[] = {
    {"INT3407", 0},
    {"INT3532", 0},
    {"INTC1047", 0},
    {"INTC1050", 0},
    {"INTC1060", 0},
    {"INTC1061", 0},
    {"INTC1065", 0},
    {"INTC1066", 0},
    {"INTC106C", 0},
    {"INTC106D", 0},
    {"INTC10A4", 0},
    {"INTC10A5", 0},
    {"INTC10D8", 0},
    {"INTC10D9", 0},
    {"INTC1100", 0},
    {"INTC1101", 0},
    {"INTC10F7", 0},
    {"INTC10F8", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, int3407_device_ids);
    static struct platform_driver dptf_power_driver = {
    .probe = dptf_power_add,
    .remove = dptf_power_remove,
    .driver = {
    .name = "dptf_power",
    .acpi_match_table = int3407_device_ids,
    },
    };
    module_platform_driver(dptf_power_driver);
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("ACPI DPTF platform power driver");
