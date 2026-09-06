//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ingenic/jz4780-cgu.c
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
// Ingenic JZ4780 SoC CGU driver
//
// Copyright (c) 2013-2015 Imagination Technologies
// Author: Paul Burton <paul.burton@mips.com>
// Copyright (c) 2020 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//

// CGU register offsets
pub const CGU_REG_CLOCKCONTROL: c_uint = 0x00;
pub const CGU_REG_LCR: c_uint = 0x04;
pub const CGU_REG_APLL: c_uint = 0x10;
pub const CGU_REG_MPLL: c_uint = 0x14;
pub const CGU_REG_EPLL: c_uint = 0x18;
pub const CGU_REG_VPLL: c_uint = 0x1c;
pub const CGU_REG_CLKGR0: c_uint = 0x20;
pub const CGU_REG_OPCR: c_uint = 0x24;
pub const CGU_REG_CLKGR1: c_uint = 0x28;
pub const CGU_REG_DDRCDR: c_uint = 0x2c;
pub const CGU_REG_VPUCDR: c_uint = 0x30;
pub const CGU_REG_USBPCR: c_uint = 0x3c;
pub const CGU_REG_USBRDT: c_uint = 0x40;
pub const CGU_REG_USBVBFIL: c_uint = 0x44;
pub const CGU_REG_USBPCR1: c_uint = 0x48;
pub const CGU_REG_LP0CDR: c_uint = 0x54;
pub const CGU_REG_I2SCDR: c_uint = 0x60;
pub const CGU_REG_LP1CDR: c_uint = 0x64;
pub const CGU_REG_MSC0CDR: c_uint = 0x68;
pub const CGU_REG_UHCCDR: c_uint = 0x6c;
pub const CGU_REG_SSICDR: c_uint = 0x74;
pub const CGU_REG_CIMCDR: c_uint = 0x7c;
pub const CGU_REG_PCMCDR: c_uint = 0x84;
pub const CGU_REG_GPUCDR: c_uint = 0x88;
pub const CGU_REG_HDMICDR: c_uint = 0x8c;
pub const CGU_REG_MSC1CDR: c_uint = 0xa4;
pub const CGU_REG_MSC2CDR: c_uint = 0xa8;
pub const CGU_REG_BCHCDR: c_uint = 0xac;
pub const CGU_REG_CLOCKSTATUS: c_uint = 0xd4;
// bits within the OPCR register

// bits within the USBPCR register

pub const USBPCR_TXVREFTUNE_MASK: c_uint = 0xf;
// bits within the USBPCR1 register
pub const USBPCR1_REFCLKSEL_SHIFT: c_int = 26;

pub const USBPCR1_REFCLKDIV_SHIFT: c_int = 24;

// bits within the USBRDT register

pub const USBRDT_USBRDT_MASK: c_uint = 0x7fffff;
// bits within the USBVBFIL register
pub const USBVBFIL_IDDIGFIL_SHIFT: c_int = 16;

// bits within the LCR register

// bits within the CLKGR1 register

    static struct ingenic_cgu *cgu;
    static unsigned long jz4780_otg_phy_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    u32 usbpcr1;
    unsigned refclk_div;
    usbpcr1 = readl(cgu.base + CGU_REG_USBPCR1);
    refclk_div = usbpcr1 & USBPCR1_REFCLKDIV_MASK;
    switch (refclk_div) {
    case USBPCR1_REFCLKDIV_12:
    return 12000000;
    case USBPCR1_REFCLKDIV_24:
    return 24000000;
    case USBPCR1_REFCLKDIV_48:
    return 48000000;
    case USBPCR1_REFCLKDIV_19_2:
    return 19200000;
    }
    return parent_rate;
    }
    static int jz4780_otg_phy_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    if (req.rate < 15600000)
    req.rate = 12000000;
#[no_mangle]
pub unsafe extern "C" fn if(21600000: req->rate <) -> else {
    else if (req.rate < 21600000)
    req.rate = 19200000;
#[no_mangle]
pub unsafe extern "C" fn if(36000000: req->rate <) -> else {
    else if (req.rate < 36000000)
    req.rate = 24000000;
    else
    req.rate = 48000000;
    return 0;
    }
    static int jz4780_otg_phy_set_rate(struct clk_hw *hw, unsigned long req_rate,
    unsigned long parent_rate)
    {
    unsigned long flags;
    u32 usbpcr1, div_bits;
    switch (req_rate) {
    case 12000000:
    div_bits = USBPCR1_REFCLKDIV_12;
    break;
    case 19200000:
    div_bits = USBPCR1_REFCLKDIV_19_2;
    break;
    case 24000000:
    div_bits = USBPCR1_REFCLKDIV_24;
    break;
    case 48000000:
    div_bits = USBPCR1_REFCLKDIV_48;
    break;
    default:
    return -EINVAL;
    }
    spin_lock_irqsave(&cgu.lock, flags);
    usbpcr1 = readl(cgu.base + CGU_REG_USBPCR1);
    usbpcr1 &= ~USBPCR1_REFCLKDIV_MASK;
    usbpcr1 |= div_bits;
    writel(usbpcr1, cgu.base + CGU_REG_USBPCR1);
    spin_unlock_irqrestore(&cgu.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_otg_phy_enable(hw: *mut clk_hw) -> c_int {
    static int jz4780_otg_phy_enable(struct clk_hw *hw)
    {
    void __iomem *reg_opcr		= cgu.base + CGU_REG_OPCR;
    void __iomem *reg_usbpcr	= cgu.base + CGU_REG_USBPCR;
    writel(readl(reg_opcr) | OPCR_SPENDN0, reg_opcr);
    writel(readl(reg_usbpcr) & ~USBPCR_OTG_DISABLE & ~USBPCR_SIDDQ, reg_usbpcr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn jz4780_otg_phy_disable(hw: *mut clk_hw) {
    static void jz4780_otg_phy_disable(struct clk_hw *hw)
    {
    void __iomem *reg_opcr		= cgu.base + CGU_REG_OPCR;
    void __iomem *reg_usbpcr	= cgu.base + CGU_REG_USBPCR;
    writel(readl(reg_opcr) & ~OPCR_SPENDN0, reg_opcr);
    writel(readl(reg_usbpcr) | USBPCR_OTG_DISABLE | USBPCR_SIDDQ, reg_usbpcr);
    }
#[no_mangle]
unsafe extern "C" fn jz4780_otg_phy_is_enabled(hw: *mut clk_hw) -> c_int {
    static int jz4780_otg_phy_is_enabled(struct clk_hw *hw)
    {
    void __iomem *reg_opcr		= cgu.base + CGU_REG_OPCR;
    void __iomem *reg_usbpcr	= cgu.base + CGU_REG_USBPCR;
    return (readl(reg_opcr) & OPCR_SPENDN0) &&
    !(readl(reg_usbpcr) & USBPCR_SIDDQ) &&
    !(readl(reg_usbpcr) & USBPCR_OTG_DISABLE);
    }
    static const struct clk_ops jz4780_otg_phy_ops = {
    .recalc_rate = jz4780_otg_phy_recalc_rate,
    .determine_rate = jz4780_otg_phy_determine_rate,
    .set_rate = jz4780_otg_phy_set_rate,
    .enable		= jz4780_otg_phy_enable,
    .disable	= jz4780_otg_phy_disable,
    .is_enabled	= jz4780_otg_phy_is_enabled,
    };
#[no_mangle]
unsafe extern "C" fn jz4780_core1_enable(hw: *mut clk_hw) -> c_int {
    static int jz4780_core1_enable(struct clk_hw *hw)
    {
    struct ingenic_clk *ingenic_clk = to_ingenic_clk(hw);
    struct ingenic_cgu *cgu = ingenic_clk.cgu;
    let mut timeout: c_uint = 5000;
    unsigned long flags;
    int retval;
    u32 lcr, clkgr1;
    spin_lock_irqsave(&cgu.lock, flags);
    lcr = readl(cgu.base + CGU_REG_LCR);
    lcr &= ~LCR_PD_SCPU;
    writel(lcr, cgu.base + CGU_REG_LCR);
    clkgr1 = readl(cgu.base + CGU_REG_CLKGR1);
    clkgr1 &= ~CLKGR1_CORE1;
    writel(clkgr1, cgu.base + CGU_REG_CLKGR1);
    spin_unlock_irqrestore(&cgu.lock, flags);
// wait for the CPU to be powered up
    retval = readl_poll_timeout(cgu.base + CGU_REG_LCR, lcr,
    !(lcr & LCR_SCPUS), 10, timeout);
    if (retval == -ETIMEDOUT) {
    pr_err("%s: Wait for power up core1 timeout\n", __func__);
    return retval;
    }
    return 0;
    }
    static const struct clk_ops jz4780_core1_ops = {
    .enable = jz4780_core1_enable,
    };
    static const s8 pll_od_encoding[16] = {
    0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7,
    0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    };
    static const struct ingenic_cgu_clk_info jz4780_cgu_clocks[] = {
// External clocks
    [JZ4780_CLK_EXCLK] = { "ext", CGU_CLK_EXT },
    [JZ4780_CLK_RTCLK] = { "rtc", CGU_CLK_EXT },
// PLLs

    .reg = CGU_REG_ ## name, \
    .rate_multiplier = 1, \
    .m_shift = 19, \
    .m_bits = 13, \
    .m_offset = 1, \
    .n_shift = 13, \
    .n_bits = 6, \
    .n_offset = 1, \
    .od_shift = 9, \
    .od_bits = 4, \
    .od_max = 16, \
    .od_encoding = pll_od_encoding, \
    .stable_bit = 6, \
    .bypass_reg = CGU_REG_ ## name, \
    .bypass_bit = 1, \
    .enable_bit = 0, \
    }
    [JZ4780_CLK_APLL] = {
    "apll", CGU_CLK_PLL,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .pll = DEF_PLL(APLL),
    },
    [JZ4780_CLK_MPLL] = {
    "mpll", CGU_CLK_PLL,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .pll = DEF_PLL(MPLL),
    },
    [JZ4780_CLK_EPLL] = {
    "epll", CGU_CLK_PLL,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .pll = DEF_PLL(EPLL),
    },
    [JZ4780_CLK_VPLL] = {
    "vpll", CGU_CLK_PLL,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .pll = DEF_PLL(VPLL),
    },

// Custom (SoC-specific) OTG PHY
    [JZ4780_CLK_OTGPHY] = {
    "otg_phy", CGU_CLK_CUSTOM,
    .parents = { -1, -1, JZ4780_CLK_EXCLK, -1 },
    .custom = { &jz4780_otg_phy_ops },
    },
// Muxes & dividers
    [JZ4780_CLK_SCLKA] = {
    "sclk_a", CGU_CLK_MUX,
    .parents = { -1, JZ4780_CLK_APLL, JZ4780_CLK_EXCLK,
    JZ4780_CLK_RTCLK },
    .mux = { CGU_REG_CLOCKCONTROL, 30, 2 },
    },
    [JZ4780_CLK_CPUMUX] = {
    "cpumux", CGU_CLK_MUX,
    .parents = { -1, JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_EPLL },
    .mux = { CGU_REG_CLOCKCONTROL, 28, 2 },
    },
    [JZ4780_CLK_CPU] = {
    "cpu", CGU_CLK_DIV,
//
// Disabling the CPU clock or any parent clocks will hang the
// system; mark it critical.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { JZ4780_CLK_CPUMUX, -1, -1, -1 },
    .div = { CGU_REG_CLOCKCONTROL, 0, 1, 4, 22, -1, -1 },
    },
    [JZ4780_CLK_L2CACHE] = {
    "l2cache", CGU_CLK_DIV,
//
// The L2 cache clock is critical if caches are enabled and
// disabling it or any parent clocks will hang the system.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { JZ4780_CLK_CPUMUX, -1, -1, -1 },
    .div = { CGU_REG_CLOCKCONTROL, 4, 1, 4, -1, -1, -1 },
    },
    [JZ4780_CLK_AHB0] = {
    "ahb0", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { -1, JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_EPLL },
    .mux = { CGU_REG_CLOCKCONTROL, 26, 2 },
    .div = { CGU_REG_CLOCKCONTROL, 8, 1, 4, 21, -1, -1 },
    },
    [JZ4780_CLK_AHB2PMUX] = {
    "ahb2_apb_mux", CGU_CLK_MUX,
    .parents = { -1, JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_RTCLK },
    .mux = { CGU_REG_CLOCKCONTROL, 24, 2 },
    },
    [JZ4780_CLK_AHB2] = {
    "ahb2", CGU_CLK_DIV,
    .parents = { JZ4780_CLK_AHB2PMUX, -1, -1, -1 },
    .div = { CGU_REG_CLOCKCONTROL, 12, 1, 4, 20, -1, -1 },
    },
    [JZ4780_CLK_PCLK] = {
    "pclk", CGU_CLK_DIV,
    .parents = { JZ4780_CLK_AHB2PMUX, -1, -1, -1 },
    .div = { CGU_REG_CLOCKCONTROL, 16, 1, 4, 20, -1, -1 },
    },
    [JZ4780_CLK_DDR] = {
    "ddr", CGU_CLK_MUX | CGU_CLK_DIV,
//
// Disabling DDR clock or its parents will render DRAM
// inaccessible; mark it critical.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { -1, JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL, -1 },
    .mux = { CGU_REG_DDRCDR, 30, 2 },
    .div = { CGU_REG_DDRCDR, 0, 1, 4, 29, 28, 27 },
    },
    [JZ4780_CLK_VPU] = {
    "vpu", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_EPLL, -1 },
    .mux = { CGU_REG_VPUCDR, 30, 2 },
    .div = { CGU_REG_VPUCDR, 0, 1, 4, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR1, 2 },
    },
    [JZ4780_CLK_I2SPLL] = {
    "i2s_pll", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { JZ4780_CLK_SCLKA, JZ4780_CLK_EPLL, -1, -1 },
    .mux = { CGU_REG_I2SCDR, 30, 1 },
    .div = { CGU_REG_I2SCDR, 0, 1, 8, 29, 28, 27 },
    },
    [JZ4780_CLK_I2S] = {
    "i2s", CGU_CLK_MUX,
    .parents = { JZ4780_CLK_EXCLK, JZ4780_CLK_I2SPLL, -1, -1 },
    .mux = { CGU_REG_I2SCDR, 31, 1 },
    },
    [JZ4780_CLK_LCD0PIXCLK] = {
    "lcd0pixclk", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_VPLL, -1 },
    .mux = { CGU_REG_LP0CDR, 30, 2 },
    .div = { CGU_REG_LP0CDR, 0, 1, 8, 28, 27, 26 },
    },
    [JZ4780_CLK_LCD1PIXCLK] = {
    "lcd1pixclk", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_VPLL, -1 },
    .mux = { CGU_REG_LP1CDR, 30, 2 },
    .div = { CGU_REG_LP1CDR, 0, 1, 8, 28, 27, 26 },
    },
    [JZ4780_CLK_MSCMUX] = {
    "msc_mux", CGU_CLK_MUX,
    .parents = { -1, JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL, -1 },
    .mux = { CGU_REG_MSC0CDR, 30, 2 },
    },
    [JZ4780_CLK_MSC0] = {
    "msc0", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4780_CLK_MSCMUX, -1, -1, -1 },
    .div = { CGU_REG_MSC0CDR, 0, 2, 8, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR0, 3 },
    },
    [JZ4780_CLK_MSC1] = {
    "msc1", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4780_CLK_MSCMUX, -1, -1, -1 },
    .div = { CGU_REG_MSC1CDR, 0, 2, 8, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR0, 11 },
    },
    [JZ4780_CLK_MSC2] = {
    "msc2", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4780_CLK_MSCMUX, -1, -1, -1 },
    .div = { CGU_REG_MSC2CDR, 0, 2, 8, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR0, 12 },
    },
    [JZ4780_CLK_UHC] = {
    "uhc", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_EPLL, JZ4780_CLK_OTGPHY },
    .mux = { CGU_REG_UHCCDR, 30, 2 },
    .div = { CGU_REG_UHCCDR, 0, 1, 8, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR0, 24 },
    },
    [JZ4780_CLK_SSIPLL] = {
    "ssi_pll", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL, -1, -1 },
    .mux = { CGU_REG_SSICDR, 30, 1 },
    .div = { CGU_REG_SSICDR, 0, 1, 8, 29, 28, 27 },
    },
    [JZ4780_CLK_SSI] = {
    "ssi", CGU_CLK_MUX,
    .parents = { JZ4780_CLK_EXCLK, JZ4780_CLK_SSIPLL, -1, -1 },
    .mux = { CGU_REG_SSICDR, 31, 1 },
    },
    [JZ4780_CLK_CIMMCLK] = {
    "cim_mclk", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL, -1, -1 },
    .mux = { CGU_REG_CIMCDR, 31, 1 },
    .div = { CGU_REG_CIMCDR, 0, 1, 8, 30, 29, 28 },
    },
    [JZ4780_CLK_PCMPLL] = {
    "pcm_pll", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_EPLL, JZ4780_CLK_VPLL },
    .mux = { CGU_REG_PCMCDR, 29, 2 },
    .div = { CGU_REG_PCMCDR, 0, 1, 8, 28, 27, 26 },
    },
    [JZ4780_CLK_PCM] = {
    "pcm", CGU_CLK_MUX | CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, JZ4780_CLK_PCMPLL, -1, -1 },
    .mux = { CGU_REG_PCMCDR, 31, 1 },
    .gate = { CGU_REG_CLKGR1, 3 },
    },
    [JZ4780_CLK_GPU] = {
    "gpu", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { -1, JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_EPLL },
    .mux = { CGU_REG_GPUCDR, 30, 2 },
    .div = { CGU_REG_GPUCDR, 0, 1, 4, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR1, 4 },
    },
    [JZ4780_CLK_HDMI] = {
    "hdmi", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_VPLL, -1 },
    .mux = { CGU_REG_HDMICDR, 30, 2 },
    .div = { CGU_REG_HDMICDR, 0, 1, 8, 29, 28, 26 },
    .gate = { CGU_REG_CLKGR1, 9 },
    },
    [JZ4780_CLK_BCH] = {
    "bch", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { -1, JZ4780_CLK_SCLKA, JZ4780_CLK_MPLL,
    JZ4780_CLK_EPLL },
    .mux = { CGU_REG_BCHCDR, 30, 2 },
    .div = { CGU_REG_BCHCDR, 0, 1, 4, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR0, 1 },
    },
    [JZ4780_CLK_EXCLK_DIV512] = {
    "exclk_div512", CGU_CLK_FIXDIV,
    .parents = { JZ4780_CLK_EXCLK },
    .fixdiv = { 512 },
    },
    [JZ4780_CLK_RTC] = {
    "rtc_ercs", CGU_CLK_MUX | CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK_DIV512, JZ4780_CLK_RTCLK },
    .mux = { CGU_REG_OPCR, 2, 1},
    },
// Gate-only clocks
    [JZ4780_CLK_NEMC] = {
    "nemc", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_AHB2, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 0 },
    },
    [JZ4780_CLK_OTG0] = {
    "otg0", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 2 },
    },
    [JZ4780_CLK_SSI0] = {
    "ssi0", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_SSI, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 4 },
    },
    [JZ4780_CLK_SMB0] = {
    "smb0", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 5 },
    },
    [JZ4780_CLK_SMB1] = {
    "smb1", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 6 },
    },
    [JZ4780_CLK_SCC] = {
    "scc", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 7 },
    },
    [JZ4780_CLK_AIC] = {
    "aic", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 8 },
    },
    [JZ4780_CLK_TSSI0] = {
    "tssi0", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 9 },
    },
    [JZ4780_CLK_OWI] = {
    "owi", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 10 },
    },
    [JZ4780_CLK_KBC] = {
    "kbc", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 13 },
    },
    [JZ4780_CLK_SADC] = {
    "sadc", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 14 },
    },
    [JZ4780_CLK_UART0] = {
    "uart0", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 15 },
    },
    [JZ4780_CLK_UART1] = {
    "uart1", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 16 },
    },
    [JZ4780_CLK_UART2] = {
    "uart2", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 17 },
    },
    [JZ4780_CLK_UART3] = {
    "uart3", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 18 },
    },
    [JZ4780_CLK_SSI1] = {
    "ssi1", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_SSI, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 19 },
    },
    [JZ4780_CLK_SSI2] = {
    "ssi2", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_SSI, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 20 },
    },
    [JZ4780_CLK_PDMA] = {
    "pdma", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 21 },
    },
    [JZ4780_CLK_GPS] = {
    "gps", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 22 },
    },
    [JZ4780_CLK_MAC] = {
    "mac", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 23 },
    },
    [JZ4780_CLK_SMB2] = {
    "smb2", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 24 },
    },
    [JZ4780_CLK_CIM] = {
    "cim", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 26 },
    },
    [JZ4780_CLK_LCD] = {
    "lcd", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 28 },
    },
    [JZ4780_CLK_TVE] = {
    "tve", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_LCD, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 27 },
    },
    [JZ4780_CLK_IPU] = {
    "ipu", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 29 },
    },
    [JZ4780_CLK_DDR0] = {
    "ddr0", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_DDR, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 30 },
    },
    [JZ4780_CLK_DDR1] = {
    "ddr1", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_DDR, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 31 },
    },
    [JZ4780_CLK_SMB3] = {
    "smb3", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 0 },
    },
    [JZ4780_CLK_TSSI1] = {
    "tssi1", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 1 },
    },
    [JZ4780_CLK_COMPRESS] = {
    "compress", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 5 },
    },
    [JZ4780_CLK_AIC1] = {
    "aic1", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 6 },
    },
    [JZ4780_CLK_GPVLC] = {
    "gpvlc", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 7 },
    },
    [JZ4780_CLK_OTG1] = {
    "otg1", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 8 },
    },
    [JZ4780_CLK_UART4] = {
    "uart4", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 10 },
    },
    [JZ4780_CLK_AHBMON] = {
    "ahb_mon", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 11 },
    },
    [JZ4780_CLK_SMB4] = {
    "smb4", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 12 },
    },
    [JZ4780_CLK_DES] = {
    "des", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 13 },
    },
    [JZ4780_CLK_X2D] = {
    "x2d", CGU_CLK_GATE,
    .parents = { JZ4780_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 14 },
    },
    [JZ4780_CLK_CORE1] = {
    "core1", CGU_CLK_CUSTOM,
    .parents = { JZ4780_CLK_CPU, -1, -1, -1 },
    .custom = { &jz4780_core1_ops },
    },
    };
#[no_mangle]
unsafe extern "C" fn jz4780_cgu_init(np: *mut device_node) -> void __init {
    static void __init jz4780_cgu_init(struct device_node *np)
    {
    int retval;
    cgu = ingenic_cgu_new(jz4780_cgu_clocks,
    ARRAY_SIZE(jz4780_cgu_clocks), np);
    if (!cgu) {
    pr_err("%s: failed to initialise CGU\n", __func__);
    return;
    }
    retval = ingenic_cgu_register_clocks(cgu);
    if (retval) {
    pr_err("%s: failed to register CGU Clocks\n", __func__);
    return;
    }
    ingenic_cgu_register_syscore(cgu);
    }
    CLK_OF_DECLARE_DRIVER(jz4780_cgu, "ingenic,jz4780-cgu", jz4780_cgu_init);
