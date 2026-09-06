//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/vexpress-hwmon.c
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
// Copyright (C) 2012 ARM Limited
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vexpress_hwmon_data {
    pub hwmon_dev: *mut device,
    pub reg: *mut regmap,
}

    static ssize_t vexpress_hwmon_label_show(struct device *dev,
    struct device_attribute *dev_attr, char *buffer)
    {
    const char *label = of_get_property(dev.of_node, "label", core::ptr::null_mut());
    return sysfs_emit(buffer, "%s\n", label);
    }
    static ssize_t vexpress_hwmon_u32_show(struct device *dev,
    struct device_attribute *dev_attr, char *buffer)
    {
    struct vexpress_hwmon_data *data = dev_get_drvdata(dev);
    int err;
    u32 value;
    err = regmap_read(data.reg, 0, &value);
    if (err)
    return err;
    return sysfs_emit(buffer, "%u\n", value /
    to_sensor_dev_attr(dev_attr).index);
    }
    static ssize_t vexpress_hwmon_u64_show(struct device *dev,
    struct device_attribute *dev_attr, char *buffer)
    {
    struct vexpress_hwmon_data *data = dev_get_drvdata(dev);
    int err;
    u32 value_hi, value_lo;
    err = regmap_read(data.reg, 0, &value_lo);
    if (err)
    return err;
    err = regmap_read(data.reg, 1, &value_hi);
    if (err)
    return err;
    return sysfs_emit(buffer, "%llu\n",
    div_u64(((u64)value_hi << 32) | value_lo,
    to_sensor_dev_attr(dev_attr).index));
    }
    static umode_t vexpress_hwmon_attr_is_visible(struct kobject *kobj,
    struct attribute *attr, int index)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct device_attribute *dev_attr = container_of(attr,
    struct device_attribute, attr);
    if (dev_attr.show == vexpress_hwmon_label_show &&
    !of_property_present(dev.of_node, "label"))
    return 0;
    return attr.mode;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vexpress_hwmon_type {
    pub name: *const c_char,
    pub attr_groups: *const attribute_group,
}

    static DEVICE_ATTR(in1_label, 0444, vexpress_hwmon_label_show, core::ptr::null_mut());
    static SENSOR_DEVICE_ATTR_RO(in1_input, vexpress_hwmon_u32, 1000);
    static struct attribute *vexpress_hwmon_attrs_volt[] = {
    &dev_attr_in1_label.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group vexpress_hwmon_group_volt = {
    .is_visible = vexpress_hwmon_attr_is_visible,
    .attrs = vexpress_hwmon_attrs_volt,
    };
    static struct vexpress_hwmon_type vexpress_hwmon_volt = {
    .name = "vexpress_volt",
    .attr_groups = (const struct attribute_group *[]) {
    &vexpress_hwmon_group_volt,
    core::ptr::null_mut(),
    },
    };

    static DEVICE_ATTR(curr1_label, 0444, vexpress_hwmon_label_show, core::ptr::null_mut());
    static SENSOR_DEVICE_ATTR_RO(curr1_input, vexpress_hwmon_u32, 1000);
    static struct attribute *vexpress_hwmon_attrs_amp[] = {
    &dev_attr_curr1_label.attr,
    &sensor_dev_attr_curr1_input.dev_attr.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group vexpress_hwmon_group_amp = {
    .is_visible = vexpress_hwmon_attr_is_visible,
    .attrs = vexpress_hwmon_attrs_amp,
    };
    static struct vexpress_hwmon_type vexpress_hwmon_amp = {
    .name = "vexpress_amp",
    .attr_groups = (const struct attribute_group *[]) {
    &vexpress_hwmon_group_amp,
    core::ptr::null_mut()
    },
    };
    static DEVICE_ATTR(temp1_label, 0444, vexpress_hwmon_label_show, core::ptr::null_mut());
    static SENSOR_DEVICE_ATTR_RO(temp1_input, vexpress_hwmon_u32, 1000);
    static struct attribute *vexpress_hwmon_attrs_temp[] = {
    &dev_attr_temp1_label.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group vexpress_hwmon_group_temp = {
    .is_visible = vexpress_hwmon_attr_is_visible,
    .attrs = vexpress_hwmon_attrs_temp,
    };
    static struct vexpress_hwmon_type vexpress_hwmon_temp = {
    .name = "vexpress_temp",
    .attr_groups = (const struct attribute_group *[]) {
    &vexpress_hwmon_group_temp,
    core::ptr::null_mut()
    },
    };
    static DEVICE_ATTR(power1_label, 0444, vexpress_hwmon_label_show, core::ptr::null_mut());
    static SENSOR_DEVICE_ATTR_RO(power1_input, vexpress_hwmon_u32, 1);
    static struct attribute *vexpress_hwmon_attrs_power[] = {
    &dev_attr_power1_label.attr,
    &sensor_dev_attr_power1_input.dev_attr.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group vexpress_hwmon_group_power = {
    .is_visible = vexpress_hwmon_attr_is_visible,
    .attrs = vexpress_hwmon_attrs_power,
    };
    static struct vexpress_hwmon_type vexpress_hwmon_power = {
    .name = "vexpress_power",
    .attr_groups = (const struct attribute_group *[]) {
    &vexpress_hwmon_group_power,
    core::ptr::null_mut()
    },
    };
    static DEVICE_ATTR(energy1_label, 0444, vexpress_hwmon_label_show, core::ptr::null_mut());
    static SENSOR_DEVICE_ATTR_RO(energy1_input, vexpress_hwmon_u64, 1);
    static struct attribute *vexpress_hwmon_attrs_energy[] = {
    &dev_attr_energy1_label.attr,
    &sensor_dev_attr_energy1_input.dev_attr.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group vexpress_hwmon_group_energy = {
    .is_visible = vexpress_hwmon_attr_is_visible,
    .attrs = vexpress_hwmon_attrs_energy,
    };
    static struct vexpress_hwmon_type vexpress_hwmon_energy = {
    .name = "vexpress_energy",
    .attr_groups = (const struct attribute_group *[]) {
    &vexpress_hwmon_group_energy,
    core::ptr::null_mut()
    },
    };
    static const struct of_device_id vexpress_hwmon_of_match[] = {

    {
    .compatible = "arm,vexpress-volt",
    .data = &vexpress_hwmon_volt,
    },

    {
    .compatible = "arm,vexpress-amp",
    .data = &vexpress_hwmon_amp,
    }, {
    .compatible = "arm,vexpress-temp",
    .data = &vexpress_hwmon_temp,
    }, {
    .compatible = "arm,vexpress-power",
    .data = &vexpress_hwmon_power,
    }, {
    .compatible = "arm,vexpress-energy",
    .data = &vexpress_hwmon_energy,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, vexpress_hwmon_of_match);
#[no_mangle]
unsafe extern "C" fn vexpress_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int vexpress_hwmon_probe(struct platform_device *pdev)
    {
    struct vexpress_hwmon_data *data;
    const struct vexpress_hwmon_type *type;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    platform_set_drvdata(pdev, data);
    type = of_device_get_match_data(&pdev.dev);
    if (!type)
    return -ENODEV;
    data.reg = devm_regmap_init_vexpress_config(&pdev.dev);
    if (IS_ERR(data.reg))
    return PTR_ERR(data.reg);
    data.hwmon_dev = devm_hwmon_device_register_with_groups(&pdev.dev,
    type.name, data, type.attr_groups);
    return PTR_ERR_OR_ZERO(data.hwmon_dev);
    }
    static struct platform_driver vexpress_hwmon_driver = {
    .probe = vexpress_hwmon_probe,
    .driver	= {
    .name = DRVNAME,
    .of_match_table = vexpress_hwmon_of_match,
    },
    };
    module_platform_driver(vexpress_hwmon_driver);
    MODULE_AUTHOR("Pawel Moll <pawel.moll@arm.com>");
    MODULE_DESCRIPTION("Versatile Express hwmon sensors driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:vexpress-hwmon");
