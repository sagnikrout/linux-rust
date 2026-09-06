//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/int3472/common.c
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
// Author: Dan Scally <djrscally@gmail.com>

    union acpi_object *skl_int3472_get_acpi_buffer(struct acpi_device *adev, char *id)
    {
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    let mut handle: acpi_handle = adev.handle;
    union acpi_object *obj;
    acpi_status status;
    status = acpi_evaluate_object(handle, id, core::ptr::null_mut(), &buffer);
    if (ACPI_FAILURE(status))
    return ERR_PTR(-ENODEV);
    obj = buffer.pointer;
    if (!obj)
    return ERR_PTR(-ENODEV);
    if (obj.type != ACPI_TYPE_BUFFER) {
    acpi_handle_err(handle, "%s object is not an ACPI buffer\n", id);
    kfree(obj);
    return ERR_PTR(-EINVAL);
    }
    return obj;
    }
    EXPORT_SYMBOL_NS_GPL(skl_int3472_get_acpi_buffer, "INTEL_INT3472");
#[no_mangle]
pub unsafe extern "C" fn skl_int3472_fill_cldb(adev: *mut acpi_device, cldb: *mut int3472_cldb) -> c_int {
    int skl_int3472_fill_cldb(struct acpi_device *adev, struct int3472_cldb *cldb)
    {
    union acpi_object *obj;
    int ret;
    obj = skl_int3472_get_acpi_buffer(adev, "CLDB");
    if (IS_ERR(obj))
    return PTR_ERR(obj);
    if (obj.buffer.length > sizeof(*cldb)) {
    acpi_handle_err(adev.handle, "The CLDB buffer is too large\n");
    ret = -EINVAL;
    goto out_free_obj;
    }
    memcpy(cldb, obj.buffer.pointer, obj.buffer.length);
    ret = 0;
    out_free_obj:
    kfree(obj);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(skl_int3472_fill_cldb, "INTEL_INT3472");
// sensor_adev_ret may be NULL, name_ret must not be NULL
    int skl_int3472_get_sensor_adev_and_name(struct device *dev,
    struct acpi_device **sensor_adev_ret,
    const char **name_ret)
    {
    struct acpi_device *adev = ACPI_COMPANION(dev);
    struct acpi_device *sensor;
    let mut ret: c_int = 0;
    sensor = acpi_dev_get_next_consumer_dev(adev, core::ptr::null_mut());
    if (!sensor) {
    dev_err(dev, "INT3472 seems to have no dependents.\n");
    return -ENODEV;
    }
    dev_dbg(dev, "Sensor name %s\n", acpi_dev_name(sensor));
// name_ret = devm_kasprintf(dev, GFP_KERNEL, I2C_DEV_NAME_FORMAT,
    acpi_dev_name(sensor));
    if (!*name_ret)
    ret = -ENOMEM;
    if (ret == 0 && sensor_adev_ret)
// sensor_adev_ret = sensor;
    else
    acpi_dev_put(sensor);
    return ret;
    }
    EXPORT_SYMBOL_NS_GPL(skl_int3472_get_sensor_adev_and_name, "INTEL_INT3472");
    MODULE_DESCRIPTION("Intel SkyLake INT3472 ACPI Device Driver library");
    MODULE_AUTHOR("Daniel Scally <djrscally@gmail.com>");
    MODULE_LICENSE("GPL");
