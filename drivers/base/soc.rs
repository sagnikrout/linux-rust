//! Automatically rewritten from C to Rust
//! Source: drivers/base/soc.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) ST-Ericsson SA 2011
//
// Author: Lee Jones <lee.jones@linaro.org> for ST-Ericsson.
//

    static DEFINE_IDA(soc_ida);
// Prototype to allow declarations of DEVICE_ATTR(<foo>) before soc_info_show
    static ssize_t soc_info_show(struct device *dev, struct device_attribute *attr,
    char *buf);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_device {
    pub dev: device,
    pub attr: *mut soc_device_attribute,
    pub soc_dev_num: c_int,
}

    static const struct bus_type soc_bus_type = {
    .name  = "soc",
    };
    static bool soc_bus_registered;
    static DEVICE_ATTR(machine,		0444, soc_info_show,  core::ptr::null_mut());
    static DEVICE_ATTR(family,		0444, soc_info_show,  core::ptr::null_mut());
    static DEVICE_ATTR(serial_number,	0444, soc_info_show,  core::ptr::null_mut());
    static DEVICE_ATTR(soc_id,		0444, soc_info_show,  core::ptr::null_mut());
    static DEVICE_ATTR(revision,		0444, soc_info_show,  core::ptr::null_mut());
    struct device *soc_device_to_device(struct soc_device *soc_dev)
    {
    return &soc_dev.dev;
    }
    static umode_t soc_attribute_mode(struct kobject *kobj,
    struct attribute *attr,
    int index)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct soc_device *soc_dev = container_of(dev, struct soc_device, dev);
    if ((attr == &dev_attr_machine.attr) && soc_dev.attr.machine)
    return attr.mode;
    if ((attr == &dev_attr_family.attr) && soc_dev.attr.family)
    return attr.mode;
    if ((attr == &dev_attr_revision.attr) && soc_dev.attr.revision)
    return attr.mode;
    if ((attr == &dev_attr_serial_number.attr) && soc_dev.attr.serial_number)
    return attr.mode;
    if ((attr == &dev_attr_soc_id.attr) && soc_dev.attr.soc_id)
    return attr.mode;
// Unknown or unfilled attribute
    return 0;
    }
    static ssize_t soc_info_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct soc_device *soc_dev = container_of(dev, struct soc_device, dev);
    const char *output;
    if (attr == &dev_attr_machine)
    output = soc_dev.attr.machine;
#[no_mangle]
pub unsafe extern "C" fn if(&dev_attr_family: attr ==) -> else {
    else if (attr == &dev_attr_family)
    output = soc_dev.attr.family;
#[no_mangle]
pub unsafe extern "C" fn if(&dev_attr_revision: attr ==) -> else {
    else if (attr == &dev_attr_revision)
    output = soc_dev.attr.revision;
#[no_mangle]
pub unsafe extern "C" fn if(&dev_attr_serial_number: attr ==) -> else {
    else if (attr == &dev_attr_serial_number)
    output = soc_dev.attr.serial_number;
#[no_mangle]
pub unsafe extern "C" fn if(&dev_attr_soc_id: attr ==) -> else {
    else if (attr == &dev_attr_soc_id)
    output = soc_dev.attr.soc_id;
    else
    return -EINVAL;
    return sysfs_emit(buf, "%s\n", output);
    }
    static struct attribute *soc_attr[] = {
    &dev_attr_machine.attr,
    &dev_attr_family.attr,
    &dev_attr_serial_number.attr,
    &dev_attr_soc_id.attr,
    &dev_attr_revision.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group soc_attr_group = {
    .attrs = soc_attr,
    .is_visible = soc_attribute_mode,
    };
#[no_mangle]
unsafe extern "C" fn soc_release(dev: *mut device) {
    static void soc_release(struct device *dev)
    {
    struct soc_device *soc_dev = container_of(dev, struct soc_device, dev);
    ida_free(&soc_ida, soc_dev.soc_dev_num);
    kfree(soc_dev.dev.groups);
    kfree(soc_dev);
    }
#[no_mangle]
pub unsafe extern "C" fn soc_attr_read_machine(soc_dev_attr: *mut soc_device_attribute) -> c_int {
    int soc_attr_read_machine(struct soc_device_attribute *soc_dev_attr)
    {
    if (soc_dev_attr.machine)
    return -EBUSY;
    return of_machine_read_model(&soc_dev_attr.machine);
    }
    EXPORT_SYMBOL_GPL(soc_attr_read_machine);
    static struct soc_device_attribute *early_soc_dev_attr;
    struct soc_device *soc_device_register(struct soc_device_attribute *soc_dev_attr)
    {
    struct soc_device *soc_dev;
    const struct attribute_group **soc_attr_groups;
    int ret;
    soc_attr_read_machine(soc_dev_attr);
    if (!soc_bus_registered) {
    if (early_soc_dev_attr)
    return ERR_PTR(-EBUSY);
    early_soc_dev_attr = soc_dev_attr;
    return core::ptr::null_mut();
    }
    soc_dev = kzalloc_obj(*soc_dev);
    if (!soc_dev) {
    ret = -ENOMEM;
    goto out1;
    }
    soc_attr_groups = kzalloc_objs(*soc_attr_groups, 3);
    if (!soc_attr_groups) {
    ret = -ENOMEM;
    goto out2;
    }
    soc_attr_groups[0] = &soc_attr_group;
    soc_attr_groups[1] = soc_dev_attr.custom_attr_group;
// Fetch a unique (reclaimable) SOC ID.
    ret = ida_alloc(&soc_ida, GFP_KERNEL);
    if (ret < 0)
    goto out3;
    soc_dev.soc_dev_num = ret;
    soc_dev.attr = soc_dev_attr;
    soc_dev.dev.bus = &soc_bus_type;
    soc_dev.dev.groups = soc_attr_groups;
    soc_dev.dev.release = soc_release;
    dev_set_name(&soc_dev.dev, "soc%d", soc_dev.soc_dev_num);
    ret = device_register(&soc_dev.dev);
    if (ret) {
    put_device(&soc_dev.dev);
    return ERR_PTR(ret);
    }
    return soc_dev;
    out3:
    kfree(soc_attr_groups);
    out2:
    kfree(soc_dev);
    out1:
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(soc_device_register);
// Ensure soc_dev->attr is freed after calling soc_device_unregister.
#[no_mangle]
pub unsafe extern "C" fn soc_device_unregister(soc_dev: *mut soc_device) {
    void soc_device_unregister(struct soc_device *soc_dev)
    {
    device_unregister(&soc_dev.dev);
    early_soc_dev_attr = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(soc_device_unregister);
#[no_mangle]
unsafe extern "C" fn soc_bus_register() -> int __init {
    static int __init soc_bus_register(void)
    {
    struct soc_device *soc_dev;
    int ret;
    ret = bus_register(&soc_bus_type);
    if (ret)
    return ret;
    soc_bus_registered = true;
    if (early_soc_dev_attr) {
    soc_dev = soc_device_register(early_soc_dev_attr);
    if (IS_ERR(soc_dev)) {
    ret = PTR_ERR(soc_dev);
    goto err_unregister_bus;
    }
    }
    return 0;
    err_unregister_bus:
    soc_bus_registered = false;
    bus_unregister(&soc_bus_type);
    return ret;
    }
    core_initcall(soc_bus_register);
    static int soc_device_match_attr(const struct soc_device_attribute *attr,
    const struct soc_device_attribute *match)
    {
    if (match.machine &&
    (!attr.machine || !glob_match(match.machine, attr.machine)))
    return 0;
    if (match.family &&
    (!attr.family || !glob_match(match.family, attr.family)))
    return 0;
    if (match.revision &&
    (!attr.revision || !glob_match(match.revision, attr.revision)))
    return 0;
    if (match.soc_id &&
    (!attr.soc_id || !glob_match(match.soc_id, attr.soc_id)))
    return 0;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn soc_device_match_one(dev: *mut device, arg: *mut c_void) -> c_int {
    static int soc_device_match_one(struct device *dev, void *arg)
    {
    struct soc_device *soc_dev = container_of(dev, struct soc_device, dev);
    return soc_device_match_attr(soc_dev.attr, arg);
    }
//
// soc_device_match - identify the SoC in the machine
// @matches: zero-terminated array of possible matches
//
// returns the first matching entry of the argument array, or NULL
// if none of them match.
//
// This function is meant as a helper in place of of_match_node()
// in cases where either no device tree is available or the information
// in a device node is insufficient to identify a particular variant
// by its compatible strings or other properties. For new devices,
// the DT binding should always provide unique compatible strings
// that allow the use of of_match_node() instead.
//
// The calling function can use the .data entry of the
// soc_device_attribute to pass a structure or function pointer for
// each entry.
//
    const struct soc_device_attribute *soc_device_match(
    const struct soc_device_attribute *matches)
    {
    int ret;
    if (!matches)
    return core::ptr::null_mut();
    while (matches.machine || matches.family || matches.revision ||
    matches.soc_id) {
    ret = bus_for_each_dev(&soc_bus_type, core::ptr::null_mut(), (void *)matches,
    soc_device_match_one);
    if (ret < 0 && early_soc_dev_attr)
    ret = soc_device_match_attr(early_soc_dev_attr,
    matches);
    if (ret < 0)
    return core::ptr::null_mut();
    if (ret)
    return matches;
    matches++;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(soc_device_match);
