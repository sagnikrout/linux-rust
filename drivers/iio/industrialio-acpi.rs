//! Automatically rewritten from C to Rust
//! Source: drivers/iio/industrialio-acpi.c
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
// IIO ACPI helper functions

//
// iio_read_acpi_mount_matrix() - Read accelerometer mount matrix info from ACPI
// @dev:		Device structure
// @orientation:	iio_mount_matrix struct to fill
// @acpi_method:	ACPI method name to read the matrix from, usually "ROTM"
//
// Try to read the mount-matrix by calling the specified method on the device's
// ACPI firmware-node. If the device has no ACPI firmware-node; or the method
// does not exist then this will fail silently. This expects the method to
// return data in the ACPI "ROTM" format defined by Microsoft:
// https://learn.microsoft.com/en-us/windows-hardware/drivers/sensors/sensors-acpi-entries
// This is a Microsoft extension and not part of the official ACPI spec.
// The method name is configurable because some dual-accel setups define 2 mount
// matrices in a single ACPI device using separate "ROMK" and "ROMS" methods.
//
// Returns: true if the matrix was successfully, false otherwise.
//
    bool iio_read_acpi_mount_matrix(struct device *dev,
    struct iio_mount_matrix *orientation,
    char *acpi_method)
    {
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    char *str;
    union acpi_object *obj, *elements;
    acpi_handle handle;
    acpi_status status;
    int i, j, val[3];
    let mut ret: bool = false;
    handle = ACPI_HANDLE(dev);
    if (!handle)
    return false;
    if (!acpi_has_method(handle, acpi_method))
    return false;
    status = acpi_evaluate_object(handle, acpi_method, core::ptr::null_mut(), &buffer);
    if (ACPI_FAILURE(status)) {
    dev_err(dev, "Failed to get ACPI mount matrix: %d\n", status);
    return false;
    }
    obj = buffer.pointer;
    if (obj.type != ACPI_TYPE_PACKAGE || obj.package.count != 3) {
    dev_err(dev, "Unknown ACPI mount matrix package format\n");
    goto out_free_buffer;
    }
    elements = obj.package.elements;
    for (i = 0; i < 3; i++) {
    if (elements[i].type != ACPI_TYPE_STRING) {
    dev_err(dev, "Unknown ACPI mount matrix element format\n");
    goto out_free_buffer;
    }
    str = elements[i].string.pointer;
    if (sscanf(str, "%d %d %d", &val[0], &val[1], &val[2]) != 3) {
    dev_err(dev, "Incorrect ACPI mount matrix string format\n");
    goto out_free_buffer;
    }
    for (j = 0; j < 3; j++) {
    switch (val[j]) {
    case -1: str = "-1"; break;
    case 0:  str = "0";  break;
    case 1:  str = "1";  break;
    default:
    dev_err(dev, "Invalid value in ACPI mount matrix: %d\n", val[j]);
    goto out_free_buffer;
    }
    orientation.rotation[i * 3 + j] = str;
    }
    }
    ret = true;
    out_free_buffer:
    kfree(buffer.pointer);
    return ret;
    }
    EXPORT_SYMBOL_GPL(iio_read_acpi_mount_matrix);
//
// iio_get_acpi_device_name_and_data() - Return ACPI device instance name and driver data
// @dev:		Device structure
// @data:		Optional pointer to return driver data
//
// When device was enumerated by ACPI ID matching, the user might
// want to set description for the physical chip. In such cases
// the ACPI device instance name might be used. This call may be
// performed to retrieve this information.
//
// NOTE: This helper function exists only for backward compatibility,
// do not use in a new code!
//
// Returns: ACPI device instance name or %NULL.
//
    const char *iio_get_acpi_device_name_and_data(struct device *dev, const void **data)
    {
    const struct acpi_device_id *id;
    acpi_handle handle;
    handle = ACPI_HANDLE(dev);
    if (!handle)
    return core::ptr::null_mut();
    id = acpi_match_device(dev.driver.acpi_match_table, dev);
    if (!id)
    return core::ptr::null_mut();
    if (data)
// data = (const void *)id->driver_data;
    return dev_name(dev);
    }
    EXPORT_SYMBOL_GPL(iio_get_acpi_device_name_and_data);
