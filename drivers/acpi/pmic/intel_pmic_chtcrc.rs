//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/pmic/intel_pmic_chtcrc.c
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
// Intel Cherry Trail Crystal Cove PMIC operation region driver
//
// Copyright (C) 2019 Hans de Goede <hdegoede@redhat.com>
//

//
// We have no docs for the CHT Crystal Cove PMIC. The Asus Zenfone-2 kernel
// code has 2 Crystal Cove regulator drivers, one calls the PMIC a "Crystal
// Cove Plus" PMIC and talks about Cherry Trail, so presumably that one
// could be used to get register info for the regulators if we need to
// implement regulator support in the future.
//
// For now the sole purpose of this driver is to make
// intel_soc_pmic_exec_mipi_pmic_seq_element work on devices with a
// CHT Crystal Cove PMIC.
//
    static const struct intel_pmic_opregion_data intel_chtcrc_pmic_opregion_data = {
    .lpat_raw_to_temp = acpi_lpat_raw_to_temp,
    .pmic_i2c_address = 0x6e,
    };
#[no_mangle]
unsafe extern "C" fn intel_chtcrc_pmic_opregion_probe(pdev: *mut platform_device) -> c_int {
    static int intel_chtcrc_pmic_opregion_probe(struct platform_device *pdev)
    {
    struct intel_soc_pmic *pmic = dev_get_drvdata(pdev.dev.parent);
    return intel_pmic_install_opregion_handler(&pdev.dev,
    ACPI_HANDLE(pdev.dev.parent), pmic.regmap,
    &intel_chtcrc_pmic_opregion_data);
    }
    static struct platform_driver intel_chtcrc_pmic_opregion_driver = {
    .probe = intel_chtcrc_pmic_opregion_probe,
    .driver = {
    .name = "cht_crystal_cove_pmic",
    },
    };
    builtin_platform_driver(intel_chtcrc_pmic_opregion_driver);
