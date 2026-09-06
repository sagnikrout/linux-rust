//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/virtual.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// reg-virtual-consumer.c
//
// Copyright 2008 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtual_consumer_data {
    pub lock: mutex,
    pub regulator: *mut regulator,
    pub enabled: bool,
    pub min_uV: c_int,
    pub max_uV: c_int,
    pub min_uA: c_int,
    pub max_uA: c_int,
    pub mode: c_uint,
}

    static void update_voltage_constraints(struct device *dev,
    struct virtual_consumer_data *data)
    {
    int ret;
    if (data.min_uV && data.max_uV
    && data.min_uV <= data.max_uV) {
    dev_dbg(dev, "Requesting %d-%duV\n",
    data.min_uV, data.max_uV);
    ret = regulator_set_voltage(data.regulator,
    data.min_uV, data.max_uV);
    if (ret != 0) {
    dev_err(dev,
    "regulator_set_voltage() failed: %d\n", ret);
    return;
    }
    }
    if (data.min_uV && data.max_uV && !data.enabled) {
    dev_dbg(dev, "Enabling regulator\n");
    ret = regulator_enable(data.regulator);
    if (ret == 0)
    data.enabled = true;
    else
    dev_err(dev, "regulator_enable() failed: %d\n",
    ret);
    }
    if (!(data.min_uV && data.max_uV) && data.enabled) {
    dev_dbg(dev, "Disabling regulator\n");
    ret = regulator_disable(data.regulator);
    if (ret == 0)
    data.enabled = false;
    else
    dev_err(dev, "regulator_disable() failed: %d\n",
    ret);
    }
    }
    static void update_current_limit_constraints(struct device *dev,
    struct virtual_consumer_data *data)
    {
    int ret;
    if (data.max_uA
    && data.min_uA <= data.max_uA) {
    dev_dbg(dev, "Requesting %d-%duA\n",
    data.min_uA, data.max_uA);
    ret = regulator_set_current_limit(data.regulator,
    data.min_uA, data.max_uA);
    if (ret != 0) {
    dev_err(dev,
    "regulator_set_current_limit() failed: %d\n",
    ret);
    return;
    }
    }
    if (data.max_uA && !data.enabled) {
    dev_dbg(dev, "Enabling regulator\n");
    ret = regulator_enable(data.regulator);
    if (ret == 0)
    data.enabled = true;
    else
    dev_err(dev, "regulator_enable() failed: %d\n",
    ret);
    }
    if (!(data.min_uA && data.max_uA) && data.enabled) {
    dev_dbg(dev, "Disabling regulator\n");
    ret = regulator_disable(data.regulator);
    if (ret == 0)
    data.enabled = false;
    else
    dev_err(dev, "regulator_disable() failed: %d\n",
    ret);
    }
    }
    static ssize_t show_min_uV(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n", data.min_uV);
    }
    static ssize_t set_min_uV(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    long val;
    if (kstrtol(buf, 10, &val) != 0)
    return count;
    mutex_lock(&data.lock);
    data.min_uV = val;
    update_voltage_constraints(dev, data);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t show_max_uV(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n", data.max_uV);
    }
    static ssize_t set_max_uV(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    long val;
    if (kstrtol(buf, 10, &val) != 0)
    return count;
    mutex_lock(&data.lock);
    data.max_uV = val;
    update_voltage_constraints(dev, data);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t show_min_uA(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n", data.min_uA);
    }
    static ssize_t set_min_uA(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    long val;
    if (kstrtol(buf, 10, &val) != 0)
    return count;
    mutex_lock(&data.lock);
    data.min_uA = val;
    update_current_limit_constraints(dev, data);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t show_max_uA(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    return sprintf(buf, "%d\n", data.max_uA);
    }
    static ssize_t set_max_uA(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    long val;
    if (kstrtol(buf, 10, &val) != 0)
    return count;
    mutex_lock(&data.lock);
    data.max_uA = val;
    update_current_limit_constraints(dev, data);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t show_mode(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    switch (data.mode) {
    case REGULATOR_MODE_FAST:
    return sprintf(buf, "fast\n");
    case REGULATOR_MODE_NORMAL:
    return sprintf(buf, "normal\n");
    case REGULATOR_MODE_IDLE:
    return sprintf(buf, "idle\n");
    case REGULATOR_MODE_STANDBY:
    return sprintf(buf, "standby\n");
    default:
    return sprintf(buf, "unknown\n");
    }
    }
    static ssize_t set_mode(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct virtual_consumer_data *data = dev_get_drvdata(dev);
    unsigned int mode;
    int ret;
//
// sysfs_streq() doesn't need the \n's, but we add them so the strings
// will be shared with show_mode(), above.
//
    if (sysfs_streq(buf, "fast\n"))
    mode = REGULATOR_MODE_FAST;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: "normal\n")) -> else {
    else if (sysfs_streq(buf, "normal\n"))
    mode = REGULATOR_MODE_NORMAL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: "idle\n")) -> else {
    else if (sysfs_streq(buf, "idle\n"))
    mode = REGULATOR_MODE_IDLE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: "standby\n")) -> else {
    else if (sysfs_streq(buf, "standby\n"))
    mode = REGULATOR_MODE_STANDBY;
    else {
    dev_err(dev, "Configuring invalid mode\n");
    return count;
    }
    mutex_lock(&data.lock);
    ret = regulator_set_mode(data.regulator, mode);
    if (ret == 0)
    data.mode = mode;
    else
    dev_err(dev, "Failed to configure mode: %d\n", ret);
    mutex_unlock(&data.lock);
    return count;
    }
    static DEVICE_ATTR(min_microvolts, 0664, show_min_uV, set_min_uV);
    static DEVICE_ATTR(max_microvolts, 0664, show_max_uV, set_max_uV);
    static DEVICE_ATTR(min_microamps, 0664, show_min_uA, set_min_uA);
    static DEVICE_ATTR(max_microamps, 0664, show_max_uA, set_max_uA);
    static DEVICE_ATTR(mode, 0664, show_mode, set_mode);
    static struct attribute *regulator_virtual_attributes[] = {
    &dev_attr_min_microvolts.attr,
    &dev_attr_max_microvolts.attr,
    &dev_attr_min_microamps.attr,
    &dev_attr_max_microamps.attr,
    &dev_attr_mode.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group regulator_virtual_attr_group = {
    .attrs	= regulator_virtual_attributes,
    };

    static const struct of_device_id regulator_virtual_consumer_of_match[] = {
    { .compatible = "regulator-virtual-consumer" },
    {},
    };
    MODULE_DEVICE_TABLE(of, regulator_virtual_consumer_of_match);

#[no_mangle]
unsafe extern "C" fn regulator_virtual_probe(pdev: *mut platform_device) -> c_int {
    static int regulator_virtual_probe(struct platform_device *pdev)
    {
    char *reg_id = dev_get_platdata(&pdev.dev);
    struct virtual_consumer_data *drvdata;
    static bool warned;
    int ret;
    if (!warned) {
    warned = true;
    pr_warn("**********************************************************\n");
    pr_warn("**   NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE   **\n");
    pr_warn("**                                                      **\n");
    pr_warn("** regulator-virtual-consumer is only for testing and   **\n");
    pr_warn("** debugging.  Do not use it in a production kernel.    **\n");
    pr_warn("**                                                      **\n");
    pr_warn("**   NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE   **\n");
    pr_warn("**********************************************************\n");
    }
    drvdata = devm_kzalloc(&pdev.dev, sizeof(struct virtual_consumer_data),
    GFP_KERNEL);
    if (drvdata == core::ptr::null_mut())
    return -ENOMEM;
//
// This virtual consumer does not have any hardware-defined supply
// name, so just allow the regulator to be specified in a property
// named "default-supply" when we're being probed from devicetree.
//
    if (!reg_id && pdev.dev.of_node)
    reg_id = "default";
    mutex_init(&drvdata.lock);
    drvdata.regulator = devm_regulator_get(&pdev.dev, reg_id);
    if (IS_ERR(drvdata.regulator))
    return dev_err_probe(&pdev.dev, PTR_ERR(drvdata.regulator),
    "Failed to obtain supply '%s'\n",
    reg_id);
    ret = sysfs_create_group(&pdev.dev.kobj,
    &regulator_virtual_attr_group);
    if (ret != 0) {
    dev_err(&pdev.dev,
    "Failed to create attribute group: %d\n", ret);
    return ret;
    }
    drvdata.mode = regulator_get_mode(drvdata.regulator);
    platform_set_drvdata(pdev, drvdata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regulator_virtual_remove(pdev: *mut platform_device) {
    static void regulator_virtual_remove(struct platform_device *pdev)
    {
    struct virtual_consumer_data *drvdata = platform_get_drvdata(pdev);
    sysfs_remove_group(&pdev.dev.kobj, &regulator_virtual_attr_group);
    if (drvdata.enabled)
    regulator_disable(drvdata.regulator);
    }
    static struct platform_driver regulator_virtual_consumer_driver = {
    .probe		= regulator_virtual_probe,
    .remove		= regulator_virtual_remove,
    .driver		= {
    .name		= "reg-virt-consumer",
    .probe_type	= PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = of_match_ptr(regulator_virtual_consumer_of_match),
    },
    };
    module_platform_driver(regulator_virtual_consumer_driver);
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_DESCRIPTION("Virtual regulator consumer");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:reg-virt-consumer");
