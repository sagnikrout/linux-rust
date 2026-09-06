//! Automatically rewritten from C to Rust
//! Source: drivers/platform/surface/surface_gpe.c
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
// Surface GPE/Lid driver to enable wakeup from suspend via the lid by
// properly configuring the respective GPEs. Required for wakeup via lid on
// newer Intel-based Microsoft Surface devices.
//
// Copyright (C) 2020-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

//
// Note: The GPE numbers for the lid devices found below have been obtained
// from ACPI/the DSDT table, specifically from the GPE handler for the
// lid.
//
    static const struct property_entry lid_device_props_l17[] = {
    PROPERTY_ENTRY_U32("gpe", 0x17),
    {},
    };
    static const struct property_entry lid_device_props_l4B[] = {
    PROPERTY_ENTRY_U32("gpe", 0x4B),
    {},
    };
    static const struct property_entry lid_device_props_l4D[] = {
    PROPERTY_ENTRY_U32("gpe", 0x4D),
    {},
    };
    static const struct property_entry lid_device_props_l4F[] = {
    PROPERTY_ENTRY_U32("gpe", 0x4F),
    {},
    };
    static const struct property_entry lid_device_props_l57[] = {
    PROPERTY_ENTRY_U32("gpe", 0x57),
    {},
    };
//
// Note: When changing this, don't forget to check that the MODULE_ALIAS below
// still fits.
//
    static const struct dmi_system_id dmi_lid_device_table[] = {
    {
    .ident = "Surface Pro 4",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Pro 4"),
    },
    .driver_data = (void *)lid_device_props_l17,
    },
    {
    .ident = "Surface Pro 5",
    .matches = {
//
// We match for SKU here due to generic product name
// "Surface Pro".
//
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "Surface_Pro_1796"),
    },
    .driver_data = (void *)lid_device_props_l4F,
    },
    {
    .ident = "Surface Pro 5 (LTE)",
    .matches = {
//
// We match for SKU here due to generic product name
// "Surface Pro"
//
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "Surface_Pro_1807"),
    },
    .driver_data = (void *)lid_device_props_l4F,
    },
    {
    .ident = "Surface Pro 6",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Pro 6"),
    },
    .driver_data = (void *)lid_device_props_l4F,
    },
    {
    .ident = "Surface Pro 7",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Pro 7"),
    },
    .driver_data = (void *)lid_device_props_l4D,
    },
    {
    .ident = "Surface Pro 8",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Pro 8"),
    },
    .driver_data = (void *)lid_device_props_l4B,
    },
    {
    .ident = "Surface Book 1",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Book"),
    },
    .driver_data = (void *)lid_device_props_l17,
    },
    {
    .ident = "Surface Book 2",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Book 2"),
    },
    .driver_data = (void *)lid_device_props_l17,
    },
    {
    .ident = "Surface Book 3",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Book 3"),
    },
    .driver_data = (void *)lid_device_props_l4D,
    },
    {
    .ident = "Surface Laptop 1",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Laptop"),
    },
    .driver_data = (void *)lid_device_props_l57,
    },
    {
    .ident = "Surface Laptop 2",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Laptop 2"),
    },
    .driver_data = (void *)lid_device_props_l57,
    },
    {
    .ident = "Surface Laptop 3 (Intel 13\")",
    .matches = {
//
// We match for SKU here due to different variants: The
// AMD (15") version does not rely on GPEs.
//
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "Surface_Laptop_3_1867:1868"),
    },
    .driver_data = (void *)lid_device_props_l4D,
    },
    {
    .ident = "Surface Laptop 3 (Intel 15\")",
    .matches = {
//
// We match for SKU here due to different variants: The
// AMD (15") version does not rely on GPEs.
//
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "Surface_Laptop_3_1872"),
    },
    .driver_data = (void *)lid_device_props_l4D,
    },
    {
    .ident = "Surface Laptop 4 (Intel 13\")",
    .matches = {
//
// We match for SKU here due to different variants: The
// AMD (15") version does not rely on GPEs.
//
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_SKU, "Surface_Laptop_4_1950:1951"),
    },
    .driver_data = (void *)lid_device_props_l4B,
    },
    {
    .ident = "Surface Laptop Studio",
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Microsoft Corporation"),
    DMI_EXACT_MATCH(DMI_PRODUCT_NAME, "Surface Laptop Studio"),
    },
    .driver_data = (void *)lid_device_props_l4B,
    },
    { }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface_lid_device {
    pub gpe_number: u32,
}

#[no_mangle]
unsafe extern "C" fn surface_lid_enable_wakeup(dev: *mut device, enable: bool) -> c_int {
    static int surface_lid_enable_wakeup(struct device *dev, bool enable)
    {
    const struct surface_lid_device *lid = dev_get_drvdata(dev);
    let mut action: c_int = enable ? ACPI_GPE_ENABLE : ACPI_GPE_DISABLE;
    acpi_status status;
    status = acpi_set_gpe_wake_mask(core::ptr::null_mut(), lid.gpe_number, action);
    if (ACPI_FAILURE(status)) {
    dev_err(dev, "failed to set GPE wake mask: %s\n",
    acpi_format_exception(status));
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn surface_gpe_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused surface_gpe_suspend(struct device *dev)
    {
    return surface_lid_enable_wakeup(dev, true);
    }
#[no_mangle]
unsafe extern "C" fn surface_gpe_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused surface_gpe_resume(struct device *dev)
    {
    return surface_lid_enable_wakeup(dev, false);
    }
    static SIMPLE_DEV_PM_OPS(surface_gpe_pm, surface_gpe_suspend, surface_gpe_resume);
#[no_mangle]
unsafe extern "C" fn surface_gpe_probe(pdev: *mut platform_device) -> c_int {
    static int surface_gpe_probe(struct platform_device *pdev)
    {
    struct surface_lid_device *lid;
    u32 gpe_number;
    acpi_status status;
    int ret;
    ret = device_property_read_u32(&pdev.dev, "gpe", &gpe_number);
    if (ret) {
    dev_err(&pdev.dev, "failed to read 'gpe' property: %d\n", ret);
    return ret;
    }
    lid = devm_kzalloc(&pdev.dev, sizeof(*lid), GFP_KERNEL);
    if (!lid)
    return -ENOMEM;
    lid.gpe_number = gpe_number;
    platform_set_drvdata(pdev, lid);
    status = acpi_mark_gpe_for_wake(core::ptr::null_mut(), gpe_number);
    if (ACPI_FAILURE(status)) {
    dev_err(&pdev.dev, "failed to mark GPE for wake: %s\n",
    acpi_format_exception(status));
    return -EINVAL;
    }
    status = acpi_enable_gpe(core::ptr::null_mut(), gpe_number);
    if (ACPI_FAILURE(status)) {
    dev_err(&pdev.dev, "failed to enable GPE: %s\n",
    acpi_format_exception(status));
    return -EINVAL;
    }
    ret = surface_lid_enable_wakeup(&pdev.dev, false);
    if (ret)
    acpi_disable_gpe(core::ptr::null_mut(), gpe_number);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn surface_gpe_remove(pdev: *mut platform_device) {
    static void surface_gpe_remove(struct platform_device *pdev)
    {
    struct surface_lid_device *lid = dev_get_drvdata(&pdev.dev);
// restore default behavior without this module
    surface_lid_enable_wakeup(&pdev.dev, false);
    acpi_disable_gpe(core::ptr::null_mut(), lid.gpe_number);
    }
    static struct platform_driver surface_gpe_driver = {
    .probe = surface_gpe_probe,
    .remove = surface_gpe_remove,
    .driver = {
    .name = "surface_gpe",
    .pm = &surface_gpe_pm,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    static struct platform_device *surface_gpe_device;
#[no_mangle]
unsafe extern "C" fn surface_gpe_init() -> int __init {
    static int __init surface_gpe_init(void)
    {
    struct platform_device_info pdevinfo;
    const struct dmi_system_id *match;
    struct platform_device *pdev;
    int status;
    match = dmi_first_match(dmi_lid_device_table);
    if (!match) {
    pr_info("no compatible Microsoft Surface device found, exiting\n");
    return -ENODEV;
    }
    status = platform_driver_register(&surface_gpe_driver);
    if (status)
    return status;
    pdevinfo = (struct platform_device_info){
    .name = "surface_gpe",
    .id = PLATFORM_DEVID_NONE,
    .properties = match.driver_data,
    };
    pdev = platform_device_register_full(&pdevinfo);
    if (IS_ERR(pdev)) {
    platform_driver_unregister(&surface_gpe_driver);
    return PTR_ERR(pdev);
    }
    surface_gpe_device = pdev;
    return 0;
    }
    module_init(surface_gpe_init);
#[no_mangle]
unsafe extern "C" fn surface_gpe_exit() -> void __exit {
    static void __exit surface_gpe_exit(void)
    {
    platform_device_unregister(surface_gpe_device);
    platform_driver_unregister(&surface_gpe_driver);
    }
    module_exit(surface_gpe_exit);
    MODULE_AUTHOR("Maximilian Luz <luzmaximilian@gmail.com>");
    MODULE_DESCRIPTION("Surface GPE/Lid Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("dmi:*:svnMicrosoftCorporation:pnSurface*:*");
