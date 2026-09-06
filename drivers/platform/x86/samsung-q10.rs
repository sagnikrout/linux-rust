//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/samsung-q10.c
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
// Driver for Samsung Q10 and related laptops: controls the backlight
//
// Copyright (c) 2011 Frederick van der Wyck <fvanderwyck@gmail.com>
//

pub const SAMSUNGQ10_BL_MAX_INTENSITY: c_int = 7;
    static acpi_handle ec_handle;
    static bool force;
    module_param(force, bool, 0);
    MODULE_PARM_DESC(force,
    "Disable the DMI check and force the driver to be loaded");
#[no_mangle]
unsafe extern "C" fn samsungq10_bl_set_intensity(bd: *mut backlight_device) -> c_int {
    static int samsungq10_bl_set_intensity(struct backlight_device *bd)
    {
    acpi_status status;
    int i;
    for (i = 0; i < SAMSUNGQ10_BL_MAX_INTENSITY; i++) {
    status = acpi_evaluate_object(ec_handle, "_Q63", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    return -EIO;
    }
    for (i = 0; i < bd.props.brightness; i++) {
    status = acpi_evaluate_object(ec_handle, "_Q64", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    return -EIO;
    }
    return 0;
    }
    static const struct backlight_ops samsungq10_bl_ops = {
    .update_status	= samsungq10_bl_set_intensity,
    };
#[no_mangle]
unsafe extern "C" fn samsungq10_probe(pdev: *mut platform_device) -> c_int {
    static int samsungq10_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    struct backlight_device *bd;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_PLATFORM;
    props.max_brightness = SAMSUNGQ10_BL_MAX_INTENSITY;
    bd = backlight_device_register("samsung", &pdev.dev, core::ptr::null_mut(),
    &samsungq10_bl_ops, &props);
    if (IS_ERR(bd))
    return PTR_ERR(bd);
    platform_set_drvdata(pdev, bd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn samsungq10_remove(pdev: *mut platform_device) {
    static void samsungq10_remove(struct platform_device *pdev)
    {
    struct backlight_device *bd = platform_get_drvdata(pdev);
    backlight_device_unregister(bd);
    }
    static struct platform_driver samsungq10_driver = {
    .driver		= {
    .name	= KBUILD_MODNAME,
    },
    .probe		= samsungq10_probe,
    .remove		= samsungq10_remove,
    };
    static struct platform_device *samsungq10_device;
#[no_mangle]
unsafe extern "C" fn dmi_check_callback(id: *const dmi_system_id) -> int __init {
    static int __init dmi_check_callback(const struct dmi_system_id *id)
    {
    printk(KERN_INFO KBUILD_MODNAME ": found model '%s'\n", id.ident);
    return 1;
    }
    static const struct dmi_system_id samsungq10_dmi_table[] __initconst = {
    {
    .ident = "Samsung Q10",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Samsung"),
    DMI_MATCH(DMI_PRODUCT_NAME, "SQ10"),
    },
    .callback = dmi_check_callback,
    },
    {
    .ident = "Samsung Q20",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "SAMSUNG Electronics"),
    DMI_MATCH(DMI_PRODUCT_NAME, "SENS Q20"),
    },
    .callback = dmi_check_callback,
    },
    {
    .ident = "Samsung Q25",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "SAMSUNG Electronics"),
    DMI_MATCH(DMI_PRODUCT_NAME, "NQ25"),
    },
    .callback = dmi_check_callback,
    },
    {
    .ident = "Dell Latitude X200",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Computer Corporation"),
    DMI_MATCH(DMI_PRODUCT_NAME, "X200"),
    },
    .callback = dmi_check_callback,
    },
    { },
    };
    MODULE_DEVICE_TABLE(dmi, samsungq10_dmi_table);
#[no_mangle]
unsafe extern "C" fn samsungq10_init() -> int __init {
    static int __init samsungq10_init(void)
    {
    if (!force && !dmi_check_system(samsungq10_dmi_table))
    return -ENODEV;
    ec_handle = ec_get_handle();
    if (!ec_handle)
    return -ENODEV;
    samsungq10_device = platform_create_bundle(&samsungq10_driver,
    samsungq10_probe,
    core::ptr::null_mut(), 0, core::ptr::null_mut(), 0);
    return PTR_ERR_OR_ZERO(samsungq10_device);
    }
#[no_mangle]
unsafe extern "C" fn samsungq10_exit() -> void __exit {
    static void __exit samsungq10_exit(void)
    {
    platform_device_unregister(samsungq10_device);
    platform_driver_unregister(&samsungq10_driver);
    }
    module_init(samsungq10_init);
    module_exit(samsungq10_exit);
    MODULE_AUTHOR("Frederick van der Wyck <fvanderwyck@gmail.com>");
    MODULE_DESCRIPTION("Samsung Q10 Driver");
    MODULE_LICENSE("GPL");
