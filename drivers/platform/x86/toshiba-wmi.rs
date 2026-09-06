//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/toshiba-wmi.c
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
// toshiba_wmi.c - Toshiba WMI Hotkey Driver
//
// Copyright (C) 2015 Azael Avalos <coproscefalo@gmail.com>
//

    MODULE_AUTHOR("Azael Avalos");
    MODULE_DESCRIPTION("Toshiba WMI Hotkey Driver");
    MODULE_LICENSE("GPL");

    MODULE_ALIAS("wmi:"WMI_EVENT_GUID);
    static struct input_dev *toshiba_wmi_input_dev;
    static const struct key_entry toshiba_wmi_keymap[] __initconst = {
// TODO: Add keymap values once found...
// { KE_KEY, 0x00, { KEY_ } },
    { KE_END, 0 }
    };
#[no_mangle]
unsafe extern "C" fn toshiba_wmi_notify(obj: *mut union acpi_object, context: *mut c_void) {
    static void toshiba_wmi_notify(union acpi_object *obj, void *context)
    {
    if (!obj)
    return;
// TODO: Add proper checks once we have data
    pr_debug("Unknown event received, obj type %x\n", obj.type);
    }
    static const struct dmi_system_id toshiba_wmi_dmi_table[] __initconst = {
    {
    .ident = "Toshiba laptop",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "TOSHIBA"),
    },
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn toshiba_wmi_input_setup() -> int __init {
    static int __init toshiba_wmi_input_setup(void)
    {
    acpi_status status;
    int err;
    toshiba_wmi_input_dev = input_allocate_device();
    if (!toshiba_wmi_input_dev)
    return -ENOMEM;
    toshiba_wmi_input_dev.name = "Toshiba WMI hotkeys";
    toshiba_wmi_input_dev.phys = "wmi/input0";
    toshiba_wmi_input_dev.id.bustype = BUS_HOST;
    err = sparse_keymap_setup(toshiba_wmi_input_dev,
    toshiba_wmi_keymap, core::ptr::null_mut());
    if (err)
    goto err_free_dev;
    status = wmi_install_notify_handler(WMI_EVENT_GUID,
    toshiba_wmi_notify, core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    err = -EIO;
    goto err_free_dev;
    }
    err = input_register_device(toshiba_wmi_input_dev);
    if (err)
    goto err_remove_notifier;
    return 0;
    err_remove_notifier:
    wmi_remove_notify_handler(WMI_EVENT_GUID);
    err_free_dev:
    input_free_device(toshiba_wmi_input_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn toshiba_wmi_input_destroy() {
    static void toshiba_wmi_input_destroy(void)
    {
    wmi_remove_notify_handler(WMI_EVENT_GUID);
    input_unregister_device(toshiba_wmi_input_dev);
    }
#[no_mangle]
unsafe extern "C" fn toshiba_wmi_init() -> int __init {
    static int __init toshiba_wmi_init(void)
    {
    int ret;
    if (!wmi_has_guid(WMI_EVENT_GUID) ||
    !dmi_check_system(toshiba_wmi_dmi_table))
    return -ENODEV;
    ret = toshiba_wmi_input_setup();
    if (ret) {
    pr_err("Failed to setup input device\n");
    return ret;
    }
    pr_info("Toshiba WMI Hotkey Driver\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn toshiba_wmi_exit() -> void __exit {
    static void __exit toshiba_wmi_exit(void)
    {
    if (wmi_has_guid(WMI_EVENT_GUID))
    toshiba_wmi_input_destroy();
    }
    module_init(toshiba_wmi_init);
    module_exit(toshiba_wmi_exit);
