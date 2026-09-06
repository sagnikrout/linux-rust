//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ingenic/x1830-cgu.c
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
// X1830 SoC CGU driver
// Copyright (c) 2019 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//

// CGU register offsets
pub const CGU_REG_CPCCR: c_uint = 0x00;
pub const CGU_REG_CPPCR: c_uint = 0x0c;
pub const CGU_REG_APLL: c_uint = 0x10;
pub const CGU_REG_MPLL: c_uint = 0x14;
pub const CGU_REG_CLKGR0: c_uint = 0x20;
pub const CGU_REG_OPCR: c_uint = 0x24;
pub const CGU_REG_CLKGR1: c_uint = 0x28;
pub const CGU_REG_DDRCDR: c_uint = 0x2c;
pub const CGU_REG_USBPCR: c_uint = 0x3c;
pub const CGU_REG_USBRDT: c_uint = 0x40;
pub const CGU_REG_USBVBFIL: c_uint = 0x44;
pub const CGU_REG_USBPCR1: c_uint = 0x48;
pub const CGU_REG_MACCDR: c_uint = 0x54;
pub const CGU_REG_EPLL: c_uint = 0x58;
pub const CGU_REG_I2SCDR: c_uint = 0x60;
pub const CGU_REG_LPCDR: c_uint = 0x64;
pub const CGU_REG_MSC0CDR: c_uint = 0x68;
pub const CGU_REG_I2SCDR1: c_uint = 0x70;
pub const CGU_REG_SSICDR: c_uint = 0x74;
pub const CGU_REG_CIMCDR: c_uint = 0x7c;
pub const CGU_REG_MSC1CDR: c_uint = 0xa4;
pub const CGU_REG_CMP_INTR: c_uint = 0xb0;
pub const CGU_REG_CMP_INTRE: c_uint = 0xb4;
pub const CGU_REG_DRCG: c_uint = 0xd0;
pub const CGU_REG_CPCSR: c_uint = 0xd4;
pub const CGU_REG_VPLL: c_uint = 0xe0;
pub const CGU_REG_MACPHYC: c_uint = 0xe8;
// bits within the OPCR register

// bits within the USBPCR register

    static struct ingenic_cgu *cgu;
#[no_mangle]
unsafe extern "C" fn x1830_usb_phy_enable(hw: *mut clk_hw) -> c_int {
    static int x1830_usb_phy_enable(struct clk_hw *hw)
    {
    void __iomem *reg_opcr		= cgu.base + CGU_REG_OPCR;
    void __iomem *reg_usbpcr	= cgu.base + CGU_REG_USBPCR;
    writel((readl(reg_opcr) | OPCR_SPENDN0) & ~OPCR_GATE_USBPHYCLK, reg_opcr);
    writel(readl(reg_usbpcr) & ~USBPCR_OTG_DISABLE & ~USBPCR_SIDDQ, reg_usbpcr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x1830_usb_phy_disable(hw: *mut clk_hw) {
    static void x1830_usb_phy_disable(struct clk_hw *hw)
    {
    void __iomem *reg_opcr		= cgu.base + CGU_REG_OPCR;
    void __iomem *reg_usbpcr	= cgu.base + CGU_REG_USBPCR;
    writel((readl(reg_opcr) & ~OPCR_SPENDN0) | OPCR_GATE_USBPHYCLK, reg_opcr);
    writel(readl(reg_usbpcr) | USBPCR_OTG_DISABLE | USBPCR_SIDDQ, reg_usbpcr);
    }
#[no_mangle]
unsafe extern "C" fn x1830_usb_phy_is_enabled(hw: *mut clk_hw) -> c_int {
    static int x1830_usb_phy_is_enabled(struct clk_hw *hw)
    {
    void __iomem *reg_opcr		= cgu.base + CGU_REG_OPCR;
    void __iomem *reg_usbpcr	= cgu.base + CGU_REG_USBPCR;
    return (readl(reg_opcr) & OPCR_SPENDN0) &&
    !(readl(reg_usbpcr) & USBPCR_SIDDQ) &&
    !(readl(reg_usbpcr) & USBPCR_OTG_DISABLE);
    }
    static const struct clk_ops x1830_otg_phy_ops = {
    .enable		= x1830_usb_phy_enable,
    .disable	= x1830_usb_phy_disable,
    .is_enabled	= x1830_usb_phy_is_enabled,
    };
    static const s8 pll_od_encoding[64] = {
    0x0, 0x1,  -1, 0x2,  -1,  -1,  -1, 0x3,
    -1,  -1,  -1,  -1,  -1,  -1,  -1, 0x4,
    -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
    -1,  -1,  -1,  -1,  -1,  -1,  -1, 0x5,
    -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
    -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
    -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
    -1,  -1,  -1,  -1,  -1,  -1,  -1, 0x6,
    };
    static const struct ingenic_cgu_clk_info x1830_cgu_clocks[] = {
// External clocks
    [X1830_CLK_EXCLK] = { "ext", CGU_CLK_EXT },
    [X1830_CLK_RTCLK] = { "rtc", CGU_CLK_EXT },
// PLLs
    [X1830_CLK_APLL] = {
    "apll", CGU_CLK_PLL,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .pll = {
    .reg = CGU_REG_APLL,
    .rate_multiplier = 2,
    .m_shift = 20,
    .m_bits = 9,
    .m_offset = 1,
    .n_shift = 14,
    .n_bits = 6,
    .n_offset = 1,
    .od_shift = 11,
    .od_bits = 3,
    .od_max = 64,
    .od_encoding = pll_od_encoding,
    .bypass_reg = CGU_REG_CPPCR,
    .bypass_bit = 30,
    .enable_bit = 0,
    .stable_bit = 3,
    },
    },
    [X1830_CLK_MPLL] = {
    "mpll", CGU_CLK_PLL,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .pll = {
    .reg = CGU_REG_MPLL,
    .rate_multiplier = 2,
    .m_shift = 20,
    .m_bits = 9,
    .m_offset = 1,
    .n_shift = 14,
    .n_bits = 6,
    .n_offset = 1,
    .od_shift = 11,
    .od_bits = 3,
    .od_max = 64,
    .od_encoding = pll_od_encoding,
    .bypass_reg = CGU_REG_CPPCR,
    .bypass_bit = 28,
    .enable_bit = 0,
    .stable_bit = 3,
    },
    },
    [X1830_CLK_EPLL] = {
    "epll", CGU_CLK_PLL,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .pll = {
    .reg = CGU_REG_EPLL,
    .rate_multiplier = 2,
    .m_shift = 20,
    .m_bits = 9,
    .m_offset = 1,
    .n_shift = 14,
    .n_bits = 6,
    .n_offset = 1,
    .od_shift = 11,
    .od_bits = 3,
    .od_max = 64,
    .od_encoding = pll_od_encoding,
    .bypass_reg = CGU_REG_CPPCR,
    .bypass_bit = 24,
    .enable_bit = 0,
    .stable_bit = 3,
    },
    },
    [X1830_CLK_VPLL] = {
    "vpll", CGU_CLK_PLL,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .pll = {
    .reg = CGU_REG_VPLL,
    .rate_multiplier = 2,
    .m_shift = 20,
    .m_bits = 9,
    .m_offset = 1,
    .n_shift = 14,
    .n_bits = 6,
    .n_offset = 1,
    .od_shift = 11,
    .od_bits = 3,
    .od_max = 64,
    .od_encoding = pll_od_encoding,
    .bypass_reg = CGU_REG_CPPCR,
    .bypass_bit = 26,
    .enable_bit = 0,
    .stable_bit = 3,
    },
    },
// Custom (SoC-specific) OTG PHY
    [X1830_CLK_OTGPHY] = {
    "otg_phy", CGU_CLK_CUSTOM,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .custom = { &x1830_otg_phy_ops },
    },
// Muxes & dividers
    [X1830_CLK_SCLKA] = {
    "sclk_a", CGU_CLK_MUX,
    .parents = { -1, X1830_CLK_EXCLK, X1830_CLK_APLL, -1 },
    .mux = { CGU_REG_CPCCR, 30, 2 },
    },
    [X1830_CLK_CPUMUX] = {
    "cpu_mux", CGU_CLK_MUX,
    .parents = { -1, X1830_CLK_SCLKA, X1830_CLK_MPLL, -1 },
    .mux = { CGU_REG_CPCCR, 28, 2 },
    },
    [X1830_CLK_CPU] = {
    "cpu", CGU_CLK_DIV | CGU_CLK_GATE,
    .flags = CLK_IS_CRITICAL,
    .parents = { X1830_CLK_CPUMUX, -1, -1, -1 },
    .div = { CGU_REG_CPCCR, 0, 1, 4, 22, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 15 },
    },
    [X1830_CLK_L2CACHE] = {
    "l2cache", CGU_CLK_DIV,
//
// The L2 cache clock is critical if caches are enabled and
// disabling it or any parent clocks will hang the system.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { X1830_CLK_CPUMUX, -1, -1, -1 },
    .div = { CGU_REG_CPCCR, 4, 1, 4, 22, -1, -1 },
    },
    [X1830_CLK_AHB0] = {
    "ahb0", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { -1, X1830_CLK_SCLKA, X1830_CLK_MPLL, -1 },
    .mux = { CGU_REG_CPCCR, 26, 2 },
    .div = { CGU_REG_CPCCR, 8, 1, 4, 21, -1, -1 },
    },
    [X1830_CLK_AHB2PMUX] = {
    "ahb2_apb_mux", CGU_CLK_MUX,
    .parents = { -1, X1830_CLK_SCLKA, X1830_CLK_MPLL, -1 },
    .mux = { CGU_REG_CPCCR, 24, 2 },
    },
    [X1830_CLK_AHB2] = {
    "ahb2", CGU_CLK_DIV,
    .parents = { X1830_CLK_AHB2PMUX, -1, -1, -1 },
    .div = { CGU_REG_CPCCR, 12, 1, 4, 20, -1, -1 },
    },
    [X1830_CLK_PCLK] = {
    "pclk", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { X1830_CLK_AHB2PMUX, -1, -1, -1 },
    .div = { CGU_REG_CPCCR, 16, 1, 4, 20, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 14 },
    },
    [X1830_CLK_DDR] = {
    "ddr", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
//
// Disabling DDR clock or its parents will render DRAM
// inaccessible; mark it critical.
//
    .flags = CLK_IS_CRITICAL,
    .parents = { -1, X1830_CLK_SCLKA, X1830_CLK_MPLL, -1 },
    .mux = { CGU_REG_DDRCDR, 30, 2 },
    .div = { CGU_REG_DDRCDR, 0, 1, 4, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR0, 31 },
    },
    [X1830_CLK_MAC] = {
    "mac", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { X1830_CLK_SCLKA, X1830_CLK_MPLL,
    X1830_CLK_VPLL, X1830_CLK_EPLL },
    .mux = { CGU_REG_MACCDR, 30, 2 },
    .div = { CGU_REG_MACCDR, 0, 1, 8, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR1, 4 },
    },
    [X1830_CLK_LCD] = {
    "lcd", CGU_CLK_MUX | CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { X1830_CLK_SCLKA, X1830_CLK_MPLL,
    X1830_CLK_VPLL, X1830_CLK_EPLL },
    .mux = { CGU_REG_LPCDR, 30, 2 },
    .div = { CGU_REG_LPCDR, 0, 1, 8, 28, 27, 26 },
    .gate = { CGU_REG_CLKGR1, 9 },
    },
    [X1830_CLK_MSCMUX] = {
    "msc_mux", CGU_CLK_MUX,
    .parents = { X1830_CLK_SCLKA, X1830_CLK_MPLL,
    X1830_CLK_VPLL, X1830_CLK_EPLL },
    .mux = { CGU_REG_MSC0CDR, 30, 2 },
    },
    [X1830_CLK_MSC0] = {
    "msc0", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { X1830_CLK_MSCMUX, -1, -1, -1 },
    .div = { CGU_REG_MSC0CDR, 0, 2, 8, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR0, 4 },
    },
    [X1830_CLK_MSC1] = {
    "msc1", CGU_CLK_DIV | CGU_CLK_GATE,
    .parents = { X1830_CLK_MSCMUX, -1, -1, -1 },
    .div = { CGU_REG_MSC1CDR, 0, 2, 8, 29, 28, 27 },
    .gate = { CGU_REG_CLKGR0, 5 },
    },
    [X1830_CLK_SSIPLL] = {
    "ssi_pll", CGU_CLK_MUX | CGU_CLK_DIV,
    .parents = { X1830_CLK_SCLKA, X1830_CLK_MPLL,
    X1830_CLK_VPLL, X1830_CLK_EPLL },
    .mux = { CGU_REG_SSICDR, 30, 2 },
    .div = { CGU_REG_SSICDR, 0, 1, 8, 28, 27, 26 },
    },
    [X1830_CLK_SSIPLL_DIV2] = {
    "ssi_pll_div2", CGU_CLK_FIXDIV,
    .parents = { X1830_CLK_SSIPLL },
    .fixdiv = { 2 },
    },
    [X1830_CLK_SSIMUX] = {
    "ssi_mux", CGU_CLK_MUX,
    .parents = { X1830_CLK_EXCLK, X1830_CLK_SSIPLL_DIV2, -1, -1 },
    .mux = { CGU_REG_SSICDR, 29, 1 },
    },
    [X1830_CLK_EXCLK_DIV512] = {
    "exclk_div512", CGU_CLK_FIXDIV,
    .parents = { X1830_CLK_EXCLK },
    .fixdiv = { 512 },
    },
    [X1830_CLK_RTC] = {
    "rtc_ercs", CGU_CLK_MUX | CGU_CLK_GATE,
    .parents = { X1830_CLK_EXCLK_DIV512, X1830_CLK_RTCLK },
    .mux = { CGU_REG_OPCR, 2, 1},
    .gate = { CGU_REG_CLKGR0, 29 },
    },
// Gate-only clocks
    [X1830_CLK_EMC] = {
    "emc", CGU_CLK_GATE,
    .parents = { X1830_CLK_AHB2, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 0 },
    },
    [X1830_CLK_EFUSE] = {
    "efuse", CGU_CLK_GATE,
    .parents = { X1830_CLK_AHB2, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 1 },
    },
    [X1830_CLK_OTG] = {
    "otg", CGU_CLK_GATE,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 3 },
    },
    [X1830_CLK_SSI0] = {
    "ssi0", CGU_CLK_GATE,
    .parents = { X1830_CLK_SSIMUX, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 6 },
    },
    [X1830_CLK_SMB0] = {
    "smb0", CGU_CLK_GATE,
    .parents = { X1830_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 7 },
    },
    [X1830_CLK_SMB1] = {
    "smb1", CGU_CLK_GATE,
    .parents = { X1830_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 8 },
    },
    [X1830_CLK_SMB2] = {
    "smb2", CGU_CLK_GATE,
    .parents = { X1830_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 9 },
    },
    [X1830_CLK_UART0] = {
    "uart0", CGU_CLK_GATE,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 14 },
    },
    [X1830_CLK_UART1] = {
    "uart1", CGU_CLK_GATE,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 15 },
    },
    [X1830_CLK_SSI1] = {
    "ssi1", CGU_CLK_GATE,
    .parents = { X1830_CLK_SSIMUX, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 19 },
    },
    [X1830_CLK_SFC] = {
    "sfc", CGU_CLK_GATE,
    .parents = { X1830_CLK_SSIPLL, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 20 },
    },
    [X1830_CLK_PDMA] = {
    "pdma", CGU_CLK_GATE,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 21 },
    },
    [X1830_CLK_TCU] = {
    "tcu", CGU_CLK_GATE,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR0, 30 },
    },
    [X1830_CLK_DTRNG] = {
    "dtrng", CGU_CLK_GATE,
    .parents = { X1830_CLK_PCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 1 },
    },
    [X1830_CLK_OST] = {
    "ost", CGU_CLK_GATE,
    .parents = { X1830_CLK_EXCLK, -1, -1, -1 },
    .gate = { CGU_REG_CLKGR1, 11 },
    },
    };
#[no_mangle]
unsafe extern "C" fn x1830_cgu_init(np: *mut device_node) -> void __init {
    static void __init x1830_cgu_init(struct device_node *np)
    {
    int retval;
    cgu = ingenic_cgu_new(x1830_cgu_clocks,
    ARRAY_SIZE(x1830_cgu_clocks), np);
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
//
// CGU has some children devices, this is useful for probing children devices
// in the case where the device node is compatible with "simple-mfd".
//
    CLK_OF_DECLARE_DRIVER(x1830_cgu, "ingenic,x1830-cgu", x1830_cgu_init);
