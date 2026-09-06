//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/mediatek/pinctrl-mt6397.c
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
// Copyright (c) 2015 MediaTek Inc.
// Author: Hongzhou.Yang <hongzhou.yang@mediatek.com>
//

pub const MT6397_PIN_REG_BASE: c_uint = 0xc000;
    static const struct mtk_pinctrl_devdata mt6397_pinctrl_data = {
    .pins = mtk_pins_mt6397,
    .npins = ARRAY_SIZE(mtk_pins_mt6397),
    .dir_offset = (MT6397_PIN_REG_BASE + 0x000),
    .ies_offset = MTK_PINCTRL_NOT_SUPPORT,
    .smt_offset = MTK_PINCTRL_NOT_SUPPORT,
    .pullen_offset = (MT6397_PIN_REG_BASE + 0x020),
    .pullsel_offset = (MT6397_PIN_REG_BASE + 0x040),
    .dout_offset = (MT6397_PIN_REG_BASE + 0x080),
    .din_offset = (MT6397_PIN_REG_BASE + 0x0a0),
    .pinmux_offset = (MT6397_PIN_REG_BASE + 0x0c0),
    .type1_start = 41,
    .type1_end = 41,
    .port_shf = 3,
    .port_mask = 0x3,
    .port_align = 2,
    .mode_mask = 0xf,
    .mode_per_reg = 5,
    .mode_shf = 4,
    };
#[no_mangle]
unsafe extern "C" fn mt6397_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int mt6397_pinctrl_probe(struct platform_device *pdev)
    {
    struct mt6397_chip *mt6397;
    mt6397 = dev_get_drvdata(pdev.dev.parent);
    return mtk_pctrl_init(pdev, &mt6397_pinctrl_data, mt6397.regmap);
    }
    static const struct of_device_id mt6397_pctrl_match[] = {
    { .compatible = "mediatek,mt6397-pinctrl", },
    { }
    };
    MODULE_DEVICE_TABLE(of, mt6397_pctrl_match);
    static struct platform_driver mtk_pinctrl_driver = {
    .probe = mt6397_pinctrl_probe,
    .driver = {
    .name = "mediatek-mt6397-pinctrl",
    .of_match_table = mt6397_pctrl_match,
    },
    };
    builtin_platform_driver(mtk_pinctrl_driver);
    MODULE_DESCRIPTION("MediaTek MT6397 Pinctrl Driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("MTK_PINCTRL");
