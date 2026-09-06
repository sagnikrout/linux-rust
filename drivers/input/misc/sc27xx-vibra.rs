//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/sc27xx-vibra.c
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
// Copyright (C) 2018 Spreadtrum Communications Inc.
//

pub const SC2730_CUR_DRV_CAL_SEL: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sc27xx_vibra_data {
    pub cur_drv_cal_sel: u32,
    pub slp_pd_en: u32,
    pub ldo_pd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vibra_info {
    pub input_dev: *mut input_dev,
    pub play_work: work_struct,
    pub regmap: *mut regmap,
    pub data: *const sc27xx_vibra_data,
    pub base: u32,
    pub strength: u32,
    pub enabled: bool,
}

    static const struct sc27xx_vibra_data sc2731_data = {
    .cur_drv_cal_sel = CUR_DRV_CAL_SEL,
    .slp_pd_en = SLP_LDOVIBR_PD_EN,
    .ldo_pd = LDO_VIBR_PD,
    };
    static const struct sc27xx_vibra_data sc2730_data = {
    .cur_drv_cal_sel = SC2730_CUR_DRV_CAL_SEL,
    .slp_pd_en = SC2730_SLP_LDOVIBR_PD_EN,
    .ldo_pd = SC2730_LDO_VIBR_PD,
    };
    static const struct sc27xx_vibra_data sc2721_data = {
    .cur_drv_cal_sel = CUR_DRV_CAL_SEL,
    .slp_pd_en = SLP_LDOVIBR_PD_EN,
    .ldo_pd = LDO_VIBR_PD,
    };
#[no_mangle]
unsafe extern "C" fn sc27xx_vibra_set(info: *mut vibra_info, on: bool) {
    static void sc27xx_vibra_set(struct vibra_info *info, bool on)
    {
    const struct sc27xx_vibra_data *data = info.data;
    if (on) {
    regmap_update_bits(info.regmap, info.base, data.ldo_pd, 0);
    regmap_update_bits(info.regmap, info.base,
    data.slp_pd_en, 0);
    info.enabled = true;
    } else {
    regmap_update_bits(info.regmap, info.base, data.ldo_pd,
    data.ldo_pd);
    regmap_update_bits(info.regmap, info.base,
    data.slp_pd_en, data.slp_pd_en);
    info.enabled = false;
    }
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_vibra_hw_init(info: *mut vibra_info) -> c_int {
    static int sc27xx_vibra_hw_init(struct vibra_info *info)
    {
    const struct sc27xx_vibra_data *data = info.data;
    if (!data.cur_drv_cal_sel)
    return 0;
    return regmap_update_bits(info.regmap, info.base,
    data.cur_drv_cal_sel, 0);
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_vibra_play_work(work: *mut work_struct) {
    static void sc27xx_vibra_play_work(struct work_struct *work)
    {
    struct vibra_info *info = container_of(work, struct vibra_info,
    play_work);
    if (info.strength && !info.enabled)
    sc27xx_vibra_set(info, true);
#[no_mangle]
pub unsafe extern "C" fn if(info->enabled: info->strength == 0 &&) -> else {
    else if (info.strength == 0 && info.enabled)
    sc27xx_vibra_set(info, false);
    }
    static int sc27xx_vibra_play(struct input_dev *input, void *data,
    struct ff_effect *effect)
    {
    struct vibra_info *info = input_get_drvdata(input);
    info.strength = effect.u.rumble.weak_magnitude;
    schedule_work(&info.play_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_vibra_close(input: *mut input_dev) {
    static void sc27xx_vibra_close(struct input_dev *input)
    {
    struct vibra_info *info = input_get_drvdata(input);
    cancel_work_sync(&info.play_work);
    if (info.enabled)
    sc27xx_vibra_set(info, false);
    }
#[no_mangle]
unsafe extern "C" fn sc27xx_vibra_probe(pdev: *mut platform_device) -> c_int {
    static int sc27xx_vibra_probe(struct platform_device *pdev)
    {
    struct vibra_info *info;
    const struct sc27xx_vibra_data *data;
    int error;
    data = device_get_match_data(&pdev.dev);
    if (!data) {
    dev_err(&pdev.dev, "no matching driver data found\n");
    return -EINVAL;
    }
    info = devm_kzalloc(&pdev.dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!info.regmap) {
    dev_err(&pdev.dev, "failed to get vibrator regmap.\n");
    return -ENODEV;
    }
    error = device_property_read_u32(&pdev.dev, "reg", &info.base);
    if (error) {
    dev_err(&pdev.dev, "failed to get vibrator base address.\n");
    return error;
    }
    info.input_dev = devm_input_allocate_device(&pdev.dev);
    if (!info.input_dev) {
    dev_err(&pdev.dev, "failed to allocate input device.\n");
    return -ENOMEM;
    }
    info.input_dev.name = "sc27xx:vibrator";
    info.input_dev.id.version = 0;
    info.input_dev.close = sc27xx_vibra_close;
    info.data = data;
    input_set_drvdata(info.input_dev, info);
    input_set_capability(info.input_dev, EV_FF, FF_RUMBLE);
    INIT_WORK(&info.play_work, sc27xx_vibra_play_work);
    info.enabled = false;
    error = sc27xx_vibra_hw_init(info);
    if (error) {
    dev_err(&pdev.dev, "failed to initialize the vibrator.\n");
    return error;
    }
    error = input_ff_create_memless(info.input_dev, core::ptr::null_mut(),
    sc27xx_vibra_play);
    if (error) {
    dev_err(&pdev.dev, "failed to register vibrator to FF.\n");
    return error;
    }
    error = input_register_device(info.input_dev);
    if (error) {
    dev_err(&pdev.dev, "failed to register input device.\n");
    return error;
    }
    return 0;
    }
    static const struct of_device_id sc27xx_vibra_of_match[] = {
    { .compatible = "sprd,sc2721-vibrator", .data = &sc2721_data },
    { .compatible = "sprd,sc2730-vibrator", .data = &sc2730_data },
    { .compatible = "sprd,sc2731-vibrator", .data = &sc2731_data },
    {}
    };
    MODULE_DEVICE_TABLE(of, sc27xx_vibra_of_match);
    static struct platform_driver sc27xx_vibra_driver = {
    .driver = {
    .name = "sc27xx-vibrator",
    .of_match_table = sc27xx_vibra_of_match,
    },
    .probe = sc27xx_vibra_probe,
    };
    module_platform_driver(sc27xx_vibra_driver);
    MODULE_DESCRIPTION("Spreadtrum SC27xx Vibrator Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Xiaotong Lu <xiaotong.lu@spreadtrum.com>");
