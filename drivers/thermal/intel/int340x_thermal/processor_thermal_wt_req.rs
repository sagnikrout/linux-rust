//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/int340x_thermal/processor_thermal_wt_req.c
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
// processor thermal device for Workload type hints
// update from user space
//
// Copyright (c) 2020-2023, Intel Corporation.
//

// List of workload types
    static const char * const workload_types[] = {
    "none",
    "idle",
    "semi_active",
    "bursty",
    "sustained",
    "battery_life",
    core::ptr::null_mut()
    };
    static ssize_t workload_available_types_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    while (workload_types[i] != core::ptr::null_mut())
    ret += sysfs_emit_at(buf, ret, "%s ", workload_types[i++]);
    ret += sysfs_emit_at(buf, ret, "\n");
    return ret;
    }
    static DEVICE_ATTR_RO(workload_available_types);
    static ssize_t workload_type_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    char str_preference[15];
    let mut data: u32 = 0;
    ssize_t ret;
    ret = sscanf(buf, "%14s", str_preference);
    if (ret != 1)
    return -EINVAL;
    ret = match_string(workload_types, -1, str_preference);
    if (ret < 0)
    return ret;
    ret &= 0xff;
    if (ret)
    data = BIT(MBOX_DATA_BIT_VALID) | BIT(MBOX_DATA_BIT_AC_DC);
    data |= ret;
    ret = processor_thermal_send_mbox_write_cmd(pdev, MBOX_CMD_WORKLOAD_TYPE_WRITE, data);
    if (ret)
    return false;
    return count;
    }
    static ssize_t workload_type_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    u64 cmd_resp;
    int ret;
    ret = processor_thermal_send_mbox_read_cmd(pdev, MBOX_CMD_WORKLOAD_TYPE_READ, &cmd_resp);
    if (ret)
    return false;
    cmd_resp &= 0xff;
    if (cmd_resp > ARRAY_SIZE(workload_types) - 1)
    return -EINVAL;
    return sysfs_emit(buf, "%s\n", workload_types[cmd_resp]);
    }
    static DEVICE_ATTR_RW(workload_type);
    static struct attribute *workload_req_attrs[] = {
    &dev_attr_workload_available_types.attr,
    &dev_attr_workload_type.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group workload_req_attribute_group = {
    .attrs = workload_req_attrs,
    .name = "workload_request"
    };
    static bool workload_req_created;
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_wt_req_add(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device) -> c_int {
    int proc_thermal_wt_req_add(struct pci_dev *pdev, struct proc_thermal_device *proc_priv)
    {
    u64 cmd_resp;
    int ret;
// Check if there is a mailbox support, if fails return success
    ret = processor_thermal_send_mbox_read_cmd(pdev, MBOX_CMD_WORKLOAD_TYPE_READ, &cmd_resp);
    if (ret)
    return 0;
    ret = sysfs_create_group(&pdev.dev.kobj, &workload_req_attribute_group);
    if (ret)
    return ret;
    workload_req_created = true;
    return 0;
    }
    EXPORT_SYMBOL_GPL(proc_thermal_wt_req_add);
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_wt_req_remove(pdev: *mut pci_dev) {
    void proc_thermal_wt_req_remove(struct pci_dev *pdev)
    {
    if (workload_req_created)
    sysfs_remove_group(&pdev.dev.kobj, &workload_req_attribute_group);
    workload_req_created = false;
    }
    EXPORT_SYMBOL_GPL(proc_thermal_wt_req_remove);
    MODULE_IMPORT_NS("INT340X_THERMAL");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Processor Thermal Work Load type request Interface");
