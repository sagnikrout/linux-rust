//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-wmi-sysman/passwordattr-interface.c
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
// Functions corresponding to SET password methods under BIOS attributes interface GUID
//
// Copyright (c) 2020 Dell Inc.
//

#[no_mangle]
unsafe extern "C" fn call_password_interface(wdev: *mut wmi_device, in_args: *mut u8, size: usize) -> c_int {
    static int call_password_interface(struct wmi_device *wdev, u8 *in_args, size_t size)
    {
    let mut output: acpi_buffer = {ACPI_ALLOCATE_BUFFER, core::ptr::null_mut()};
    struct acpi_buffer input;
    union acpi_object *obj;
    acpi_status status;
    let mut ret: c_int = -EIO;
    input.length =  (acpi_size) size;
    input.pointer = in_args;
    status = wmidev_evaluate_method(wdev, 0, 1, &input, &output);
    if (ACPI_FAILURE(status))
    return -EIO;
    obj = (union acpi_object *)output.pointer;
    if (obj.type == ACPI_TYPE_INTEGER)
    ret = obj.integer.value;
    kfree(output.pointer);
// let userland know it may need to check is_password_set again
    kobject_uevent(&wmi_priv.class_dev.kobj, KOBJ_CHANGE);
    return map_wmi_error(ret);
    }
//
// set_new_password() - Sets a system admin password
// @password_type: The type of password to set
// @new: The new password
//
// Sets the password using plaintext interface
//
#[no_mangle]
pub unsafe extern "C" fn set_new_password(password_type: *const c_char, new: *const c_char) -> c_int {
    int set_new_password(const char *password_type, const char *new)
    {
    size_t password_type_size, current_password_size, new_size;
    size_t security_area_size, buffer_size;
    u8 *buffer = core::ptr::null_mut(), *start;
    char *current_password;
    int ret;
    mutex_lock(&wmi_priv.mutex);
    if (!wmi_priv.password_attr_wdev) {
    ret = -ENODEV;
    goto out;
    }
    if (strcmp(password_type, "Admin") == 0) {
    current_password = wmi_priv.current_admin_password;
    } else if (strcmp(password_type, "System") == 0) {
    current_password = wmi_priv.current_system_password;
    } else {
    ret = -EINVAL;
    dev_err(&wmi_priv.password_attr_wdev.dev, "unknown password type %s\n",
    password_type);
    goto out;
    }
// build/calculate buffer
    security_area_size = calculate_security_buffer(wmi_priv.current_admin_password);
    password_type_size = calculate_string_buffer(password_type);
    current_password_size = calculate_string_buffer(current_password);
    new_size = calculate_string_buffer(new);
    buffer_size = security_area_size + password_type_size + current_password_size + new_size;
    buffer = kzalloc(buffer_size, GFP_KERNEL);
    if (!buffer) {
    ret = -ENOMEM;
    goto out;
    }
// build security area
    populate_security_buffer(buffer, wmi_priv.current_admin_password);
// build variables to set
    start = buffer + security_area_size;
    ret = populate_string_buffer(start, password_type_size, password_type);
    if (ret < 0)
    goto out;
    start += ret;
    ret = populate_string_buffer(start, current_password_size, current_password);
    if (ret < 0)
    goto out;
    start += ret;
    ret = populate_string_buffer(start, new_size, new);
    if (ret < 0)
    goto out;
    ret = call_password_interface(wmi_priv.password_attr_wdev, buffer, buffer_size);
// on success copy the new password to current password
    if (!ret)
    strscpy(current_password, new, MAX_BUFF);
// explain to user the detailed failure reason
#[no_mangle]
pub unsafe extern "C" fn if(-EOPNOTSUPP: ret ==) -> else {
    else if (ret == -EOPNOTSUPP)
    dev_err(&wmi_priv.password_attr_wdev.dev, "admin password must be configured\n");
#[no_mangle]
pub unsafe extern "C" fn if(-EACCES: ret ==) -> else {
    else if (ret == -EACCES)
    dev_err(&wmi_priv.password_attr_wdev.dev, "invalid password\n");
    out:
    kfree(buffer);
    mutex_unlock(&wmi_priv.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bios_attr_pass_interface_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int bios_attr_pass_interface_probe(struct wmi_device *wdev, const void *context)
    {
    mutex_lock(&wmi_priv.mutex);
    wmi_priv.password_attr_wdev = wdev;
    mutex_unlock(&wmi_priv.mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bios_attr_pass_interface_remove(wdev: *mut wmi_device) {
    static void bios_attr_pass_interface_remove(struct wmi_device *wdev)
    {
    mutex_lock(&wmi_priv.mutex);
    wmi_priv.password_attr_wdev = core::ptr::null_mut();
    mutex_unlock(&wmi_priv.mutex);
    }
    static const struct wmi_device_id bios_attr_pass_interface_id_table[] = {
    { .guid_string = DELL_WMI_BIOS_PASSWORD_INTERFACE_GUID },
    { },
    };
    static struct wmi_driver bios_attr_pass_interface_driver = {
    .driver = {
    .name = DRIVER_NAME"-password"
    },
    .probe = bios_attr_pass_interface_probe,
    .remove = bios_attr_pass_interface_remove,
    .id_table = bios_attr_pass_interface_id_table,
    };
#[no_mangle]
pub unsafe extern "C" fn init_bios_attr_pass_interface() -> c_int {
    int init_bios_attr_pass_interface(void)
    {
    return wmi_driver_register(&bios_attr_pass_interface_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn exit_bios_attr_pass_interface() {
    void exit_bios_attr_pass_interface(void)
    {
    wmi_driver_unregister(&bios_attr_pass_interface_driver);
    }
    MODULE_DEVICE_TABLE(wmi, bios_attr_pass_interface_id_table);
