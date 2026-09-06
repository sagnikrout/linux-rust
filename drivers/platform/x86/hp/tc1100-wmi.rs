//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/hp/tc1100-wmi.c
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
// HP Compaq TC1100 Tablet WMI Extras Driver
//
// Copyright (C) 2007 Carlos Corbacho <carlos@strangeworlds.co.uk>
// Copyright (C) 2004 Jamey Hicks <jamey.hicks@hp.com>
// Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
// Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
//

pub const TC1100_INSTANCE_WIRELESS: c_int = 1;
pub const TC1100_INSTANCE_JOGDIAL: c_int = 2;
    MODULE_AUTHOR("Jamey Hicks, Carlos Corbacho");
    MODULE_DESCRIPTION("HP Compaq TC1100 Tablet WMI Extras");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("wmi:C364AC71-36DB-495A-8494-B439D472A505");
    static struct platform_device *tc1100_device;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc1100_data {
    pub wireless: u32,
    pub jogdial: u32,
}

    static struct tc1100_data suspend_data;

// --------------------------------------------------------------------------
    Device Management
    -------------------------------------------------------------------------- */
#[no_mangle]
unsafe extern "C" fn get_state(out: *mut u32, instance: u8) -> c_int {
    static int get_state(u32 *out, u8 instance)
    {
    u32 tmp;
    acpi_status status;
    let mut result: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    union acpi_object *obj;
    if (!out)
    return -EINVAL;
    if (instance > 2)
    return -ENODEV;
    status = wmi_query_block(GUID, instance, &result);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    obj = (union acpi_object *) result.pointer;
    if (obj && obj.type == ACPI_TYPE_INTEGER) {
    tmp = obj.integer.value;
    } else {
    tmp = 0;
    }
    if (result.length > 0)
    kfree(result.pointer);
    switch (instance) {
    case TC1100_INSTANCE_WIRELESS:
// out = (tmp == 3) ? 1 : 0;
    return 0;
    case TC1100_INSTANCE_JOGDIAL:
// out = (tmp == 1) ? 0 : 1;
    return 0;
    default:
    return -ENODEV;
    }
    }
#[no_mangle]
unsafe extern "C" fn set_state(in: *mut u32, instance: u8) -> c_int {
    static int set_state(u32 *in, u8 instance)
    {
    u32 value;
    acpi_status status;
    struct acpi_buffer input;
    if (!in)
    return -EINVAL;
    if (instance > 2)
    return -ENODEV;
    switch (instance) {
    case TC1100_INSTANCE_WIRELESS:
    value = (*in) ? 1 : 2;
    break;
    case TC1100_INSTANCE_JOGDIAL:
    value = (*in) ? 0 : 1;
    break;
    default:
    return -ENODEV;
    }
    input.length = sizeof(u32);
    input.pointer = &value;
    status = wmi_set_block(GUID, instance, &input);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    return 0;
    }
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn Interface(_arg: /sys) -> FS {
    FS Interface (/sys)
    -------------------------------------------------------------------------- */
//
// Read/ write bool sysfs macro
//

    static ssize_t \
    show_bool_##value(struct device *dev, struct device_attribute *attr, \
    char *buf) \
    { \
    u32 result; \
    acpi_status status = get_state(&result, instance); \
    if (ACPI_SUCCESS(status)) \
    return sprintf(buf, "%d\n", result); \
    return sprintf(buf, "Read error\n"); \
    } \
    \
    static ssize_t \
    set_bool_##value(struct device *dev, struct device_attribute *attr, \
    const char *buf, size_t count) \
    { \
    u32 tmp = simple_strtoul(buf, core::ptr::null_mut(), 10); \
    acpi_status status = set_state(&tmp, instance); \
    if (ACPI_FAILURE(status)) \
    return -EINVAL; \
    return count; \
    } \
    static DEVICE_ATTR(value, S_IRUGO | S_IWUSR, \
    show_bool_##value, set_bool_##value);
    show_set_bool(wireless, TC1100_INSTANCE_WIRELESS);
    show_set_bool(jogdial, TC1100_INSTANCE_JOGDIAL);
    static struct attribute *tc1100_attributes[] = {
    &dev_attr_wireless.attr,
    &dev_attr_jogdial.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group tc1100_attribute_group = {
    .attrs	= tc1100_attributes,
    };
// --------------------------------------------------------------------------
    Driver Model
    -------------------------------------------------------------------------- */
#[no_mangle]
unsafe extern "C" fn tc1100_probe(device: *mut platform_device) -> int __init {
    static int __init tc1100_probe(struct platform_device *device)
    {
    return sysfs_create_group(&device.dev.kobj, &tc1100_attribute_group);
    }
#[no_mangle]
unsafe extern "C" fn tc1100_remove(device: *mut platform_device) {
    static void tc1100_remove(struct platform_device *device)
    {
    sysfs_remove_group(&device.dev.kobj, &tc1100_attribute_group);
    }

#[no_mangle]
unsafe extern "C" fn tc1100_suspend(dev: *mut device) -> c_int {
    static int tc1100_suspend(struct device *dev)
    {
    int ret;
    ret = get_state(&suspend_data.wireless, TC1100_INSTANCE_WIRELESS);
    if (ret)
    return ret;
    ret = get_state(&suspend_data.jogdial, TC1100_INSTANCE_JOGDIAL);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc1100_resume(dev: *mut device) -> c_int {
    static int tc1100_resume(struct device *dev)
    {
    int ret;
    ret = set_state(&suspend_data.wireless, TC1100_INSTANCE_WIRELESS);
    if (ret)
    return ret;
    ret = set_state(&suspend_data.jogdial, TC1100_INSTANCE_JOGDIAL);
    if (ret)
    return ret;
    return 0;
    }
    static const struct dev_pm_ops tc1100_pm_ops = {
    .suspend	= tc1100_suspend,
    .resume		= tc1100_resume,
    .freeze		= tc1100_suspend,
    .restore	= tc1100_resume,
    };

    static struct platform_driver tc1100_driver = {
    .driver = {
    .name = "tc1100-wmi",

    .pm = &tc1100_pm_ops,

    },
    .remove = tc1100_remove,
    };
#[no_mangle]
unsafe extern "C" fn tc1100_init() -> int __init {
    static int __init tc1100_init(void)
    {
    int error;
    if (!wmi_has_guid(GUID))
    return -ENODEV;
    tc1100_device = platform_device_alloc("tc1100-wmi", PLATFORM_DEVID_NONE);
    if (!tc1100_device)
    return -ENOMEM;
    error = platform_device_add(tc1100_device);
    if (error)
    goto err_device_put;
    error = platform_driver_probe(&tc1100_driver, tc1100_probe);
    if (error)
    goto err_device_del;
    pr_info("HP Compaq TC1100 Tablet WMI Extras loaded\n");
    return 0;
    err_device_del:
    platform_device_del(tc1100_device);
    err_device_put:
    platform_device_put(tc1100_device);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn tc1100_exit() -> void __exit {
    static void __exit tc1100_exit(void)
    {
    platform_device_unregister(tc1100_device);
    platform_driver_unregister(&tc1100_driver);
    }
    module_init(tc1100_init);
    module_exit(tc1100_exit);
