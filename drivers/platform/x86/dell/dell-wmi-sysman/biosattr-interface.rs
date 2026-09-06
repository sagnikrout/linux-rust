//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-wmi-sysman/biosattr-interface.c
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
// Functions corresponding to SET methods under BIOS attributes interface GUID for use
// with dell-wmi-sysman
//
// Copyright (c) 2020 Dell Inc.
//

pub const SETDEFAULTVALUES_METHOD_ID: c_uint = 0x02;
pub const SETBIOSDEFAULTS_METHOD_ID: c_uint = 0x03;
pub const SETATTRIBUTE_METHOD_ID: c_uint = 0x04;
    static int call_biosattributes_interface(struct wmi_device *wdev, u8 *in_args,
    size_t size, int method_id)
    {
    let mut output: acpi_buffer = {ACPI_ALLOCATE_BUFFER, core::ptr::null_mut()};
    struct acpi_buffer input;
    union acpi_object *obj;
    acpi_status status;
    let mut ret: c_int = -EIO;
    input.length =  (acpi_size) size;
    input.pointer = in_args;
    status = wmidev_evaluate_method(wdev, 0, method_id, &input, &output);
    if (ACPI_FAILURE(status))
    return -EIO;
    obj = (union acpi_object *)output.pointer;
    if (obj.type == ACPI_TYPE_INTEGER)
    ret = obj.integer.value;
    if (wmi_priv.pending_changes == 0) {
    wmi_priv.pending_changes = 1;
// let userland know it may need to check reboot pending again
    kobject_uevent(&wmi_priv.class_dev.kobj, KOBJ_CHANGE);
    }
    kfree(output.pointer);
    return map_wmi_error(ret);
    }
//
// set_attribute() - Update an attribute value
// @a_name: The attribute name
// @a_value: The attribute value
//
// Sets an attribute to new value
//
#[no_mangle]
pub unsafe extern "C" fn set_attribute(a_name: *const c_char, a_value: *const c_char) -> c_int {
    int set_attribute(const char *a_name, const char *a_value)
    {
    size_t security_area_size, buffer_size;
    size_t a_name_size, a_value_size;
    u8 *buffer = core::ptr::null_mut(), *start;
    int ret;
    mutex_lock(&wmi_priv.mutex);
    if (!wmi_priv.bios_attr_wdev) {
    ret = -ENODEV;
    goto out;
    }
// build/calculate buffer
    security_area_size = calculate_security_buffer(wmi_priv.current_admin_password);
    a_name_size = calculate_string_buffer(a_name);
    a_value_size = calculate_string_buffer(a_value);
    buffer_size = security_area_size + a_name_size + a_value_size;
    buffer = kzalloc(buffer_size, GFP_KERNEL);
    if (!buffer) {
    ret = -ENOMEM;
    goto out;
    }
// build security area
    populate_security_buffer(buffer, wmi_priv.current_admin_password);
// build variables to set
    start = buffer + security_area_size;
    ret = populate_string_buffer(start, a_name_size, a_name);
    if (ret < 0)
    goto out;
    start += ret;
    ret = populate_string_buffer(start, a_value_size, a_value);
    if (ret < 0)
    goto out;
    ret = call_biosattributes_interface(wmi_priv.bios_attr_wdev,
    buffer, buffer_size,
    SETATTRIBUTE_METHOD_ID);
    if (ret == -EOPNOTSUPP)
    dev_err(&wmi_priv.bios_attr_wdev.dev, "admin password must be configured\n");
#[no_mangle]
pub unsafe extern "C" fn if(-EACCES: ret ==) -> else {
    else if (ret == -EACCES)
    dev_err(&wmi_priv.bios_attr_wdev.dev, "invalid password\n");
    out:
    kfree(buffer);
    mutex_unlock(&wmi_priv.mutex);
    return ret;
    }
//
// set_bios_defaults() - Resets BIOS defaults
// @deftype: the type of BIOS value reset to issue.
//
// Resets BIOS defaults
//
#[no_mangle]
pub unsafe extern "C" fn set_bios_defaults(deftype: u8) -> c_int {
    int set_bios_defaults(u8 deftype)
    {
    size_t security_area_size, buffer_size;
    let mut integer_area_size: usize = sizeof(u8);
    u8 *buffer = core::ptr::null_mut();
    u8 *defaultType;
    int ret;
    mutex_lock(&wmi_priv.mutex);
    if (!wmi_priv.bios_attr_wdev) {
    ret = -ENODEV;
    goto out;
    }
    security_area_size = calculate_security_buffer(wmi_priv.current_admin_password);
    buffer_size = security_area_size + integer_area_size;
    buffer = kzalloc(buffer_size, GFP_KERNEL);
    if (!buffer) {
    ret = -ENOMEM;
    goto out;
    }
// build security area
    populate_security_buffer(buffer, wmi_priv.current_admin_password);
    defaultType = buffer + security_area_size;
// defaultType = deftype;
    ret = call_biosattributes_interface(wmi_priv.bios_attr_wdev, buffer, buffer_size,
    SETBIOSDEFAULTS_METHOD_ID);
    if (ret)
    dev_err(&wmi_priv.bios_attr_wdev.dev, "reset BIOS defaults failed: %d\n", ret);
    kfree(buffer);
    out:
    mutex_unlock(&wmi_priv.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bios_attr_set_interface_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int bios_attr_set_interface_probe(struct wmi_device *wdev, const void *context)
    {
    mutex_lock(&wmi_priv.mutex);
    wmi_priv.bios_attr_wdev = wdev;
    mutex_unlock(&wmi_priv.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bios_attr_set_interface_remove(wdev: *mut wmi_device) {
    static void bios_attr_set_interface_remove(struct wmi_device *wdev)
    {
    mutex_lock(&wmi_priv.mutex);
    wmi_priv.bios_attr_wdev = core::ptr::null_mut();
    mutex_unlock(&wmi_priv.mutex);
    }
    static const struct wmi_device_id bios_attr_set_interface_id_table[] = {
    { .guid_string = DELL_WMI_BIOS_ATTRIBUTES_INTERFACE_GUID },
    { },
    };
    static struct wmi_driver bios_attr_set_interface_driver = {
    .driver = {
    .name = DRIVER_NAME
    },
    .probe = bios_attr_set_interface_probe,
    .remove = bios_attr_set_interface_remove,
    .id_table = bios_attr_set_interface_id_table,
    };
#[no_mangle]
pub unsafe extern "C" fn init_bios_attr_set_interface() -> c_int {
    int init_bios_attr_set_interface(void)
    {
    return wmi_driver_register(&bios_attr_set_interface_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn exit_bios_attr_set_interface() {
    void exit_bios_attr_set_interface(void)
    {
    wmi_driver_unregister(&bios_attr_set_interface_driver);
    }
    MODULE_DEVICE_TABLE(wmi, bios_attr_set_interface_id_table);
