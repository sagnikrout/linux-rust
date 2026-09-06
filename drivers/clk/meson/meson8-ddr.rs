//! Automatically rewritten from C to Rust
//! Source: drivers/clk/meson/meson8-ddr.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Amlogic Meson8 DDR clock controller
//
// Copyright (C) 2019 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//

pub const AM_DDR_PLL_CNTL: c_uint = 0x00;
pub const AM_DDR_PLL_CNTL1: c_uint = 0x04;
pub const AM_DDR_PLL_CNTL2: c_uint = 0x08;
pub const AM_DDR_PLL_CNTL3: c_uint = 0x0c;
pub const AM_DDR_PLL_CNTL4: c_uint = 0x10;
pub const AM_DDR_PLL_STS: c_uint = 0x14;
pub const DDR_CLK_CNTL: c_uint = 0x18;
pub const DDR_CLK_STS: c_uint = 0x1c;
    static struct clk_regmap meson8_ddr_pll_dco = {
    .data = &(struct meson_clk_pll_data){
    .en = {
    .reg_off = AM_DDR_PLL_CNTL,
    .shift   = 30,
    .width   = 1,
    },
    .m = {
    .reg_off = AM_DDR_PLL_CNTL,
    .shift   = 0,
    .width   = 9,
    },
    .n = {
    .reg_off = AM_DDR_PLL_CNTL,
    .shift   = 9,
    .width   = 5,
    },
    .l = {
    .reg_off = AM_DDR_PLL_CNTL,
    .shift   = 31,
    .width   = 1,
    },
    .rst = {
    .reg_off = AM_DDR_PLL_CNTL,
    .shift   = 29,
    .width   = 1,
    },
    },
    .hw.init = &(struct clk_init_data){
    .name = "ddr_pll_dco",
    .ops = &meson_clk_pll_ro_ops,
    .parent_data = &(const struct clk_parent_data) {
    .fw_name = "xtal",
    },
    .num_parents = 1,
    },
    };
    static struct clk_regmap meson8_ddr_pll = {
    .data = &(struct clk_regmap_div_data){
    .offset = AM_DDR_PLL_CNTL,
    .shift = 16,
    .width = 2,
    .flags = CLK_DIVIDER_POWER_OF_TWO,
    },
    .hw.init = &(struct clk_init_data){
    .name = "ddr_pll",
    .ops = &clk_regmap_divider_ro_ops,
    .parent_hws = (const struct clk_hw *[]) {
    &meson8_ddr_pll_dco.hw
    },
    .num_parents = 1,
    },
    };
    static struct clk_hw *meson8_ddr_hw_clks[] = {
    [DDR_CLKID_DDR_PLL_DCO]		= &meson8_ddr_pll_dco.hw,
    [DDR_CLKID_DDR_PLL]		= &meson8_ddr_pll.hw,
    };
    static const struct meson_clkc_data meson8_ddr_clkc_data = {
    .hw_clks = {
    .hws = meson8_ddr_hw_clks,
    .num = ARRAY_SIZE(meson8_ddr_hw_clks),
    },
    };
    static const struct of_device_id meson8_ddr_clkc_match_table[] = {
    {
    .compatible = "amlogic,meson8-ddr-clkc",
    .data = &meson8_ddr_clkc_data,
    }, {
    .compatible = "amlogic,meson8b-ddr-clkc",
    .data = &meson8_ddr_clkc_data,
    },
    { /* sentinel */ }
    };
    static struct platform_driver meson8_ddr_clkc_driver = {
    .probe		= meson_clkc_mmio_probe,
    .driver		= {
    .name	= "meson8-ddr-clkc",
    .of_match_table = meson8_ddr_clkc_match_table,
    },
    };
    builtin_platform_driver(meson8_ddr_clkc_driver);
