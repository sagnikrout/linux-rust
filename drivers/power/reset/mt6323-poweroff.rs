//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/mt6323-poweroff.c
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
// Power off through MediaTek PMIC
//
// Copyright (C) 2018 MediaTek Inc.
//
// Author: Sean Wang <sean.wang@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6323_pwrc {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub base: u32,
}

#[no_mangle]
unsafe extern "C" fn mt6323_do_pwroff(data: *mut sys_off_data) -> c_int {
    static int mt6323_do_pwroff(struct sys_off_data *data)
    {
    struct mt6323_pwrc *pwrc = data.cb_data;
    unsigned int val;
    int ret;
    regmap_write(pwrc.regmap, pwrc.base + RTC_BBPU, RTC_BBPU_KEY);
    regmap_write(pwrc.regmap, pwrc.base + RTC_WRTGR_MT6323, 1);
    ret = regmap_read_poll_timeout(pwrc.regmap,
    pwrc.base + RTC_BBPU, val,
    !(val & RTC_BBPU_CBUSY),
    MTK_RTC_POLL_DELAY_US,
    MTK_RTC_POLL_TIMEOUT);
    if (ret)
    dev_err(pwrc.dev, "failed to write BBPU: %d\n", ret);
// Wait some time until system down, otherwise, notice with a warn
    mdelay(1000);
    WARN_ONCE(1, "Unable to power off system\n");
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn mt6323_pwrc_probe(pdev: *mut platform_device) -> c_int {
    static int mt6323_pwrc_probe(struct platform_device *pdev)
    {
    struct mt6397_chip *mt6397_chip = dev_get_drvdata(pdev.dev.parent);
    struct mt6323_pwrc *pwrc;
    struct resource *res;
    int ret;
    pwrc = devm_kzalloc(&pdev.dev, sizeof(*pwrc), GFP_KERNEL);
    if (!pwrc)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EINVAL;
    pwrc.base = res.start;
    pwrc.regmap = mt6397_chip.regmap;
    pwrc.dev = &pdev.dev;
    ret = devm_register_sys_off_handler(pwrc.dev,
    SYS_OFF_MODE_POWER_OFF,
    SYS_OFF_PRIO_DEFAULT,
    mt6323_do_pwroff,
    pwrc);
    if (ret)
    return dev_err_probe(pwrc.dev, ret, "failed to register power-off handler\n");
    return 0;
    }
    static const struct of_device_id mt6323_pwrc_dt_match[] = {
    { .compatible = "mediatek,mt6323-pwrc" },
    {},
    };
    MODULE_DEVICE_TABLE(of, mt6323_pwrc_dt_match);
    static struct platform_driver mt6323_pwrc_driver = {
    .probe          = mt6323_pwrc_probe,
    .driver         = {
    .name   = "mt6323-pwrc",
    .of_match_table = mt6323_pwrc_dt_match,
    },
    };
    module_platform_driver(mt6323_pwrc_driver);
    MODULE_DESCRIPTION("Poweroff driver for MT6323 PMIC");
    MODULE_AUTHOR("Sean Wang <sean.wang@mediatek.com>");
