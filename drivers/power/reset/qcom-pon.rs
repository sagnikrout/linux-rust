//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/qcom-pon.c
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
// Copyright (c) 2017-18 Linaro Limited

pub const PON_SOFT_RB_SPARE: c_uint = 0x8f;
pub const GEN1_REASON_SHIFT: c_int = 2;
pub const GEN2_REASON_SHIFT: c_int = 1;
pub const NO_REASON_SHIFT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pon {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub baseaddr: u32,
    pub reboot_mode: reboot_mode_driver,
    pub reason_shift: c_long,
}

    static int qcom_pon_reboot_mode_write(struct reboot_mode_driver *reboot,
    unsigned int magic)
    {
    struct qcom_pon *pon = container_of
    (reboot, struct qcom_pon, reboot_mode);
    int ret;
    ret = regmap_update_bits(pon.regmap,
    pon.baseaddr + PON_SOFT_RB_SPARE,
    GENMASK(7, pon.reason_shift),
    magic << pon.reason_shift);
    if (ret < 0)
    dev_err(pon.dev, "update reboot mode bits failed\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pon_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_pon_probe(struct platform_device *pdev)
    {
    struct qcom_pon *pon;
    long reason_shift;
    int error;
    pon = devm_kzalloc(&pdev.dev, sizeof(*pon), GFP_KERNEL);
    if (!pon)
    return -ENOMEM;
    pon.dev = &pdev.dev;
    pon.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!pon.regmap) {
    dev_err(&pdev.dev, "failed to locate regmap\n");
    return -ENODEV;
    }
    error = of_property_read_u32(pdev.dev.of_node, "reg",
    &pon.baseaddr);
    if (error)
    return error;
    reason_shift = (long)of_device_get_match_data(&pdev.dev);
    if (reason_shift != NO_REASON_SHIFT) {
    pon.reboot_mode.dev = &pdev.dev;
    pon.reason_shift = reason_shift;
    pon.reboot_mode.write = qcom_pon_reboot_mode_write;
    error = devm_reboot_mode_register(&pdev.dev, &pon.reboot_mode);
    if (error) {
    dev_err(&pdev.dev, "can't register reboot mode\n");
    return error;
    }
    }
    platform_set_drvdata(pdev, pon);
    return devm_of_platform_populate(&pdev.dev);
    }
    static const struct of_device_id qcom_pon_id_table[] = {
    { .compatible = "qcom,pm8916-pon", .data = (void *)GEN1_REASON_SHIFT },
    { .compatible = "qcom,pm8941-pon", .data = (void *)NO_REASON_SHIFT },
    { .compatible = "qcom,pms405-pon", .data = (void *)GEN1_REASON_SHIFT },
    { .compatible = "qcom,pm8998-pon", .data = (void *)GEN2_REASON_SHIFT },
    { .compatible = "qcom,pmk8350-pon", .data = (void *)GEN2_REASON_SHIFT },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_pon_id_table);
    static struct platform_driver qcom_pon_driver = {
    .probe = qcom_pon_probe,
    .driver = {
    .name = "qcom-pon",
    .of_match_table = qcom_pon_id_table,
    },
    };
    module_platform_driver(qcom_pon_driver);
    MODULE_DESCRIPTION("Qualcomm Power On driver");
    MODULE_LICENSE("GPL v2");
