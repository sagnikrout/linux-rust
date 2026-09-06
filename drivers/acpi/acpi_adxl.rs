//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpi_adxl.c
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
// Address translation interface via ACPI DSM.
// Copyright (C) 2018 Intel Corporation
//
// Specification for this interface is available at:
//
// https://cdrdv2.intel.com/v1/dl/getContent/603354
//

pub const ADXL_REVISION: c_uint = 0x1;
pub const ADXL_IDX_GET_ADDR_PARAMS: c_uint = 0x1;
pub const ADXL_IDX_FORWARD_TRANSLATE: c_uint = 0x2;

//
// The specification doesn't provide a limit on how many
// components are in a memory address. But since we allocate
// memory based on the number the BIOS tells us, we should
// defend against insane values.
//
pub const ADXL_MAX_COMPONENTS: c_int = 500;

    static acpi_handle handle;
    static union acpi_object *params;
    static const guid_t adxl_guid =
    GUID_INIT(0xAA3C050A, 0x7EA4, 0x4C1F,
    0xAF, 0xDA, 0x12, 0x67, 0xDF, 0xD3, 0xD4, 0x8D);
    static int adxl_count;
    static char **adxl_component_names;
    static union acpi_object *adxl_dsm(int cmd, union acpi_object argv[])
    {
    union acpi_object *obj, *o;
    obj = acpi_evaluate_dsm_typed(handle, &adxl_guid, ADXL_REVISION,
    cmd, argv, ACPI_TYPE_PACKAGE);
    if (!obj) {
    pr_info("DSM call failed for cmd=%d\n", cmd);
    return core::ptr::null_mut();
    }
    if (obj.package.count != 2) {
    pr_info("Bad pkg count %d\n", obj.package.count);
    goto err;
    }
    o = obj.package.elements;
    if (o.type != ACPI_TYPE_INTEGER) {
    pr_info("Bad 1st element type %d\n", o.type);
    goto err;
    }
    if (o.integer.value) {
    pr_info("Bad ret val %llu\n", o.integer.value);
    goto err;
    }
    o = obj.package.elements + 1;
    if (o.type != ACPI_TYPE_PACKAGE) {
    pr_info("Bad 2nd element type %d\n", o.type);
    goto err;
    }
    return obj;
    err:
    ACPI_FREE(obj);
    return core::ptr::null_mut();
    }
//
// adxl_get_component_names - get list of memory component names
// Returns NULL terminated list of string names
//
// Give the caller a pointer to the list of memory component names
// e.g. { "SystemAddress", "ProcessorSocketId", "ChannelId", ... NULL }
// Caller should count how many strings in order to allocate a buffer
// for the return from adxl_decode().
//
    const char * const *adxl_get_component_names(void)
    {
    return (const char * const *)adxl_component_names;
    }
    EXPORT_SYMBOL_GPL(adxl_get_component_names);
//
// adxl_decode - ask BIOS to decode a system address to memory address
// @addr: the address to decode
// @component_values: pointer to array of values for each component
// Returns 0 on success, negative error code otherwise
//
// The index of each value returned in the array matches the index of
// each component name returned by adxl_get_component_names().
// Components that are not defined for this address translation (e.g.
// mirror channel number for a non-mirrored address) are set to ~0ull.
//
#[no_mangle]
pub unsafe extern "C" fn adxl_decode(addr: u64, component_values[]: u64) -> c_int {
    int adxl_decode(u64 addr, u64 component_values[])
    {
    union acpi_object argv4[2], *results, *r;
    int i, cnt;
    if (!adxl_component_names)
    return -EOPNOTSUPP;
    argv4[0].type = ACPI_TYPE_PACKAGE;
    argv4[0].package.count = 1;
    argv4[0].package.elements = &argv4[1];
    argv4[1].integer.type = ACPI_TYPE_INTEGER;
    argv4[1].integer.value = addr;
    results = adxl_dsm(ADXL_IDX_FORWARD_TRANSLATE, argv4);
    if (!results)
    return -EINVAL;
    r = results.package.elements + 1;
    cnt = r.package.count;
    if (cnt != adxl_count) {
    ACPI_FREE(results);
    return -EINVAL;
    }
    r = r.package.elements;
    for (i = 0; i < cnt; i++)
    component_values[i] = r[i].integer.value;
    ACPI_FREE(results);
    return 0;
    }
    EXPORT_SYMBOL_GPL(adxl_decode);
#[no_mangle]
unsafe extern "C" fn adxl_init() -> int __init {
    static int __init adxl_init(void)
    {
    char *path = ACPI_ADXL_PATH;
    union acpi_object *p;
    acpi_status status;
    int i;
    status = acpi_get_handle(core::ptr::null_mut(), path, &handle);
    if (ACPI_FAILURE(status)) {
    pr_debug("No ACPI handle for path %s\n", path);
    return -ENODEV;
    }
    if (!acpi_has_method(handle, "_DSM")) {
    pr_info("No DSM method\n");
    return -ENODEV;
    }
    if (!acpi_check_dsm(handle, &adxl_guid, ADXL_REVISION,
    ADXL_IDX_GET_ADDR_PARAMS |
    ADXL_IDX_FORWARD_TRANSLATE)) {
    pr_info("DSM method does not support forward translate\n");
    return -ENODEV;
    }
    params = adxl_dsm(ADXL_IDX_GET_ADDR_PARAMS, core::ptr::null_mut());
    if (!params) {
    pr_info("Failed to get component names\n");
    return -ENODEV;
    }
    p = params.package.elements + 1;
    adxl_count = p.package.count;
    if (adxl_count > ADXL_MAX_COMPONENTS) {
    pr_info("Insane number of address component names %d\n", adxl_count);
    ACPI_FREE(params);
    return -ENODEV;
    }
    p = p.package.elements;
//
// Allocate one extra for NULL termination.
//
    adxl_component_names = kcalloc(adxl_count + 1, sizeof(char *), GFP_KERNEL);
    if (!adxl_component_names) {
    ACPI_FREE(params);
    return -ENOMEM;
    }
    for (i = 0; i < adxl_count; i++)
    adxl_component_names[i] = p[i].string.pointer;
    return 0;
    }
    subsys_initcall(adxl_init);
