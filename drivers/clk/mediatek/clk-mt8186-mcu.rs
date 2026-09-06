//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-mt8186-mcu.c
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

    static const char * const mcu_armpll_ll_parents[] = {
    "clk26m",
    "armpll_ll",
    "mainpll",
    "univpll_d2"
    };
    static const char * const mcu_armpll_bl_parents[] = {
    "clk26m",
    "armpll_bl",
    "mainpll",
    "univpll_d2"
    };
    static const char * const mcu_armpll_bus_parents[] = {
    "clk26m",
    "ccipll",
    "mainpll",
    "univpll_d2"
    };
//
// We only configure the CPU muxes when adjust CPU frequency in MediaTek CPUFreq Driver.
// Other fields like divider always keep the same value. (set once in bootloader)
//
    static struct mtk_composite mcu_muxes[] = {
// CPU_PLLDIV_CFG0
    MUX(CLK_MCU_ARMPLL_LL_SEL, "mcu_armpll_ll_sel", mcu_armpll_ll_parents, 0x2A0, 9, 2),
// CPU_PLLDIV_CFG1
    MUX(CLK_MCU_ARMPLL_BL_SEL, "mcu_armpll_bl_sel", mcu_armpll_bl_parents, 0x2A4, 9, 2),
// BUS_PLLDIV_CFG
    MUX(CLK_MCU_ARMPLL_BUS_SEL, "mcu_armpll_bus_sel", mcu_armpll_bus_parents, 0x2E0, 9, 2),
    };
    static const struct mtk_clk_desc mcu_desc = {
    .composite_clks = mcu_muxes,
    .num_composite_clks = ARRAY_SIZE(mcu_muxes),
    };
    static const struct of_device_id of_match_clk_mt8186_mcu[] = {
    { .compatible = "mediatek,mt8186-mcusys", .data = &mcu_desc },
    { /* sentinel */}
    };
    MODULE_DEVICE_TABLE(of, of_match_clk_mt8186_mcu);
    static struct platform_driver clk_mt8186_mcu_drv = {
    .driver = {
    .name = "clk-mt8186-mcu",
    .of_match_table = of_match_clk_mt8186_mcu,
    },
    .probe = mtk_clk_simple_probe,
    .remove = mtk_clk_simple_remove,
    };
    module_platform_driver(clk_mt8186_mcu_drv);
    MODULE_DESCRIPTION("MediaTek MT8186 mcusys clocks driver");
    MODULE_LICENSE("GPL");
