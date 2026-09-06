//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/chromeos_privacy_screen.c
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
//
// ChromeOS Privacy Screen support
//
// Copyright (C) 2022 Google LLC
//
// This is the Chromeos privacy screen provider, present on certain chromebooks,
// represented by a GOOG0010 device in the ACPI. This ACPI device, if present,
// will cause the i915 drm driver to probe defer until this driver registers
// the privacy-screen.
//

//
// The DSM (Device Specific Method) constants below are the agreed API with
// the firmware team, on how to control privacy screen using ACPI methods.
//

    static const guid_t chromeos_privacy_screen_dsm_guid =
    GUID_INIT(0xc7033113, 0x8720, 0x4ceb,
    0x90, 0x90, 0x9d, 0x52, 0xb3, 0xe5, 0x2d, 0x73);
    static void
    chromeos_privacy_screen_get_hw_state(struct drm_privacy_screen
// drm_privacy_screen)
    {
    union acpi_object *obj;
    struct device *privacy_screen =
    drm_privacy_screen_get_drvdata(drm_privacy_screen);
    let mut handle: acpi_handle = ACPI_HANDLE(privacy_screen);
    obj = acpi_evaluate_dsm(handle, &chromeos_privacy_screen_dsm_guid,
    PRIV_SCRN_DSM_REVID,
    PRIV_SCRN_DSM_FN_GET_STATUS, core::ptr::null_mut());
    if (!obj) {
    dev_err(privacy_screen,
    "_DSM failed to get privacy-screen state\n");
    return;
    }
    if (obj.type != ACPI_TYPE_INTEGER)
    dev_err(privacy_screen,
    "Bad _DSM to get privacy-screen state\n");
#[no_mangle]
pub unsafe extern "C" fn if(1: obj->integer.value ==) -> else {
    else if (obj.integer.value == 1)
    drm_privacy_screen.hw_state = drm_privacy_screen.sw_state =
    PRIVACY_SCREEN_ENABLED;
    else
    drm_privacy_screen.hw_state = drm_privacy_screen.sw_state =
    PRIVACY_SCREEN_DISABLED;
    ACPI_FREE(obj);
    }
    static int
    chromeos_privacy_screen_set_sw_state(struct drm_privacy_screen
// drm_privacy_screen,
    enum drm_privacy_screen_status state)
    {
    union acpi_object *obj = core::ptr::null_mut();
    struct device *privacy_screen =
    drm_privacy_screen_get_drvdata(drm_privacy_screen);
    let mut handle: acpi_handle = ACPI_HANDLE(privacy_screen);
    if (state == PRIVACY_SCREEN_DISABLED) {
    obj = acpi_evaluate_dsm(handle,
    &chromeos_privacy_screen_dsm_guid,
    PRIV_SCRN_DSM_REVID,
    PRIV_SCRN_DSM_FN_DISABLE, core::ptr::null_mut());
    } else if (state == PRIVACY_SCREEN_ENABLED) {
    obj = acpi_evaluate_dsm(handle,
    &chromeos_privacy_screen_dsm_guid,
    PRIV_SCRN_DSM_REVID,
    PRIV_SCRN_DSM_FN_ENABLE, core::ptr::null_mut());
    } else {
    dev_err(privacy_screen,
    "Bad attempt to set privacy-screen status to %u\n",
    state);
    return -EINVAL;
    }
    if (!obj) {
    dev_err(privacy_screen,
    "_DSM failed to set privacy-screen state\n");
    return -EIO;
    }
    drm_privacy_screen.hw_state = drm_privacy_screen.sw_state = state;
    ACPI_FREE(obj);
    return 0;
    }
    static const struct drm_privacy_screen_ops chromeos_privacy_screen_ops = {
    .get_hw_state = chromeos_privacy_screen_get_hw_state,
    .set_sw_state = chromeos_privacy_screen_set_sw_state,
    };
#[no_mangle]
unsafe extern "C" fn chromeos_privacy_screen_probe(pdev: *mut platform_device) -> c_int {
    static int chromeos_privacy_screen_probe(struct platform_device *pdev)
    {
    if (!ACPI_COMPANION(&pdev.dev))
    return -ENODEV;
    struct drm_privacy_screen *drm_privacy_screen =
    drm_privacy_screen_register(&pdev.dev,
    &chromeos_privacy_screen_ops,
    &pdev.dev);
    if (IS_ERR(drm_privacy_screen)) {
    dev_err(&pdev.dev, "Error registering privacy-screen\n");
    return PTR_ERR(drm_privacy_screen);
    }
    platform_set_drvdata(pdev, drm_privacy_screen);
    dev_info(&pdev.dev, "registered privacy-screen '%s'\n",
    dev_name(&drm_privacy_screen.dev));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chromeos_privacy_screen_remove(pdev: *mut platform_device) {
    static void chromeos_privacy_screen_remove(struct platform_device *pdev)
    {
    drm_privacy_screen_unregister(platform_get_drvdata(pdev));
    }
    static const struct acpi_device_id chromeos_privacy_screen_device_ids[] = {
    {"GOOG0010", 0}, /* Google's electronic privacy screen for eDP-1 */
    {}
    };
    MODULE_DEVICE_TABLE(acpi, chromeos_privacy_screen_device_ids);
    static struct platform_driver chromeos_privacy_screen_driver = {
    .probe = chromeos_privacy_screen_probe,
    .remove = chromeos_privacy_screen_remove,
    .driver = {
    .name = "chromeos_privacy_screen_driver",
    .acpi_match_table = chromeos_privacy_screen_device_ids,
    },
    };
    module_platform_driver(chromeos_privacy_screen_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("ChromeOS ACPI Privacy Screen driver");
    MODULE_AUTHOR("Rajat Jain <rajatja@google.com>");
