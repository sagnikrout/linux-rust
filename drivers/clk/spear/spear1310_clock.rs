//! Automatically rewritten from C to Rust
//! Source: drivers/clk/spear/spear1310_clock.c
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
// arch/arm/mach-spear13xx/spear1310_clock.c
//
// SPEAr1310 machine clock framework source file
//
// Copyright (C) 2012 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//

// PLL related registers and bit values

// PLL_CFG bit values
pub const SPEAR1310_CLCD_SYNT_CLK_MASK: c_int = 1;
pub const SPEAR1310_CLCD_SYNT_CLK_SHIFT: c_int = 31;
pub const SPEAR1310_RAS_SYNT2_3_CLK_MASK: c_int = 2;
pub const SPEAR1310_RAS_SYNT2_3_CLK_SHIFT: c_int = 29;
pub const SPEAR1310_RAS_SYNT_CLK_MASK: c_int = 2;
pub const SPEAR1310_RAS_SYNT0_1_CLK_SHIFT: c_int = 27;
pub const SPEAR1310_PLL_CLK_MASK: c_int = 2;
pub const SPEAR1310_PLL3_CLK_SHIFT: c_int = 24;
pub const SPEAR1310_PLL2_CLK_SHIFT: c_int = 22;
pub const SPEAR1310_PLL1_CLK_SHIFT: c_int = 20;

// PERIP_CLK_CFG bit values
pub const SPEAR1310_GPT_OSC24_VAL: c_int = 0;
pub const SPEAR1310_GPT_APB_VAL: c_int = 1;
pub const SPEAR1310_GPT_CLK_MASK: c_int = 1;
pub const SPEAR1310_GPT3_CLK_SHIFT: c_int = 11;
pub const SPEAR1310_GPT2_CLK_SHIFT: c_int = 10;
pub const SPEAR1310_GPT1_CLK_SHIFT: c_int = 9;
pub const SPEAR1310_GPT0_CLK_SHIFT: c_int = 8;
pub const SPEAR1310_UART_CLK_PLL5_VAL: c_int = 0;
pub const SPEAR1310_UART_CLK_OSC24_VAL: c_int = 1;
pub const SPEAR1310_UART_CLK_SYNT_VAL: c_int = 2;
pub const SPEAR1310_UART_CLK_MASK: c_int = 2;
pub const SPEAR1310_UART_CLK_SHIFT: c_int = 4;
pub const SPEAR1310_AUX_CLK_PLL5_VAL: c_int = 0;
pub const SPEAR1310_AUX_CLK_SYNT_VAL: c_int = 1;
pub const SPEAR1310_CLCD_CLK_MASK: c_int = 2;
pub const SPEAR1310_CLCD_CLK_SHIFT: c_int = 2;
pub const SPEAR1310_C3_CLK_MASK: c_int = 1;
pub const SPEAR1310_C3_CLK_SHIFT: c_int = 1;

pub const SPEAR1310_GMAC_PHY_IF_SEL_MASK: c_int = 3;
pub const SPEAR1310_GMAC_PHY_IF_SEL_SHIFT: c_int = 4;
pub const SPEAR1310_GMAC_PHY_CLK_MASK: c_int = 1;
pub const SPEAR1310_GMAC_PHY_CLK_SHIFT: c_int = 3;
pub const SPEAR1310_GMAC_PHY_INPUT_CLK_MASK: c_int = 2;
pub const SPEAR1310_GMAC_PHY_INPUT_CLK_SHIFT: c_int = 1;

// I2S_CLK_CFG register mask
pub const SPEAR1310_I2S_SCLK_X_MASK: c_uint = 0x1F;
pub const SPEAR1310_I2S_SCLK_X_SHIFT: c_int = 27;
pub const SPEAR1310_I2S_SCLK_Y_MASK: c_uint = 0x1F;
pub const SPEAR1310_I2S_SCLK_Y_SHIFT: c_int = 22;
pub const SPEAR1310_I2S_SCLK_EQ_SEL_SHIFT: c_int = 21;
pub const SPEAR1310_I2S_SCLK_SYNTH_ENB: c_int = 20;
pub const SPEAR1310_I2S_PRS1_CLK_X_MASK: c_uint = 0xFF;
pub const SPEAR1310_I2S_PRS1_CLK_X_SHIFT: c_int = 12;
pub const SPEAR1310_I2S_PRS1_CLK_Y_MASK: c_uint = 0xFF;
pub const SPEAR1310_I2S_PRS1_CLK_Y_SHIFT: c_int = 4;
pub const SPEAR1310_I2S_PRS1_EQ_SEL_SHIFT: c_int = 3;
pub const SPEAR1310_I2S_REF_SEL_MASK: c_int = 1;
pub const SPEAR1310_I2S_REF_SHIFT: c_int = 2;
pub const SPEAR1310_I2S_SRC_CLK_MASK: c_int = 2;
pub const SPEAR1310_I2S_SRC_CLK_SHIFT: c_int = 0;

// Check Fractional synthesizer reg masks

// PERIP1_CLK_ENB register masks
pub const SPEAR1310_RTC_CLK_ENB: c_int = 31;
pub const SPEAR1310_ADC_CLK_ENB: c_int = 30;
pub const SPEAR1310_C3_CLK_ENB: c_int = 29;
pub const SPEAR1310_JPEG_CLK_ENB: c_int = 28;
pub const SPEAR1310_CLCD_CLK_ENB: c_int = 27;
pub const SPEAR1310_DMA_CLK_ENB: c_int = 25;
pub const SPEAR1310_GPIO1_CLK_ENB: c_int = 24;
pub const SPEAR1310_GPIO0_CLK_ENB: c_int = 23;
pub const SPEAR1310_GPT1_CLK_ENB: c_int = 22;
pub const SPEAR1310_GPT0_CLK_ENB: c_int = 21;
pub const SPEAR1310_I2S0_CLK_ENB: c_int = 20;
pub const SPEAR1310_I2S1_CLK_ENB: c_int = 19;
pub const SPEAR1310_I2C0_CLK_ENB: c_int = 18;
pub const SPEAR1310_SSP_CLK_ENB: c_int = 17;
pub const SPEAR1310_UART_CLK_ENB: c_int = 15;
pub const SPEAR1310_PCIE_SATA_2_CLK_ENB: c_int = 14;
pub const SPEAR1310_PCIE_SATA_1_CLK_ENB: c_int = 13;
pub const SPEAR1310_PCIE_SATA_0_CLK_ENB: c_int = 12;
pub const SPEAR1310_UOC_CLK_ENB: c_int = 11;
pub const SPEAR1310_UHC1_CLK_ENB: c_int = 10;
pub const SPEAR1310_UHC0_CLK_ENB: c_int = 9;
pub const SPEAR1310_GMAC_CLK_ENB: c_int = 8;
pub const SPEAR1310_CFXD_CLK_ENB: c_int = 7;
pub const SPEAR1310_SDHCI_CLK_ENB: c_int = 6;
pub const SPEAR1310_SMI_CLK_ENB: c_int = 5;
pub const SPEAR1310_FSMC_CLK_ENB: c_int = 4;
pub const SPEAR1310_SYSRAM0_CLK_ENB: c_int = 3;
pub const SPEAR1310_SYSRAM1_CLK_ENB: c_int = 2;
pub const SPEAR1310_SYSROM_CLK_ENB: c_int = 1;
pub const SPEAR1310_BUS_CLK_ENB: c_int = 0;

// PERIP2_CLK_ENB register masks
pub const SPEAR1310_THSENS_CLK_ENB: c_int = 8;
pub const SPEAR1310_I2S_REF_PAD_CLK_ENB: c_int = 7;
pub const SPEAR1310_ACP_CLK_ENB: c_int = 6;
pub const SPEAR1310_GPT3_CLK_ENB: c_int = 5;
pub const SPEAR1310_GPT2_CLK_ENB: c_int = 4;
pub const SPEAR1310_KBD_CLK_ENB: c_int = 3;
pub const SPEAR1310_CPU_DBG_CLK_ENB: c_int = 2;
pub const SPEAR1310_DDR_CORE_CLK_ENB: c_int = 1;
pub const SPEAR1310_DDR_CTRL_CLK_ENB: c_int = 0;

// RAS_CLK_ENB register masks
pub const SPEAR1310_SYNT3_CLK_ENB: c_int = 17;
pub const SPEAR1310_SYNT2_CLK_ENB: c_int = 16;
pub const SPEAR1310_SYNT1_CLK_ENB: c_int = 15;
pub const SPEAR1310_SYNT0_CLK_ENB: c_int = 14;
pub const SPEAR1310_PCLK3_CLK_ENB: c_int = 13;
pub const SPEAR1310_PCLK2_CLK_ENB: c_int = 12;
pub const SPEAR1310_PCLK1_CLK_ENB: c_int = 11;
pub const SPEAR1310_PCLK0_CLK_ENB: c_int = 10;
pub const SPEAR1310_PLL3_CLK_ENB: c_int = 9;
pub const SPEAR1310_PLL2_CLK_ENB: c_int = 8;
pub const SPEAR1310_C125M_PAD_CLK_ENB: c_int = 7;
pub const SPEAR1310_C30M_CLK_ENB: c_int = 6;
pub const SPEAR1310_C48M_CLK_ENB: c_int = 5;
pub const SPEAR1310_OSC_25M_CLK_ENB: c_int = 4;
pub const SPEAR1310_OSC_32K_CLK_ENB: c_int = 3;
pub const SPEAR1310_OSC_24M_CLK_ENB: c_int = 2;
pub const SPEAR1310_PCLK_CLK_ENB: c_int = 1;
pub const SPEAR1310_ACLK_CLK_ENB: c_int = 0;
// RAS Area Control Register

pub const SPEAR1310_SSP1_CLK_MASK: c_int = 3;
pub const SPEAR1310_SSP1_CLK_SHIFT: c_int = 26;
pub const SPEAR1310_TDM_CLK_MASK: c_int = 1;
pub const SPEAR1310_TDM2_CLK_SHIFT: c_int = 24;
pub const SPEAR1310_TDM1_CLK_SHIFT: c_int = 23;
pub const SPEAR1310_I2C_CLK_MASK: c_int = 1;
pub const SPEAR1310_I2C7_CLK_SHIFT: c_int = 22;
pub const SPEAR1310_I2C6_CLK_SHIFT: c_int = 21;
pub const SPEAR1310_I2C5_CLK_SHIFT: c_int = 20;
pub const SPEAR1310_I2C4_CLK_SHIFT: c_int = 19;
pub const SPEAR1310_I2C3_CLK_SHIFT: c_int = 18;
pub const SPEAR1310_I2C2_CLK_SHIFT: c_int = 17;
pub const SPEAR1310_I2C1_CLK_SHIFT: c_int = 16;
pub const SPEAR1310_GPT64_CLK_MASK: c_int = 1;
pub const SPEAR1310_GPT64_CLK_SHIFT: c_int = 15;
pub const SPEAR1310_RAS_UART_CLK_MASK: c_int = 1;
pub const SPEAR1310_UART5_CLK_SHIFT: c_int = 14;
pub const SPEAR1310_UART4_CLK_SHIFT: c_int = 13;
pub const SPEAR1310_UART3_CLK_SHIFT: c_int = 12;
pub const SPEAR1310_UART2_CLK_SHIFT: c_int = 11;
pub const SPEAR1310_UART1_CLK_SHIFT: c_int = 10;
pub const SPEAR1310_PCI_CLK_MASK: c_int = 1;
pub const SPEAR1310_PCI_CLK_SHIFT: c_int = 0;

pub const SPEAR1310_PHY_CLK_MASK: c_uint = 0x3;
pub const SPEAR1310_RMII_PHY_CLK_SHIFT: c_int = 0;
pub const SPEAR1310_SMII_RGMII_PHY_CLK_SHIFT: c_int = 2;

pub const SPEAR1310_CAN1_CLK_ENB: c_int = 25;
pub const SPEAR1310_CAN0_CLK_ENB: c_int = 24;
pub const SPEAR1310_GPT64_CLK_ENB: c_int = 23;
pub const SPEAR1310_SSP1_CLK_ENB: c_int = 22;
pub const SPEAR1310_I2C7_CLK_ENB: c_int = 21;
pub const SPEAR1310_I2C6_CLK_ENB: c_int = 20;
pub const SPEAR1310_I2C5_CLK_ENB: c_int = 19;
pub const SPEAR1310_I2C4_CLK_ENB: c_int = 18;
pub const SPEAR1310_I2C3_CLK_ENB: c_int = 17;
pub const SPEAR1310_I2C2_CLK_ENB: c_int = 16;
pub const SPEAR1310_I2C1_CLK_ENB: c_int = 15;
pub const SPEAR1310_UART5_CLK_ENB: c_int = 14;
pub const SPEAR1310_UART4_CLK_ENB: c_int = 13;
pub const SPEAR1310_UART3_CLK_ENB: c_int = 12;
pub const SPEAR1310_UART2_CLK_ENB: c_int = 11;
pub const SPEAR1310_UART1_CLK_ENB: c_int = 10;
pub const SPEAR1310_RS485_1_CLK_ENB: c_int = 9;
pub const SPEAR1310_RS485_0_CLK_ENB: c_int = 8;
pub const SPEAR1310_TDM2_CLK_ENB: c_int = 7;
pub const SPEAR1310_TDM1_CLK_ENB: c_int = 6;
pub const SPEAR1310_PCI_CLK_ENB: c_int = 5;
pub const SPEAR1310_GMII_CLK_ENB: c_int = 4;
pub const SPEAR1310_MII2_CLK_ENB: c_int = 3;
pub const SPEAR1310_MII1_CLK_ENB: c_int = 2;
pub const SPEAR1310_MII0_CLK_ENB: c_int = 1;
pub const SPEAR1310_ESRAM_CLK_ENB: c_int = 0;
    static DEFINE_SPINLOCK(_lock);
// pll rate configuration table, in ascending order of rates
    static struct pll_rate_tbl pll_rtbl[] = {
// PCLK 24MHz
    {.mode = 0, .m = 0x83, .n = 0x04, .p = 0x5}, /* vco 1572, pll 49.125 MHz */
    {.mode = 0, .m = 0x7D, .n = 0x06, .p = 0x3}, /* vco 1000, pll 125 MHz */
    {.mode = 0, .m = 0x64, .n = 0x06, .p = 0x1}, /* vco 800, pll 400 MHz */
    {.mode = 0, .m = 0x7D, .n = 0x06, .p = 0x1}, /* vco 1000, pll 500 MHz */
    {.mode = 0, .m = 0xA6, .n = 0x06, .p = 0x1}, /* vco 1328, pll 664 MHz */
    {.mode = 0, .m = 0xC8, .n = 0x06, .p = 0x1}, /* vco 1600, pll 800 MHz */
    {.mode = 0, .m = 0x7D, .n = 0x06, .p = 0x0}, /* vco 1, pll 1 GHz */
    };
// vco-pll4 rate configuration table, in ascending order of rates
    static struct pll_rate_tbl pll4_rtbl[] = {
    {.mode = 0, .m = 0x7D, .n = 0x06, .p = 0x2}, /* vco 1000, pll 250 MHz */
    {.mode = 0, .m = 0xA6, .n = 0x06, .p = 0x2}, /* vco 1328, pll 332 MHz */
    {.mode = 0, .m = 0xC8, .n = 0x06, .p = 0x2}, /* vco 1600, pll 400 MHz */
    {.mode = 0, .m = 0x7D, .n = 0x06, .p = 0x0}, /* vco 1, pll 1 GHz */
    };
// aux rate configuration table, in ascending order of rates
    static struct aux_rate_tbl aux_rtbl[] = {
// For VCO1div2 = 500 MHz
    {.xscale = 10, .yscale = 204, .eq = 0}, /* 12.29 MHz */
    {.xscale = 4, .yscale = 21, .eq = 0}, /* 48 MHz */
    {.xscale = 2, .yscale = 6, .eq = 0}, /* 83 MHz */
    {.xscale = 2, .yscale = 4, .eq = 0}, /* 125 MHz */
    {.xscale = 1, .yscale = 3, .eq = 1}, /* 166 MHz */
    {.xscale = 1, .yscale = 2, .eq = 1}, /* 250 MHz */
    };
// gmac rate configuration table, in ascending order of rates
    static struct aux_rate_tbl gmac_rtbl[] = {
// For gmac phy input clk
    {.xscale = 2, .yscale = 6, .eq = 0}, /* divided by 6 */
    {.xscale = 2, .yscale = 4, .eq = 0}, /* divided by 4 */
    {.xscale = 1, .yscale = 3, .eq = 1}, /* divided by 3 */
    {.xscale = 1, .yscale = 2, .eq = 1}, /* divided by 2 */
    };
// clcd rate configuration table, in ascending order of rates
    static struct frac_rate_tbl clcd_rtbl[] = {
    {.div = 0x14000}, /* 25 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x1284B}, /* 27 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x0D8D3}, /* 58 Mhz , for vco1div4 = 393 MHz */
    {.div = 0x0B72C}, /* 58 Mhz , for vco1div4 = 332 MHz */
    {.div = 0x089EE}, /* 58 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x06f1C}, /* 72 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x06E58}, /* 58 Mhz , for vco1div4 = 200 MHz */
    {.div = 0x06c1B}, /* 74 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x04A12}, /* 108 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x0378E}, /* 144 Mhz , for vc01div4 = 250 MHz*/
    };
// i2s prescaler1 masks
    static const struct aux_clk_masks i2s_prs1_masks = {
    .eq_sel_mask = AUX_EQ_SEL_MASK,
    .eq_sel_shift = SPEAR1310_I2S_PRS1_EQ_SEL_SHIFT,
    .eq1_mask = AUX_EQ1_SEL,
    .eq2_mask = AUX_EQ2_SEL,
    .xscale_sel_mask = SPEAR1310_I2S_PRS1_CLK_X_MASK,
    .xscale_sel_shift = SPEAR1310_I2S_PRS1_CLK_X_SHIFT,
    .yscale_sel_mask = SPEAR1310_I2S_PRS1_CLK_Y_MASK,
    .yscale_sel_shift = SPEAR1310_I2S_PRS1_CLK_Y_SHIFT,
    };
// i2s sclk (bit clock) syynthesizers masks
    static struct aux_clk_masks i2s_sclk_masks = {
    .eq_sel_mask = AUX_EQ_SEL_MASK,
    .eq_sel_shift = SPEAR1310_I2S_SCLK_EQ_SEL_SHIFT,
    .eq1_mask = AUX_EQ1_SEL,
    .eq2_mask = AUX_EQ2_SEL,
    .xscale_sel_mask = SPEAR1310_I2S_SCLK_X_MASK,
    .xscale_sel_shift = SPEAR1310_I2S_SCLK_X_SHIFT,
    .yscale_sel_mask = SPEAR1310_I2S_SCLK_Y_MASK,
    .yscale_sel_shift = SPEAR1310_I2S_SCLK_Y_SHIFT,
    .enable_bit = SPEAR1310_I2S_SCLK_SYNTH_ENB,
    };
// i2s prs1 aux rate configuration table, in ascending order of rates
    static struct aux_rate_tbl i2s_prs1_rtbl[] = {
// For parent clk = 49.152 MHz
    {.xscale = 1, .yscale = 12, .eq = 0}, /* 2.048 MHz, smp freq = 8Khz */
    {.xscale = 11, .yscale = 96, .eq = 0}, /* 2.816 MHz, smp freq = 11Khz */
    {.xscale = 1, .yscale = 6, .eq = 0}, /* 4.096 MHz, smp freq = 16Khz */
    {.xscale = 11, .yscale = 48, .eq = 0}, /* 5.632 MHz, smp freq = 22Khz */
//
// with parent clk = 49.152, freq gen is 8.192 MHz, smp freq = 32Khz
// with parent clk = 12.288, freq gen is 2.048 MHz, smp freq = 8Khz
//
    {.xscale = 1, .yscale = 3, .eq = 0},
// For parent clk = 49.152 MHz
    {.xscale = 17, .yscale = 37, .eq = 0}, /* 11.289 MHz, smp freq = 44Khz*/
    {.xscale = 1, .yscale = 2, .eq = 0}, /* 12.288 MHz */
    };
// i2s sclk aux rate configuration table, in ascending order of rates
    static struct aux_rate_tbl i2s_sclk_rtbl[] = {
// For i2s_ref_clk = 12.288MHz
    {.xscale = 1, .yscale = 4, .eq = 0}, /* 1.53 MHz */
    {.xscale = 1, .yscale = 2, .eq = 0}, /* 3.07 Mhz */
    };
// adc rate configuration table, in ascending order of rates
// possible adc range is 2.5 MHz to 20 MHz.
    static struct aux_rate_tbl adc_rtbl[] = {
// For ahb = 166.67 MHz
    {.xscale = 1, .yscale = 31, .eq = 0}, /* 2.68 MHz */
    {.xscale = 2, .yscale = 21, .eq = 0}, /* 7.94 MHz */
    {.xscale = 4, .yscale = 21, .eq = 0}, /* 15.87 MHz */
    {.xscale = 10, .yscale = 42, .eq = 0}, /* 19.84 MHz */
    };
// General synth rate configuration table, in ascending order of rates
    static struct frac_rate_tbl gen_rtbl[] = {
// For vco1div4 = 250 MHz
    {.div = 0x14000}, /* 25 MHz */
    {.div = 0x0A000}, /* 50 MHz */
    {.div = 0x05000}, /* 100 MHz */
    {.div = 0x02000}, /* 250 MHz */
    };
// clock parents
    static const char *vco_parents[] = { "osc_24m_clk", "osc_25m_clk", };
    static const char *gpt_parents[] = { "osc_24m_clk", "apb_clk", };
    static const char *uart0_parents[] = { "pll5_clk", "uart_syn_gclk", };
    static const char *c3_parents[] = { "pll5_clk", "c3_syn_gclk", };
    static const char *gmac_phy_input_parents[] = { "gmii_pad_clk", "pll2_clk",
    "osc_25m_clk", };
    static const char *gmac_phy_parents[] = { "phy_input_mclk", "phy_syn_gclk", };
    static const char *clcd_synth_parents[] = { "vco1div4_clk", "pll2_clk", };
    static const char *clcd_pixel_parents[] = { "pll5_clk", "clcd_syn_clk", };
    static const char *i2s_src_parents[] = { "vco1div2_clk", "none", "pll3_clk",
    "i2s_src_pad_clk", };
    static const char *i2s_ref_parents[] = { "i2s_src_mclk", "i2s_prs1_clk", };
    static const char *gen_synth0_1_parents[] = { "vco1div4_clk", "vco3div2_clk",
    "pll3_clk", };
    static const char *gen_synth2_3_parents[] = { "vco1div4_clk", "vco3div2_clk",
    "pll2_clk", };
    static const char *rmii_phy_parents[] = { "ras_tx50_clk", "none",
    "ras_pll2_clk", "ras_syn0_clk", };
    static const char *smii_rgmii_phy_parents[] = { "none", "ras_tx125_clk",
    "ras_pll2_clk", "ras_syn0_clk", };
    static const char *uart_parents[] = { "ras_apb_clk", "gen_syn3_clk", };
    static const char *i2c_parents[] = { "ras_apb_clk", "gen_syn1_clk", };
    static const char *ssp1_parents[] = { "ras_apb_clk", "gen_syn1_clk",
    "ras_plclk0_clk", };
    static const char *pci_parents[] = { "ras_pll3_clk", "gen_syn2_clk", };
    static const char *tdm_parents[] = { "ras_pll3_clk", "gen_syn1_clk", };
#[no_mangle]
pub unsafe extern "C" fn spear1310_clk_init(misc_base: *mut void __iomem, ras_base: *mut void __iomem) -> void __init {
    void __init spear1310_clk_init(void __iomem *misc_base, void __iomem *ras_base)
    {
    struct clk *clk, *clk1;
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "osc_32k_clk", core::ptr::null_mut(), 0, 32000);
    clk_register_clkdev(clk, "osc_32k_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "osc_24m_clk", core::ptr::null_mut(), 0, 24000000);
    clk_register_clkdev(clk, "osc_24m_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "osc_25m_clk", core::ptr::null_mut(), 0, 25000000);
    clk_register_clkdev(clk, "osc_25m_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "gmii_pad_clk", core::ptr::null_mut(), 0, 125000000);
    clk_register_clkdev(clk, "gmii_pad_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "i2s_src_pad_clk", core::ptr::null_mut(), 0,
    12288000);
    clk_register_clkdev(clk, "i2s_src_pad_clk", core::ptr::null_mut());
// clock derived from 32 KHz osc clk
    clk = clk_register_gate(core::ptr::null_mut(), "rtc-spear", "osc_32k_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_RTC_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0580000.rtc");
// clock derived from 24 or 25 MHz osc clk
// vco-pll
    clk = clk_register_mux(core::ptr::null_mut(), "vco1_mclk", vco_parents,
    ARRAY_SIZE(vco_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PLL_CFG, SPEAR1310_PLL1_CLK_SHIFT,
    SPEAR1310_PLL_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "vco1_mclk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco1_clk", "pll1_clk", core::ptr::null_mut(), "vco1_mclk",
    0, SPEAR1310_PLL1_CTR, SPEAR1310_PLL1_FRQ, pll_rtbl,
    ARRAY_SIZE(pll_rtbl), &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco1_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll1_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "vco2_mclk", vco_parents,
    ARRAY_SIZE(vco_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PLL_CFG, SPEAR1310_PLL2_CLK_SHIFT,
    SPEAR1310_PLL_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "vco2_mclk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco2_clk", "pll2_clk", core::ptr::null_mut(), "vco2_mclk",
    0, SPEAR1310_PLL2_CTR, SPEAR1310_PLL2_FRQ, pll_rtbl,
    ARRAY_SIZE(pll_rtbl), &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco2_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll2_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "vco3_mclk", vco_parents,
    ARRAY_SIZE(vco_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PLL_CFG, SPEAR1310_PLL3_CLK_SHIFT,
    SPEAR1310_PLL_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "vco3_mclk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco3_clk", "pll3_clk", core::ptr::null_mut(), "vco3_mclk",
    0, SPEAR1310_PLL3_CTR, SPEAR1310_PLL3_FRQ, pll_rtbl,
    ARRAY_SIZE(pll_rtbl), &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco3_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll3_clk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco4_clk", "pll4_clk", core::ptr::null_mut(), "osc_24m_clk",
    0, SPEAR1310_PLL4_CTR, SPEAR1310_PLL4_FRQ, pll4_rtbl,
    ARRAY_SIZE(pll4_rtbl), &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco4_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll4_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "pll5_clk", "osc_24m_clk", 0,
    48000000);
    clk_register_clkdev(clk, "pll5_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "pll6_clk", "osc_25m_clk", 0,
    25000000);
    clk_register_clkdev(clk, "pll6_clk", core::ptr::null_mut());
// vco div n clocks
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "vco1div2_clk", "vco1_clk", 0, 1,
    2);
    clk_register_clkdev(clk, "vco1div2_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "vco1div4_clk", "vco1_clk", 0, 1,
    4);
    clk_register_clkdev(clk, "vco1div4_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "vco2div2_clk", "vco2_clk", 0, 1,
    2);
    clk_register_clkdev(clk, "vco2div2_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "vco3div2_clk", "vco3_clk", 0, 1,
    2);
    clk_register_clkdev(clk, "vco3div2_clk", core::ptr::null_mut());
// peripherals
    clk_register_fixed_factor(core::ptr::null_mut(), "thermal_clk", "osc_24m_clk", 0, 1,
    128);
    clk = clk_register_gate(core::ptr::null_mut(), "thermal_gclk", "thermal_clk", 0,
    SPEAR1310_PERIP2_CLK_ENB, SPEAR1310_THSENS_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "spear_thermal");
// clock derived from pll4 clk
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "ddr_clk", "pll4_clk", 0, 1,
    1);
    clk_register_clkdev(clk, "ddr_clk", core::ptr::null_mut());
// clock derived from pll1 clk
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "cpu_clk", "pll1_clk",
    CLK_SET_RATE_PARENT, 1, 2);
    clk_register_clkdev(clk, "cpu_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "wdt_clk", "cpu_clk", 0, 1,
    2);
    clk_register_clkdev(clk, core::ptr::null_mut(), "ec800620.wdt");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "smp_twd_clk", "cpu_clk", 0, 1,
    2);
    clk_register_clkdev(clk, core::ptr::null_mut(), "smp_twd");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "ahb_clk", "pll1_clk", 0, 1,
    6);
    clk_register_clkdev(clk, "ahb_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "apb_clk", "pll1_clk", 0, 1,
    12);
    clk_register_clkdev(clk, "apb_clk", core::ptr::null_mut());
// gpt clocks
    clk = clk_register_mux(core::ptr::null_mut(), "gpt0_mclk", gpt_parents,
    ARRAY_SIZE(gpt_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PERIP_CLK_CFG, SPEAR1310_GPT0_CLK_SHIFT,
    SPEAR1310_GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt0_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt0_clk", "gpt0_mclk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_GPT0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt0");
    clk = clk_register_mux(core::ptr::null_mut(), "gpt1_mclk", gpt_parents,
    ARRAY_SIZE(gpt_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PERIP_CLK_CFG, SPEAR1310_GPT1_CLK_SHIFT,
    SPEAR1310_GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt1_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt1_clk", "gpt1_mclk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_GPT1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt1");
    clk = clk_register_mux(core::ptr::null_mut(), "gpt2_mclk", gpt_parents,
    ARRAY_SIZE(gpt_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PERIP_CLK_CFG, SPEAR1310_GPT2_CLK_SHIFT,
    SPEAR1310_GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt2_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt2_clk", "gpt2_mclk", 0,
    SPEAR1310_PERIP2_CLK_ENB, SPEAR1310_GPT2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt2");
    clk = clk_register_mux(core::ptr::null_mut(), "gpt3_mclk", gpt_parents,
    ARRAY_SIZE(gpt_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PERIP_CLK_CFG, SPEAR1310_GPT3_CLK_SHIFT,
    SPEAR1310_GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt3_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt3_clk", "gpt3_mclk", 0,
    SPEAR1310_PERIP2_CLK_ENB, SPEAR1310_GPT3_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt3");
// others
    clk = clk_register_aux("uart_syn_clk", "uart_syn_gclk", "vco1div2_clk",
    0, SPEAR1310_UART_CLK_SYNT, core::ptr::null_mut(), aux_rtbl,
    ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "uart_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "uart_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "uart0_mclk", uart0_parents,
    ARRAY_SIZE(uart0_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PERIP_CLK_CFG, SPEAR1310_UART_CLK_SHIFT,
    SPEAR1310_UART_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "uart0_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart0_clk", "uart0_mclk",
    CLK_SET_RATE_PARENT, SPEAR1310_PERIP1_CLK_ENB,
    SPEAR1310_UART_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0000000.serial");
    clk = clk_register_aux("sdhci_syn_clk", "sdhci_syn_gclk",
    "vco1div2_clk", 0, SPEAR1310_SDHCI_CLK_SYNT, core::ptr::null_mut(),
    aux_rtbl, ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "sdhci_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "sdhci_syn_gclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "sdhci_clk", "sdhci_syn_gclk",
    CLK_SET_RATE_PARENT, SPEAR1310_PERIP1_CLK_ENB,
    SPEAR1310_SDHCI_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b3000000.sdhci");
    clk = clk_register_aux("cfxd_syn_clk", "cfxd_syn_gclk", "vco1div2_clk",
    0, SPEAR1310_CFXD_CLK_SYNT, core::ptr::null_mut(), aux_rtbl,
    ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "cfxd_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "cfxd_syn_gclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "cfxd_clk", "cfxd_syn_gclk",
    CLK_SET_RATE_PARENT, SPEAR1310_PERIP1_CLK_ENB,
    SPEAR1310_CFXD_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2800000.cf");
    clk_register_clkdev(clk, core::ptr::null_mut(), "arasan_xd");
    clk = clk_register_aux("c3_syn_clk", "c3_syn_gclk", "vco1div2_clk",
    0, SPEAR1310_C3_CLK_SYNT, core::ptr::null_mut(), aux_rtbl,
    ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "c3_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "c3_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "c3_mclk", c3_parents,
    ARRAY_SIZE(c3_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PERIP_CLK_CFG, SPEAR1310_C3_CLK_SHIFT,
    SPEAR1310_C3_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "c3_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "c3_clk", "c3_mclk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_C3_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "c3");
// gmac
    clk = clk_register_mux(core::ptr::null_mut(), "phy_input_mclk", gmac_phy_input_parents,
    ARRAY_SIZE(gmac_phy_input_parents),
    CLK_SET_RATE_NO_REPARENT, SPEAR1310_GMAC_CLK_CFG,
    SPEAR1310_GMAC_PHY_INPUT_CLK_SHIFT,
    SPEAR1310_GMAC_PHY_INPUT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "phy_input_mclk", core::ptr::null_mut());
    clk = clk_register_aux("phy_syn_clk", "phy_syn_gclk", "phy_input_mclk",
    0, SPEAR1310_GMAC_CLK_SYNT, core::ptr::null_mut(), gmac_rtbl,
    ARRAY_SIZE(gmac_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "phy_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "phy_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "phy_mclk", gmac_phy_parents,
    ARRAY_SIZE(gmac_phy_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PERIP_CLK_CFG, SPEAR1310_GMAC_PHY_CLK_SHIFT,
    SPEAR1310_GMAC_PHY_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "stmmacphy.0", core::ptr::null_mut());
// clcd
    clk = clk_register_mux(core::ptr::null_mut(), "clcd_syn_mclk", clcd_synth_parents,
    ARRAY_SIZE(clcd_synth_parents),
    CLK_SET_RATE_NO_REPARENT, SPEAR1310_CLCD_CLK_SYNT,
    SPEAR1310_CLCD_SYNT_CLK_SHIFT,
    SPEAR1310_CLCD_SYNT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "clcd_syn_mclk", core::ptr::null_mut());
    clk = clk_register_frac("clcd_syn_clk", "clcd_syn_mclk", 0,
    SPEAR1310_CLCD_CLK_SYNT, clcd_rtbl,
    ARRAY_SIZE(clcd_rtbl), &_lock);
    clk_register_clkdev(clk, "clcd_syn_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "clcd_pixel_mclk", clcd_pixel_parents,
    ARRAY_SIZE(clcd_pixel_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_PERIP_CLK_CFG, SPEAR1310_CLCD_CLK_SHIFT,
    SPEAR1310_CLCD_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "clcd_pixel_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "clcd_clk", "clcd_pixel_mclk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_CLCD_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e1000000.clcd");
// i2s
    clk = clk_register_mux(core::ptr::null_mut(), "i2s_src_mclk", i2s_src_parents,
    ARRAY_SIZE(i2s_src_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_I2S_CLK_CFG, SPEAR1310_I2S_SRC_CLK_SHIFT,
    SPEAR1310_I2S_SRC_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2s_src_mclk", core::ptr::null_mut());
    clk = clk_register_aux("i2s_prs1_clk", core::ptr::null_mut(), "i2s_src_mclk", 0,
    SPEAR1310_I2S_CLK_CFG, &i2s_prs1_masks, i2s_prs1_rtbl,
    ARRAY_SIZE(i2s_prs1_rtbl), &_lock, core::ptr::null_mut());
    clk_register_clkdev(clk, "i2s_prs1_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "i2s_ref_mclk", i2s_ref_parents,
    ARRAY_SIZE(i2s_ref_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_I2S_CLK_CFG, SPEAR1310_I2S_REF_SHIFT,
    SPEAR1310_I2S_REF_SEL_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2s_ref_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "i2s_ref_pad_clk", "i2s_ref_mclk", 0,
    SPEAR1310_PERIP2_CLK_ENB, SPEAR1310_I2S_REF_PAD_CLK_ENB,
    0, &_lock);
    clk_register_clkdev(clk, "i2s_ref_pad_clk", core::ptr::null_mut());
    clk = clk_register_aux("i2s_sclk_clk", "i2s_sclk_gclk",
    "i2s_ref_mclk", 0, SPEAR1310_I2S_CLK_CFG,
    &i2s_sclk_masks, i2s_sclk_rtbl,
    ARRAY_SIZE(i2s_sclk_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "i2s_sclk_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "i2s_sclk_gclk", core::ptr::null_mut());
// clock derived from ahb clk
    clk = clk_register_gate(core::ptr::null_mut(), "i2c0_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_I2C0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0280000.i2c");
    clk = clk_register_gate(core::ptr::null_mut(), "dma_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_DMA_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "ea800000.dma");
    clk_register_clkdev(clk, core::ptr::null_mut(), "eb000000.dma");
    clk = clk_register_gate(core::ptr::null_mut(), "jpeg_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_JPEG_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2000000.jpeg");
    clk = clk_register_gate(core::ptr::null_mut(), "gmac_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_GMAC_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e2000000.eth");
    clk = clk_register_gate(core::ptr::null_mut(), "fsmc_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_FSMC_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b0000000.flash");
    clk = clk_register_gate(core::ptr::null_mut(), "smi_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_SMI_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "ea000000.flash");
    clk = clk_register_gate(core::ptr::null_mut(), "usbh0_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_UHC0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e4000000.ohci");
    clk_register_clkdev(clk, core::ptr::null_mut(), "e4800000.ehci");
    clk = clk_register_gate(core::ptr::null_mut(), "usbh1_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_UHC1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e5000000.ohci");
    clk_register_clkdev(clk, core::ptr::null_mut(), "e5800000.ehci");
    clk = clk_register_gate(core::ptr::null_mut(), "uoc_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_UOC_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e3800000.otg");
    clk = clk_register_gate(core::ptr::null_mut(), "pcie_sata_0_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_PCIE_SATA_0_CLK_ENB,
    0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b1000000.pcie");
    clk_register_clkdev(clk, core::ptr::null_mut(), "b1000000.ahci");
    clk = clk_register_gate(core::ptr::null_mut(), "pcie_sata_1_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_PCIE_SATA_1_CLK_ENB,
    0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b1800000.pcie");
    clk_register_clkdev(clk, core::ptr::null_mut(), "b1800000.ahci");
    clk = clk_register_gate(core::ptr::null_mut(), "pcie_sata_2_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_PCIE_SATA_2_CLK_ENB,
    0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b4000000.pcie");
    clk_register_clkdev(clk, core::ptr::null_mut(), "b4000000.ahci");
    clk = clk_register_gate(core::ptr::null_mut(), "sysram0_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_SYSRAM0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "sysram0_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "sysram1_clk", "ahb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_SYSRAM1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "sysram1_clk", core::ptr::null_mut());
    clk = clk_register_aux("adc_syn_clk", "adc_syn_gclk", "ahb_clk",
    0, SPEAR1310_ADC_CLK_SYNT, core::ptr::null_mut(), adc_rtbl,
    ARRAY_SIZE(adc_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "adc_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "adc_syn_gclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "adc_clk", "adc_syn_gclk",
    CLK_SET_RATE_PARENT, SPEAR1310_PERIP1_CLK_ENB,
    SPEAR1310_ADC_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0080000.adc");
// clock derived from apb clk
    clk = clk_register_gate(core::ptr::null_mut(), "ssp0_clk", "apb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_SSP_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0100000.spi");
    clk = clk_register_gate(core::ptr::null_mut(), "gpio0_clk", "apb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_GPIO0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0600000.gpio");
    clk = clk_register_gate(core::ptr::null_mut(), "gpio1_clk", "apb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_GPIO1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0680000.gpio");
    clk = clk_register_gate(core::ptr::null_mut(), "i2s0_clk", "apb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_I2S0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0180000.i2s");
    clk = clk_register_gate(core::ptr::null_mut(), "i2s1_clk", "apb_clk", 0,
    SPEAR1310_PERIP1_CLK_ENB, SPEAR1310_I2S1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0200000.i2s");
    clk = clk_register_gate(core::ptr::null_mut(), "kbd_clk", "apb_clk", 0,
    SPEAR1310_PERIP2_CLK_ENB, SPEAR1310_KBD_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0300000.kbd");
// RAS clks
    clk = clk_register_mux(core::ptr::null_mut(), "gen_syn0_1_mclk", gen_synth0_1_parents,
    ARRAY_SIZE(gen_synth0_1_parents),
    CLK_SET_RATE_NO_REPARENT, SPEAR1310_PLL_CFG,
    SPEAR1310_RAS_SYNT0_1_CLK_SHIFT,
    SPEAR1310_RAS_SYNT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gen_syn0_1_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "gen_syn2_3_mclk", gen_synth2_3_parents,
    ARRAY_SIZE(gen_synth2_3_parents),
    CLK_SET_RATE_NO_REPARENT, SPEAR1310_PLL_CFG,
    SPEAR1310_RAS_SYNT2_3_CLK_SHIFT,
    SPEAR1310_RAS_SYNT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gen_syn2_3_clk", core::ptr::null_mut());
    clk = clk_register_frac("gen_syn0_clk", "gen_syn0_1_clk", 0,
    SPEAR1310_RAS_CLK_SYNT0, gen_rtbl, ARRAY_SIZE(gen_rtbl),
    &_lock);
    clk_register_clkdev(clk, "gen_syn0_clk", core::ptr::null_mut());
    clk = clk_register_frac("gen_syn1_clk", "gen_syn0_1_clk", 0,
    SPEAR1310_RAS_CLK_SYNT1, gen_rtbl, ARRAY_SIZE(gen_rtbl),
    &_lock);
    clk_register_clkdev(clk, "gen_syn1_clk", core::ptr::null_mut());
    clk = clk_register_frac("gen_syn2_clk", "gen_syn2_3_clk", 0,
    SPEAR1310_RAS_CLK_SYNT2, gen_rtbl, ARRAY_SIZE(gen_rtbl),
    &_lock);
    clk_register_clkdev(clk, "gen_syn2_clk", core::ptr::null_mut());
    clk = clk_register_frac("gen_syn3_clk", "gen_syn2_3_clk", 0,
    SPEAR1310_RAS_CLK_SYNT3, gen_rtbl, ARRAY_SIZE(gen_rtbl),
    &_lock);
    clk_register_clkdev(clk, "gen_syn3_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_osc_24m_clk", "osc_24m_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_OSC_24M_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_osc_24m_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_osc_25m_clk", "osc_25m_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_OSC_25M_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_osc_25m_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_osc_32k_clk", "osc_32k_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_OSC_32K_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_osc_32k_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_pll2_clk", "pll2_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_PLL2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_pll2_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_pll3_clk", "pll3_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_PLL3_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_pll3_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_tx125_clk", "gmii_pad_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_C125M_PAD_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_tx125_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "ras_30m_fixed_clk", "pll5_clk", 0,
    30000000);
    clk = clk_register_gate(core::ptr::null_mut(), "ras_30m_clk", "ras_30m_fixed_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_C30M_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_30m_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "ras_48m_fixed_clk", "pll5_clk", 0,
    48000000);
    clk = clk_register_gate(core::ptr::null_mut(), "ras_48m_clk", "ras_48m_fixed_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_C48M_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_48m_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_ahb_clk", "ahb_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_ACLK_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_ahb_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ras_apb_clk", "apb_clk", 0,
    SPEAR1310_RAS_CLK_ENB, SPEAR1310_PCLK_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "ras_apb_clk", core::ptr::null_mut());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "ras_plclk0_clk", core::ptr::null_mut(), 0,
    50000000);
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "ras_tx50_clk", core::ptr::null_mut(), 0, 50000000);
    clk = clk_register_gate(core::ptr::null_mut(), "can0_clk", "apb_clk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_CAN0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "c_can_platform.0");
    clk = clk_register_gate(core::ptr::null_mut(), "can1_clk", "apb_clk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_CAN1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "c_can_platform.1");
    clk = clk_register_gate(core::ptr::null_mut(), "ras_smii0_clk", "ras_ahb_clk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_MII0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5c400000.eth");
    clk = clk_register_gate(core::ptr::null_mut(), "ras_smii1_clk", "ras_ahb_clk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_MII1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5c500000.eth");
    clk = clk_register_gate(core::ptr::null_mut(), "ras_smii2_clk", "ras_ahb_clk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_MII2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5c600000.eth");
    clk = clk_register_gate(core::ptr::null_mut(), "ras_rgmii_clk", "ras_ahb_clk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_GMII_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5c700000.eth");
    clk = clk_register_mux(core::ptr::null_mut(), "smii_rgmii_phy_mclk",
    smii_rgmii_phy_parents,
    ARRAY_SIZE(smii_rgmii_phy_parents),
    CLK_SET_RATE_NO_REPARENT, SPEAR1310_RAS_CTRL_REG1,
    SPEAR1310_SMII_RGMII_PHY_CLK_SHIFT,
    SPEAR1310_PHY_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "stmmacphy.1", core::ptr::null_mut());
    clk_register_clkdev(clk, "stmmacphy.2", core::ptr::null_mut());
    clk_register_clkdev(clk, "stmmacphy.4", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "rmii_phy_mclk", rmii_phy_parents,
    ARRAY_SIZE(rmii_phy_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG1, SPEAR1310_RMII_PHY_CLK_SHIFT,
    SPEAR1310_PHY_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "stmmacphy.3", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "uart1_mclk", uart_parents,
    ARRAY_SIZE(uart_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_UART1_CLK_SHIFT,
    SPEAR1310_RAS_UART_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "uart1_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart1_clk", "uart1_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_UART1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5c800000.serial");
    clk = clk_register_mux(core::ptr::null_mut(), "uart2_mclk", uart_parents,
    ARRAY_SIZE(uart_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_UART2_CLK_SHIFT,
    SPEAR1310_RAS_UART_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "uart2_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart2_clk", "uart2_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_UART2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5c900000.serial");
    clk = clk_register_mux(core::ptr::null_mut(), "uart3_mclk", uart_parents,
    ARRAY_SIZE(uart_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_UART3_CLK_SHIFT,
    SPEAR1310_RAS_UART_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "uart3_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart3_clk", "uart3_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_UART3_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5ca00000.serial");
    clk = clk_register_mux(core::ptr::null_mut(), "uart4_mclk", uart_parents,
    ARRAY_SIZE(uart_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_UART4_CLK_SHIFT,
    SPEAR1310_RAS_UART_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "uart4_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart4_clk", "uart4_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_UART4_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5cb00000.serial");
    clk = clk_register_mux(core::ptr::null_mut(), "uart5_mclk", uart_parents,
    ARRAY_SIZE(uart_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_UART5_CLK_SHIFT,
    SPEAR1310_RAS_UART_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "uart5_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart5_clk", "uart5_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_UART5_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5cc00000.serial");
    clk = clk_register_mux(core::ptr::null_mut(), "i2c1_mclk", i2c_parents,
    ARRAY_SIZE(i2c_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_I2C1_CLK_SHIFT,
    SPEAR1310_I2C_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2c1_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "i2c1_clk", "i2c1_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_I2C1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5cd00000.i2c");
    clk = clk_register_mux(core::ptr::null_mut(), "i2c2_mclk", i2c_parents,
    ARRAY_SIZE(i2c_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_I2C2_CLK_SHIFT,
    SPEAR1310_I2C_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2c2_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "i2c2_clk", "i2c2_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_I2C2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5ce00000.i2c");
    clk = clk_register_mux(core::ptr::null_mut(), "i2c3_mclk", i2c_parents,
    ARRAY_SIZE(i2c_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_I2C3_CLK_SHIFT,
    SPEAR1310_I2C_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2c3_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "i2c3_clk", "i2c3_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_I2C3_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5cf00000.i2c");
    clk = clk_register_mux(core::ptr::null_mut(), "i2c4_mclk", i2c_parents,
    ARRAY_SIZE(i2c_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_I2C4_CLK_SHIFT,
    SPEAR1310_I2C_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2c4_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "i2c4_clk", "i2c4_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_I2C4_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5d000000.i2c");
    clk = clk_register_mux(core::ptr::null_mut(), "i2c5_mclk", i2c_parents,
    ARRAY_SIZE(i2c_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_I2C5_CLK_SHIFT,
    SPEAR1310_I2C_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2c5_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "i2c5_clk", "i2c5_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_I2C5_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5d100000.i2c");
    clk = clk_register_mux(core::ptr::null_mut(), "i2c6_mclk", i2c_parents,
    ARRAY_SIZE(i2c_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_I2C6_CLK_SHIFT,
    SPEAR1310_I2C_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2c6_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "i2c6_clk", "i2c6_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_I2C6_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5d200000.i2c");
    clk = clk_register_mux(core::ptr::null_mut(), "i2c7_mclk", i2c_parents,
    ARRAY_SIZE(i2c_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_I2C7_CLK_SHIFT,
    SPEAR1310_I2C_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2c7_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "i2c7_clk", "i2c7_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_I2C7_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5d300000.i2c");
    clk = clk_register_mux(core::ptr::null_mut(), "ssp1_mclk", ssp1_parents,
    ARRAY_SIZE(ssp1_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_SSP1_CLK_SHIFT,
    SPEAR1310_SSP1_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "ssp1_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "ssp1_clk", "ssp1_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_SSP1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "5d400000.spi");
    clk = clk_register_mux(core::ptr::null_mut(), "pci_mclk", pci_parents,
    ARRAY_SIZE(pci_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_PCI_CLK_SHIFT,
    SPEAR1310_PCI_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "pci_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "pci_clk", "pci_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_PCI_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "pci");
    clk = clk_register_mux(core::ptr::null_mut(), "tdm1_mclk", tdm_parents,
    ARRAY_SIZE(tdm_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_TDM1_CLK_SHIFT,
    SPEAR1310_TDM_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "tdm1_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "tdm1_clk", "tdm1_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_TDM1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "tdm_hdlc.0");
    clk = clk_register_mux(core::ptr::null_mut(), "tdm2_mclk", tdm_parents,
    ARRAY_SIZE(tdm_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1310_RAS_CTRL_REG0, SPEAR1310_TDM2_CLK_SHIFT,
    SPEAR1310_TDM_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "tdm2_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "tdm2_clk", "tdm2_mclk", 0,
    SPEAR1310_RAS_SW_CLK_CTRL, SPEAR1310_TDM2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "tdm_hdlc.1");
    }
