//! Automatically rewritten from C to Rust
//! Source: drivers/clk/qcom/gxclkctl-kaanapali.c
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

    enum {
    DT_BI_TCXO,
    };
    static struct gdsc gx_clkctl_gx_gdsc = {
    .gdscr = 0x4024,
    .en_rest_wait_val = 0x2,
    .en_few_wait_val = 0x2,
    .clk_dis_wait_val = 0xf,
    .pd = {
    .name = "gx_clkctl_gx_gdsc",
    .power_on = gdsc_gx_do_nothing_enable,
    .power_off = gdsc_gx_disable,
    },
    .pwrsts = PWRSTS_OFF_ON,
    .flags = POLL_CFG_GDSCR | RETAIN_FF_ENABLE,
    };
    static struct gdsc *gx_clkctl_gdscs[] = {
    [GX_CLKCTL_GX_GDSC] = &gx_clkctl_gx_gdsc,
    };
    static const struct regmap_config gx_clkctl_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = 0x4038,
    .fast_io = true,
    };
    static const struct qcom_cc_desc gx_clkctl_kaanapali_desc = {
    .config = &gx_clkctl_regmap_config,
    .gdscs = gx_clkctl_gdscs,
    .num_gdscs = ARRAY_SIZE(gx_clkctl_gdscs),
    .use_rpm = true,
    };
    static const struct of_device_id gx_clkctl_kaanapali_match_table[] = {
    { .compatible = "qcom,glymur-gxclkctl" },
    { .compatible = "qcom,kaanapali-gxclkctl" },
    { .compatible = "qcom,milos-gxclkctl" },
    { .compatible = "qcom,sm8750-gxclkctl" },
    { }
    };
    MODULE_DEVICE_TABLE(of, gx_clkctl_kaanapali_match_table);
#[no_mangle]
unsafe extern "C" fn gx_clkctl_kaanapali_probe(pdev: *mut platform_device) -> c_int {
    static int gx_clkctl_kaanapali_probe(struct platform_device *pdev)
    {
    int ret;
    ret = qcom_cc_probe(pdev, &gx_clkctl_kaanapali_desc);
    if (ret)
    return ret;
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
    static struct platform_driver gx_clkctl_kaanapali_driver = {
    .probe = gx_clkctl_kaanapali_probe,
    .driver = {
    .name = "gxclkctl-kaanapali",
    .of_match_table = gx_clkctl_kaanapali_match_table,
    },
    };
    module_platform_driver(gx_clkctl_kaanapali_driver);
    MODULE_DESCRIPTION("QTI GXCLKCTL Kaanapali Driver");
    MODULE_LICENSE("GPL");
