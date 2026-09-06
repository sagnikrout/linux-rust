//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/pwr-mlxbf.c
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-3-Clause
//
// Copyright (c) 2022 NVIDIA CORPORATION & AFFILIATES.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwr_mlxbf {
    pub reboot_work: work_struct,
    pub hid: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn pwr_mlxbf_reboot_work(work: *mut work_struct) {
    static void pwr_mlxbf_reboot_work(struct work_struct *work)
    {
    acpi_bus_generate_netlink_event("button/reboot.*", "Reboot Button", 0x80, 1);
    }
#[no_mangle]
unsafe extern "C" fn pwr_mlxbf_irq(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    static irqreturn_t pwr_mlxbf_irq(int irq, void *ptr)
    {
    const char *rst_pwr_hid = "MLNXBF24";
    const char *shutdown_hid = "MLNXBF29";
    struct pwr_mlxbf *priv = ptr;
    if (!strncmp(priv.hid, rst_pwr_hid, 8))
    schedule_work(&priv.reboot_work);
    if (!strncmp(priv.hid, shutdown_hid, 8))
    orderly_poweroff(true);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pwr_mlxbf_probe(pdev: *mut platform_device) -> c_int {
    static int pwr_mlxbf_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct acpi_device *adev;
    struct pwr_mlxbf *priv;
    const char *hid;
    int irq, err;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    adev = ACPI_COMPANION(dev);
    if (!adev)
    return -ENXIO;
    hid = acpi_device_hid(adev);
    priv.hid = hid;
    irq = acpi_dev_gpio_irq_get(ACPI_COMPANION(dev), 0);
    if (irq < 0)
    return dev_err_probe(dev, irq, "Error getting %s irq.\n", priv.hid);
    err = devm_work_autocancel(dev, &priv.reboot_work, pwr_mlxbf_reboot_work);
    if (err)
    return err;
    err = devm_request_irq(dev, irq, pwr_mlxbf_irq, 0, hid, priv);
    return err;
    }
    static const struct acpi_device_id __maybe_unused pwr_mlxbf_acpi_match[] = {
    { "MLNXBF24", 0 },
    { "MLNXBF29", 0 },
    {},
    };
    MODULE_DEVICE_TABLE(acpi, pwr_mlxbf_acpi_match);
    static struct platform_driver pwr_mlxbf_driver = {
    .driver = {
    .name = "pwr_mlxbf",
    .acpi_match_table = pwr_mlxbf_acpi_match,
    },
    .probe    = pwr_mlxbf_probe,
    };
    module_platform_driver(pwr_mlxbf_driver);
    MODULE_DESCRIPTION("Mellanox BlueField power driver");
    MODULE_AUTHOR("Asmaa Mnebhi <asmaa@nvidia.com>");
    MODULE_LICENSE("Dual BSD/GPL");
