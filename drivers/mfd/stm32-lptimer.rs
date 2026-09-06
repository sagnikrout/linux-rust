//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/stm32-lptimer.c
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
// STM32 Low-Power Timer parent driver.
// Copyright (C) STMicroelectronics 2017
// Author: Fabrice Gasnier <fabrice.gasnier@st.com>
// Inspired by Benjamin Gaignard's stm32-timers driver
//

pub const STM32_LPTIM_MAX_REGISTER: c_uint = 0x3fc;
    static const struct regmap_config stm32_lptimer_regmap_cfg = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = sizeof(u32),
    .max_register = STM32_LPTIM_MAX_REGISTER,
    };
#[no_mangle]
unsafe extern "C" fn stm32_lptimer_detect_encoder(ddata: *mut stm32_lptimer) -> c_int {
    static int stm32_lptimer_detect_encoder(struct stm32_lptimer *ddata)
    {
    u32 val;
    int ret;
//
// Quadrature encoder mode bit can only be written and read back when
// Low-Power Timer supports it.
//
    ret = regmap_update_bits(ddata.regmap, STM32_LPTIM_CFGR,
    STM32_LPTIM_ENC, STM32_LPTIM_ENC);
    if (ret)
    return ret;
    ret = regmap_read(ddata.regmap, STM32_LPTIM_CFGR, &val);
    if (ret)
    return ret;
    ret = regmap_update_bits(ddata.regmap, STM32_LPTIM_CFGR,
    STM32_LPTIM_ENC, 0);
    if (ret)
    return ret;
    ddata.has_encoder = !!(val & STM32_LPTIM_ENC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_lptimer_detect_hwcfgr(ddata: *mut stm32_lptimer) -> c_int {
    static int stm32_lptimer_detect_hwcfgr(struct stm32_lptimer *ddata)
    {
    u32 val;
    int ret;
    ret = regmap_read(ddata.regmap, STM32_LPTIM_VERR, &ddata.version);
    if (ret)
    return ret;
// Try to guess parameters from HWCFGR: e.g. encoder mode (STM32MP15)
    ret = regmap_read(ddata.regmap, STM32_LPTIM_HWCFGR1, &val);
    if (ret)
    return ret;
// Fallback to legacy init if HWCFGR isn't present
    if (!val)
    return stm32_lptimer_detect_encoder(ddata);
    ddata.has_encoder = FIELD_GET(STM32_LPTIM_HWCFGR1_ENCODER, val);
    ret = regmap_read(ddata.regmap, STM32_LPTIM_HWCFGR2, &val);
    if (ret)
    return ret;
// Number of capture/compare channels
    ddata.num_cc_chans = FIELD_GET(STM32_LPTIM_HWCFGR2_CHAN_NUM, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_lptimer_probe(pdev: *mut platform_device) -> c_int {
    static int stm32_lptimer_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct stm32_lptimer *ddata;
    void __iomem *mmio;
    int ret;
    ddata = devm_kzalloc(dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    mmio = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(mmio))
    return PTR_ERR(mmio);
    ddata.regmap = devm_regmap_init_mmio_clk(dev, "mux", mmio,
    &stm32_lptimer_regmap_cfg);
    if (IS_ERR(ddata.regmap))
    return PTR_ERR(ddata.regmap);
    ddata.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(ddata.clk))
    return PTR_ERR(ddata.clk);
    ret = stm32_lptimer_detect_hwcfgr(ddata);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, ddata);
    return devm_of_platform_populate(&pdev.dev);
    }
    static const struct of_device_id stm32_lptimer_of_match[] = {
    { .compatible = "st,stm32-lptimer", },
    {},
    };
    MODULE_DEVICE_TABLE(of, stm32_lptimer_of_match);
    static struct platform_driver stm32_lptimer_driver = {
    .probe = stm32_lptimer_probe,
    .driver = {
    .name = "stm32-lptimer",
    .of_match_table = stm32_lptimer_of_match,
    },
    };
    module_platform_driver(stm32_lptimer_driver);
    MODULE_AUTHOR("Fabrice Gasnier <fabrice.gasnier@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STM32 Low-Power Timer");
    MODULE_ALIAS("platform:stm32-lptimer");
    MODULE_LICENSE("GPL v2");
