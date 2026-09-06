//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ingenic/jz4770-cgu.c
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
// JZ4770 SoC CGU driver
// Copyright 2018, Paul Cercueil <paul@crapouillou.net>
//

//
// CPM registers offset address definition
//
pub const CGU_REG_CPCCR: c_uint = 0x00;
pub const CGU_REG_LCR: c_uint = 0x04;
pub const CGU_REG_CPPCR0: c_uint = 0x10;
pub const CGU_REG_CLKGR0: c_uint = 0x20;
pub const CGU_REG_OPCR: c_uint = 0x24;
pub const CGU_REG_CLKGR1: c_uint = 0x28;
pub const CGU_REG_CPPCR1: c_uint = 0x30;
pub const CGU_REG_USBPCR1: c_uint = 0x48;
pub const CGU_REG_USBCDR: c_uint = 0x50;
pub const CGU_REG_I2SCDR: c_uint = 0x60;
pub const CGU_REG_LPCDR: c_uint = 0x64;
pub const CGU_REG_MSC0CDR: c_uint = 0x68;
pub const CGU_REG_UHCCDR: c_uint = 0x6c;
pub const CGU_REG_SSICDR: c_uint = 0x74;
pub const CGU_REG_CIMCDR: c_uint = 0x7c;
pub const CGU_REG_GPSCDR: c_uint = 0x80;
pub const CGU_REG_PCMCDR: c_uint = 0x84;
pub const CGU_REG_GPUCDR: c_uint = 0x88;
pub const CGU_REG_MSC1CDR: c_uint = 0xA4;
pub const CGU_REG_MSC2CDR: c_uint = 0xA8;
pub const CGU_REG_BCHCDR: c_uint = 0xAC;
// bits within the OPCR register

// bits within the USBPCR1 register

    static struct ingenic_cgu *cgu;
#[no_mangle]
unsafe extern "C" fn jz4770_uhc_phy_enable(hw: *mut clk_hw) -> c_int {
    static int jz4770_uhc_phy_enable(struct clk_hw *hw)
    {
    void __iomem *reg_opcr		= cgu.base + CGU_REG_OPCR;
    void __iomem *reg_usbpcr1	= cgu.base + CGU_REG_USBPCR1;
    writel(readl(reg_opcr) & ~OPCR_SPENDH, reg_opcr);
    writel(readl(reg_usbpcr1) | USBPCR1_UHC_POWER, reg_usbpcr1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4770_uhc_phy_disable(hw: *mut clk_hw) {
    static void jz4770_uhc_phy_disable(struct clk_hw *hw)
    {
    void __iomem *reg_opcr		= cgu.base + CGU_REG_OPCR;
    void __iomem *reg_usbpcr1	= cgu.base + CGU_REG_USBPCR1;
    writel(readl(reg_usbpcr1) & ~USBPCR1_UHC_POWER, reg_usbpcr1);
    writel(readl(reg_opcr) | OPCR_SPENDH, reg_opcr);
    }
#[no_mangle]
unsafe extern "C" fn jz4770_uhc_phy_is_enabled(hw: *mut clk_hw) -> c_int {
    static int jz4770_uhc_phy_is_enabled(struct clk_hw *hw)
    {
    void __iomem *reg_opcr		= cgu.base + CGU_REG_OPCR;
    void __iomem *reg_usbpcr1	= cgu.base + CGU_REG_USBPCR1;
    return !(readl(reg_opcr) & OPCR_SPENDH) &&
    (readl(reg_usbpcr1) & USBPCR1_UHC_POWER);
    }
    static const struct clk_ops jz4770_uhc_phy_ops = {
    .enable = jz4770_uhc_phy_enable,
    .disable = jz4770_uhc_phy_disable,
    .is_enabled = jz4770_uhc_phy_is_enabled,
    };
    static const s8 pll_od_encoding[8] = {
    0x0, 0x1, -1, 0x2, -1, -1, -1, 0x3,
    };
    static const u8 jz4770_cgu_cpccr_div_table[] = {
    1, 2, 3, 4, 6, 8, 12,
    };
    static const struct ingenic_cgu_clk_info jz4770_cgu_clocks[] = {
// External clocks
    [JZ4770_CLK_EXT] = { "ext", CGU_CLK_EXT },
    [JZ4770_CLK_OSC32K] = { "osc32k", CGU_CLK_EXT },
// PLLs
    [JZ4770_CLK_PLL0] = {
    "pll0", CGU_CLK_PLL,
    .parents = { JZ4770_CLK_EXT },
    .pll = {
    .reg = CGU_REG_CPPCR0,
    .rate_multiplier = 1,
    .m_shift = 24,
    .m_bits = 7,
    .m_offset = 1,
    .n_shift = 18,
    .n_bits = 5,
    .n_offset = 1,
    .od_shift = 16,
    .od_bits = 2,
    .od_max = 8,
    .od_encoding = pll_od_encoding,
    .bypass_reg = CGU_REG_CPPCR0,
    .bypass_bit = 9,
    .enable_bit = 8,
    .stable_bit = 10,
    },
    },
    [JZ4770_CLK_PLL1] = {
// TODO: PLL1 can depend on PLL0
    "pll1", CGU_CLK_PLL,
    .parents = { JZ4770_CLK_EXT },
    .pll = {
    .reg = CGU_REG_CPPCR1,
    .rate_multiplier = 1,
    .m_shift = 24,
    .m_bits = 7,
    .m_offset = 1,
    .n_shift = 18,
    .n_bits = 5,
    .n_offset = 1,
    .od_shift = 16,
    .od_bits = 2,
    .od_max = 8,
    .od_encoding = pll_od_encoding,
    .bypass_bit = -1,
    .enable_bit = 7,
    .stable_bit = 6,
    },
    },
// Main clocks
    [JZ4770_CLK_CCLK] = {
    "cclk", CGU_CLK_DIV,
//
// Disabling the CPU clock or any parent clocks will hang the
// system; mark it critical.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { JZ4770_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 0, 1, 4, 22, -1, -1, 0,
    jz4770_cgu_cpccr_div_table,
    },
    },
    [JZ4770_CLK_H0CLK] = {
    "h0clk", CGU_CLK_DIV,
    .parents = { JZ4770_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 4, 1, 4, 22, -1, -1, 0,
    jz4770_cgu_cpccr_div_table,
    },
    },
    [JZ4770_CLK_H1CLK] = {
    "h1clk", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4770_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 24, 1, 4, 22, -1, -1, 0,
    jz4770_cgu_cpccr_div_table,
    },
    .gate = { CGU_REG_CLKGR1, 7 },
    },
    [JZ4770_CLK_H2CLK] = {
    "h2clk", CGU_CLK_DIV,
    .parents = { JZ4770_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 16, 1, 4, 22, -1, -1, 0,
    jz4770_cgu_cpccr_div_table,
    },
    },
    [JZ4770_CLK_C1CLK] = {
    "c1clk", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4770_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 12, 1, 4, 22, -1, -1, 0,
    jz4770_cgu_cpccr_div_table,
    },
    .gate = { CGU_REG_OPCR, 31, true }, // disable CCLK stop on idle
    },
    [JZ4770_CLK_PCLK] = {
    "pclk", CGU_CLK_DIV,
    .parents = { JZ4770_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 8, 1, 4, 22, -1, -1, 0,
    jz4770_cgu_cpccr_div_table,
    },
    },
// Those divided clocks can connect to PLL0 or PLL1
    [JZ4770_CLK_MMC0_MUX] = {
    "mmc0_mux", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_PLL0, JZ4770_CLK_PLL1, },
    .mux = { CGU_REG_MSC0CDR, 30, 1 },
    .div = { CGU_REG_MSC0CDR, 0, 1, 7, -1, -1, 31 },
    .gate = { CGU_REG_MSC0CDR, 31 },
    },
    [JZ4770_CLK_MMC1_MUX] = {
    "mmc1_mux", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_PLL0, JZ4770_CLK_PLL1, },
    .mux = { CGU_REG_MSC1CDR, 30, 1 },
    .div = { CGU_REG_MSC1CDR, 0, 1, 7, -1, -1, 31 },
    .gate = { CGU_REG_MSC1CDR, 31 },
    },
    [JZ4770_CLK_MMC2_MUX] = {
    "mmc2_mux", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_PLL0, JZ4770_CLK_PLL1, },
    .mux = { CGU_REG_MSC2CDR, 30, 1 },
    .div = { CGU_REG_MSC2CDR, 0, 1, 7, -1, -1, 31 },
    .gate = { CGU_REG_MSC2CDR, 31 },
    },
    [JZ4770_CLK_CIM] = {
    "cim", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_PLL0, JZ4770_CLK_PLL1, },
    .mux = { CGU_REG_CIMCDR, 31, 1 },
    .div = { CGU_REG_CIMCDR, 0, 1, 8, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 26 },
    },
    [JZ4770_CLK_UHC] = {
    "uhc", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_PLL0, JZ4770_CLK_PLL1, },
    .mux = { CGU_REG_UHCCDR, 29, 1 },
    .div = { CGU_REG_UHCCDR, 0, 1, 4, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 24 },
    },
    [JZ4770_CLK_GPU] = {
    "gpu", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_PLL0, JZ4770_CLK_PLL1, -1 },
    .mux = { CGU_REG_GPUCDR, 31, 1 },
    .div = { CGU_REG_GPUCDR, 0, 1, 3, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 9 },
    },
    [JZ4770_CLK_BCH] = {
    "bch", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_PLL0, JZ4770_CLK_PLL1, },
    .mux = { CGU_REG_BCHCDR, 31, 1 },
    .div = { CGU_REG_BCHCDR, 0, 1, 3, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 1 },
    },
    [JZ4770_CLK_LPCLK_MUX] = {
    "lpclk", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_PLL0, JZ4770_CLK_PLL1, },
    .mux = { CGU_REG_LPCDR, 29, 1 },
    .div = { CGU_REG_LPCDR, 0, 1, 11, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 28 },
    },
    [JZ4770_CLK_GPS] = {
    "gps", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_PLL0, JZ4770_CLK_PLL1, },
    .mux = { CGU_REG_GPSCDR, 31, 1 },
    .div = { CGU_REG_GPSCDR, 0, 1, 4, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 22 },
    },
// Those divided clocks can connect to EXT, PLL0 or PLL1
    [JZ4770_CLK_SSI_MUX] = {
    "ssi_mux", CGU_CLK_DIV | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_EXT, -1,
    JZ4770_CLK_PLL0, JZ4770_CLK_PLL1 },
    .mux = { CGU_REG_SSICDR, 30, 2 },
    .div = { CGU_REG_SSICDR, 0, 1, 6, -1, -1, -1 },
    },
    [JZ4770_CLK_PCM_MUX] = {
    "pcm_mux", CGU_CLK_DIV | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_EXT, -1,
    JZ4770_CLK_PLL0, JZ4770_CLK_PLL1 },
    .mux = { CGU_REG_PCMCDR, 30, 2 },
    .div = { CGU_REG_PCMCDR, 0, 1, 9, -1, -1, -1 },
    },
    [JZ4770_CLK_I2S] = {
    "i2s", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_EXT, -1,
    JZ4770_CLK_PLL0, JZ4770_CLK_PLL1 },
    .mux = { CGU_REG_I2SCDR, 30, 2 },
    .div = { CGU_REG_I2SCDR, 0, 1, 9, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 13 },
    },
    [JZ4770_CLK_OTG] = {
    "usb", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4770_CLK_EXT, -1,
    JZ4770_CLK_PLL0, JZ4770_CLK_PLL1 },
    .mux = { CGU_REG_USBCDR, 30, 2 },
    .div = { CGU_REG_USBCDR, 0, 1, 8, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 2 },
    },
// Gate-only clocks
    [JZ4770_CLK_SSI0] = {
    "ssi0", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_SSI_MUX, },
    .gate = { CGU_REG_CLKGR0, 4 },
    },
    [JZ4770_CLK_SSI1] = {
    "ssi1", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_SSI_MUX, },
    .gate = { CGU_REG_CLKGR0, 19 },
    },
    [JZ4770_CLK_SSI2] = {
    "ssi2", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_SSI_MUX, },
    .gate = { CGU_REG_CLKGR0, 20 },
    },
    [JZ4770_CLK_PCM0] = {
    "pcm0", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_PCM_MUX, },
    .gate = { CGU_REG_CLKGR1, 8 },
    },
    [JZ4770_CLK_PCM1] = {
    "pcm1", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_PCM_MUX, },
    .gate = { CGU_REG_CLKGR1, 10 },
    },
    [JZ4770_CLK_DMA] = {
    "dma", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_H2CLK, },
    .gate = { CGU_REG_CLKGR0, 21 },
    },
    [JZ4770_CLK_BDMA] = {
    "bdma", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_H2CLK, },
    .gate = { CGU_REG_CLKGR1, 0 },
    },
    [JZ4770_CLK_I2C0] = {
    "i2c0", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 5 },
    },
    [JZ4770_CLK_I2C1] = {
    "i2c1", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 6 },
    },
    [JZ4770_CLK_I2C2] = {
    "i2c2", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_EXT, },
    .gate = { CGU_REG_CLKGR1, 15 },
    },
    [JZ4770_CLK_UART0] = {
    "uart0", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 15 },
    },
    [JZ4770_CLK_UART1] = {
    "uart1", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 16 },
    },
    [JZ4770_CLK_UART2] = {
    "uart2", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 17 },
    },
    [JZ4770_CLK_UART3] = {
    "uart3", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 18 },
    },
    [JZ4770_CLK_IPU] = {
    "ipu", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_H0CLK, },
    .gate = { CGU_REG_CLKGR0, 29 },
    },
    [JZ4770_CLK_ADC] = {
    "adc", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 14 },
    },
    [JZ4770_CLK_AIC] = {
    "aic", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 8 },
    },
    [JZ4770_CLK_AUX] = {
    "aux", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_C1CLK, },
    .gate = { CGU_REG_CLKGR1, 14 },
    },
    [JZ4770_CLK_VPU] = {
    "vpu", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_H1CLK, },
    .gate = { CGU_REG_LCR, 30, false, 150 },
    },
    [JZ4770_CLK_MMC0] = {
    "mmc0", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_MMC0_MUX, },
    .gate = { CGU_REG_CLKGR0, 3 },
    },
    [JZ4770_CLK_MMC1] = {
    "mmc1", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_MMC1_MUX, },
    .gate = { CGU_REG_CLKGR0, 11 },
    },
    [JZ4770_CLK_MMC2] = {
    "mmc2", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_MMC2_MUX, },
    .gate = { CGU_REG_CLKGR0, 12 },
    },
    [JZ4770_CLK_OTG_PHY] = {
    "usb_phy", CGU_CLK_GATE,
    .parents = { JZ4770_CLK_OTG },
    .gate = { CGU_REG_OPCR, 7, true, 50 },
    },
// Custom clocks
    [JZ4770_CLK_UHC_PHY] = {
    "uhc_phy", CGU_CLK_CUSTOM,
    .parents = { JZ4770_CLK_UHC, -1, -1, -1 },
    .custom = { &jz4770_uhc_phy_ops },
    },
    [JZ4770_CLK_EXT512] = {
    "ext/512", CGU_CLK_FIXDIV,
    .parents = { JZ4770_CLK_EXT },
    .fixdiv = { 512 },
    },
    [JZ4770_CLK_RTC] = {
    "rtc", CGU_CLK_MUX,
    .parents = { JZ4770_CLK_EXT512, JZ4770_CLK_OSC32K, },
    .mux = { CGU_REG_OPCR, 2, 1},
    },
    };
#[no_mangle]
unsafe extern "C" fn jz4770_cgu_init(np: *mut device_node) -> void __init {
    static void __init jz4770_cgu_init(struct device_node *np)
    {
    int retval;
    cgu = ingenic_cgu_new(jz4770_cgu_clocks,
    ARRAY_SIZE(jz4770_cgu_clocks), np);
    if (!cgu) {
    pr_err("%s: failed to initialise CGU\n", __func__);
    return;
    }
    retval = ingenic_cgu_register_clocks(cgu);
    if (retval)
    pr_err("%s: failed to register CGU Clocks\n", __func__);
    ingenic_cgu_register_syscore(cgu);
    }
// We only probe via devicetree, no need for a platform driver
    CLK_OF_DECLARE_DRIVER(jz4770_cgu, "ingenic,jz4770-cgu", jz4770_cgu_init);
