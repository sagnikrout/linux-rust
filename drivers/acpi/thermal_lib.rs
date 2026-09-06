//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/thermal_lib.c
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
// Copyright 2023 Linaro Limited
// Copyright 2023 Intel Corporation
//
// Library routines for retrieving trip point temperature values from the
// platform firmware via ACPI.
//

//
// Minimum temperature for full military grade is 218°K (-55°C) and
// max temperature is 448°K (175°C). We can consider those values as
// the boundaries for the [trips] temperature returned by the
// firmware. Any values out of these boundaries may be considered
// bogus and we can assume the firmware has no data to provide.
//

    static int acpi_trip_temp(struct acpi_device *adev, char *obj_name,
    int *ret_temp)
    {
    unsigned long long temp;
    acpi_status status;
    status = acpi_evaluate_integer(adev.handle, obj_name, core::ptr::null_mut(), &temp);
    if (ACPI_FAILURE(status)) {
    acpi_handle_debug(adev.handle, "%s evaluation failed\n", obj_name);
    return -ENODATA;
    }
    if (temp >= TEMP_MIN_DECIK && temp <= TEMP_MAX_DECIK) {
// ret_temp = temp;
    } else {
    acpi_handle_debug(adev.handle, "%s result %llu out of range\n",
    obj_name, temp);
// ret_temp = THERMAL_TEMP_INVALID;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_active_trip_temp(adev: *mut acpi_device, id: c_int, ret_temp: *mut c_int) -> c_int {
    int acpi_active_trip_temp(struct acpi_device *adev, int id, int *ret_temp)
    {
    char obj_name[] = {'_', 'A', 'C', '0' + id, '\0'};
    if (id < 0 || id > 9)
    return -EINVAL;
    return acpi_trip_temp(adev, obj_name, ret_temp);
    }
    EXPORT_SYMBOL_NS_GPL(acpi_active_trip_temp, "ACPI_THERMAL");
#[no_mangle]
pub unsafe extern "C" fn acpi_passive_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    int acpi_passive_trip_temp(struct acpi_device *adev, int *ret_temp)
    {
    return acpi_trip_temp(adev, "_PSV", ret_temp);
    }
    EXPORT_SYMBOL_NS_GPL(acpi_passive_trip_temp, "ACPI_THERMAL");
#[no_mangle]
pub unsafe extern "C" fn acpi_hot_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    int acpi_hot_trip_temp(struct acpi_device *adev, int *ret_temp)
    {
    return acpi_trip_temp(adev, "_HOT", ret_temp);
    }
    EXPORT_SYMBOL_NS_GPL(acpi_hot_trip_temp, "ACPI_THERMAL");
#[no_mangle]
pub unsafe extern "C" fn acpi_critical_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    int acpi_critical_trip_temp(struct acpi_device *adev, int *ret_temp)
    {
    return acpi_trip_temp(adev, "_CRT", ret_temp);
    }
    EXPORT_SYMBOL_NS_GPL(acpi_critical_trip_temp, "ACPI_THERMAL");
#[no_mangle]
unsafe extern "C" fn thermal_temp(error: c_int, temp_decik: c_int, ret_temp: *mut c_int) -> c_int {
    static int thermal_temp(int error, int temp_decik, int *ret_temp)
    {
    if (error)
    return error;
    if (temp_decik == THERMAL_TEMP_INVALID)
// ret_temp = THERMAL_TEMP_INVALID;
    else
// ret_temp = deci_kelvin_to_millicelsius(temp_decik);
    return 0;
    }
//
// thermal_acpi_active_trip_temp - Retrieve active trip point temperature
// @adev: Target thermal zone ACPI device object.
// @id: Active cooling level (0 - 9).
// @ret_temp: Address to store the retrieved temperature value on success.
//
// Evaluate the _ACx object for the thermal zone represented by @adev to obtain
// the temperature of the active cooling trip point corresponding to the active
// cooling level given by @id.
//
// Return 0 on success or a negative error value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn thermal_acpi_active_trip_temp(adev: *mut acpi_device, id: c_int, ret_temp: *mut c_int) -> c_int {
    int thermal_acpi_active_trip_temp(struct acpi_device *adev, int id, int *ret_temp)
    {
    let mut temp_decik: c_int = 0;
    let mut ret: c_int = acpi_active_trip_temp(adev, id, &temp_decik);
    return thermal_temp(ret, temp_decik, ret_temp);
    }
    EXPORT_SYMBOL_GPL(thermal_acpi_active_trip_temp);
//
// thermal_acpi_passive_trip_temp - Retrieve passive trip point temperature
// @adev: Target thermal zone ACPI device object.
// @ret_temp: Address to store the retrieved temperature value on success.
//
// Evaluate the _PSV object for the thermal zone represented by @adev to obtain
// the temperature of the passive cooling trip point.
//
// Return 0 on success or -ENODATA on failure.
//
#[no_mangle]
pub unsafe extern "C" fn thermal_acpi_passive_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    int thermal_acpi_passive_trip_temp(struct acpi_device *adev, int *ret_temp)
    {
    let mut temp_decik: c_int = 0;
    let mut ret: c_int = acpi_passive_trip_temp(adev, &temp_decik);
    return thermal_temp(ret, temp_decik, ret_temp);
    }
    EXPORT_SYMBOL_GPL(thermal_acpi_passive_trip_temp);
//
// thermal_acpi_hot_trip_temp - Retrieve hot trip point temperature
// @adev: Target thermal zone ACPI device object.
// @ret_temp: Address to store the retrieved temperature value on success.
//
// Evaluate the _HOT object for the thermal zone represented by @adev to obtain
// the temperature of the trip point at which the system is expected to be put
// into the S4 sleep state.
//
// Return 0 on success or -ENODATA on failure.
//
#[no_mangle]
pub unsafe extern "C" fn thermal_acpi_hot_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    int thermal_acpi_hot_trip_temp(struct acpi_device *adev, int *ret_temp)
    {
    let mut temp_decik: c_int = 0;
    let mut ret: c_int = acpi_hot_trip_temp(adev, &temp_decik);
    return thermal_temp(ret, temp_decik, ret_temp);
    }
    EXPORT_SYMBOL_GPL(thermal_acpi_hot_trip_temp);
//
// thermal_acpi_critical_trip_temp - Retrieve critical trip point temperature
// @adev: Target thermal zone ACPI device object.
// @ret_temp: Address to store the retrieved temperature value on success.
//
// Evaluate the _CRT object for the thermal zone represented by @adev to obtain
// the temperature of the critical cooling trip point.
//
// Return 0 on success or -ENODATA on failure.
//
#[no_mangle]
pub unsafe extern "C" fn thermal_acpi_critical_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int {
    int thermal_acpi_critical_trip_temp(struct acpi_device *adev, int *ret_temp)
    {
    let mut temp_decik: c_int = 0;
    let mut ret: c_int = acpi_critical_trip_temp(adev, &temp_decik);
    return thermal_temp(ret, temp_decik, ret_temp);
    }
    EXPORT_SYMBOL_GPL(thermal_acpi_critical_trip_temp);
