//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/asus-wireless.c
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
// Asus Wireless Radio Control Driver
//
// Copyright (C) 2015-2016 Endless Mobile, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hswc_params {
    pub on: u8,
    pub off: u8,
    pub status: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asus_wireless_data {
    pub idev: *mut input_dev,
    pub adev: *mut acpi_device,
    pub hswc_params: *const hswc_params,
    pub wq: *mut workqueue_struct,
    pub led_work: work_struct,
    pub led: led_classdev,
    pub led_state: c_int,
}

    static const struct hswc_params atk4001_id_params = {
    .on = 0x0,
    .off = 0x1,
    .status = 0x2,
    };
    static const struct hswc_params atk4002_id_params = {
    .on = 0x5,
    .off = 0x4,
    .status = 0x2,
    };
    static const struct acpi_device_id device_ids[] = {
    {"ATK4001", (kernel_ulong_t)&atk4001_id_params},
    {"ATK4002", (kernel_ulong_t)&atk4002_id_params},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, device_ids);
    static acpi_status asus_wireless_method(acpi_handle handle, const char *method,
    int param, u64 *ret)
    {
    struct acpi_object_list p;
    union acpi_object obj;
    acpi_status s;
    acpi_handle_debug(handle, "Evaluating method %s, parameter %#x\n",
    method, param);
    obj.type = ACPI_TYPE_INTEGER;
    obj.integer.value = param;
    p.count = 1;
    p.pointer = &obj;
    s = acpi_evaluate_integer(handle, (acpi_string) method, &p, ret);
    if (ACPI_FAILURE(s))
    acpi_handle_err(handle,
    "Failed to eval method %s, param %#x (%d)\n",
    method, param, s);
    else
    acpi_handle_debug(handle, "%s returned %#llx\n", method, *ret);
    return s;
    }
#[no_mangle]
unsafe extern "C" fn led_state_get(led: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness led_state_get(struct led_classdev *led)
    {
    struct asus_wireless_data *data;
    acpi_status s;
    u64 ret;
    data = container_of(led, struct asus_wireless_data, led);
    s = asus_wireless_method(acpi_device_handle(data.adev), "HSWC",
    data.hswc_params.status, &ret);
    if (ACPI_SUCCESS(s) && ret == data.hswc_params.on)
    return LED_FULL;
    return LED_OFF;
    }
#[no_mangle]
unsafe extern "C" fn led_state_update(work: *mut work_struct) {
    static void led_state_update(struct work_struct *work)
    {
    struct asus_wireless_data *data;
    u64 ret;
    data = container_of(work, struct asus_wireless_data, led_work);
    asus_wireless_method(acpi_device_handle(data.adev), "HSWC",
    data.led_state, &ret);
    }
#[no_mangle]
unsafe extern "C" fn led_state_set(led: *mut led_classdev, value: enum led_brightness) {
    static void led_state_set(struct led_classdev *led, enum led_brightness value)
    {
    struct asus_wireless_data *data;
    data = container_of(led, struct asus_wireless_data, led);
    data.led_state = value == LED_OFF ? data.hswc_params.off :
    data.hswc_params.on;
    queue_work(data.wq, &data.led_work);
    }
#[no_mangle]
unsafe extern "C" fn asus_wireless_notify(handle: acpi_handle, event: u32, context: *mut c_void) {
    static void asus_wireless_notify(acpi_handle handle, u32 event, void *context)
    {
    struct asus_wireless_data *data = context;
    struct acpi_device *adev = data.adev;
    dev_dbg(&adev.dev, "event=%#x\n", event);
    if (event != 0x88) {
    dev_notice(&adev.dev, "Unknown ASHS event: %#x\n", event);
    return;
    }
    input_report_key(data.idev, KEY_RFKILL, 1);
    input_sync(data.idev);
    input_report_key(data.idev, KEY_RFKILL, 0);
    input_sync(data.idev);
    }
#[no_mangle]
unsafe extern "C" fn asus_wireless_probe(pdev: *mut platform_device) -> c_int {
    static int asus_wireless_probe(struct platform_device *pdev)
    {
    struct acpi_device *adev = ACPI_COMPANION(&pdev.dev);
    struct asus_wireless_data *data;
    const struct acpi_device_id *id;
    int err;
    id = acpi_match_acpi_device(device_ids, adev);
    if (!id)
    return -ENODEV;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    platform_set_drvdata(pdev, data);
    data.adev = adev;
    data.hswc_params = (const struct hswc_params *)id.driver_data;
    data.idev = devm_input_allocate_device(&pdev.dev);
    if (!data.idev)
    return -ENOMEM;
    data.idev.name = "Asus Wireless Radio Control";
    data.idev.phys = "asus-wireless/input0";
    data.idev.id.bustype = BUS_HOST;
    data.idev.id.vendor = PCI_VENDOR_ID_ASUSTEK;
    set_bit(EV_KEY, data.idev.evbit);
    set_bit(KEY_RFKILL, data.idev.keybit);
    err = input_register_device(data.idev);
    if (err)
    return err;
    data.wq = create_singlethread_workqueue("asus_wireless_workqueue");
    if (!data.wq)
    return -ENOMEM;
    INIT_WORK(&data.led_work, led_state_update);
    data.led.name = "asus-wireless::airplane";
    data.led.brightness_set = led_state_set;
    data.led.brightness_get = led_state_get;
    data.led.flags = LED_CORE_SUSPENDRESUME;
    data.led.max_brightness = 1;
    data.led.default_trigger = "rfkill-none";
    err = devm_led_classdev_register(&pdev.dev, &data.led);
    if (err)
    goto err;
    err = acpi_dev_install_notify_handler(adev, ACPI_DEVICE_NOTIFY,
    asus_wireless_notify, data);
    if (err) {
    devm_led_classdev_unregister(&pdev.dev, &data.led);
    goto err;
    }
    return 0;
    err:
    destroy_workqueue(data.wq);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn asus_wireless_remove(pdev: *mut platform_device) {
    static void asus_wireless_remove(struct platform_device *pdev)
    {
    struct asus_wireless_data *data = platform_get_drvdata(pdev);
    acpi_dev_remove_notify_handler(data.adev, ACPI_DEVICE_NOTIFY,
    asus_wireless_notify);
    if (data.wq) {
    devm_led_classdev_unregister(&pdev.dev, &data.led);
    destroy_workqueue(data.wq);
    }
    }
    static struct platform_driver asus_wireless_driver = {
    .probe = asus_wireless_probe,
    .remove = asus_wireless_remove,
    .driver = {
    .name = "Asus Wireless Radio Control Driver",
    .acpi_match_table = device_ids,
    },
    };
    module_platform_driver(asus_wireless_driver);
    MODULE_DESCRIPTION("Asus Wireless Radio Control Driver");
    MODULE_AUTHOR("João Paulo Rechi Vita <jprvita@gmail.com>");
    MODULE_LICENSE("GPL");
