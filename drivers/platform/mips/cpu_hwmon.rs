//! Automatically rewritten from C to Rust
//! Source: drivers/platform/mips/cpu_hwmon.c
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

    static int csr_temp_enable;
//
// Loongson-3 series cpu has two sensors inside,
// each of them from 0 to 255,
// if more than 127, that is dangerous.
// here only provide sensor1 data, because it always hot than sensor0
//
#[no_mangle]
pub unsafe extern "C" fn loongson3_cpu_temp(cpu: c_int) -> c_int {
    int loongson3_cpu_temp(int cpu)
    {
    u32 reg, prid_rev;
    if (csr_temp_enable) {
    reg = (csr_readl(LOONGSON_CSR_CPUTEMP) & 0xff);
    goto out;
    }
    reg = LOONGSON_CHIPTEMP(cpu);
    prid_rev = read_c0_prid() & PRID_REV_MASK;
    switch (prid_rev) {
    case PRID_REV_LOONGSON3A_R1:
    reg = (reg >> 8) & 0xff;
    break;
    case PRID_REV_LOONGSON3B_R1:
    case PRID_REV_LOONGSON3B_R2:
    case PRID_REV_LOONGSON3A_R2_0:
    case PRID_REV_LOONGSON3A_R2_1:
    reg = ((reg >> 8) & 0xff) - 100;
    break;
    case PRID_REV_LOONGSON3A_R3_0:
    case PRID_REV_LOONGSON3A_R3_1:
    default:
    reg = (reg & 0xffff) * 731 / 0x4000 - 273;
    break;
    }
    out:
    return (int)reg * 1000;
    }
    static int nr_packages;
    static struct device *cpu_hwmon_dev;
    static ssize_t cpu_temp_label(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut id: c_int = (to_sensor_dev_attr(attr)).index - 1;
    return sprintf(buf, "CPU %d Temperature\n", id);
    }
    static ssize_t get_cpu_temp(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut id: c_int = (to_sensor_dev_attr(attr)).index - 1;
    let mut value: c_int = loongson3_cpu_temp(id);
    return sprintf(buf, "%d\n", value);
    }
    static SENSOR_DEVICE_ATTR(temp1_input, 0444, get_cpu_temp, core::ptr::null_mut(), 1);
    static SENSOR_DEVICE_ATTR(temp1_label, 0444, cpu_temp_label, core::ptr::null_mut(), 1);
    static SENSOR_DEVICE_ATTR(temp2_input, 0444, get_cpu_temp, core::ptr::null_mut(), 2);
    static SENSOR_DEVICE_ATTR(temp2_label, 0444, cpu_temp_label, core::ptr::null_mut(), 2);
    static SENSOR_DEVICE_ATTR(temp3_input, 0444, get_cpu_temp, core::ptr::null_mut(), 3);
    static SENSOR_DEVICE_ATTR(temp3_label, 0444, cpu_temp_label, core::ptr::null_mut(), 3);
    static SENSOR_DEVICE_ATTR(temp4_input, 0444, get_cpu_temp, core::ptr::null_mut(), 4);
    static SENSOR_DEVICE_ATTR(temp4_label, 0444, cpu_temp_label, core::ptr::null_mut(), 4);
    static struct attribute *cpu_hwmon_attributes[] = {
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_label.dev_attr.attr,
    &sensor_dev_attr_temp2_input.dev_attr.attr,
    &sensor_dev_attr_temp2_label.dev_attr.attr,
    &sensor_dev_attr_temp3_input.dev_attr.attr,
    &sensor_dev_attr_temp3_label.dev_attr.attr,
    &sensor_dev_attr_temp4_input.dev_attr.attr,
    &sensor_dev_attr_temp4_label.dev_attr.attr,
    core::ptr::null_mut()
    };
    static umode_t cpu_hwmon_is_visible(struct kobject *kobj,
    struct attribute *attr, int i)
    {
    let mut id: c_int = i / 2;
    if (id < nr_packages)
    return attr.mode;
    return 0;
    }
    static struct attribute_group cpu_hwmon_group = {
    .attrs = cpu_hwmon_attributes,
    .is_visible = cpu_hwmon_is_visible,
    };
    static const struct attribute_group *cpu_hwmon_groups[] = {
    &cpu_hwmon_group,
    core::ptr::null_mut()
    };
pub const CPU_THERMAL_THRESHOLD: c_int = 90000;
    static struct delayed_work thermal_work;
#[no_mangle]
unsafe extern "C" fn do_thermal_timer(work: *mut work_struct) {
    static void do_thermal_timer(struct work_struct *work)
    {
    int i, value;
    for (i = 0; i < nr_packages; i++) {
    value = loongson3_cpu_temp(i);
    if (value > CPU_THERMAL_THRESHOLD) {
    pr_emerg("Power off due to high temp: %d\n", value);
    orderly_poweroff(true);
    }
    }
    schedule_delayed_work(&thermal_work, msecs_to_jiffies(5000));
    }
#[no_mangle]
unsafe extern "C" fn loongson_hwmon_init() -> int __init {
    static int __init loongson_hwmon_init(void)
    {
    pr_info("Loongson Hwmon Enter...\n");
    if (cpu_has_csr())
    csr_temp_enable = csr_readl(LOONGSON_CSR_FEATURES) &
    LOONGSON_CSRF_TEMP;
    if (!csr_temp_enable && !loongson_chiptemp[0])
    return -ENODEV;
    nr_packages = loongson_sysconf.nr_cpus /
    loongson_sysconf.cores_per_package;
    cpu_hwmon_dev = hwmon_device_register_with_groups(core::ptr::null_mut(), "cpu_hwmon",
    core::ptr::null_mut(), cpu_hwmon_groups);
    if (IS_ERR(cpu_hwmon_dev)) {
    pr_err("hwmon_device_register fail!\n");
    return PTR_ERR(cpu_hwmon_dev);
    }
    INIT_DEFERRABLE_WORK(&thermal_work, do_thermal_timer);
    schedule_delayed_work(&thermal_work, msecs_to_jiffies(20000));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_hwmon_exit() -> void __exit {
    static void __exit loongson_hwmon_exit(void)
    {
    cancel_delayed_work_sync(&thermal_work);
    hwmon_device_unregister(cpu_hwmon_dev);
    }
    module_init(loongson_hwmon_init);
    module_exit(loongson_hwmon_exit);
    MODULE_AUTHOR("Yu Xiang <xiangy@lemote.com>");
    MODULE_AUTHOR("Huacai Chen <chenhc@lemote.com>");
    MODULE_DESCRIPTION("Loongson CPU Hwmon driver");
    MODULE_LICENSE("GPL");
