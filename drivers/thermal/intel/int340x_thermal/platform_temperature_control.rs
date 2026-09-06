//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/int340x_thermal/platform_temperature_control.c
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
// processor thermal device platform temperature controls
// Copyright (c) 2025, Intel Corporation.
//
// Platform temperature controls hardware interface
//
// The hardware control interface is via MMIO offsets in the processor
// thermal device MMIO space. There are three instances of MMIO registers.
// All registers are 64 bit wide with RW access.
//
// Name: PLATFORM_TEMPERATURE_CONTROL
// Offsets: 0x5B20, 0x5B28, 0x5B30
//
// Bits    Description
// 7:0     TARGET_TEMP : Target temperature limit to which the control
// mechanism is regulating. Units: 0.5C.
// 8:8     ENABLE: Read current enable status of the feature or enable
// feature.
// 11:9    GAIN: Sets the aggressiveness of control loop from 0 to 7
// 7 graceful, favors performance at the expense of temperature
// overshoots
// 0 aggressive, favors tight regulation over performance
// 12:12   TEMPERATURE_OVERRIDE_EN
// When set, hardware will use TEMPERATURE_OVERRIDE values instead
// of reading from corresponding sensor.
// 15:13   RESERVED
// 23:16   MIN_PERFORMANCE_LEVEL: Minimum Performance level below which the
// there will be no throttling. 0 - all levels of throttling allowed
// including survivability actions. 255 - no throttling allowed.
// 31:24   TEMPERATURE_OVERRIDE: Allows SW to override the input temperature.
// hardware will use this value instead of the sensor temperature.
// Units: 0.5C.
// 63:32   RESERVED
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmio_reg {
    pub bits: c_int,
    pub mask: u16,
    pub shift: u16,
    pub units: u16,
}

pub const MAX_ATTR_GROUP_NAME_LEN: c_int = 32;
pub const PTC_MAX_ATTRS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptc_data {
    pub offset: u32,
    pub pdev: *mut pci_dev,
    pub ptc_attr_group: attribute_group,
    pub ptc_attrs: [*mut attribute; PTC_MAX_ATTRS],
    pub temperature_target_attr: device_attribute,
    pub enable_attr: device_attribute,
    pub thermal_tolerance_attr: device_attribute,
    pub group_name: [c_char; MAX_ATTR_GROUP_NAME_LEN],
}

    static const struct mmio_reg ptc_mmio_regs[] = {
    { 8, 0xFF, 0, 500}, /* temperature_target, units 0.5C*/
    { 1, 0x01, 8, 0}, /* enable */
    { 3, 0x7, 9, 0}, /* gain */
    { 8, 0xFF, 16, 0}, /* min_performance_level */
    { 1, 0x1, 12, 0}, /* temperature_override_enable */
    { 8, 0xFF, 24, 500}, /* temperature_override, units 0.5C */
    };
pub const PTC_MAX_INSTANCES: c_int = 3;
// Unique offset for each PTC instance
    static u32 ptc_offsets[PTC_MAX_INSTANCES] = {0x5B20, 0x5B28, 0x5B30};
// These will represent sysfs attribute names
    static const char * const ptc_strings[] = {
    "temperature_target",
    "enable",
    "thermal_tolerance",
    core::ptr::null_mut()
    };
// Lock to protect concurrent read/write and read-modify-write
    static DEFINE_MUTEX(ptc_lock);
    static ssize_t ptc_mmio_show(struct ptc_data *data, struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    struct proc_thermal_device *proc_priv;
    const struct mmio_reg *mmio_regs;
    int ret, units;
    u64 reg_val;
    proc_priv = pci_get_drvdata(pdev);
    mmio_regs = ptc_mmio_regs;
    ret = match_string(ptc_strings, -1, attr.attr.name);
    if (ret < 0)
    return ret;
    units = mmio_regs[ret].units;
    guard(mutex)(&ptc_lock);
    reg_val = readq((void __iomem *) (proc_priv.mmio_base + data.offset));
    ret = (reg_val >> mmio_regs[ret].shift) & mmio_regs[ret].mask;
    if (units)
    ret *= units;
    return sysfs_emit(buf, "%d\n", ret);
    }
// Macro flag: #define PTC_SHOW(suffix)\
    static ssize_t suffix##_show(struct device *dev,\
    struct device_attribute *attr,\
    char *buf)\
    {\
    struct ptc_data *data = container_of(attr, struct ptc_data, suffix##_attr);\
    return ptc_mmio_show(data, dev, attr, buf);\
    }
#[no_mangle]
unsafe extern "C" fn ptc_mmio_write(pdev: *mut pci_dev, offset: u32, index: c_int, value: u32) {
    static void ptc_mmio_write(struct pci_dev *pdev, u32 offset, int index, u32 value)
    {
    struct proc_thermal_device *proc_priv;
    u64 mask, reg_val;
    proc_priv = pci_get_drvdata(pdev);
    mask = GENMASK_ULL(ptc_mmio_regs[index].shift + ptc_mmio_regs[index].bits - 1,
    ptc_mmio_regs[index].shift);
    guard(mutex)(&ptc_lock);
    reg_val = readq((void __iomem *) (proc_priv.mmio_base + offset));
    reg_val &= ~mask;
    reg_val |= ((u64)value << ptc_mmio_regs[index].shift);
    writeq(reg_val, (void __iomem *) (proc_priv.mmio_base + offset));
    }
    static int ptc_store(struct ptc_data *data, struct device *dev, struct device_attribute *attr,
    const char *buf,  size_t count)
    {
    struct pci_dev *pdev = to_pci_dev(dev);
    unsigned int input;
    int ret;
    ret = kstrtouint(buf, 10, &input);
    if (ret)
    return ret;
    ret = match_string(ptc_strings, -1, attr.attr.name);
    if (ret < 0)
    return ret;
    if (ptc_mmio_regs[ret].units)
    input /= ptc_mmio_regs[ret].units;
    if (input > ptc_mmio_regs[ret].mask)
    return -EINVAL;
    ptc_mmio_write(pdev, data.offset, ret, input);
    return count;
    }
// Macro flag: #define PTC_STORE(suffix)\
    static ssize_t suffix##_store(struct device *dev,\
    struct device_attribute *attr,\
    const char *buf, size_t count)\
    {\
    struct ptc_data *data = container_of(attr, struct ptc_data, suffix##_attr);\
    return ptc_store(data, dev, attr, buf, count);\
    }
    PTC_SHOW(temperature_target);
    PTC_STORE(temperature_target);
    PTC_SHOW(enable);
    PTC_STORE(enable);
    PTC_SHOW(thermal_tolerance);
    PTC_STORE(thermal_tolerance);
// Macro flag: #define ptc_init_attribute(_name)\
    do {\
    sysfs_attr_init(&data._name##_attr.attr);\
    data._name##_attr.show = _name##_show;\
    data._name##_attr.store = _name##_store;\
    data._name##_attr.attr.name = #_name;\
    data._name##_attr.attr.mode = 0644;\
    } while (0)
#[no_mangle]
unsafe extern "C" fn ptc_create_groups(pdev: *mut pci_dev, instance: c_int, data: *mut ptc_data) -> c_int {
    static int ptc_create_groups(struct pci_dev *pdev, int instance, struct ptc_data *data)
    {
    int ret, index = 0;
    ptc_init_attribute(temperature_target);
    ptc_init_attribute(enable);
    ptc_init_attribute(thermal_tolerance);
    data.ptc_attrs[index++] = &data.temperature_target_attr.attr;
    data.ptc_attrs[index++] = &data.enable_attr.attr;
    data.ptc_attrs[index++] = &data.thermal_tolerance_attr.attr;
    data.ptc_attrs[index] = core::ptr::null_mut();
    snprintf(data.group_name, MAX_ATTR_GROUP_NAME_LEN,
    "ptc_%d_control", instance);
    data.ptc_attr_group.name = data.group_name;
    data.ptc_attr_group.attrs = data.ptc_attrs;
    ret = sysfs_create_group(&pdev.dev.kobj, &data.ptc_attr_group);
    return ret;
    }
    static struct ptc_data ptc_instance[PTC_MAX_INSTANCES];
    static struct dentry *ptc_debugfs;
pub const PTC_TEMP_OVERRIDE_ENABLE_INDEX: c_int = 4;
pub const PTC_TEMP_OVERRIDE_INDEX: c_int = 5;
    static ssize_t ptc_temperature_write(struct file *file, const char __user *data,
    size_t count, loff_t *ppos)
    {
    struct ptc_data *ptc_instance = file.private_data;
    struct pci_dev *pdev = ptc_instance.pdev;
    u32 value;
    int ret;
    ret = kstrtou32_from_user(data, count, 0, &value);
    if (unlikely(ret))
    return ret;
    if (ptc_mmio_regs[PTC_TEMP_OVERRIDE_INDEX].units)
    value /= ptc_mmio_regs[PTC_TEMP_OVERRIDE_INDEX].units;
    if (value > ptc_mmio_regs[PTC_TEMP_OVERRIDE_INDEX].mask)
    return -EINVAL;
    if (!value) {
    ptc_mmio_write(pdev, ptc_instance.offset, PTC_TEMP_OVERRIDE_ENABLE_INDEX, 0);
    } else {
    ptc_mmio_write(pdev, ptc_instance.offset, PTC_TEMP_OVERRIDE_INDEX, value);
    ptc_mmio_write(pdev, ptc_instance.offset, PTC_TEMP_OVERRIDE_ENABLE_INDEX, 1);
    }
    return count;
    }
    static const struct file_operations ptc_fops = {
    .open = simple_open,
    .write = ptc_temperature_write,
    .llseek = generic_file_llseek,
    };
#[no_mangle]
unsafe extern "C" fn ptc_create_debugfs() {
    static void ptc_create_debugfs(void)
    {
    ptc_debugfs = debugfs_create_dir("platform_temperature_control", core::ptr::null_mut());
    debugfs_create_file("temperature_0",  0200, ptc_debugfs,  &ptc_instance[0], &ptc_fops);
    debugfs_create_file("temperature_1",  0200, ptc_debugfs,  &ptc_instance[1], &ptc_fops);
    debugfs_create_file("temperature_2",  0200, ptc_debugfs,  &ptc_instance[2], &ptc_fops);
    }
#[no_mangle]
unsafe extern "C" fn ptc_delete_debugfs() {
    static void ptc_delete_debugfs(void)
    {
    debugfs_remove_recursive(ptc_debugfs);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_ptc_add(pdev: *mut pci_dev, proc_priv: *mut proc_thermal_device) -> c_int {
    int proc_thermal_ptc_add(struct pci_dev *pdev, struct proc_thermal_device *proc_priv)
    {
    if (proc_priv.mmio_feature_mask & PROC_THERMAL_FEATURE_PTC) {
    int i, ret;
    for (i = 0; i < PTC_MAX_INSTANCES; i++) {
    ptc_instance[i].offset = ptc_offsets[i];
    ptc_instance[i].pdev = pdev;
    ret = ptc_create_groups(pdev, i, &ptc_instance[i]);
    if (ret) {
    while (i--)
    sysfs_remove_group(&pdev.dev.kobj,
    &ptc_instance[i].ptc_attr_group);
    return ret;
    }
    }
    ptc_create_debugfs();
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(proc_thermal_ptc_add);
#[no_mangle]
pub unsafe extern "C" fn proc_thermal_ptc_remove(pdev: *mut pci_dev) {
    void proc_thermal_ptc_remove(struct pci_dev *pdev)
    {
    struct proc_thermal_device *proc_priv = pci_get_drvdata(pdev);
    if (proc_priv.mmio_feature_mask & PROC_THERMAL_FEATURE_PTC) {
    int i;
    for (i = 0; i < PTC_MAX_INSTANCES; i++)
    sysfs_remove_group(&pdev.dev.kobj, &ptc_instance[i].ptc_attr_group);
    ptc_delete_debugfs();
    }
    }
    EXPORT_SYMBOL_GPL(proc_thermal_ptc_remove);
    MODULE_IMPORT_NS("INT340X_THERMAL");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Processor Thermal PTC Interface");
