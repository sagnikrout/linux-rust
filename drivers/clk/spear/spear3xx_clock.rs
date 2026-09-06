//! Automatically rewritten from C to Rust
//! Source: drivers/clk/spear/spear3xx_clock.c
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
// SPEAr3xx machines clock framework source file
//
// Copyright (C) 2012 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//

    static DEFINE_SPINLOCK(_lock);

// PLL_CLK_CFG register masks
pub const MCTR_CLK_SHIFT: c_int = 28;
pub const MCTR_CLK_MASK: c_int = 3;

// CORE CLK CFG register masks
pub const GEN_SYNTH2_3_CLK_SHIFT: c_int = 18;
pub const GEN_SYNTH2_3_CLK_MASK: c_int = 1;
pub const HCLK_RATIO_SHIFT: c_int = 10;
pub const HCLK_RATIO_MASK: c_int = 2;
pub const PCLK_RATIO_SHIFT: c_int = 8;
pub const PCLK_RATIO_MASK: c_int = 2;

// PERIP_CLK_CFG register masks
pub const UART_CLK_SHIFT: c_int = 4;
pub const UART_CLK_MASK: c_int = 1;
pub const FIRDA_CLK_SHIFT: c_int = 5;
pub const FIRDA_CLK_MASK: c_int = 2;
pub const GPT0_CLK_SHIFT: c_int = 8;
pub const GPT1_CLK_SHIFT: c_int = 11;
pub const GPT2_CLK_SHIFT: c_int = 12;
pub const GPT_CLK_MASK: c_int = 1;

// PERIP1_CLK_ENB register masks
pub const UART_CLK_ENB: c_int = 3;
pub const SSP_CLK_ENB: c_int = 5;
pub const I2C_CLK_ENB: c_int = 7;
pub const JPEG_CLK_ENB: c_int = 8;
pub const FIRDA_CLK_ENB: c_int = 10;
pub const GPT1_CLK_ENB: c_int = 11;
pub const GPT2_CLK_ENB: c_int = 12;
pub const ADC_CLK_ENB: c_int = 15;
pub const RTC_CLK_ENB: c_int = 17;
pub const GPIO_CLK_ENB: c_int = 18;
pub const DMA_CLK_ENB: c_int = 19;
pub const SMI_CLK_ENB: c_int = 21;
pub const GMAC_CLK_ENB: c_int = 23;
pub const USBD_CLK_ENB: c_int = 24;
pub const USBH_CLK_ENB: c_int = 25;
pub const C3_CLK_ENB: c_int = 31;

pub const RAS_AHB_CLK_ENB: c_int = 0;
pub const RAS_PLL1_CLK_ENB: c_int = 1;
pub const RAS_APB_CLK_ENB: c_int = 2;
pub const RAS_32K_CLK_ENB: c_int = 3;
pub const RAS_24M_CLK_ENB: c_int = 4;
pub const RAS_48M_CLK_ENB: c_int = 5;
pub const RAS_PLL2_CLK_ENB: c_int = 7;
pub const RAS_SYNT0_CLK_ENB: c_int = 8;
pub const RAS_SYNT1_CLK_ENB: c_int = 9;
pub const RAS_SYNT2_CLK_ENB: c_int = 10;
pub const RAS_SYNT3_CLK_ENB: c_int = 11;

pub const AMEM_CLK_ENB: c_int = 0;

// pll rate configuration table, in ascending order of rates
    static struct pll_rate_tbl pll_rtbl[] = {
    {.mode = 0, .m = 0x53, .n = 0x0C, .p = 0x1}, /* vco 332 & pll 166 MHz */
    {.mode = 0, .m = 0x85, .n = 0x0C, .p = 0x1}, /* vco 532 & pll 266 MHz */
    {.mode = 0, .m = 0xA6, .n = 0x0C, .p = 0x1}, /* vco 664 & pll 332 MHz */
    };
// aux rate configuration table, in ascending order of rates
    static struct aux_rate_tbl aux_rtbl[] = {
// For PLL1 = 332 MHz
    {.xscale = 1, .yscale = 81, .eq = 0}, /* 2.049 MHz */
    {.xscale = 1, .yscale = 59, .eq = 0}, /* 2.822 MHz */
    {.xscale = 2, .yscale = 81, .eq = 0}, /* 4.098 MHz */
    {.xscale = 3, .yscale = 89, .eq = 0}, /* 5.644 MHz */
    {.xscale = 4, .yscale = 81, .eq = 0}, /* 8.197 MHz */
    {.xscale = 4, .yscale = 59, .eq = 0}, /* 11.254 MHz */
    {.xscale = 2, .yscale = 27, .eq = 0}, /* 12.296 MHz */
    {.xscale = 2, .yscale = 8, .eq = 0}, /* 41.5 MHz */
    {.xscale = 2, .yscale = 4, .eq = 0}, /* 83 MHz */
    {.xscale = 1, .yscale = 2, .eq = 1}, /* 166 MHz */
    };
// gpt rate configuration table, in ascending order of rates
    static struct gpt_rate_tbl gpt_rtbl[] = {
// For pll1 = 332 MHz
    {.mscale = 4, .nscale = 0}, /* 41.5 MHz */
    {.mscale = 2, .nscale = 0}, /* 55.3 MHz */
    {.mscale = 1, .nscale = 0}, /* 83 MHz */
    };
// clock parents
    static const char *uart0_parents[] = { "pll3_clk", "uart_syn_gclk", };
    static const char *firda_parents[] = { "pll3_clk", "firda_syn_gclk",
    };
    static const char *gpt0_parents[] = { "pll3_clk", "gpt0_syn_clk", };
    static const char *gpt1_parents[] = { "pll3_clk", "gpt1_syn_clk", };
    static const char *gpt2_parents[] = { "pll3_clk", "gpt2_syn_clk", };
    static const char *gen2_3_parents[] = { "pll1_clk", "pll2_clk", };
    static const char *ddr_parents[] = { "ahb_clk", "ahbmult2_clk", "none",
    "pll2_clk", };

#[no_mangle]
unsafe extern "C" fn spear300_clk_init() -> void __init {
    static void __init spear300_clk_init(void)
    {
    struct clk *clk;
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "clcd_clk", "ras_pll3_clk", 0,
    1, 1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "60000000.clcd");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "fsmc_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "94000000.flash");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "sdhci_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "70000000.sdhci");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "gpio1_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a9000000.gpio");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "kbd_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a0000000.kbd");
    }

    static inline void spear300_clk_init(void) { }

// array of all spear 310 clock lookups

#[no_mangle]
unsafe extern "C" fn spear310_clk_init() -> void __init {
    static void __init spear310_clk_init(void)
    {
    struct clk *clk;
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "emi_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, "emi", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "fsmc_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "44000000.flash");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "tdm_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "tdm");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "uart1_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2000000.serial");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "uart2_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2080000.serial");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "uart3_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2100000.serial");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "uart4_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2180000.serial");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "uart5_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2200000.serial");
    }

    static inline void spear310_clk_init(void) { }

// array of all spear 320 clock lookups

pub const SPEAR320_UARTX_PCLK_MASK: c_uint = 0x1;
pub const SPEAR320_UART2_PCLK_SHIFT: c_int = 8;
pub const SPEAR320_UART3_PCLK_SHIFT: c_int = 9;
pub const SPEAR320_UART4_PCLK_SHIFT: c_int = 10;
pub const SPEAR320_UART5_PCLK_SHIFT: c_int = 11;
pub const SPEAR320_UART6_PCLK_SHIFT: c_int = 12;
pub const SPEAR320_RS485_PCLK_SHIFT: c_int = 13;
pub const SMII_PCLK_SHIFT: c_int = 18;
pub const SMII_PCLK_MASK: c_int = 2;
pub const SMII_PCLK_VAL_PAD: c_uint = 0x0;
pub const SMII_PCLK_VAL_PLL2: c_uint = 0x1;
pub const SMII_PCLK_VAL_SYNTH0: c_uint = 0x2;
pub const SDHCI_PCLK_SHIFT: c_int = 15;
pub const SDHCI_PCLK_MASK: c_int = 1;
pub const SDHCI_PCLK_VAL_48M: c_uint = 0x0;
pub const SDHCI_PCLK_VAL_SYNTH3: c_uint = 0x1;
pub const I2S_REF_PCLK_SHIFT: c_int = 8;
pub const I2S_REF_PCLK_MASK: c_int = 1;
pub const I2S_REF_PCLK_SYNTH_VAL: c_uint = 0x1;
pub const I2S_REF_PCLK_PLL2_VAL: c_uint = 0x0;
pub const UART1_PCLK_SHIFT: c_int = 6;
pub const UART1_PCLK_MASK: c_int = 1;
pub const SPEAR320_UARTX_PCLK_VAL_SYNTH1: c_uint = 0x0;
pub const SPEAR320_UARTX_PCLK_VAL_APB: c_uint = 0x1;
    static const char *i2s_ref_parents[] = { "ras_pll2_clk", "ras_syn2_gclk", };
    static const char *sdhci_parents[] = { "ras_pll3_clk", "ras_syn3_gclk", };
    static const char *smii0_parents[] = { "smii_125m_pad", "ras_pll2_clk",
    "ras_syn0_gclk", };
    static const char *uartx_parents[] = { "ras_syn1_gclk", "ras_apb_clk", };
    static void __init spear320_clk_init(void __iomem *soc_config_base,
    struct clk *ras_apb_clk)
    {
    struct clk *clk;
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "smii_125m_pad_clk", core::ptr::null_mut(),
    0, 125000000);
    clk_register_clkdev(clk, "smii_125m_pad", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "clcd_clk", "ras_pll3_clk", 0,
    1, 1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "90000000.clcd");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "emi_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, "emi", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "fsmc_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "4c000000.flash");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "i2c1_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a7000000.i2c");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "pwm_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a8000000.pwm");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "ssp1_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a5000000.spi");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "ssp2_clk", "ras_ahb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a6000000.spi");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "can0_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "c_can_platform.0");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "can1_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "c_can_platform.1");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "i2s_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a9400000.i2s");
    clk = clk_register_mux(core::ptr::null_mut(), "i2s_ref_clk", i2s_ref_parents,
    ARRAY_SIZE(i2s_ref_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR320_CONTROL_REG, I2S_REF_PCLK_SHIFT,
    I2S_REF_PCLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2s_ref_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "i2s_sclk", "i2s_ref_clk",
    CLK_SET_RATE_PARENT, 1,
    4);
    clk_register_clkdev(clk, "i2s_sclk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "macb1_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, "hclk", "aa000000.eth");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "macb2_clk", "ras_apb_clk", 0, 1,
    1);
    clk_register_clkdev(clk, "hclk", "ab000000.eth");
    clk = clk_register_mux(core::ptr::null_mut(), "rs485_clk", uartx_parents,
    ARRAY_SIZE(uartx_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR320_EXT_CTRL_REG, SPEAR320_RS485_PCLK_SHIFT,
    SPEAR320_UARTX_PCLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a9300000.serial");
    clk = clk_register_mux(core::ptr::null_mut(), "sdhci_clk", sdhci_parents,
    ARRAY_SIZE(sdhci_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR320_CONTROL_REG, SDHCI_PCLK_SHIFT, SDHCI_PCLK_MASK,
    0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "70000000.sdhci");
    clk = clk_register_mux(core::ptr::null_mut(), "smii_pclk", smii0_parents,
    ARRAY_SIZE(smii0_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR320_CONTROL_REG, SMII_PCLK_SHIFT, SMII_PCLK_MASK,
    0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "smii_pclk");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "smii_clk", "smii_pclk", 0, 1, 1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "smii");
    clk = clk_register_mux(core::ptr::null_mut(), "uart1_clk", uartx_parents,
    ARRAY_SIZE(uartx_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR320_CONTROL_REG, UART1_PCLK_SHIFT, UART1_PCLK_MASK,
    0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a3000000.serial");
// Enforce ras_apb_clk
    clk_set_parent(clk, ras_apb_clk);
    clk = clk_register_mux(core::ptr::null_mut(), "uart2_clk", uartx_parents,
    ARRAY_SIZE(uartx_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR320_EXT_CTRL_REG, SPEAR320_UART2_PCLK_SHIFT,
    SPEAR320_UARTX_PCLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a4000000.serial");
// Enforce ras_apb_clk
    clk_set_parent(clk, ras_apb_clk);
    clk = clk_register_mux(core::ptr::null_mut(), "uart3_clk", uartx_parents,
    ARRAY_SIZE(uartx_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR320_EXT_CTRL_REG, SPEAR320_UART3_PCLK_SHIFT,
    SPEAR320_UARTX_PCLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a9100000.serial");
    clk = clk_register_mux(core::ptr::null_mut(), "uart4_clk", uartx_parents,
    ARRAY_SIZE(uartx_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR320_EXT_CTRL_REG, SPEAR320_UART4_PCLK_SHIFT,
    SPEAR320_UARTX_PCLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "a9200000.serial");
    clk = clk_register_mux(core::ptr::null_mut(), "uart5_clk", uartx_parents,
    ARRAY_SIZE(uartx_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR320_EXT_CTRL_REG, SPEAR320_UART5_PCLK_SHIFT,
    SPEAR320_UARTX_PCLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "60000000.serial");
    clk = clk_register_mux(core::ptr::null_mut(), "uart6_clk", uartx_parents,
    ARRAY_SIZE(uartx_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR320_EXT_CTRL_REG, SPEAR320_UART6_PCLK_SHIFT,
    SPEAR320_UARTX_PCLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "60100000.serial");
    }

    static inline void spear320_clk_init(void __iomem *sb, struct clk *rc) { }

#[no_mangle]
pub unsafe extern "C" fn spear3xx_clk_init(misc_base: *mut void __iomem, soc_config_base: *mut void __iomem) -> void __init {
    void __init spear3xx_clk_init(void __iomem *misc_base, void __iomem *soc_config_base)
    {
    struct clk *clk, *clk1, *ras_apb_clk;
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "osc_32k_clk", core::ptr::null_mut(), 0, 32000);
    clk_register_clkdev(clk, "osc_32k_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "osc_24m_clk", core::ptr::null_mut(), 0, 24000000);
    clk_register_clkdev(clk, "osc_24m_clk", core::ptr::null_mut());
// clock derived from 32 KHz osc clk
    clk = clk_register_gate(core::ptr::null_mut(), "rtc-spear", "osc_32k_clk", 0,
    PERIP1_CLK_ENB, RTC_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc900000.rtc");
// clock derived from 24 MHz osc clk
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "pll3_clk", "osc_24m_clk", 0,
    48000000);
    clk_register_clkdev(clk, "pll3_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "wdt_clk", "osc_24m_clk", 0, 1,
    1);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc880000.wdt");
    clk = clk_register_vco_pll("vco1_clk", "pll1_clk", core::ptr::null_mut(),
    "osc_24m_clk", 0, PLL1_CTR, PLL1_FRQ, pll_rtbl,
    ARRAY_SIZE(pll_rtbl), &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco1_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll1_clk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco2_clk", "pll2_clk", core::ptr::null_mut(),
    "osc_24m_clk", 0, PLL2_CTR, PLL2_FRQ, pll_rtbl,
    ARRAY_SIZE(pll_rtbl), &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco2_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll2_clk", core::ptr::null_mut());
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
    clk = clk_register_mux(core::ptr::null_mut(), "uart0_mclk", uart0_parents,
    ARRAY_SIZE(uart0_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, UART_CLK_SHIFT, UART_CLK_MASK, 0,
    &_lock);
    clk_register_clkdev(clk, "uart0_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart0", "uart0_mclk",
    CLK_SET_RATE_PARENT, PERIP1_CLK_ENB, UART_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0000000.serial");
    clk = clk_register_aux("firda_syn_clk", "firda_syn_gclk", "pll1_clk", 0,
    FIRDA_CLK_SYNT, core::ptr::null_mut(), aux_rtbl, ARRAY_SIZE(aux_rtbl),
    &_lock, &clk1);
    clk_register_clkdev(clk, "firda_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "firda_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "firda_mclk", firda_parents,
    ARRAY_SIZE(firda_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, FIRDA_CLK_SHIFT, FIRDA_CLK_MASK, 0,
    &_lock);
    clk_register_clkdev(clk, "firda_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "firda_clk", "firda_mclk",
    CLK_SET_RATE_PARENT, PERIP1_CLK_ENB, FIRDA_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "firda");
// gpt clocks
    clk_register_gpt("gpt0_syn_clk", "pll1_clk", 0, PRSC0_CLK_CFG, gpt_rtbl,
    ARRAY_SIZE(gpt_rtbl), &_lock);
    clk = clk_register_mux(core::ptr::null_mut(), "gpt0_clk", gpt0_parents,
    ARRAY_SIZE(gpt0_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, GPT0_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt0");
    clk_register_gpt("gpt1_syn_clk", "pll1_clk", 0, PRSC1_CLK_CFG, gpt_rtbl,
    ARRAY_SIZE(gpt_rtbl), &_lock);
    clk = clk_register_mux(core::ptr::null_mut(), "gpt1_mclk", gpt1_parents,
    ARRAY_SIZE(gpt1_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, GPT1_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt1_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt1_clk", "gpt1_mclk",
    CLK_SET_RATE_PARENT, PERIP1_CLK_ENB, GPT1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt1");
    clk_register_gpt("gpt2_syn_clk", "pll1_clk", 0, PRSC2_CLK_CFG, gpt_rtbl,
    ARRAY_SIZE(gpt_rtbl), &_lock);
    clk = clk_register_mux(core::ptr::null_mut(), "gpt2_mclk", gpt2_parents,
    ARRAY_SIZE(gpt2_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    PERIP_CLK_CFG, GPT2_CLK_SHIFT, GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt2_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt2_clk", "gpt2_mclk",
    CLK_SET_RATE_PARENT, PERIP1_CLK_ENB, GPT2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt2");
// general synths clocks
    clk = clk_register_aux("gen0_syn_clk", "gen0_syn_gclk", "pll1_clk",
    0, GEN0_CLK_SYNT, core::ptr::null_mut(), aux_rtbl, ARRAY_SIZE(aux_rtbl),
    &_lock, &clk1);
    clk_register_clkdev(clk, "gen0_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "gen0_syn_gclk", core::ptr::null_mut());
    clk = clk_register_aux("gen1_syn_clk", "gen1_syn_gclk", "pll1_clk",
    0, GEN1_CLK_SYNT, core::ptr::null_mut(), aux_rtbl, ARRAY_SIZE(aux_rtbl),
    &_lock, &clk1);
    clk_register_clkdev(clk, "gen1_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "gen1_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "gen2_3_par_clk", gen2_3_parents,
    ARRAY_SIZE(gen2_3_parents), CLK_SET_RATE_NO_REPARENT,
    CORE_CLK_CFG, GEN_SYNTH2_3_CLK_SHIFT,
    GEN_SYNTH2_3_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gen2_3_par_clk", core::ptr::null_mut());
    clk = clk_register_aux("gen2_syn_clk", "gen2_syn_gclk",
    "gen2_3_par_clk", 0, GEN2_CLK_SYNT, core::ptr::null_mut(), aux_rtbl,
    ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "gen2_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "gen2_syn_gclk", core::ptr::null_mut());
    clk = clk_register_aux("gen3_syn_clk", "gen3_syn_gclk",
    "gen2_3_par_clk", 0, GEN3_CLK_SYNT, core::ptr::null_mut(), aux_rtbl,
    ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "gen3_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "gen3_syn_gclk", core::ptr::null_mut());
// clock derived from pll3 clk
    clk = clk_register_gate(core::ptr::null_mut(), "usbh_clk", "pll3_clk", 0, PERIP1_CLK_ENB,
    USBH_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e1800000.ehci");
    clk_register_clkdev(clk, core::ptr::null_mut(), "e1900000.ohci");
    clk_register_clkdev(clk, core::ptr::null_mut(), "e2100000.ohci");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "usbh.0_clk", "usbh_clk", 0, 1,
    1);
    clk_register_clkdev(clk, "usbh.0_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "usbh.1_clk", "usbh_clk", 0, 1,
    1);
    clk_register_clkdev(clk, "usbh.1_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "usbd_clk", "pll3_clk", 0, PERIP1_CLK_ENB,
    USBD_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e1100000.usbd");
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
    clk = clk_register_gate(core::ptr::null_mut(), "amem_clk", "ahb_clk", 0, AMEM_CLK_CFG,
    AMEM_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, "amem_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "c3_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    C3_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "c3_clk");
    clk = clk_register_gate(core::ptr::null_mut(), "dma_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    DMA_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc400000.dma");
    clk = clk_register_gate(core::ptr::null_mut(), "gmac_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    GMAC_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0800000.eth");
    clk = clk_register_gate(core::ptr::null_mut(), "i2c0_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    I2C_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0180000.i2c");
    clk = clk_register_gate(core::ptr::null_mut(), "jpeg_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    JPEG_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "jpeg");
    clk = clk_register_gate(core::ptr::null_mut(), "smi_clk", "ahb_clk", 0, PERIP1_CLK_ENB,
    SMI_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc000000.flash");
// clock derived from apb clk
    clk = clk_register_gate(core::ptr::null_mut(), "adc_clk", "apb_clk", 0, PERIP1_CLK_ENB,
    ADC_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0080000.adc");
    clk = clk_register_gate(core::ptr::null_mut(), "gpio0_clk", "apb_clk", 0, PERIP1_CLK_ENB,
    GPIO_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "fc980000.gpio");
    clk = clk_register_gate(core::ptr::null_mut(), "ssp0_clk", "apb_clk", 0, PERIP1_CLK_ENB,
    SSP_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0100000.spi");
// RAS clk enable
    clk = clk_register_gate(core::ptr::null_mut(), "ras_ahb_clk", "ahb_clk", 0, RAS_CLK_ENB,
    RAS_AHB_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, "ras_ahb_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_apb_clk", "apb_clk", 0, RAS_CLK_ENB,
    RAS_APB_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, "ras_apb_clk", core::ptr::null_mut());
    ras_apb_clk = clk;
    clk = clk_register_gate(core::ptr::null_mut(), "ras_32k_clk", "osc_32k_clk", 0,
    RAS_CLK_ENB, RAS_32K_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, "ras_32k_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_24m_clk", "osc_24m_clk", 0,
    RAS_CLK_ENB, RAS_24M_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, "ras_24m_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_pll1_clk", "pll1_clk", 0,
    RAS_CLK_ENB, RAS_PLL1_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, "ras_pll1_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_pll2_clk", "pll2_clk", 0,
    RAS_CLK_ENB, RAS_PLL2_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, "ras_pll2_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_pll3_clk", "pll3_clk", 0,
    RAS_CLK_ENB, RAS_48M_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, "ras_pll3_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_syn0_gclk", "gen0_syn_gclk",
    CLK_SET_RATE_PARENT, RAS_CLK_ENB, RAS_SYNT0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_syn0_gclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_syn1_gclk", "gen1_syn_gclk",
    CLK_SET_RATE_PARENT, RAS_CLK_ENB, RAS_SYNT1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_syn1_gclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_syn2_gclk", "gen2_syn_gclk",
    CLK_SET_RATE_PARENT, RAS_CLK_ENB, RAS_SYNT2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_syn2_gclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_syn3_gclk", "gen3_syn_gclk",
    CLK_SET_RATE_PARENT, RAS_CLK_ENB, RAS_SYNT3_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_syn3_gclk", core::ptr::null_mut());
    if (of_machine_is_compatible("st,spear300"))
    spear300_clk_init();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: of_machine_is_compatible("st, _arg: spear310")) -> else {
    else if (of_machine_is_compatible("st,spear310"))
    spear310_clk_init();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: of_machine_is_compatible("st, _arg: spear320")) -> else {
    else if (of_machine_is_compatible("st,spear320"))
    spear320_clk_init(soc_config_base, ras_apb_clk);
    }
