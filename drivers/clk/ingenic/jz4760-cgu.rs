//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ingenic/jz4760-cgu.c
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
// JZ4760 SoC CGU driver
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
pub const CGU_REG_USBPCR: c_uint = 0x3c;
pub const CGU_REG_USBCDR: c_uint = 0x50;
pub const CGU_REG_I2SCDR: c_uint = 0x60;
pub const CGU_REG_LPCDR: c_uint = 0x64;
pub const CGU_REG_MSCCDR: c_uint = 0x68;
pub const CGU_REG_UHCCDR: c_uint = 0x6c;
pub const CGU_REG_SSICDR: c_uint = 0x74;
pub const CGU_REG_CIMCDR: c_uint = 0x7c;
pub const CGU_REG_GPSCDR: c_uint = 0x80;
pub const CGU_REG_PCMCDR: c_uint = 0x84;
pub const CGU_REG_GPUCDR: c_uint = 0x88;
    static const s8 pll_od_encoding[8] = {
    0x0, 0x1, -1, 0x2, -1, -1, -1, 0x3,
    };
    static const u8 jz4760_cgu_cpccr_div_table[] = {
    1, 2, 3, 4, 6, 8,
    };
    static const u8 jz4760_cgu_pll_half_div_table[] = {
    2, 1,
    };
    static void
    jz4760_cgu_calc_m_n_od(const struct ingenic_cgu_pll_info *pll_info,
    unsigned long rate, unsigned long parent_rate,
    unsigned int *pm, unsigned int *pn, unsigned int *pod)
    {
    unsigned int m, n, od, m_max = (1 << pll_info.m_bits) - 1;
// The frequency after the N divider must be between 1 and 50 MHz.
    n = parent_rate / (1 * MHZ);
// The N divider must be >= 2.
    n = clamp_val(n, 2, 1 << pll_info.n_bits);
    rate /= MHZ;
    parent_rate /= MHZ;
    for (m = m_max; m >= m_max && n >= 2; n--) {
    m = rate * n / parent_rate;
    od = m & 1;
    m <<= od;
    }
// pm = m;
// pn = n + 1;
// pod = 1 << od;
    }
    static const struct ingenic_cgu_clk_info jz4760_cgu_clocks[] = {
// External clocks
    [JZ4760_CLK_EXT] = { "ext", CGU_CLK_EXT },
    [JZ4760_CLK_OSC32K] = { "osc32k", CGU_CLK_EXT },
// PLLs
    [JZ4760_CLK_PLL0] = {
    "pll0", CGU_CLK_PLL,
    .parents = { JZ4760_CLK_EXT },
    .pll = {
    .reg = CGU_REG_CPPCR0,
    .rate_multiplier = 1,
    .m_shift = 23,
    .m_bits = 8,
    .m_offset = 0,
    .n_shift = 18,
    .n_bits = 4,
    .n_offset = 0,
    .od_shift = 16,
    .od_bits = 2,
    .od_max = 8,
    .od_encoding = pll_od_encoding,
    .bypass_reg = CGU_REG_CPPCR0,
    .bypass_bit = 9,
    .enable_bit = 8,
    .stable_bit = 10,
    .calc_m_n_od = jz4760_cgu_calc_m_n_od,
    },
    },
    [JZ4760_CLK_PLL1] = {
// TODO: PLL1 can depend on PLL0
    "pll1", CGU_CLK_PLL,
    .parents = { JZ4760_CLK_EXT },
    .pll = {
    .reg = CGU_REG_CPPCR1,
    .rate_multiplier = 1,
    .m_shift = 23,
    .m_bits = 8,
    .m_offset = 0,
    .n_shift = 18,
    .n_bits = 4,
    .n_offset = 0,
    .od_shift = 16,
    .od_bits = 2,
    .od_max = 8,
    .od_encoding = pll_od_encoding,
    .bypass_bit = -1,
    .enable_bit = 7,
    .stable_bit = 6,
    .calc_m_n_od = jz4760_cgu_calc_m_n_od,
    },
    },
// Main clocks
    [JZ4760_CLK_CCLK] = {
    "cclk", CGU_CLK_DIV,
//
// Disabling the CPU clock or any parent clocks will hang the
// system; mark it critical.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { JZ4760_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 0, 1, 4, 22, -1, -1, 0,
    jz4760_cgu_cpccr_div_table,
    },
    },
    [JZ4760_CLK_HCLK] = {
    "hclk", CGU_CLK_DIV,
    .parents = { JZ4760_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 4, 1, 4, 22, -1, -1, 0,
    jz4760_cgu_cpccr_div_table,
    },
    },
    [JZ4760_CLK_SCLK] = {
    "sclk", CGU_CLK_DIV,
    .parents = { JZ4760_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 24, 1, 4, 22, -1, -1, 0,
    jz4760_cgu_cpccr_div_table,
    },
    },
    [JZ4760_CLK_H2CLK] = {
    "h2clk", CGU_CLK_DIV,
    .parents = { JZ4760_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 16, 1, 4, 22, -1, -1, 0,
    jz4760_cgu_cpccr_div_table,
    },
    },
    [JZ4760_CLK_MCLK] = {
    "mclk", CGU_CLK_DIV,
//
// Disabling MCLK or its parents will render DRAM
// inaccessible; mark it critical.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { JZ4760_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 12, 1, 4, 22, -1, -1, 0,
    jz4760_cgu_cpccr_div_table,
    },
    },
    [JZ4760_CLK_PCLK] = {
    "pclk", CGU_CLK_DIV,
    .parents = { JZ4760_CLK_PLL0, },
    .div = {
    CGU_REG_CPCCR, 8, 1, 4, 22, -1, -1, 0,
    jz4760_cgu_cpccr_div_table,
    },
    },
// Divided clocks
    [JZ4760_CLK_PLL0_HALF] = {
    "pll0_half", CGU_CLK_DIV,
    .parents = { JZ4760_CLK_PLL0 },
    .div = {
    CGU_REG_CPCCR, 21, 1, 1, 22, -1, -1, 0,
    jz4760_cgu_pll_half_div_table,
    },
    },
// Those divided clocks can connect to PLL0 or PLL1
    [JZ4760_CLK_UHC] = {
    "uhc", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_PLL0_HALF, JZ4760_CLK_PLL1, },
    .mux = { CGU_REG_UHCCDR, 31, 1 },
    .div = { CGU_REG_UHCCDR, 0, 1, 4, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 24 },
    },
    [JZ4760_CLK_GPU] = {
    "gpu", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_PLL0_HALF, JZ4760_CLK_PLL1, },
    .mux = { CGU_REG_GPUCDR, 31, 1 },
    .div = { CGU_REG_GPUCDR, 0, 1, 3, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 9 },
    },
    [JZ4760_CLK_LPCLK_DIV] = {
    "lpclk_div", CGU_CLK_DIV | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_PLL0_HALF, JZ4760_CLK_PLL1, },
    .mux = { CGU_REG_LPCDR, 29, 1 },
    .div = { CGU_REG_LPCDR, 0, 1, 11, -1, -1, -1 },
    },
    [JZ4760_CLK_TVE] = {
    "tve", CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_LPCLK_DIV, JZ4760_CLK_EXT, },
    .mux = { CGU_REG_LPCDR, 31, 1 },
    .gate = { CGU_REG_CLKGR0, 27 },
    },
    [JZ4760_CLK_LPCLK] = {
    "lpclk", CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_LPCLK_DIV, JZ4760_CLK_TVE, },
    .mux = { CGU_REG_LPCDR, 30, 1 },
    .gate = { CGU_REG_CLKGR0, 28 },
    },
    [JZ4760_CLK_GPS] = {
    "gps", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_PLL0_HALF, JZ4760_CLK_PLL1, },
    .mux = { CGU_REG_GPSCDR, 31, 1 },
    .div = { CGU_REG_GPSCDR, 0, 1, 4, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 22 },
    },
// Those divided clocks can connect to EXT, PLL0 or PLL1
    [JZ4760_CLK_PCM] = {
    "pcm", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_EXT, -1,
    JZ4760_CLK_PLL0_HALF, JZ4760_CLK_PLL1 },
    .mux = { CGU_REG_PCMCDR, 30, 2 },
    .div = { CGU_REG_PCMCDR, 0, 1, 9, -1, -1, -1, BIT(0) },
    .gate = { CGU_REG_CLKGR1, 8 },
    },
    [JZ4760_CLK_I2S] = {
    "i2s", CGU_CLK_DIV | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_EXT, -1,
    JZ4760_CLK_PLL0_HALF, JZ4760_CLK_PLL1 },
    .mux = { CGU_REG_I2SCDR, 30, 2 },
    .div = { CGU_REG_I2SCDR, 0, 1, 9, -1, -1, -1, BIT(0) },
    },
    [JZ4760_CLK_OTG] = {
    "usb", CGU_CLK_DIV | CGU_CLK_GATE | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_EXT, -1,
    JZ4760_CLK_PLL0_HALF, JZ4760_CLK_PLL1 },
    .mux = { CGU_REG_USBCDR, 30, 2 },
    .div = { CGU_REG_USBCDR, 0, 1, 8, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 2 },
    },
// Those divided clocks can connect to EXT or PLL0
    [JZ4760_CLK_MMC_MUX] = {
    "mmc_mux", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { JZ4760_CLK_EXT, JZ4760_CLK_PLL0_HALF, },
    .mux = { CGU_REG_MSCCDR, 31, 1 },
    .div = { CGU_REG_MSCCDR, 0, 1, 6, -1, -1, -1, BIT(0) },
    },
    [JZ4760_CLK_SSI_MUX] = {
    "ssi_mux", CGU_CLK_DIV | CGU_CLK_MUX,
    .parents = { JZ4760_CLK_EXT, JZ4760_CLK_PLL0_HALF, },
    .mux = { CGU_REG_SSICDR, 31, 1 },
    .div = { CGU_REG_SSICDR, 0, 1, 6, -1, -1, -1, BIT(0) },
    },
// These divided clock can connect to PLL0 only
    [JZ4760_CLK_CIM] = {
    "cim", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4760_CLK_PLL0_HALF },
    .div = { CGU_REG_CIMCDR, 0, 1, 8, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 26 },
    },
// Gate-only clocks
    [JZ4760_CLK_SSI0] = {
    "ssi0", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_SSI_MUX, },
    .gate = { CGU_REG_CLKGR0, 4 },
    },
    [JZ4760_CLK_SSI1] = {
    "ssi1", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_SSI_MUX, },
    .gate = { CGU_REG_CLKGR0, 19 },
    },
    [JZ4760_CLK_SSI2] = {
    "ssi2", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_SSI_MUX, },
    .gate = { CGU_REG_CLKGR0, 20 },
    },
    [JZ4760_CLK_DMA] = {
    "dma", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_H2CLK, },
    .gate = { CGU_REG_CLKGR0, 21 },
    },
    [JZ4760_CLK_MDMA] = {
    "mdma", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_HCLK, },
    .gate = { CGU_REG_CLKGR0, 25 },
    },
    [JZ4760_CLK_BDMA] = {
    "bdma", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_HCLK, },
    .gate = { CGU_REG_CLKGR1, 0 },
    },
    [JZ4760_CLK_I2C0] = {
    "i2c0", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 5 },
    },
    [JZ4760_CLK_I2C1] = {
    "i2c1", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 6 },
    },
    [JZ4760_CLK_UART0] = {
    "uart0", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 15 },
    },
    [JZ4760_CLK_UART1] = {
    "uart1", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 16 },
    },
    [JZ4760_CLK_UART2] = {
    "uart2", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 17 },
    },
    [JZ4760_CLK_UART3] = {
    "uart3", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 18 },
    },
    [JZ4760_CLK_IPU] = {
    "ipu", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_HCLK, },
    .gate = { CGU_REG_CLKGR0, 29 },
    },
    [JZ4760_CLK_ADC] = {
    "adc", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 14 },
    },
    [JZ4760_CLK_AIC] = {
    "aic", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_EXT, },
    .gate = { CGU_REG_CLKGR0, 8 },
    },
    [JZ4760_CLK_VPU] = {
    "vpu", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_HCLK, },
    .gate = { CGU_REG_LCR, 30, false, 150 },
    },
    [JZ4760_CLK_MMC0] = {
    "mmc0", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_MMC_MUX, },
    .gate = { CGU_REG_CLKGR0, 3 },
    },
    [JZ4760_CLK_MMC1] = {
    "mmc1", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_MMC_MUX, },
    .gate = { CGU_REG_CLKGR0, 11 },
    },
    [JZ4760_CLK_MMC2] = {
    "mmc2", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_MMC_MUX, },
    .gate = { CGU_REG_CLKGR0, 12 },
    },
    [JZ4760_CLK_UHC_PHY] = {
    "uhc_phy", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_UHC, },
    .gate = { CGU_REG_OPCR, 5 },
    },
    [JZ4760_CLK_OTG_PHY] = {
    "usb_phy", CGU_CLK_GATE,
    .parents = { JZ4760_CLK_OTG },
    .gate = { CGU_REG_OPCR, 7, true, 50 },
    },
// Custom clocks
    [JZ4760_CLK_EXT512] = {
    "ext/512", CGU_CLK_FIXDIV,
    .parents = { JZ4760_CLK_EXT },
    .fixdiv = { 512 },
    },
    [JZ4760_CLK_RTC] = {
    "rtc", CGU_CLK_MUX,
    .parents = { JZ4760_CLK_EXT512, JZ4760_CLK_OSC32K, },
    .mux = { CGU_REG_OPCR, 2, 1},
    },
    };
#[no_mangle]
unsafe extern "C" fn jz4760_cgu_init(np: *mut device_node) -> void __init {
    static void __init jz4760_cgu_init(struct device_node *np)
    {
    struct ingenic_cgu *cgu;
    int retval;
    cgu = ingenic_cgu_new(jz4760_cgu_clocks,
    ARRAY_SIZE(jz4760_cgu_clocks), np);
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
    CLK_OF_DECLARE_DRIVER(jz4760_cgu, "ingenic,jz4760-cgu", jz4760_cgu_init);
// JZ4760B has some small differences, but we don't implement them.
    CLK_OF_DECLARE_DRIVER(jz4760b_cgu, "ingenic,jz4760b-cgu", jz4760_cgu_init);
