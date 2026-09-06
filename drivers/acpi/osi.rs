//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/osi.c
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
// osi.c - _OSI implementation
//
// Copyright (C) 2016 Intel Corporation
// Author: Lv Zheng <lv.zheng@intel.com>
//
// Uncomment next line to get verbose printout
// #define DEBUG

pub const OSI_STRING_LENGTH_MAX: c_int = 64;
pub const OSI_STRING_ENTRIES_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_osi_entry {
    pub string: [c_char; OSI_STRING_LENGTH_MAX],
    pub enable: bool,
}

    static struct acpi_osi_config {
    u8		default_disabling;
    unsigned int	linux_enable:1;
    unsigned int	linux_dmi:1;
    unsigned int	linux_cmdline:1;
    unsigned int	darwin_enable:1;
    unsigned int	darwin_dmi:1;
    unsigned int	darwin_cmdline:1;
    } osi_config;
    static struct acpi_osi_config osi_config;
    static struct acpi_osi_entry
    osi_setup_entries[OSI_STRING_ENTRIES_MAX] __initdata = {
    {"Module Device", true},
    {"Processor Device", true},
    {"Processor Aggregator Device", true},
    };
#[no_mangle]
unsafe extern "C" fn acpi_osi_handler(interface: acpi_string, supported: u32) -> u32 {
    static u32 acpi_osi_handler(acpi_string interface, u32 supported)
    {
    if (!strcmp("Linux", interface)) {
    pr_notice_once(FW_BUG
    "BIOS _OSI(Linux) query %s%s\n",
    osi_config.linux_enable ? "honored" : "ignored",
    osi_config.linux_cmdline ? " via cmdline" :
    osi_config.linux_dmi ? " via DMI" : "");
    }
    if (!strcmp("Darwin", interface)) {
    pr_notice_once(
    "BIOS _OSI(Darwin) query %s%s\n",
    osi_config.darwin_enable ? "honored" : "ignored",
    osi_config.darwin_cmdline ? " via cmdline" :
    osi_config.darwin_dmi ? " via DMI" : "");
    }
    return supported;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_osi_setup(str: *mut c_char) -> void __init {
    void __init acpi_osi_setup(char *str)
    {
    struct acpi_osi_entry *osi;
    let mut enable: bool = true;
    int i;
    if (!acpi_gbl_create_osi_method)
    return;
    if (str == core::ptr::null_mut() || *str == '\0') {
    pr_info("_OSI method disabled\n");
    acpi_gbl_create_osi_method = FALSE;
    return;
    }
    if (*str == '!') {
    str++;
    if (*str == '\0') {
// Do not override acpi_osi=!*
    if (!osi_config.default_disabling)
    osi_config.default_disabling =
    ACPI_DISABLE_ALL_VENDOR_STRINGS;
    return;
    } else if (*str == '*') {
    osi_config.default_disabling = ACPI_DISABLE_ALL_STRINGS;
    for (i = 0; i < OSI_STRING_ENTRIES_MAX; i++) {
    osi = &osi_setup_entries[i];
    osi.enable = false;
    }
    return;
    } else if (*str == '!') {
    osi_config.default_disabling = 0;
    return;
    }
    enable = false;
    }
    for (i = 0; i < OSI_STRING_ENTRIES_MAX; i++) {
    osi = &osi_setup_entries[i];
    if (!strcmp(osi.string, str)) {
    osi.enable = enable;
    break;
    } else if (osi.string[0] == '\0') {
    osi.enable = enable;
    strscpy(osi.string, str, OSI_STRING_LENGTH_MAX);
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn __acpi_osi_setup_darwin(enable: bool) -> void __init {
    static void __init __acpi_osi_setup_darwin(bool enable)
    {
    osi_config.darwin_enable = !!enable;
    if (enable) {
    acpi_osi_setup("!");
    acpi_osi_setup("Darwin");
    } else {
    acpi_osi_setup("!!");
    acpi_osi_setup("!Darwin");
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_osi_setup_darwin(enable: bool) -> void __init {
    static void __init acpi_osi_setup_darwin(bool enable)
    {
// Override acpi_osi_dmi_blacklisted()
    osi_config.darwin_dmi = 0;
    osi_config.darwin_cmdline = 1;
    __acpi_osi_setup_darwin(enable);
    }
//
// The story of _OSI(Linux)
//
// From pre-history through Linux-2.6.22, Linux responded TRUE upon a BIOS
// OSI(Linux) query.
//
// Unfortunately, reference BIOS writers got wind of this and put
// OSI(Linux) in their example code, quickly exposing this string as
// ill-conceived and opening the door to an un-bounded number of BIOS
// incompatibilities.
//
// For example, OSI(Linux) was used on resume to re-POST a video card on
// one system, because Linux at that time could not do a speedy restore in
// its native driver. But then upon gaining quick native restore
// capability, Linux has no way to tell the BIOS to skip the time-consuming
// POST -- putting Linux at a permanent performance disadvantage. On
// another system, the BIOS writer used OSI(Linux) to infer native OS
// support for IPMI!  On other systems, OSI(Linux) simply got in the way of
// Linux claiming to be compatible with other operating systems, exposing
// BIOS issues such as skipped device initialization.
//
// So "Linux" turned out to be a really poor chose of OSI string, and from
// Linux-2.6.23 onward we respond FALSE.
//
// BIOS writers should NOT query _OSI(Linux) on future systems. Linux will
// complain on the console when it sees it, and return FALSE. To get Linux
// to return TRUE for your system  will require a kernel source update to
// add a DMI entry, or boot with "acpi_osi=Linux"
//
#[no_mangle]
unsafe extern "C" fn __acpi_osi_setup_linux(enable: bool) -> void __init {
    static void __init __acpi_osi_setup_linux(bool enable)
    {
    osi_config.linux_enable = !!enable;
    if (enable)
    acpi_osi_setup("Linux");
    else
    acpi_osi_setup("!Linux");
    }
#[no_mangle]
unsafe extern "C" fn acpi_osi_setup_linux(enable: bool) -> void __init {
    static void __init acpi_osi_setup_linux(bool enable)
    {
// Override acpi_osi_dmi_blacklisted()
    osi_config.linux_dmi = 0;
    osi_config.linux_cmdline = 1;
    __acpi_osi_setup_linux(enable);
    }
//
// Modify the list of "OS Interfaces" reported to BIOS via _OSI
//
// empty string disables _OSI
// string starting with '!' disables that string
// otherwise string is added to list, augmenting built-in strings
//
#[no_mangle]
unsafe extern "C" fn acpi_osi_setup_late() -> void __init {
    static void __init acpi_osi_setup_late(void)
    {
    struct acpi_osi_entry *osi;
    char *str;
    int i;
    acpi_status status;
    if (osi_config.default_disabling) {
    status = acpi_update_interfaces(osi_config.default_disabling);
    if (ACPI_SUCCESS(status))
    pr_info("Disabled all _OSI OS vendors%s\n",
    osi_config.default_disabling ==
    ACPI_DISABLE_ALL_STRINGS ?
    " and feature groups" : "");
    }
    for (i = 0; i < OSI_STRING_ENTRIES_MAX; i++) {
    osi = &osi_setup_entries[i];
    str = osi.string;
    if (*str == '\0')
    break;
    if (osi.enable) {
    status = acpi_install_interface(str);
    if (ACPI_SUCCESS(status))
    pr_info("Added _OSI(%s)\n", str);
    } else {
    status = acpi_remove_interface(str);
    if (ACPI_SUCCESS(status))
    pr_info("Deleted _OSI(%s)\n", str);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn osi_setup(str: *mut c_char) -> int __init {
    static int __init osi_setup(char *str)
    {
    if (str && !strcmp("Linux", str))
    acpi_osi_setup_linux(true);
#[no_mangle]
pub unsafe extern "C" fn if(!strcmp("!Linux": str &&, _arg: str)) -> else {
    else if (str && !strcmp("!Linux", str))
    acpi_osi_setup_linux(false);
#[no_mangle]
pub unsafe extern "C" fn if(!strcmp("Darwin": str &&, _arg: str)) -> else {
    else if (str && !strcmp("Darwin", str))
    acpi_osi_setup_darwin(true);
#[no_mangle]
pub unsafe extern "C" fn if(!strcmp("!Darwin": str &&, _arg: str)) -> else {
    else if (str && !strcmp("!Darwin", str))
    acpi_osi_setup_darwin(false);
    else
    acpi_osi_setup(str);
    return 1;
    }
    __setup("acpi_osi=", osi_setup);
#[no_mangle]
pub unsafe extern "C" fn acpi_osi_is_win8() -> bool {
    bool acpi_osi_is_win8(void)
    {
    return acpi_gbl_osi_data >= ACPI_OSI_WIN_8;
    }
    EXPORT_SYMBOL(acpi_osi_is_win8);
#[no_mangle]
unsafe extern "C" fn acpi_osi_dmi_darwin() -> void __init {
    static void __init acpi_osi_dmi_darwin(void)
    {
    pr_notice("DMI detected to setup _OSI(\"Darwin\"): Apple hardware\n");
    osi_config.darwin_dmi = 1;
    __acpi_osi_setup_darwin(true);
    }
    static void __init acpi_osi_dmi_linux(bool enable,
    const struct dmi_system_id *d)
    {
    pr_notice("DMI detected to setup _OSI(\"Linux\"): %s\n", d.ident);
    osi_config.linux_dmi = 1;
    __acpi_osi_setup_linux(enable);
    }
#[no_mangle]
unsafe extern "C" fn dmi_enable_osi_linux(d: *const dmi_system_id) -> int __init {
    static int __init dmi_enable_osi_linux(const struct dmi_system_id *d)
    {
    acpi_osi_dmi_linux(true, d);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmi_disable_osi_vista(d: *const dmi_system_id) -> int __init {
    static int __init dmi_disable_osi_vista(const struct dmi_system_id *d)
    {
    pr_notice("DMI detected: %s\n", d.ident);
    acpi_osi_setup("!Windows 2006");
    acpi_osi_setup("!Windows 2006 SP1");
    acpi_osi_setup("!Windows 2006 SP2");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmi_disable_osi_win7(d: *const dmi_system_id) -> int __init {
    static int __init dmi_disable_osi_win7(const struct dmi_system_id *d)
    {
    pr_notice("DMI detected: %s\n", d.ident);
    acpi_osi_setup("!Windows 2009");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dmi_disable_osi_win8(d: *const dmi_system_id) -> int __init {
    static int __init dmi_disable_osi_win8(const struct dmi_system_id *d)
    {
    pr_notice("DMI detected: %s\n", d.ident);
    acpi_osi_setup("!Windows 2012");
    return 0;
    }
//
// Linux default _OSI response behavior is determined by this DMI table.
//
// Note that _OSI("Linux")/_OSI("Darwin") determined here can be overridden
// by acpi_osi=!Linux/acpi_osi=!Darwin command line options.
//
    static const struct dmi_system_id acpi_osi_dmi_table[] __initconst = {
    {
    .callback = dmi_disable_osi_vista,
    .ident = "Fujitsu Siemens",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "FUJITSU SIEMENS"),
    DMI_MATCH(DMI_PRODUCT_NAME, "ESPRIMO Mobile V5505"),
    },
    },
    {
//
// There have a NVIF method in MSI GX723 DSDT need call by Nvidia
// driver (e.g. nouveau) when user press brightness hotkey.
// Currently, nouveau driver didn't do the job and it causes there
// have a infinite while loop in DSDT when user press hotkey.
// We add MSI GX723's dmi information to this table for workaround
// this issue.
// Will remove MSI GX723 from the table after nouveau grows support.
//
    .callback = dmi_disable_osi_vista,
    .ident = "MSI GX723",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Micro-Star International"),
    DMI_MATCH(DMI_PRODUCT_NAME, "GX723"),
    },
    },
    {
    .callback = dmi_disable_osi_vista,
    .ident = "Sony VGN-NS10J_S",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Sony Corporation"),
    DMI_MATCH(DMI_PRODUCT_NAME, "VGN-NS10J_S"),
    },
    },
    {
    .callback = dmi_disable_osi_vista,
    .ident = "Sony VGN-SR290J",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Sony Corporation"),
    DMI_MATCH(DMI_PRODUCT_NAME, "VGN-SR290J"),
    },
    },
    {
    .callback = dmi_disable_osi_vista,
    .ident = "VGN-NS50B_L",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Sony Corporation"),
    DMI_MATCH(DMI_PRODUCT_NAME, "VGN-NS50B_L"),
    },
    },
    {
    .callback = dmi_disable_osi_vista,
    .ident = "VGN-SR19XN",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Sony Corporation"),
    DMI_MATCH(DMI_PRODUCT_NAME, "VGN-SR19XN"),
    },
    },
    {
    .callback = dmi_disable_osi_vista,
    .ident = "Toshiba Satellite L355",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "TOSHIBA"),
    DMI_MATCH(DMI_PRODUCT_VERSION, "Satellite L355"),
    },
    },
    {
    .callback = dmi_disable_osi_win7,
    .ident = "ASUS K50IJ",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "ASUSTeK Computer Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "K50IJ"),
    },
    },
    {
    .callback = dmi_disable_osi_vista,
    .ident = "Toshiba P305D",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "TOSHIBA"),
    DMI_MATCH(DMI_PRODUCT_NAME, "Satellite P305D"),
    },
    },
    {
    .callback = dmi_disable_osi_vista,
    .ident = "Toshiba NB100",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "TOSHIBA"),
    DMI_MATCH(DMI_PRODUCT_NAME, "NB100"),
    },
    },
//
// The screen backlight turns off during udev device creation
// when returning true for _OSI("Windows 2009")
//
    {
    .callback = dmi_disable_osi_win7,
    .ident = "Acer Aspire One D255",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Acer"),
    DMI_MATCH(DMI_PRODUCT_NAME, "AOD255"),
    },
    },
//
// The wireless hotkey does not work on those machines when
// returning true for _OSI("Windows 2012")
//
    {
    .callback = dmi_disable_osi_win8,
    .ident = "Dell Inspiron 7737",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Inspiron 7737"),
    },
    },
    {
    .callback = dmi_disable_osi_win8,
    .ident = "Dell Inspiron 7537",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Inspiron 7537"),
    },
    },
    {
    .callback = dmi_disable_osi_win8,
    .ident = "Dell Inspiron 5437",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Inspiron 5437"),
    },
    },
    {
    .callback = dmi_disable_osi_win8,
    .ident = "Dell Inspiron 3437",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Inspiron 3437"),
    },
    },
    {
    .callback = dmi_disable_osi_win8,
    .ident = "Dell Vostro 3446",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Vostro 3446"),
    },
    },
    {
    .callback = dmi_disable_osi_win8,
    .ident = "Dell Vostro 3546",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Dell Inc."),
    DMI_MATCH(DMI_PRODUCT_NAME, "Vostro 3546"),
    },
    },
//
// BIOS invocation of _OSI(Linux) is almost always a BIOS bug.
// Linux ignores it, except for the machines enumerated below.
//
// Without this EEEpc exports a non working WMI interface, with
// this it exports a working "good old" eeepc_laptop interface,
// fixing both brightness control, and rfkill not working.
//
    {
    .callback = dmi_enable_osi_linux,
    .ident = "Asus EEE PC 1015PX",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "ASUSTeK Computer INC."),
    DMI_MATCH(DMI_PRODUCT_NAME, "1015PX"),
    },
    },
    {}
    };
#[no_mangle]
unsafe extern "C" fn acpi_osi_dmi_blacklisted() -> __init void {
    static __init void acpi_osi_dmi_blacklisted(void)
    {
    dmi_check_system(acpi_osi_dmi_table);
// Enable _OSI("Darwin") for Apple platforms.
    if (x86_apple_machine)
    acpi_osi_dmi_darwin();
    }
#[no_mangle]
pub unsafe extern "C" fn early_acpi_osi_init() -> int __init {
    int __init early_acpi_osi_init(void)
    {
    acpi_osi_dmi_blacklisted();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_osi_init() -> int __init {
    int __init acpi_osi_init(void)
    {
    acpi_install_interface_handler(acpi_osi_handler);
    acpi_osi_setup_late();
    return 0;
    }
