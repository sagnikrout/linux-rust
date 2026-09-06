//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/host1x/mipi.c
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
// Copyright (C) 2013 NVIDIA Corporation
// Copyright (C) 2025 Svyatoslav Ryhel <clamor95@gmail.com>
//

// only need to support one provider
    static struct {
    struct device_node *np;
    const struct tegra_mipi_ops *ops;
    } provider;
//
// tegra_mipi_enable - Enable the Tegra MIPI calibration device.
// @device: Handle to the Tegra MIPI calibration device.
//
// This calls the enable sequence for the Tegra MIPI calibration device.
//
// Returns 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn tegra_mipi_enable(device: *mut tegra_mipi_device) -> c_int {
    int tegra_mipi_enable(struct tegra_mipi_device *device)
    {
    if (device.ops.enable)
    return device.ops.enable(device);
    return 0;
    }
    EXPORT_SYMBOL(tegra_mipi_enable);
//
// tegra_mipi_disable - Disable the Tegra MIPI calibration device.
// @device: Handle to the Tegra MIPI calibration device.
//
// This calls the disable sequence for the Tegra MIPI calibration device.
//
// Returns 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn tegra_mipi_disable(device: *mut tegra_mipi_device) -> c_int {
    int tegra_mipi_disable(struct tegra_mipi_device *device)
    {
    if (device.ops.disable)
    return device.ops.disable(device);
    return 0;
    }
    EXPORT_SYMBOL(tegra_mipi_disable);
//
// tegra_mipi_start_calibration - Start the Tegra MIPI calibration sequence.
// @device: Handle to the Tegra MIPI calibration device.
//
// This initiates the calibration of CSI/DSI interfaces via the Tegra MIPI
// calibration device.
//
// Returns 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn tegra_mipi_start_calibration(device: *mut tegra_mipi_device) -> c_int {
    int tegra_mipi_start_calibration(struct tegra_mipi_device *device)
    {
    if (device.ops.start_calibration)
    return device.ops.start_calibration(device);
    return 0;
    }
    EXPORT_SYMBOL(tegra_mipi_start_calibration);
//
// tegra_mipi_finish_calibration - Finish the Tegra MIPI calibration sequence.
// @device: Handle to the Tegra MIPI calibration device.
//
// This completes the calibration of CSI/DSI interfaces via the Tegra MIPI
// calibration device.
//
// Returns 0 on success or a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn tegra_mipi_finish_calibration(device: *mut tegra_mipi_device) -> c_int {
    int tegra_mipi_finish_calibration(struct tegra_mipi_device *device)
    {
    if (device.ops.finish_calibration)
    return device.ops.finish_calibration(device);
    return 0;
    }
    EXPORT_SYMBOL(tegra_mipi_finish_calibration);
//
// tegra_mipi_request - Request a Tegra MIPI calibration device.
// @device: Handle of the device requesting the MIPI calibration function.
// @np: Device node pointer of the device requesting the MIPI calibration
// function.
//
// This function requests a reference to a Tegra MIPI calibration device.
//
// Returns a pointer to the Tegra MIPI calibration device on success,
// or an ERR_PTR-encoded error code on failure.
//
    struct tegra_mipi_device *tegra_mipi_request(struct device *device,
    struct device_node *np)
    {
    struct tegra_mipi_device *mipidev;
    struct of_phandle_args args;
    int err;
    err = of_parse_phandle_with_args(np, "nvidia,mipi-calibrate",
    "#nvidia,mipi-calibrate-cells", 0,
    &args);
    if (err < 0)
    return ERR_PTR(err);
    if (provider.np != args.np) {
    err = -ENODEV;
    goto out;
    }
    mipidev = kzalloc_obj(*mipidev);
    if (!mipidev) {
    err = -ENOMEM;
    goto out;
    }
    mipidev.pdev = of_find_device_by_node(args.np);
    if (!mipidev.pdev) {
    err = -ENODEV;
    goto free;
    }
    of_node_put(args.np);
    mipidev.ops = provider.ops;
    mipidev.pads = args.args[0];
    return mipidev;
    free:
    kfree(mipidev);
    out:
    of_node_put(args.np);
    return ERR_PTR(err);
    }
    EXPORT_SYMBOL(tegra_mipi_request);
//
// tegra_mipi_free - Free a Tegra MIPI calibration device.
// @mipidev: Handle to the Tegra MIPI calibration device.
//
// This function releases a reference to a Tegra MIPI calibration device
// previously requested by tegra_mipi_request().
//
#[no_mangle]
pub unsafe extern "C" fn tegra_mipi_free(mipidev: *mut tegra_mipi_device) {
    void tegra_mipi_free(struct tegra_mipi_device *mipidev)
    {
    platform_device_put(mipidev.pdev);
    kfree(mipidev);
    }
    EXPORT_SYMBOL(tegra_mipi_free);
#[no_mangle]
unsafe extern "C" fn tegra_mipi_remove_provider(data: *mut c_void) {
    static void tegra_mipi_remove_provider(void *data)
    {
    provider.np = core::ptr::null_mut();
    provider.ops = core::ptr::null_mut();
    }
//
// devm_tegra_mipi_add_provider - Managed registration of a Tegra MIPI
// calibration function provider.
// @device: Handle to the device providing the MIPI calibration function.
// @np: Device node pointer of the device providing the MIPI calibration
// function.
// @ops: Operations supported by the MIPI calibration device.
//
// This registers a device that provides MIPI calibration functions.
// For Tegra20 and Tegra30, this is the CSI block, while Tegra114 and
// newer SoC generations have a dedicated hardware block for these
// functions.
//
// Returns 0 on success or a negative error code on failure.
//
    int devm_tegra_mipi_add_provider(struct device *device, struct device_node *np,
    const struct tegra_mipi_ops *ops)
    {
    if (provider.np)
    return -EBUSY;
    provider.np = np;
    provider.ops = ops;
    return devm_add_action_or_reset(device, tegra_mipi_remove_provider, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(devm_tegra_mipi_add_provider);
