//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/mediatek/pinctrl-mt6797.c
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
// Based on pinctrl-mt6765.c
//
// Copyright (C) 2018 MediaTek Inc.
//
// Author: ZH Chen <zh.chen@mediatek.com>
//
// Copyright (C) Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
//

//
// MT6797 have multiple bases to program pin configuration listed as the below:
// gpio:0x10005000, iocfg[l]:0x10002000, iocfg[b]:0x10002400,
// iocfg[r]:0x10002800, iocfg[t]:0x10002C00.
// _i_base could be used to indicate what base the pin should be mapped into.
//
    static const struct mtk_pin_field_calc mt6797_pin_mode_range[] = {
    PIN_FIELD(0, 261, 0x300, 0x10, 0, 4),
    };
    static const struct mtk_pin_field_calc mt6797_pin_dir_range[] = {
    PIN_FIELD(0, 261, 0x0, 0x10, 0, 1),
    };
    static const struct mtk_pin_field_calc mt6797_pin_di_range[] = {
    PIN_FIELD(0, 261, 0x200, 0x10, 0, 1),
    };
    static const struct mtk_pin_field_calc mt6797_pin_do_range[] = {
    PIN_FIELD(0, 261, 0x100, 0x10, 0, 1),
    };
    static const struct mtk_pin_reg_calc mt6797_reg_cals[PINCTRL_PIN_REG_MAX] = {
    [PINCTRL_PIN_REG_MODE] = MTK_RANGE(mt6797_pin_mode_range),
    [PINCTRL_PIN_REG_DIR] = MTK_RANGE(mt6797_pin_dir_range),
    [PINCTRL_PIN_REG_DI] = MTK_RANGE(mt6797_pin_di_range),
    [PINCTRL_PIN_REG_DO] = MTK_RANGE(mt6797_pin_do_range),
    };
    static const char * const mt6797_pinctrl_register_base_names[] = {
    "gpio", "iocfgl", "iocfgb", "iocfgr", "iocfgt",
    };
    static const struct mtk_pin_soc mt6797_data = {
    .reg_cal = mt6797_reg_cals,
    .pins = mtk_pins_mt6797,
    .npins = ARRAY_SIZE(mtk_pins_mt6797),
    .ngrps = ARRAY_SIZE(mtk_pins_mt6797),
    .gpio_m = 0,
    .base_names = mt6797_pinctrl_register_base_names,
    .nbase_names = ARRAY_SIZE(mt6797_pinctrl_register_base_names),
    };
    static const struct of_device_id mt6797_pinctrl_of_match[] = {
    { .compatible = "mediatek,mt6797-pinctrl", .data = &mt6797_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, mt6797_pinctrl_of_match);
    static struct platform_driver mt6797_pinctrl_driver = {
    .driver = {
    .name = "mt6797-pinctrl",
    .of_match_table = mt6797_pinctrl_of_match,
    },
    .probe = mtk_paris_pinctrl_probe,
    };
#[no_mangle]
unsafe extern "C" fn mt6797_pinctrl_init() -> int __init {
    static int __init mt6797_pinctrl_init(void)
    {
    return platform_driver_register(&mt6797_pinctrl_driver);
    }
    arch_initcall(mt6797_pinctrl_init);
    MODULE_DESCRIPTION("MediaTek MT6797 Pinctrl Driver");
    MODULE_LICENSE("GPL v2");
