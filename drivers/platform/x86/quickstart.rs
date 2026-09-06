//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/quickstart.c
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
// ACPI Direct App Launch driver
//
// Copyright (C) 2024 Armin Wolf <W_Armin@gmx.de>
// Copyright (C) 2022 Arvid Norlander <lkml@vorapal.se>
// Copyright (C) 2007-2010 Angelo Arrifano <miknix@gmail.com>
//
// Information gathered from disassembled dsdt and from here:
// <https://archive.org/details/microsoft-acpi-dirapplaunch>
//

//
// There will be two events:
// 0x02 - Button was pressed while device was off/sleeping.
// 0x80 - Button was pressed while device was up.
//
pub const QUICKSTART_EVENT_RUNTIME: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quickstart_data {
    pub dev: *mut device,
    pub /: *mut *mut mutex input_lock; / Protects input sequence during notify,
    pub input_device: *mut input_dev,
    pub input_name: [c_char; 32],
    pub phys: [c_char; 32],
    pub id: u32,
}

//
// Knowing what these buttons do require system specific knowledge.
// This could be done by matching on DMI data in a long quirk table.
// However, it is easier to leave it up to user space to figure this out.
//
// Using for example udev hwdb the scancode 0x1 can be remapped suitably.
//
    static const struct key_entry quickstart_keymap[] = {
    { KE_KEY, 0x1, { KEY_UNKNOWN } },
    { KE_END, 0 },
    };
#[no_mangle]
unsafe extern "C" fn button_id_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t button_id_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct quickstart_data *data = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%u\n", data.id);
    }
    static DEVICE_ATTR_RO(button_id);
    static struct attribute *quickstart_attrs[] = {
    &dev_attr_button_id.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(quickstart);
#[no_mangle]
unsafe extern "C" fn quickstart_notify(handle: acpi_handle, event: u32, context: *mut c_void) {
    static void quickstart_notify(acpi_handle handle, u32 event, void *context)
    {
    struct quickstart_data *data = context;
    switch (event) {
    case QUICKSTART_EVENT_RUNTIME:
    mutex_lock(&data.input_lock);
    sparse_keymap_report_event(data.input_device, 0x1, 1, true);
    mutex_unlock(&data.input_lock);
    acpi_bus_generate_netlink_event(DRIVER_NAME, dev_name(data.dev), event, 0);
    break;
    default:
    dev_err(data.dev, FW_INFO "Unexpected ACPI notify event (%u)\n", event);
    break;
    }
    }
//
// The GHID ACPI method is used to indicate the "role" of the button.
// However, all the meanings of these values are vendor defined.
//
// We do however expose this value to user space.
//
#[no_mangle]
unsafe extern "C" fn quickstart_get_ghid(data: *mut quickstart_data) -> c_int {
    static int quickstart_get_ghid(struct quickstart_data *data)
    {
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    let mut handle: acpi_handle = ACPI_HANDLE(data.dev);
    union acpi_object *obj;
    acpi_status status;
    let mut ret: c_int = 0;
//
// This returns a buffer telling the button usage ID,
// and triggers pending notify events (The ones before booting).
//
    status = acpi_evaluate_object_typed(handle, "GHID", core::ptr::null_mut(), &buffer, ACPI_TYPE_BUFFER);
    if (ACPI_FAILURE(status))
    return -EIO;
    obj = buffer.pointer;
    if (!obj)
    return -ENODATA;
//
// Quoting the specification:
// "The GHID method can return a BYTE, WORD, or DWORD.
// The value must be encoded in little-endian byte
// order (least significant byte first)."
//
    switch (obj.buffer.length) {
    case 1:
    data.id = obj.buffer.pointer[0];
    break;
    case 2:
    data.id = get_unaligned_le16(obj.buffer.pointer);
    break;
    case 4:
    data.id = get_unaligned_le32(obj.buffer.pointer);
    break;
    default:
    dev_err(data.dev,
    FW_BUG "GHID method returned buffer of unexpected length %u\n",
    obj.buffer.length);
    ret = -EIO;
    break;
    }
    kfree(obj);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn quickstart_notify_remove(context: *mut c_void) {
    static void quickstart_notify_remove(void *context)
    {
    struct quickstart_data *data = context;
    acpi_handle handle;
    handle = ACPI_HANDLE(data.dev);
    acpi_remove_notify_handler(handle, ACPI_DEVICE_NOTIFY, quickstart_notify);
    }
#[no_mangle]
unsafe extern "C" fn quickstart_probe(pdev: *mut platform_device) -> c_int {
    static int quickstart_probe(struct platform_device *pdev)
    {
    struct quickstart_data *data;
    acpi_handle handle;
    acpi_status status;
    int ret;
    handle = ACPI_HANDLE(&pdev.dev);
    if (!handle)
    return -ENODEV;
    data = devm_kzalloc(&pdev.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.dev = &pdev.dev;
    dev_set_drvdata(&pdev.dev, data);
    ret = devm_mutex_init(&pdev.dev, &data.input_lock);
    if (ret < 0)
    return ret;
//
// We have to initialize the device wakeup before evaluating GHID because
// doing so will notify the device if the button was used to wake the machine
// from S5.
//
    device_init_wakeup(&pdev.dev, true);
    ret = quickstart_get_ghid(data);
    if (ret < 0)
    return ret;
    data.input_device = devm_input_allocate_device(&pdev.dev);
    if (!data.input_device)
    return -ENOMEM;
    ret = sparse_keymap_setup(data.input_device, quickstart_keymap, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    snprintf(data.input_name, sizeof(data.input_name), "Quickstart Button %u", data.id);
    snprintf(data.phys, sizeof(data.phys), DRIVER_NAME "/input%u", data.id);
    data.input_device.name = data.input_name;
    data.input_device.phys = data.phys;
    data.input_device.id.bustype = BUS_HOST;
    ret = input_register_device(data.input_device);
    if (ret < 0)
    return ret;
    status = acpi_install_notify_handler(handle, ACPI_DEVICE_NOTIFY, quickstart_notify, data);
    if (ACPI_FAILURE(status))
    return -EIO;
    return devm_add_action_or_reset(&pdev.dev, quickstart_notify_remove, data);
    }
    static const struct acpi_device_id quickstart_device_ids[] = {
    { "PNP0C32" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, quickstart_device_ids);
    static struct platform_driver quickstart_platform_driver = {
    .driver	= {
    .name = DRIVER_NAME,
    .dev_groups = quickstart_groups,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .acpi_match_table = quickstart_device_ids,
    },
    .probe = quickstart_probe,
    };
    module_platform_driver(quickstart_platform_driver);
    MODULE_AUTHOR("Armin Wolf <W_Armin@gmx.de>");
    MODULE_AUTHOR("Arvid Norlander <lkml@vorpal.se>");
    MODULE_AUTHOR("Angelo Arrifano");
    MODULE_DESCRIPTION("ACPI Direct App Launch driver");
    MODULE_LICENSE("GPL");
