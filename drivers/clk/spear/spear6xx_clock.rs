//! Automatically rewritten from C to Rust
//! Source: drivers/clk/spear/spear6xx_clock.c
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
// SPEAr6xx machines clock framework source file
//
// Copyright (C) 2012 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//

    static DEFINE_SPINLOCK(_lock);

// PLL_CLK_CFG register masks
pub const MCTR_CLK_SHIFT: c_int = 28;
pub const MCTR_CLK_MASK: c_int = 3;

// CORE CLK CFG register masks
pub const HCLK_RATIO_SHIFT: c_int = 10;
pub const HCLK_RATIO_MASK: c_int = 2;
pub const PCLK_RATIO_SHIFT: c_int = 8;
pub const PCLK_RATIO_MASK: c_int = 2;

// PERIP_CLK_CFG register masks
pub const CLCD_CLK_SHIFT: c_int = 2;
pub const CLCD_CLK_MASK: c_int = 2;
pub const UART_CLK_SHIFT: c_int = 4;
pub const UART_CLK_MASK: c_int = 1;
pub const FIRDA_CLK_SHIFT: c_int = 5;
pub const FIRDA_CLK_MASK: c_int = 2;
pub const GPT0_CLK_SHIFT: c_int = 8;
pub const GPT1_CLK_SHIFT: c_int = 10;
pub const GPT2_CLK_SHIFT: c_int = 11;
pub const GPT3_CLK_SHIFT: c_int = 12;
pub const GPT_CLK_MASK: c_int = 1;

// PERIP1_CLK_ENB register masks
pub const UART0_CLK_ENB: c_int = 3;
pub const UART1_CLK_ENB: c_int = 4;
pub const SSP0_CLK_ENB: c_int = 5;
pub const SSP1_CLK_ENB: c_int = 6;
pub const I2C_CLK_ENB: c_int = 7;
pub const JPEG_CLK_ENB: c_int = 8;
pub const FSMC_CLK_ENB: c_int = 9;
pub const FIRDA_CLK_ENB: c_int = 10;
pub const GPT2_CLK_ENB: c_int = 11;
pub const GPT3_CLK_ENB: c_int = 12;
pub const GPIO2_CLK_ENB: c_int = 13;
pub const SSP2_CLK_ENB: c_int = 14;
pub const ADC_CLK_ENB: c_int = 15;
pub const GPT1_CLK_ENB: c_int = 11;
pub const RTC_CLK_ENB: c_int = 17;
pub const GPIO1_CLK_ENB: c_int = 18;
pub const DMA_CLK_ENB: c_int = 19;
pub const SMI_CLK_ENB: c_int = 21;
pub const CLCD_CLK_ENB: c_int = 22;
pub const GMAC_CLK_ENB: c_int = 23;
pub const USBD_CLK_ENB: c_int = 24;
pub const USBH0_CLK_ENB: c_int = 25;
pub const USBH1_CLK_ENB: c_int = 26;

// vco rate configuration table, in ascending order of rates
    static struct pll_rate_tbl pll_rtbl[] = {
    {.mode = 0, .m = 0x53, .n = 0x0F, .p = 0x1}, /* vco 332 & pll 166 MHz */
    {.mode = 0, .m = 0x85, .n = 0x0F, .p = 0x1}, /* vco 532 & pll 266 MHz */
    {.mode = 0, .m = 0xA6, .n = 0x0F, .p = 0x1}, /* vco 664 & pll 332 MHz */
    };
// aux rate configuration table, in ascending order of rates
    static struct aux_rate_tbl aux_rtbl[] = {
// For PLL1 = 332 MHz
    {.xscale = 2, .yscale = 27, .eq = 0}, /* 12.296 MHz */
    {.xscale = 2, .yscale = 8, .eq = 0}, /* 41.5 MHz */
    {.xscale = 2, .yscale = 4, .eq = 0}, /* 83 MHz */
    {.xscale = 1, .yscale = 2, .eq = 1}, /* 166 MHz */
    };
    static const char *clcd_parents[] = { "pll3_clk", "clcd_syn_gclk", };
    static const char *firda_parents[] = { "pll3_clk", "firda_syn_gclk", };
    static const char *uart_parents[] = { "pll3_clk", "uart_syn_gclk", };
    static const char *gpt0_1_parents[] = { "pll3_clk", "gpt0_1_syn_clk", };
    static const char *gpt2_parents[] = { "pll3_clk", "gpt2_syn_clk", };
    static const char *gpt3_parents[] = { "pll3_clk", "gpt3_syn_clk", };
    static const char *ddr_parents[] = { "ahb_clk", "ahbmult2_clk", "none",
    "pll2_clk", };
// gpt rate configuration table, in ascending order of rates
    static struct gpt_rate_tbl gpt_rtbl[] = {
// For pll1 = 332 MHz
    {.mscale = 4, .nscale = 0}, /* 41.5 MHz */
    {.mscale = 2, .nscale = 0}, /* 55.3 MHz */
    {.mscale = 1, .nscale = 0}, /* 83 MHz */
    };
#[no_mangle]
pub unsafe extern "C" fn spear6xx_clk_init(misc_base: *mut void __iomem) -> void __init {
    void __init spear6xx_clk_init(void __iomem *misc_base)
    {
    struct clk *clk, *clk1;
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "osc_32k_clk", core::ptr::null_mut(), 0, 32000);
    clk_register_clkdev(clk, "osc_32k_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "osc_30m_clk", core::ptr::null_mut(), 0, 30000000);
    clk_register_clkdev(clk, "osc_30m_clk", core::ptr::null_mut());
// clock derived from 32 KHz osc clk
    clk = clk_register_gate(core::ptr::null_mut(), "rtc_spear", "osc_32k_clk", 0,
    PERIP1_CLK_ENB, RTC_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "rtc-spear");
// clock derived from 30 MHz osc clk
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "pll3_clk", "osc_24m_clk", 0,
    48000000);
    clk_register_clkdev(clk, "pll3_clk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco1_clk", "pll1_clk", core::ptr::null_mut(), "osc_30m_clk",
    0, PLL1_CTR, PLL1_FRQ, pll_rtbl, ARRAY_SIZE(pll_rtbl),
    &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco1_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll1_clk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco2_clk", "pll2_clk", core::ptr::null_mut(), "osc_30m_clk",
    0, PLL2_CTR, PLL2_FRQ, pll_rtbl, ARRAY_SIZE(pll_rtbl),
    &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco2_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll2_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "wdt_clk", "osc_30m_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc880000.wdt");
// clock derived from pll1 clk
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "cpu_clk", "pll1_clk",
    CLK_SET_RATE_PARENT, 1, 1);
    clk_register_clkdev(clk, "cpu_clk", core::ptr::null_mut());
    clk = clk_register_divider(core::ptr::null_mut(), "ahb_clk", "pll1_clk",
    CLK_SET_RATE_PARENT, CORE_CLK_CFG, HCLK_RATIO_SHIFT,
    HCLK_RATIO_MASK, 0, &_lock);
    clk_register_clkdev(clk, "ahb_clk", core::ptr::null_mut());
    clk = clk_register_aux("uart_syn_clk", "uart_syn_gclk", "pll1_clk", 0,
    UART_CLK_SYNT, core::ptr::null_mut(), aux_rtbl, ARRAY_SIZE(aux_rtbl),
    &_lock, &clk1);
    clk_register_clkdev(clk, "uart_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "uart_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "uart_mclk", uart_parents,
    ARRAY_SIZE(uart_parents), CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, UART_CLK_SHIFT, UART_CLK_MASK, 0,
    &_lock);
    clk_register_clkdev(clk, "uart_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart0", "uart_mclk", 0, PERIP1_CLK_ENB,
    UART0_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0000000.serial");
    clk = clk_register_gate(core::ptr::null_mut(), "uart1", "uart_mclk", 0, PERIP1_CLK_ENB,
    UART1_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0080000.serial");
    clk = clk_register_aux("firda_syn_clk", "firda_syn_gclk", "pll1_clk",
    0, FIRDA_CLK_SYNT, core::ptr::null_mut(), aux_rtbl, ARRAY_SIZE(aux_rtbl),
    &_lock, &clk1);
    clk_register_clkdev(clk, "firda_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "firda_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "firda_mclk", firda_parents,
    ARRAY_SIZE(firda_parents), CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, FIRDA_CLK_SHIFT, FIRDA_CLK_MASK, 0,
    &_lock);
    clk_register_clkdev(clk, "firda_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "firda_clk", "firda_mclk", 0,
    PERIP1_CLK_ENB, FIRDA_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "firda");
    clk = clk_register_aux("clcd_syn_clk", "clcd_syn_gclk", "pll1_clk",
    0, CLCD_CLK_SYNT, core::ptr::null_mut(), aux_rtbl, ARRAY_SIZE(aux_rtbl),
    &_lock, &clk1);
    clk_register_clkdev(clk, "clcd_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "clcd_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "clcd_mclk", clcd_parents,
    ARRAY_SIZE(clcd_parents), CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, CLCD_CLK_SHIFT, CLCD_CLK_MASK, 0,
    &_lock);
    clk_register_clkdev(clk, "clcd_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "clcd_clk", "clcd_mclk", 0,
    PERIP1_CLK_ENB, CLCD_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc200000.clcd");
// gpt clocks
    clk = clk_register_gpt("gpt0_1_syn_clk", "pll1_clk", 0, PRSC0_CLK_CFG,
    gpt_rtbl, ARRAY_SIZE(gpt_rtbl), &_lock);
    clk_register_clkdev(clk, "gpt0_1_syn_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "gpt0_mclk", gpt0_1_parents,
    ARRAY_SIZE(gpt0_1_parents), CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, GPT0_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt0");
    clk = clk_register_mux(core::ptr::null_mut(), "gpt1_mclk", gpt0_1_parents,
    ARRAY_SIZE(gpt0_1_parents), CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, GPT1_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt1_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt1_clk", "gpt1_mclk", 0,
    PERIP1_CLK_ENB, GPT1_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt1");
    clk = clk_register_gpt("gpt2_syn_clk", "pll1_clk", 0, PRSC1_CLK_CFG,
    gpt_rtbl, ARRAY_SIZE(gpt_rtbl), &_lock);
    clk_register_clkdev(clk, "gpt2_syn_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "gpt2_mclk", gpt2_parents,
    ARRAY_SIZE(gpt2_parents), CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, GPT2_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt2_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt2_clk", "gpt2_mclk", 0,
    PERIP1_CLK_ENB, GPT2_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt2");
    clk = clk_register_gpt("gpt3_syn_clk", "pll1_clk", 0, PRSC2_CLK_CFG,
    gpt_rtbl, ARRAY_SIZE(gpt_rtbl), &_lock);
    clk_register_clkdev(clk, "gpt3_syn_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "gpt3_mclk", gpt3_parents,
    ARRAY_SIZE(gpt3_parents), CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, GPT3_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt3_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt3_clk", "gpt3_mclk", 0,
    PERIP1_CLK_ENB, GPT3_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt3");
// clock derived from pll3 clk
    clk = clk_register_gate(core::ptr::null_mut(), "usbh0_clk", "pll3_clk", 0,
    PERIP1_CLK_ENB, USBH0_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e1800000.ehci");
    clk_register_clkdev(clk, core::ptr::null_mut(), "e1900000.ohci");
    clk = clk_register_gate(core::ptr::null_mut(), "usbh1_clk", "pll3_clk", 0,
    PERIP1_CLK_ENB, USBH1_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e2000000.ehci");
    clk_register_clkdev(clk, core::ptr::null_mut(), "e2100000.ohci");
    clk = clk_register_gate(core::ptr::null_mut(), "usbd_clk", "pll3_clk", 0, PERIP1_CLK_ENB,
    USBD_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "designware_udc");
// clock derived from ahb clk
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "ahbmult2_clk", "ahb_clk", 0, 2,
    1);
    clk_register_clkdev(clk, "ahbmult2_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "ddr_clk", ddr_parents,
    ARRAY_SIZE(ddr_parents), CLK_SET_RATE_NO_REPARENT,
    PLL_CLK_CFG, MCTR_CLK_SHIFT, MCTR_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "ddr_clk", core::ptr::null_mut());
    clk = clk_register_divider(core::ptr::null_mut(), "apb_clk", "ahb_clk",
    CLK_SET_RATE_PARENT, CORE_CLK_CFG, PCLK_RATIO_SHIFT,
    PCLK_RATIO_MASK, 0, &_lock);
    clk_register_clkdev(clk, "apb_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "dma_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    DMA_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc400000.dma");
    clk = clk_register_gate(core::ptr::null_mut(), "fsmc_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    FSMC_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d1800000.flash");
    clk = clk_register_gate(core::ptr::null_mut(), "gmac_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    GMAC_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0800000.ethernet");
    clk = clk_register_gate(core::ptr::null_mut(), "i2c_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    I2C_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0200000.i2c");
    clk = clk_register_gate(core::ptr::null_mut(), "jpeg_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    JPEG_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "jpeg");
    clk = clk_register_gate(core::ptr::null_mut(), "smi_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    SMI_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc000000.flash");
// clock derived from apb clk
    clk = clk_register_gate(core::ptr::null_mut(), "adc_clk", "apb_clk", 0, PERIP1_CLK_ENB,
    ADC_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d820b000.adc");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "gpio0_clk", "apb_clk", 0, 1, 1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "f0100000.gpio");
    clk = clk_register_gate(core::ptr::null_mut(), "gpio1_clk", "apb_clk", 0, PERIP1_CLK_ENB,
    GPIO1_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc980000.gpio");
    clk = clk_register_gate(core::ptr::null_mut(), "gpio2_clk", "apb_clk", 0, PERIP1_CLK_ENB,
    GPIO2_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d8100000.gpio");
    clk = clk_register_gate(core::ptr::null_mut(), "ssp0_clk", "apb_clk", 0, PERIP1_CLK_ENB,
    SSP0_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0100000.spi");
    clk = clk_register_gate(core::ptr::null_mut(), "ssp1_clk", "apb_clk", 0, PERIP1_CLK_ENB,
    SSP1_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0180000.spi");
    clk = clk_register_gate(core::ptr::null_mut(), "ssp2_clk", "apb_clk", 0, PERIP1_CLK_ENB,
    SSP2_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d8180000.spi");
    }
