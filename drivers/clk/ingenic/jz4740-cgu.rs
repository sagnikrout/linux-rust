//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ingenic/jz4740-cgu.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Ingenic JZ4740 SoC CGU driver
//
// Copyright (c) 2015 Imagination Technologies
// Author: Paul Burton <paul.burton@mips.com>
//

// CGU register offsets
pub const CGU_REG_CPCCR: c_uint = 0x00;
pub const CGU_REG_LCR: c_uint = 0x04;
pub const CGU_REG_CPPCR: c_uint = 0x10;
pub const CGU_REG_CLKGR: c_uint = 0x20;
pub const CGU_REG_SCR: c_uint = 0x24;
pub const CGU_REG_I2SCDR: c_uint = 0x60;
pub const CGU_REG_LPCDR: c_uint = 0x64;
pub const CGU_REG_MSCCDR: c_uint = 0x68;
pub const CGU_REG_UHCCDR: c_uint = 0x6c;
pub const CGU_REG_SSICDR: c_uint = 0x74;
// bits within a PLL control register
pub const PLLCTL_M_SHIFT: c_int = 23;

pub const PLLCTL_N_SHIFT: c_int = 18;

pub const PLLCTL_OD_SHIFT: c_int = 16;

// bits within the LCR register

// bits within the CLKGR register

    static struct ingenic_cgu *cgu;
    static const s8 pll_od_encoding[4] = {
    0x0, 0x1, -1, 0x3,
    };
    static const u8 jz4740_cgu_cpccr_div_table[] = {
    1, 2, 3, 4, 6, 8, 12, 16, 24, 32,
    };
    static const u8 jz4740_cgu_pll_half_div_table[] = {
    2, 1,
    };
    static const struct ingenic_cgu_clk_info jz4740_cgu_clocks[] = {
// External clocks
    [JZ4740_CLK_EXT] = { "ext", CGU_CLK_EXT },
    [JZ4740_CLK_RTC] = { "rtc", CGU_CLK_EXT },
    [JZ4740_CLK_PLL] = {
    "pll", CGU_CLK_PLL,
    .parents = { JZ4740_CLK_EXT, -1, -1, -1 },
    .pll = {
    .reg = CGU_REG_CPPCR,
    .rate_multiplier = 1,
    .m_shift = 23,
    .m_bits = 9,
    .m_offset = 2,
    .n_shift = 18,
    .n_bits = 5,
    .n_offset = 2,
    .od_shift = 16,
    .od_bits = 2,
    .od_max = 4,
    .od_encoding = pll_od_encoding,
    .stable_bit = 10,
    .bypass_reg = CGU_REG_CPPCR,
    .bypass_bit = 9,
    .enable_bit = 8,
    },
    },
// Muxes & dividers
    [JZ4740_CLK_PLL_HALF] = {
    "pll half", CGU_CLK_DIV,
    .parents = { JZ4740_CLK_PLL, -1, -1, -1 },
    .div = {
    CGU_REG_CPCCR, 21, 1, 1, -1, -1, -1, 0,
    jz4740_cgu_pll_half_div_table,
    },
    },
    [JZ4740_CLK_CCLK] = {
    "cclk", CGU_CLK_DIV,
//
// Disabling the CPU clock or any parent clocks will hang the
// system; mark it critical.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { JZ4740_CLK_PLL, -1, -1, -1 },
    .div = {
    CGU_REG_CPCCR, 0, 1, 4, 22, -1, -1, 0,
    jz4740_cgu_cpccr_div_table,
    },
    },
    [JZ4740_CLK_HCLK] = {
    "hclk", CGU_CLK_DIV,
    .parents = { JZ4740_CLK_PLL, -1, -1, -1 },
    .div = {
    CGU_REG_CPCCR, 4, 1, 4, 22, -1, -1, 0,
    jz4740_cgu_cpccr_div_table,
    },
    },
    [JZ4740_CLK_PCLK] = {
    "pclk", CGU_CLK_DIV,
    .parents = { JZ4740_CLK_PLL, -1, -1, -1 },
    .div = {
    CGU_REG_CPCCR, 8, 1, 4, 22, -1, -1, 0,
    jz4740_cgu_cpccr_div_table,
    },
    },
    [JZ4740_CLK_MCLK] = {
    "mclk", CGU_CLK_DIV,
//
// Disabling MCLK or its parents will render DRAM
// inaccessible; mark it critical.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { JZ4740_CLK_PLL, -1, -1, -1 },
    .div = {
    CGU_REG_CPCCR, 12, 1, 4, 22, -1, -1, 0,
    jz4740_cgu_cpccr_div_table,
    },
    },
    [JZ4740_CLK_LCD] = {
    "lcd", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4740_CLK_PLL_HALF, -1, -1, -1 },
    .div = {
    CGU_REG_CPCCR, 16, 1, 5, 22, -1, -1, 0,
    jz4740_cgu_cpccr_div_table,
    },
    .gate = { CGU_REG_CLKGR, 10 },
    },
    [JZ4740_CLK_LCD_PCLK] = {
    "lcd_pclk", CGU_CLK_DIV,
    .parents = { JZ4740_CLK_PLL_HALF, -1, -1, -1 },
    .div = { CGU_REG_LPCDR, 0, 1, 11, -1, -1, -1 },
    },
    [JZ4740_CLK_I2S] = {
    "i2s", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4740_CLK_EXT, JZ4740_CLK_PLL_HALF, -1, -1 },
    .mux = { CGU_REG_CPCCR, 31, 1 },
    .div = { CGU_REG_I2SCDR, 0, 1, 9, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 6 },
    },
    [JZ4740_CLK_SPI] = {
    "spi", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4740_CLK_EXT, JZ4740_CLK_PLL, -1, -1 },
    .mux = { CGU_REG_SSICDR, 31, 1 },
    .div = { CGU_REG_SSICDR, 0, 1, 4, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 4 },
    },
    [JZ4740_CLK_MMC] = {
    "mmc", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4740_CLK_PLL_HALF, -1, -1, -1 },
    .div = { CGU_REG_MSCCDR, 0, 1, 5, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 7 },
    },
    [JZ4740_CLK_UHC] = {
    "uhc", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4740_CLK_PLL_HALF, -1, -1, -1 },
    .div = { CGU_REG_UHCCDR, 0, 1, 4, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 14 },
    },
    [JZ4740_CLK_UDC] = {
    "udc", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4740_CLK_EXT, JZ4740_CLK_PLL_HALF, -1, -1 },
    .mux = { CGU_REG_CPCCR, 29, 1 },
    .div = { CGU_REG_CPCCR, 23, 1, 6, -1, -1, -1 },
    .gate = { CGU_REG_SCR, 6, true },
    },
// Gate-only clocks
    [JZ4740_CLK_UART0] = {
    "uart0", CGU_CLK_GATE,
    .parents = { JZ4740_CLK_EXT, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 0 },
    },
    [JZ4740_CLK_UART1] = {
    "uart1", CGU_CLK_GATE,
    .parents = { JZ4740_CLK_EXT, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 15 },
    },
    [JZ4740_CLK_DMA] = {
    "dma", CGU_CLK_GATE,
    .parents = { JZ4740_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 12 },
    },
    [JZ4740_CLK_IPU] = {
    "ipu", CGU_CLK_GATE,
    .parents = { JZ4740_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 13 },
    },
    [JZ4740_CLK_ADC] = {
    "adc", CGU_CLK_GATE,
    .parents = { JZ4740_CLK_EXT, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 8 },
    },
    [JZ4740_CLK_I2C] = {
    "i2c", CGU_CLK_GATE,
    .parents = { JZ4740_CLK_EXT, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 3 },
    },
    [JZ4740_CLK_AIC] = {
    "aic", CGU_CLK_GATE,
    .parents = { JZ4740_CLK_EXT, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 5 },
    },
    [JZ4740_CLK_TCU] = {
    "tcu", CGU_CLK_GATE,
    .parents = { JZ4740_CLK_EXT, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR, 1 },
    },
    };
#[no_mangle]
unsafe extern "C" fn jz4740_cgu_init(np: *mut device_node) -> void __init {
    static void __init jz4740_cgu_init(struct device_node *np)
    {
    int retval;
    cgu = ingenic_cgu_new(jz4740_cgu_clocks,
    ARRAY_SIZE(jz4740_cgu_clocks), np);
    if (!cgu) {
    pr_err("%s: failed to initialise CGU\n", __func__);
    return;
    }
    retval = ingenic_cgu_register_clocks(cgu);
    if (retval)
    pr_err("%s: failed to register CGU Clocks\n", __func__);
    ingenic_cgu_register_syscore(cgu);
    }
    CLK_OF_DECLARE_DRIVER(jz4740_cgu, "ingenic,jz4740-cgu", jz4740_cgu_init);
