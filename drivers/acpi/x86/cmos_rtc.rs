//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/x86/cmos_rtc.c
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
// ACPI support for CMOS RTC Address Space access
//
// Copyright (C) 2013, Intel Corporation
// Authors: Lan Tianyu <tianyu.lan@intel.com>
//

    static const struct acpi_device_id acpi_cmos_rtc_ids[] = {
    { "ACPI000E", 1 }, /* ACPI Time and Alarm Device (TAD) */
    ACPI_CMOS_RTC_IDS
    };
    bool cmos_rtc_platform_device_present;
    static acpi_status acpi_cmos_rtc_space_handler(u32 function,
    acpi_physical_address address,
    u32 bits, u64 *value64,
    void *handler_context,
    void *region_context)
    {
    unsigned int i, bytes = DIV_ROUND_UP(bits, 8);
    u8 *value = (u8 *)value64;
    if (address > 0xff || !value64)
    return AE_BAD_PARAMETER;
    guard(spinlock_irq)(&rtc_lock);
    if (function == ACPI_WRITE) {
    for (i = 0; i < bytes; i++, address++, value++)
    CMOS_WRITE(*value, address);
    return AE_OK;
    }
    if (function == ACPI_READ) {
    for (i = 0; i < bytes; i++, address++, value++)
// value = CMOS_READ(address);
    return AE_OK;
    }
    return AE_BAD_PARAMETER;
    }
#[no_mangle]
unsafe extern "C" fn acpi_install_cmos_rtc_space_handler(handle: acpi_handle) -> c_int {
    static int acpi_install_cmos_rtc_space_handler(acpi_handle handle)
    {
    static bool cmos_rtc_space_handler_present __read_mostly;
    acpi_status status;
    if (cmos_rtc_space_handler_present)
    return 0;
    status = acpi_install_address_space_handler(handle,
    ACPI_ADR_SPACE_CMOS,
    acpi_cmos_rtc_space_handler,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    pr_err("Failed to install CMOS-RTC address space handler\n");
    return -ENODEV;
    }
    cmos_rtc_space_handler_present = true;
    return 1;
    }
    static int acpi_cmos_rtc_attach(struct acpi_device *adev,
    const struct acpi_device_id *id)
    {
    int ret;
    ret = acpi_install_cmos_rtc_space_handler(adev.handle);
    if (ret < 0)
    return ret;
    if (IS_ERR_OR_NULL(acpi_create_platform_device(adev, core::ptr::null_mut()))) {
    pr_err("Failed to create a platform device for %s\n", (char *)id.id);
    return 0;
    } else if (!id.driver_data) {
    cmos_rtc_platform_device_present = true;
    }
    return 1;
    }
    static struct acpi_scan_handler cmos_rtc_handler = {
    .ids = acpi_cmos_rtc_ids,
    .attach = acpi_cmos_rtc_attach,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_cmos_rtc_init() -> void __init {
    void __init acpi_cmos_rtc_init(void)
    {
    acpi_scan_add_handler(&cmos_rtc_handler);
    }
