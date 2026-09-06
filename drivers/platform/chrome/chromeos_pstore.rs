//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/chromeos_pstore.c
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
// Driver to instantiate Chromebook ramoops device.
//
// Copyright (C) 2013 Google, Inc.

    static int ecc_size;
    module_param(ecc_size, int, 0400);
    MODULE_PARM_DESC(ecc_size, "ECC parity data size in bytes. A positive value enables ECC for the ramoops region.");
    static const struct dmi_system_id chromeos_pstore_dmi_table[] __initconst = {
    {
//
// Today all Chromebooks/boxes ship with Google_* as version and
// coreboot as bios vendor. No other systems with this
// combination are known to date.
//
    .matches = {
    DMI_MATCH(DMI_BIOS_VENDOR, "coreboot"),
    DMI_MATCH(DMI_BIOS_VERSION, "Google_"),
    },
    },
    {
// x86-alex, the first Samsung Chromebook.
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "SAMSUNG ELECTRONICS CO., LTD."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Alex"),
    },
    },
    {
// x86-mario, the Cr-48 pilot device from Google.
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "IEC"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Mario"),
    },
    },
    {
// x86-zgb, the first Acer Chromebook.
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "ACER"),
    DMI_MATCH(DMI_PRODUCT_NAME, "ZGB"),
    },
    },
    { }
    };
    MODULE_DEVICE_TABLE(dmi, chromeos_pstore_dmi_table);
//
// On x86 chromebooks/boxes, the firmware will keep the legacy VGA memory
// range untouched across reboots, so we use that to store our pstore
// contents for panic logs, etc.
//
    static struct ramoops_platform_data chromeos_ramoops_data = {
    .mem_size	= 0x100000,
    .mem_address	= 0xf00000,
    .record_size	= 0x40000,
    .console_size	= 0x20000,
    .ftrace_size	= 0x20000,
    .pmsg_size	= 0x20000,
    .max_reason	= KMSG_DUMP_OOPS,
    };
    static struct platform_device chromeos_ramoops = {
    .name = "ramoops",
    .dev = {
    .platform_data = &chromeos_ramoops_data,
    },
    };

    static const struct acpi_device_id cros_ramoops_acpi_match[] = {
    { "GOOG9999", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, cros_ramoops_acpi_match);
    static struct platform_driver chromeos_ramoops_acpi = {
    .driver		= {
    .name	= "chromeos_pstore",
    .acpi_match_table = ACPI_PTR(cros_ramoops_acpi_match),
    },
    };
#[no_mangle]
unsafe extern "C" fn chromeos_probe_acpi(pdev: *mut platform_device) -> int __init {
    static int __init chromeos_probe_acpi(struct platform_device *pdev)
    {
    struct resource *res;
    resource_size_t len;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENOMEM;
    len = resource_size(res);
    if (!res.start || !len)
    return -ENOMEM;
    pr_info("chromeos ramoops using acpi device.\n");
    chromeos_ramoops_data.mem_size = len;
    chromeos_ramoops_data.mem_address = res.start;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chromeos_check_acpi() -> bool __init {
    static bool __init chromeos_check_acpi(void)
    {
    if (!platform_driver_probe(&chromeos_ramoops_acpi, chromeos_probe_acpi))
    return true;
    return false;
    }

    static inline bool chromeos_check_acpi(void) { return false; }

#[no_mangle]
unsafe extern "C" fn chromeos_pstore_init() -> int __init {
    static int __init chromeos_pstore_init(void)
    {
    bool acpi_dev_found;
    if (ecc_size > 0)
    chromeos_ramoops_data.ecc_info.ecc_size = ecc_size;
// First check ACPI for non-hardcoded values from firmware.
    acpi_dev_found = chromeos_check_acpi();
    if (acpi_dev_found || dmi_check_system(chromeos_pstore_dmi_table))
    return platform_device_register(&chromeos_ramoops);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn chromeos_pstore_exit() -> void __exit {
    static void __exit chromeos_pstore_exit(void)
    {
    platform_device_unregister(&chromeos_ramoops);
    }
    module_init(chromeos_pstore_init);
    module_exit(chromeos_pstore_exit);
    MODULE_DESCRIPTION("ChromeOS pstore module");
    MODULE_LICENSE("GPL v2");
