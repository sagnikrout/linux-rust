//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/hi6421-spmi-pmic.c
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
// Device driver for regulators in HISI PMIC IC
//
// Copyright (c) 2013 Linaro Ltd.
// Copyright (c) 2011 Hisilicon.
// Copyright (c) 2020-2021 Huawei Technologies Co., Ltd.
//

    static const struct mfd_cell hi6421v600_devs[] = {
    { .name = "hi6421v600-irq", },
    { .name = "hi6421v600-regulator", },
    };
    static const struct regmap_config regmap_config = {
    .reg_bits	= 16,
    .val_bits	= BITS_PER_BYTE,
    .max_register	= 0xffff,
    .fast_io	= true
    };
#[no_mangle]
unsafe extern "C" fn hi6421_spmi_pmic_probe(sdev: *mut spmi_device) -> c_int {
    static int hi6421_spmi_pmic_probe(struct spmi_device *sdev)
    {
    struct device *dev = &sdev.dev;
    struct regmap *regmap;
    int ret;
    regmap = devm_regmap_init_spmi_ext(sdev, &regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    dev_set_drvdata(&sdev.dev, regmap);
    ret = devm_mfd_add_devices(&sdev.dev, PLATFORM_DEVID_NONE,
    hi6421v600_devs, ARRAY_SIZE(hi6421v600_devs),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret < 0)
    dev_err(dev, "Failed to add child devices: %d\n", ret);
    return ret;
    }
    static const struct of_device_id pmic_spmi_id_table[] = {
    { .compatible = "hisilicon,hi6421-spmi" },
    { }
    };
    MODULE_DEVICE_TABLE(of, pmic_spmi_id_table);
    static struct spmi_driver hi6421_spmi_pmic_driver = {
    .driver = {
    .name	= "hi6421-spmi-pmic",
    .of_match_table = pmic_spmi_id_table,
    },
    .probe	= hi6421_spmi_pmic_probe,
    };
    module_spmi_driver(hi6421_spmi_pmic_driver);
    MODULE_DESCRIPTION("HiSilicon Hi6421v600 SPMI PMIC driver");
    MODULE_LICENSE("GPL v2");
