//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_privacy_screen_x86.c
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


// SPDX-License-Identifier: MIT
//
// Copyright (C) 2020 Red Hat, Inc.
//
// Authors:
// Hans de Goede <hdegoede@redhat.com>
//

    static struct drm_privacy_screen_lookup arch_lookup;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_init_data {
    pub lookup: drm_privacy_screen_lookup,
    pub (*detect)(void): *mut bool,
}

    static acpi_status __init acpi_set_handle(acpi_handle handle, u32 level,
    void *context, void **return_value)
    {
// (acpi_handle *)return_value = handle;
    return AE_CTRL_TERMINATE;
    }
#[no_mangle]
unsafe extern "C" fn detect_thinkpad_privacy_screen() -> bool __init {
    static bool __init detect_thinkpad_privacy_screen(void)
    {
    let mut obj: union acpi_object = { .type = ACPI_TYPE_INTEGER };
    let mut args: acpi_object_list = { .count = 1, .pointer = &obj, };
    let mut ec_handle: acpi_handle = core::ptr::null_mut();
    unsigned long long output;
    acpi_status status;
    if (acpi_disabled)
    return false;
// Get embedded-controller handle
    status = acpi_get_devices("PNP0C09", acpi_set_handle, core::ptr::null_mut(), &ec_handle);
    if (ACPI_FAILURE(status) || !ec_handle)
    return false;
// And call the privacy-screen get-status method
    status = acpi_evaluate_integer(ec_handle, "HKEY.GSSS", &args, &output);
    if (ACPI_FAILURE(status))
    return false;
    return (output & 0x10000) ? true : false;
    }

#[no_mangle]
unsafe extern "C" fn detect_chromeos_privacy_screen() -> bool __init {
    static bool __init detect_chromeos_privacy_screen(void)
    {
    return acpi_dev_present("GOOG0010", core::ptr::null_mut(), -1);
    }

    static const struct arch_init_data arch_init_data[] __initconst = {

    {
    .lookup = {
    .dev_id = core::ptr::null_mut(),
    .con_id = core::ptr::null_mut(),
    .provider = "privacy_screen-thinkpad_acpi",
    },
    .detect = detect_thinkpad_privacy_screen,
    },

    {
    .lookup = {
    .dev_id = core::ptr::null_mut(),
    .con_id = core::ptr::null_mut(),
    .provider = "privacy_screen-GOOG0010:00",
    },
    .detect = detect_chromeos_privacy_screen,
    },

    };
#[no_mangle]
pub unsafe extern "C" fn drm_privacy_screen_lookup_init() -> void __init {
    void __init drm_privacy_screen_lookup_init(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(arch_init_data); i++) {
    if (!arch_init_data[i].detect())
    continue;
    pr_info("Found '%s' privacy-screen provider\n",
    arch_init_data[i].lookup.provider);
// Make a copy because arch_init_data is __initconst
    arch_lookup = arch_init_data[i].lookup;
    drm_privacy_screen_lookup_add(&arch_lookup);
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn drm_privacy_screen_lookup_exit() {
    void drm_privacy_screen_lookup_exit(void)
    {
    if (arch_lookup.provider)
    drm_privacy_screen_lookup_remove(&arch_lookup);
    }
