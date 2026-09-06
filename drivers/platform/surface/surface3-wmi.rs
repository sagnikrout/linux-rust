//! Automatically rewritten from C to Rust
//! Source: drivers/platform/surface/surface3-wmi.c
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
// Driver for the LID cover switch of the Surface 3
//
// Copyright (c) 2016 Red Hat Inc.
//

    MODULE_AUTHOR("Benjamin Tissoires <benjamin.tissoires@redhat.com>");
    MODULE_DESCRIPTION("Surface 3 platform driver");
    MODULE_LICENSE("GPL");

    MODULE_ALIAS("wmi:" SURFACE3_LID_GUID);
    static const struct dmi_system_id surface3_dmi_table[] = {

    {
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Surface 3"),
    },
    },

    { }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface3_wmi {
    pub touchscreen_adev: *mut acpi_device,
    pub pnp0c0d_adev: *mut acpi_device,
    pub hp: acpi_hotplug_context,
    pub input: *mut input_dev,
}

    static struct platform_device *s3_wmi_pdev;
    static struct surface3_wmi s3_wmi;
    static DEFINE_MUTEX(s3_wmi_lock);
#[no_mangle]
unsafe extern "C" fn s3_wmi_query_block(guid: *const c_char, instance: c_int, ret: *mut c_int) -> c_int {
    static int s3_wmi_query_block(const char *guid, int instance, int *ret)
    {
    let mut output: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    union acpi_object *obj = core::ptr::null_mut();
    acpi_status status;
    let mut error: c_int = 0;
    mutex_lock(&s3_wmi_lock);
    status = wmi_query_block(guid, instance, &output);
    if (ACPI_FAILURE(status)) {
    error = -EIO;
    goto out_free_unlock;
    }
    obj = output.pointer;
    if (!obj || obj.type != ACPI_TYPE_INTEGER) {
    if (obj) {
    pr_err("query block returned object type: %d - buffer length:%d\n",
    obj.type,
    obj.type == ACPI_TYPE_BUFFER ?
    obj.buffer.length : 0);
    }
    error = -EINVAL;
    goto out_free_unlock;
    }
// ret = obj->integer.value;
    out_free_unlock:
    kfree(obj);
    mutex_unlock(&s3_wmi_lock);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn s3_wmi_query_lid(ret: *mut c_int) -> c_int {
    static inline int s3_wmi_query_lid(int *ret)
    {
    return s3_wmi_query_block(SURFACE3_LID_GUID, 0, ret);
    }
#[no_mangle]
unsafe extern "C" fn s3_wmi_send_lid_state() -> c_int {
    static int s3_wmi_send_lid_state(void)
    {
    int ret, lid_sw;
    ret = s3_wmi_query_lid(&lid_sw);
    if (ret)
    return ret;
    input_report_switch(s3_wmi.input, SW_LID, lid_sw);
    input_sync(s3_wmi.input);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s3_wmi_hp_notify(adev: *mut acpi_device, value: u32) -> c_int {
    static int s3_wmi_hp_notify(struct acpi_device *adev, u32 value)
    {
    return s3_wmi_send_lid_state();
    }
    static acpi_status s3_wmi_attach_spi_device(acpi_handle handle,
    u32 level,
    void *data,
    void **return_value)
    {
    struct acpi_device *adev = acpi_fetch_acpi_dev(handle);
    struct acpi_device **ts_adev = data;
    if (!adev || strncmp(acpi_device_bid(adev), SPI_TS_OBJ_NAME,
    strlen(SPI_TS_OBJ_NAME)))
    return AE_OK;
    if (*ts_adev) {
    pr_err("duplicate entry %s\n", SPI_TS_OBJ_NAME);
    return AE_OK;
    }
// ts_adev = adev;
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn s3_wmi_check_platform_device(dev: *mut device, data: *mut c_void) -> c_int {
    static int s3_wmi_check_platform_device(struct device *dev, void *data)
    {
    struct acpi_device *adev = ACPI_COMPANION(dev);
    struct acpi_device *ts_adev = core::ptr::null_mut();
    acpi_status status;
// ignore non ACPI devices
    if (!adev)
    return 0;
// check for LID ACPI switch
    if (!strcmp(ACPI_BUTTON_HID_LID, acpi_device_hid(adev))) {
    s3_wmi.pnp0c0d_adev = adev;
    return 0;
    }
// ignore non SPI controllers
    if (strncmp(acpi_device_bid(adev), SPI_CTL_OBJ_NAME,
    strlen(SPI_CTL_OBJ_NAME)))
    return 0;
    status = acpi_walk_namespace(ACPI_TYPE_DEVICE, adev.handle, 1,
    s3_wmi_attach_spi_device, core::ptr::null_mut(),
    &ts_adev, core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    dev_warn(dev, "failed to enumerate SPI slaves\n");
    if (!ts_adev)
    return 0;
    s3_wmi.touchscreen_adev = ts_adev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s3_wmi_create_and_register_input(pdev: *mut platform_device) -> c_int {
    static int s3_wmi_create_and_register_input(struct platform_device *pdev)
    {
    struct input_dev *input;
    int error;
    input = devm_input_allocate_device(&pdev.dev);
    if (!input)
    return -ENOMEM;
    input.name = "Lid Switch";
    input.phys = "button/input0";
    input.id.bustype = BUS_HOST;
    input.id.product = 0x0005;
    input_set_capability(input, EV_SW, SW_LID);
    error = input_register_device(input);
    if (error)
    return error;
    s3_wmi.input = input;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s3_wmi_probe(pdev: *mut platform_device) -> int __init {
    static int __init s3_wmi_probe(struct platform_device *pdev)
    {
    int error;
    if (!dmi_check_system(surface3_dmi_table))
    return -ENODEV;
    memset(&s3_wmi, 0, sizeof(s3_wmi));
    bus_for_each_dev(&platform_bus_type, core::ptr::null_mut(), core::ptr::null_mut(),
    s3_wmi_check_platform_device);
    if (!s3_wmi.touchscreen_adev)
    return -ENODEV;
    acpi_bus_trim(s3_wmi.pnp0c0d_adev);
    error = s3_wmi_create_and_register_input(pdev);
    if (error)
    goto restore_acpi_lid;
    acpi_initialize_hp_context(s3_wmi.touchscreen_adev, &s3_wmi.hp,
    s3_wmi_hp_notify, core::ptr::null_mut());
    s3_wmi_send_lid_state();
    return 0;
    restore_acpi_lid:
    acpi_bus_scan(s3_wmi.pnp0c0d_adev.handle);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn s3_wmi_remove(device: *mut platform_device) {
    static void s3_wmi_remove(struct platform_device *device)
    {
// remove the hotplug context from the acpi device
    s3_wmi.touchscreen_adev.hp = core::ptr::null_mut();
// reinstall the actual PNPC0C0D LID default handle
    acpi_bus_scan(s3_wmi.pnp0c0d_adev.handle);
    }
#[no_mangle]
unsafe extern "C" fn s3_wmi_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused s3_wmi_resume(struct device *dev)
    {
    s3_wmi_send_lid_state();
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(s3_wmi_pm, core::ptr::null_mut(), s3_wmi_resume);
    static struct platform_driver s3_wmi_driver = {
    .driver = {
    .name = "surface3-wmi",
    .pm = &s3_wmi_pm,
    },
    .remove = s3_wmi_remove,
    };
#[no_mangle]
unsafe extern "C" fn s3_wmi_init() -> int __init {
    static int __init s3_wmi_init(void)
    {
    int error;
    s3_wmi_pdev = platform_device_alloc("surface3-wmi", -1);
    if (!s3_wmi_pdev)
    return -ENOMEM;
    error = platform_device_add(s3_wmi_pdev);
    if (error)
    goto err_device_put;
    error = platform_driver_probe(&s3_wmi_driver, s3_wmi_probe);
    if (error)
    goto err_device_del;
    pr_info("Surface 3 WMI Extras loaded\n");
    return 0;
    err_device_del:
    platform_device_del(s3_wmi_pdev);
    err_device_put:
    platform_device_put(s3_wmi_pdev);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn s3_wmi_exit() -> void __exit {
    static void __exit s3_wmi_exit(void)
    {
    platform_device_unregister(s3_wmi_pdev);
    platform_driver_unregister(&s3_wmi_driver);
    }
    module_init(s3_wmi_init);
    module_exit(s3_wmi_exit);
