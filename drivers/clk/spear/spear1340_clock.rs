//! Automatically rewritten from C to Rust
//! Source: drivers/clk/spear/spear1340_clock.c
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
// arch/arm/mach-spear13xx/spear1340_clock.c
//
// SPEAr1340 machine clock framework source file
//
// Copyright (C) 2012 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//

// Clock Configuration Registers

pub const SPEAR1340_HCLK_SRC_SEL_SHIFT: c_int = 27;
pub const SPEAR1340_HCLK_SRC_SEL_MASK: c_int = 1;
pub const SPEAR1340_SCLK_SRC_SEL_SHIFT: c_int = 23;
pub const SPEAR1340_SCLK_SRC_SEL_MASK: c_int = 3;
// PLL related registers and bit values

// PLL_CFG bit values
pub const SPEAR1340_CLCD_SYNT_CLK_MASK: c_int = 1;
pub const SPEAR1340_CLCD_SYNT_CLK_SHIFT: c_int = 31;
pub const SPEAR1340_GEN_SYNT2_3_CLK_SHIFT: c_int = 29;
pub const SPEAR1340_GEN_SYNT_CLK_MASK: c_int = 2;
pub const SPEAR1340_GEN_SYNT0_1_CLK_SHIFT: c_int = 27;
pub const SPEAR1340_PLL_CLK_MASK: c_int = 2;
pub const SPEAR1340_PLL3_CLK_SHIFT: c_int = 24;
pub const SPEAR1340_PLL2_CLK_SHIFT: c_int = 22;
pub const SPEAR1340_PLL1_CLK_SHIFT: c_int = 20;

// PERIP_CLK_CFG bit values
pub const SPEAR1340_SPDIF_CLK_MASK: c_int = 1;
pub const SPEAR1340_SPDIF_OUT_CLK_SHIFT: c_int = 15;
pub const SPEAR1340_SPDIF_IN_CLK_SHIFT: c_int = 14;
pub const SPEAR1340_GPT3_CLK_SHIFT: c_int = 13;
pub const SPEAR1340_GPT2_CLK_SHIFT: c_int = 12;
pub const SPEAR1340_GPT_CLK_MASK: c_int = 1;
pub const SPEAR1340_GPT1_CLK_SHIFT: c_int = 9;
pub const SPEAR1340_GPT0_CLK_SHIFT: c_int = 8;
pub const SPEAR1340_UART_CLK_MASK: c_int = 2;
pub const SPEAR1340_UART1_CLK_SHIFT: c_int = 6;
pub const SPEAR1340_UART0_CLK_SHIFT: c_int = 4;
pub const SPEAR1340_CLCD_CLK_MASK: c_int = 2;
pub const SPEAR1340_CLCD_CLK_SHIFT: c_int = 2;
pub const SPEAR1340_C3_CLK_MASK: c_int = 1;
pub const SPEAR1340_C3_CLK_SHIFT: c_int = 1;

pub const SPEAR1340_GMAC_PHY_CLK_MASK: c_int = 1;
pub const SPEAR1340_GMAC_PHY_CLK_SHIFT: c_int = 2;
pub const SPEAR1340_GMAC_PHY_INPUT_CLK_MASK: c_int = 2;
pub const SPEAR1340_GMAC_PHY_INPUT_CLK_SHIFT: c_int = 0;

// I2S_CLK_CFG register mask
pub const SPEAR1340_I2S_SCLK_X_MASK: c_uint = 0x1F;
pub const SPEAR1340_I2S_SCLK_X_SHIFT: c_int = 27;
pub const SPEAR1340_I2S_SCLK_Y_MASK: c_uint = 0x1F;
pub const SPEAR1340_I2S_SCLK_Y_SHIFT: c_int = 22;
pub const SPEAR1340_I2S_SCLK_EQ_SEL_SHIFT: c_int = 21;
pub const SPEAR1340_I2S_SCLK_SYNTH_ENB: c_int = 20;
pub const SPEAR1340_I2S_PRS1_CLK_X_MASK: c_uint = 0xFF;
pub const SPEAR1340_I2S_PRS1_CLK_X_SHIFT: c_int = 12;
pub const SPEAR1340_I2S_PRS1_CLK_Y_MASK: c_uint = 0xFF;
pub const SPEAR1340_I2S_PRS1_CLK_Y_SHIFT: c_int = 4;
pub const SPEAR1340_I2S_PRS1_EQ_SEL_SHIFT: c_int = 3;
pub const SPEAR1340_I2S_REF_SEL_MASK: c_int = 1;
pub const SPEAR1340_I2S_REF_SHIFT: c_int = 2;
pub const SPEAR1340_I2S_SRC_CLK_MASK: c_int = 2;
pub const SPEAR1340_I2S_SRC_CLK_SHIFT: c_int = 0;

pub const SPEAR1340_RTC_CLK_ENB: c_int = 31;
pub const SPEAR1340_ADC_CLK_ENB: c_int = 30;
pub const SPEAR1340_C3_CLK_ENB: c_int = 29;
pub const SPEAR1340_CLCD_CLK_ENB: c_int = 27;
pub const SPEAR1340_DMA_CLK_ENB: c_int = 25;
pub const SPEAR1340_GPIO1_CLK_ENB: c_int = 24;
pub const SPEAR1340_GPIO0_CLK_ENB: c_int = 23;
pub const SPEAR1340_GPT1_CLK_ENB: c_int = 22;
pub const SPEAR1340_GPT0_CLK_ENB: c_int = 21;
pub const SPEAR1340_I2S_PLAY_CLK_ENB: c_int = 20;
pub const SPEAR1340_I2S_REC_CLK_ENB: c_int = 19;
pub const SPEAR1340_I2C0_CLK_ENB: c_int = 18;
pub const SPEAR1340_SSP_CLK_ENB: c_int = 17;
pub const SPEAR1340_UART0_CLK_ENB: c_int = 15;
pub const SPEAR1340_PCIE_SATA_CLK_ENB: c_int = 12;
pub const SPEAR1340_UOC_CLK_ENB: c_int = 11;
pub const SPEAR1340_UHC1_CLK_ENB: c_int = 10;
pub const SPEAR1340_UHC0_CLK_ENB: c_int = 9;
pub const SPEAR1340_GMAC_CLK_ENB: c_int = 8;
pub const SPEAR1340_CFXD_CLK_ENB: c_int = 7;
pub const SPEAR1340_SDHCI_CLK_ENB: c_int = 6;
pub const SPEAR1340_SMI_CLK_ENB: c_int = 5;
pub const SPEAR1340_FSMC_CLK_ENB: c_int = 4;
pub const SPEAR1340_SYSRAM0_CLK_ENB: c_int = 3;
pub const SPEAR1340_SYSRAM1_CLK_ENB: c_int = 2;
pub const SPEAR1340_SYSROM_CLK_ENB: c_int = 1;
pub const SPEAR1340_BUS_CLK_ENB: c_int = 0;

pub const SPEAR1340_THSENS_CLK_ENB: c_int = 8;
pub const SPEAR1340_I2S_REF_PAD_CLK_ENB: c_int = 7;
pub const SPEAR1340_ACP_CLK_ENB: c_int = 6;
pub const SPEAR1340_GPT3_CLK_ENB: c_int = 5;
pub const SPEAR1340_GPT2_CLK_ENB: c_int = 4;
pub const SPEAR1340_KBD_CLK_ENB: c_int = 3;
pub const SPEAR1340_CPU_DBG_CLK_ENB: c_int = 2;
pub const SPEAR1340_DDR_CORE_CLK_ENB: c_int = 1;
pub const SPEAR1340_DDR_CTRL_CLK_ENB: c_int = 0;

pub const SPEAR1340_PLGPIO_CLK_ENB: c_int = 18;
pub const SPEAR1340_VIDEO_DEC_CLK_ENB: c_int = 16;
pub const SPEAR1340_VIDEO_ENC_CLK_ENB: c_int = 15;
pub const SPEAR1340_SPDIF_OUT_CLK_ENB: c_int = 13;
pub const SPEAR1340_SPDIF_IN_CLK_ENB: c_int = 12;
pub const SPEAR1340_VIDEO_IN_CLK_ENB: c_int = 11;
pub const SPEAR1340_CAM0_CLK_ENB: c_int = 10;
pub const SPEAR1340_CAM1_CLK_ENB: c_int = 9;
pub const SPEAR1340_CAM2_CLK_ENB: c_int = 8;
pub const SPEAR1340_CAM3_CLK_ENB: c_int = 7;
pub const SPEAR1340_MALI_CLK_ENB: c_int = 6;
pub const SPEAR1340_CEC0_CLK_ENB: c_int = 5;
pub const SPEAR1340_CEC1_CLK_ENB: c_int = 4;
pub const SPEAR1340_PWM_CLK_ENB: c_int = 3;
pub const SPEAR1340_I2C1_CLK_ENB: c_int = 2;
pub const SPEAR1340_UART1_CLK_ENB: c_int = 1;
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
    {.mode = 0, .m = 0x96, .n = 0x06, .p = 0x0}, /* vco 1200, pll 1200 MHz */
    };
// vco-pll4 rate configuration table, in ascending order of rates
    static struct pll_rate_tbl pll4_rtbl[] = {
    {.mode = 0, .m = 0x7D, .n = 0x06, .p = 0x2}, /* vco 1000, pll 250 MHz */
    {.mode = 0, .m = 0xA6, .n = 0x06, .p = 0x2}, /* vco 1328, pll 332 MHz */
    {.mode = 0, .m = 0xC8, .n = 0x06, .p = 0x2}, /* vco 1600, pll 400 MHz */
    {.mode = 0, .m = 0x7D, .n = 0x06, .p = 0x0}, /* vco 1, pll 1 GHz */
    };
//
// All below entries generate 166 MHz for
// different values of vco1div2
//
    static struct frac_rate_tbl amba_synth_rtbl[] = {
    {.div = 0x073A8}, /* for vco1div2 = 600 MHz */
    {.div = 0x06062}, /* for vco1div2 = 500 MHz */
    {.div = 0x04D1B}, /* for vco1div2 = 400 MHz */
    {.div = 0x04000}, /* for vco1div2 = 332 MHz */
    {.div = 0x03031}, /* for vco1div2 = 250 MHz */
    {.div = 0x0268D}, /* for vco1div2 = 200 MHz */
    };
//
// Synthesizer Clock derived from vcodiv2. This clock is one of the
// possible clocks to feed cpu directly.
// We can program this synthesizer to make cpu run on different clock
// frequencies.
// Following table provides configuration values to let cpu run on 200,
// 250, 332, 400 or 500 MHz considering different possibilities of input
// (vco1div2) clock.
//
// --------------------------------------------------------------------
// vco1div2(Mhz)	fout(Mhz)	cpuclk = fout/2		div
// --------------------------------------------------------------------
// 400			200		100			0x04000
// 400			250		125			0x03333
// 400			332		166			0x0268D
// 400			400		200			0x02000
// --------------------------------------------------------------------
// 500			200		100			0x05000
// 500			250		125			0x04000
// 500			332		166			0x03031
// 500			400		200			0x02800
// 500			500		250			0x02000
// --------------------------------------------------------------------
// 600			200		100			0x06000
// 600			250		125			0x04CCE
// 600			332		166			0x039D5
// 600			400		200			0x03000
// 600			500		250			0x02666
// --------------------------------------------------------------------
// 664			200		100			0x06a38
// 664			250		125			0x054FD
// 664			332		166			0x04000
// 664			400		200			0x0351E
// 664			500		250			0x02A7E
// --------------------------------------------------------------------
// 800			200		100			0x08000
// 800			250		125			0x06666
// 800			332		166			0x04D18
// 800			400		200			0x04000
// 800			500		250			0x03333
// --------------------------------------------------------------------
// sys rate configuration table is in descending order of divisor.
//
    static struct frac_rate_tbl sys_synth_rtbl[] = {
    {.div = 0x08000},
    {.div = 0x06a38},
    {.div = 0x06666},
    {.div = 0x06000},
    {.div = 0x054FD},
    {.div = 0x05000},
    {.div = 0x04D18},
    {.div = 0x04CCE},
    {.div = 0x04000},
    {.div = 0x039D5},
    {.div = 0x0351E},
    {.div = 0x03333},
    {.div = 0x03031},
    {.div = 0x03000},
    {.div = 0x02A7E},
    {.div = 0x02800},
    {.div = 0x0268D},
    {.div = 0x02666},
    {.div = 0x02000},
    };
// aux rate configuration table, in ascending order of rates
    static struct aux_rate_tbl aux_rtbl[] = {
// 12.29MHz for vic1div2=600MHz and 10.24MHz for VCO1div2=500MHz
    {.xscale = 5, .yscale = 122, .eq = 0},
// 14.70MHz for vic1div2=600MHz and 12.29MHz for VCO1div2=500MHz
    {.xscale = 10, .yscale = 204, .eq = 0},
// 48MHz for vic1div2=600MHz and 40 MHz for VCO1div2=500MHz
    {.xscale = 4, .yscale = 25, .eq = 0},
// 57.14MHz for vic1div2=600MHz and 48 MHz for VCO1div2=500MHz
    {.xscale = 4, .yscale = 21, .eq = 0},
// 83.33MHz for vic1div2=600MHz and 69.44MHz for VCO1div2=500MHz
    {.xscale = 5, .yscale = 18, .eq = 0},
// 100MHz for vic1div2=600MHz and 83.33 MHz for VCO1div2=500MHz
    {.xscale = 2, .yscale = 6, .eq = 0},
// 125MHz for vic1div2=600MHz and 104.1MHz for VCO1div2=500MHz
    {.xscale = 5, .yscale = 12, .eq = 0},
// 150MHz for vic1div2=600MHz and 125MHz for VCO1div2=500MHz
    {.xscale = 2, .yscale = 4, .eq = 0},
// 166MHz for vic1div2=600MHz and 138.88MHz for VCO1div2=500MHz
    {.xscale = 5, .yscale = 18, .eq = 1},
// 200MHz for vic1div2=600MHz and 166MHz for VCO1div2=500MHz
    {.xscale = 1, .yscale = 3, .eq = 1},
// 250MHz for vic1div2=600MHz and 208.33MHz for VCO1div2=500MHz
    {.xscale = 5, .yscale = 12, .eq = 1},
// 300MHz for vic1div2=600MHz and 250MHz for VCO1div2=500MHz
    {.xscale = 1, .yscale = 2, .eq = 1},
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
    {.div = 0x18000}, /* 25 Mhz , for vc01div4 = 300 MHz*/
    {.div = 0x1638E}, /* 27 Mhz , for vc01div4 = 300 MHz*/
    {.div = 0x14000}, /* 25 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x1284B}, /* 27 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x0D8D3}, /* 58 Mhz , for vco1div4 = 393 MHz */
    {.div = 0x0B72C}, /* 58 Mhz , for vco1div4 = 332 MHz */
    {.div = 0x0A584}, /* 58 Mhz , for vco1div4 = 300 MHz */
    {.div = 0x093B1}, /* 65 Mhz , for vc01div4 = 300 MHz*/
    {.div = 0x089EE}, /* 58 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x081BA}, /* 74 Mhz , for vc01div4 = 300 MHz*/
    {.div = 0x07BA0}, /* 65 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x06f1C}, /* 72 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x06E58}, /* 58 Mhz , for vco1div4 = 200 MHz */
    {.div = 0x06c1B}, /* 74 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x058E3}, /* 108 Mhz , for vc01div4 = 300 MHz*/
    {.div = 0x04A12}, /* 108 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x040A5}, /* 148.5 Mhz , for vc01div4 = 300 MHz*/
    {.div = 0x0378E}, /* 144 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x0360D}, /* 148 Mhz , for vc01div4 = 250 MHz*/
    {.div = 0x035E0}, /* 148.5 MHz, for vc01div4 = 250 MHz*/
    };
// i2s prescaler1 masks
    static const struct aux_clk_masks i2s_prs1_masks = {
    .eq_sel_mask = AUX_EQ_SEL_MASK,
    .eq_sel_shift = SPEAR1340_I2S_PRS1_EQ_SEL_SHIFT,
    .eq1_mask = AUX_EQ1_SEL,
    .eq2_mask = AUX_EQ2_SEL,
    .xscale_sel_mask = SPEAR1340_I2S_PRS1_CLK_X_MASK,
    .xscale_sel_shift = SPEAR1340_I2S_PRS1_CLK_X_SHIFT,
    .yscale_sel_mask = SPEAR1340_I2S_PRS1_CLK_Y_MASK,
    .yscale_sel_shift = SPEAR1340_I2S_PRS1_CLK_Y_SHIFT,
    };
// i2s sclk (bit clock) syynthesizers masks
    static const struct aux_clk_masks i2s_sclk_masks = {
    .eq_sel_mask = AUX_EQ_SEL_MASK,
    .eq_sel_shift = SPEAR1340_I2S_SCLK_EQ_SEL_SHIFT,
    .eq1_mask = AUX_EQ1_SEL,
    .eq2_mask = AUX_EQ2_SEL,
    .xscale_sel_mask = SPEAR1340_I2S_SCLK_X_MASK,
    .xscale_sel_shift = SPEAR1340_I2S_SCLK_X_SHIFT,
    .yscale_sel_mask = SPEAR1340_I2S_SCLK_Y_MASK,
    .yscale_sel_shift = SPEAR1340_I2S_SCLK_Y_SHIFT,
    .enable_bit = SPEAR1340_I2S_SCLK_SYNTH_ENB,
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
    {.xscale = 1, .yscale = 2, .eq = 0}, /* 12.288 MHz, smp freq = 48Khz*/
    };
// i2s sclk aux rate configuration table, in ascending order of rates
    static struct aux_rate_tbl i2s_sclk_rtbl[] = {
// For sclk = ref_clk * x/2/y
    {.xscale = 1, .yscale = 4, .eq = 0},
    {.xscale = 1, .yscale = 2, .eq = 0},
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
    {.div = 0x1A92B}, /* 22.5792 MHz for vco1div4=300 MHz*/
    {.div = 0x186A0}, /* 24.576 MHz for vco1div4=300 MHz*/
    {.div = 0x18000}, /* 25 MHz for vco1div4=300 MHz*/
    {.div = 0x1624E}, /* 22.5792 MHz for vco1div4=250 MHz*/
    {.div = 0x14585}, /* 24.576 MHz for vco1div4=250 MHz*/
    {.div = 0x14000}, /* 25 MHz for vco1div4=250 MHz*/
    {.div = 0x0D495}, /* 45.1584 MHz for vco1div4=300 MHz*/
    {.div = 0x0C000}, /* 50 MHz for vco1div4=300 MHz*/
    {.div = 0x0B127}, /* 45.1584 MHz for vco1div4=250 MHz*/
    {.div = 0x0A000}, /* 50 MHz for vco1div4=250 MHz*/
    {.div = 0x07530}, /* 81.92 MHz for vco1div4=300 MHz*/
    {.div = 0x061A8}, /* 81.92 MHz for vco1div4=250 MHz*/
    {.div = 0x06000}, /* 100 MHz for vco1div4=300 MHz*/
    {.div = 0x05000}, /* 100 MHz for vco1div4=250 MHz*/
    {.div = 0x03000}, /* 200 MHz for vco1div4=300 MHz*/
    {.div = 0x02DB6}, /* 210 MHz for vco1div4=300 MHz*/
    {.div = 0x02BA2}, /* 220 MHz for vco1div4=300 MHz*/
    {.div = 0x029BD}, /* 230 MHz for vco1div4=300 MHz*/
    {.div = 0x02800}, /* 200 MHz for vco1div4=250 MHz*/
    {.div = 0x02666}, /* 250 MHz for vco1div4=300 MHz*/
    {.div = 0x02620}, /* 210 MHz for vco1div4=250 MHz*/
    {.div = 0x02460}, /* 220 MHz for vco1div4=250 MHz*/
    {.div = 0x022C0}, /* 230 MHz for vco1div4=250 MHz*/
    {.div = 0x02160}, /* 240 MHz for vco1div4=250 MHz*/
    {.div = 0x02000}, /* 250 MHz for vco1div4=250 MHz*/
    };
// clock parents
    static const char *vco_parents[] = { "osc_24m_clk", "osc_25m_clk", };
    static const char *sys_parents[] = { "pll1_clk", "pll1_clk", "pll1_clk",
    "pll1_clk", "sys_syn_clk", "sys_syn_clk", "pll2_clk", "pll3_clk", };
    static const char *ahb_parents[] = { "cpu_div3_clk", "amba_syn_clk", };
    static const char *gpt_parents[] = { "osc_24m_clk", "apb_clk", };
    static const char *uart0_parents[] = { "pll5_clk", "osc_24m_clk",
    "uart0_syn_gclk", };
    static const char *uart1_parents[] = { "pll5_clk", "osc_24m_clk",
    "uart1_syn_gclk", };
    static const char *c3_parents[] = { "pll5_clk", "c3_syn_gclk", };
    static const char *gmac_phy_input_parents[] = { "gmii_pad_clk", "pll2_clk",
    "osc_25m_clk", };
    static const char *gmac_phy_parents[] = { "phy_input_mclk", "phy_syn_gclk", };
    static const char *clcd_synth_parents[] = { "vco1div4_clk", "pll2_clk", };
    static const char *clcd_pixel_parents[] = { "pll5_clk", "clcd_syn_clk", };
    static const char *i2s_src_parents[] = { "vco1div2_clk", "pll2_clk", "pll3_clk",
    "i2s_src_pad_clk", };
    static const char *i2s_ref_parents[] = { "i2s_src_mclk", "i2s_prs1_clk", };
    static const char *spdif_out_parents[] = { "i2s_src_pad_clk", "gen_syn2_clk", };
    static const char *spdif_in_parents[] = { "pll2_clk", "gen_syn3_clk", };
    static const char *gen_synth0_1_parents[] = { "vco1div4_clk", "vco3div2_clk",
    "pll3_clk", };
    static const char *gen_synth2_3_parents[] = { "vco1div4_clk", "vco2div2_clk",
    "pll2_clk", };
#[no_mangle]
pub unsafe extern "C" fn spear1340_clk_init(misc_base: *mut void __iomem) -> void __init {
    void __init spear1340_clk_init(void __iomem *misc_base)
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
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_RTC_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0580000.rtc");
// clock derived from 24 or 25 MHz osc clk
// vco-pll
    clk = clk_register_mux(core::ptr::null_mut(), "vco1_mclk", vco_parents,
    ARRAY_SIZE(vco_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PLL_CFG, SPEAR1340_PLL1_CLK_SHIFT,
    SPEAR1340_PLL_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "vco1_mclk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco1_clk", "pll1_clk", core::ptr::null_mut(), "vco1_mclk", 0,
    SPEAR1340_PLL1_CTR, SPEAR1340_PLL1_FRQ, pll_rtbl,
    ARRAY_SIZE(pll_rtbl), &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco1_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll1_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "vco2_mclk", vco_parents,
    ARRAY_SIZE(vco_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PLL_CFG, SPEAR1340_PLL2_CLK_SHIFT,
    SPEAR1340_PLL_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "vco2_mclk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco2_clk", "pll2_clk", core::ptr::null_mut(), "vco2_mclk", 0,
    SPEAR1340_PLL2_CTR, SPEAR1340_PLL2_FRQ, pll_rtbl,
    ARRAY_SIZE(pll_rtbl), &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco2_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll2_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "vco3_mclk", vco_parents,
    ARRAY_SIZE(vco_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PLL_CFG, SPEAR1340_PLL3_CLK_SHIFT,
    SPEAR1340_PLL_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "vco3_mclk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco3_clk", "pll3_clk", core::ptr::null_mut(), "vco3_mclk", 0,
    SPEAR1340_PLL3_CTR, SPEAR1340_PLL3_FRQ, pll_rtbl,
    ARRAY_SIZE(pll_rtbl), &_lock, &clk1, core::ptr::null_mut());
    clk_register_clkdev(clk, "vco3_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "pll3_clk", core::ptr::null_mut());
    clk = clk_register_vco_pll("vco4_clk", "pll4_clk", core::ptr::null_mut(), "osc_24m_clk",
    0, SPEAR1340_PLL4_CTR, SPEAR1340_PLL4_FRQ, pll4_rtbl,
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
    SPEAR1340_PERIP2_CLK_ENB, SPEAR1340_THSENS_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e07008c4.thermal");
// clock derived from pll4 clk
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "ddr_clk", "pll4_clk", 0, 1,
    1);
    clk_register_clkdev(clk, "ddr_clk", core::ptr::null_mut());
// clock derived from pll1 clk
    clk = clk_register_frac("sys_syn_clk", "vco1div2_clk", 0,
    SPEAR1340_SYS_CLK_SYNT, sys_synth_rtbl,
    ARRAY_SIZE(sys_synth_rtbl), &_lock);
    clk_register_clkdev(clk, "sys_syn_clk", core::ptr::null_mut());
    clk = clk_register_frac("amba_syn_clk", "vco1div2_clk", 0,
    SPEAR1340_AMBA_CLK_SYNT, amba_synth_rtbl,
    ARRAY_SIZE(amba_synth_rtbl), &_lock);
    clk_register_clkdev(clk, "amba_syn_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "sys_mclk", sys_parents,
    ARRAY_SIZE(sys_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_SYS_CLK_CTRL, SPEAR1340_SCLK_SRC_SEL_SHIFT,
    SPEAR1340_SCLK_SRC_SEL_MASK, 0, &_lock);
    clk_register_clkdev(clk, "sys_mclk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "cpu_clk", "sys_mclk", 0, 1,
    2);
    clk_register_clkdev(clk, "cpu_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "cpu_div3_clk", "cpu_clk", 0, 1,
    3);
    clk_register_clkdev(clk, "cpu_div3_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "wdt_clk", "cpu_clk", 0, 1,
    2);
    clk_register_clkdev(clk, core::ptr::null_mut(), "ec800620.wdt");
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "smp_twd_clk", "cpu_clk", 0, 1,
    2);
    clk_register_clkdev(clk, core::ptr::null_mut(), "smp_twd");
    clk = clk_register_mux(core::ptr::null_mut(), "ahb_clk", ahb_parents,
    ARRAY_SIZE(ahb_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_SYS_CLK_CTRL, SPEAR1340_HCLK_SRC_SEL_SHIFT,
    SPEAR1340_HCLK_SRC_SEL_MASK, 0, &_lock);
    clk_register_clkdev(clk, "ahb_clk", core::ptr::null_mut());
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "apb_clk", "ahb_clk", 0, 1,
    2);
    clk_register_clkdev(clk, "apb_clk", core::ptr::null_mut());
// gpt clocks
    clk = clk_register_mux(core::ptr::null_mut(), "gpt0_mclk", gpt_parents,
    ARRAY_SIZE(gpt_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_GPT0_CLK_SHIFT,
    SPEAR1340_GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt0_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt0_clk", "gpt0_mclk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_GPT0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt0");
    clk = clk_register_mux(core::ptr::null_mut(), "gpt1_mclk", gpt_parents,
    ARRAY_SIZE(gpt_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_GPT1_CLK_SHIFT,
    SPEAR1340_GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt1_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt1_clk", "gpt1_mclk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_GPT1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt1");
    clk = clk_register_mux(core::ptr::null_mut(), "gpt2_mclk", gpt_parents,
    ARRAY_SIZE(gpt_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_GPT2_CLK_SHIFT,
    SPEAR1340_GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt2_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt2_clk", "gpt2_mclk", 0,
    SPEAR1340_PERIP2_CLK_ENB, SPEAR1340_GPT2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt2");
    clk = clk_register_mux(core::ptr::null_mut(), "gpt3_mclk", gpt_parents,
    ARRAY_SIZE(gpt_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_GPT3_CLK_SHIFT,
    SPEAR1340_GPT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gpt3_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "gpt3_clk", "gpt3_mclk", 0,
    SPEAR1340_PERIP2_CLK_ENB, SPEAR1340_GPT3_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "gpt3");
// others
    clk = clk_register_aux("uart0_syn_clk", "uart0_syn_gclk",
    "vco1div2_clk", 0, SPEAR1340_UART0_CLK_SYNT, core::ptr::null_mut(),
    aux_rtbl, ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "uart0_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "uart0_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "uart0_mclk", uart0_parents,
    ARRAY_SIZE(uart0_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_UART0_CLK_SHIFT,
    SPEAR1340_UART_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "uart0_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart0_clk", "uart0_mclk",
    CLK_SET_RATE_PARENT, SPEAR1340_PERIP1_CLK_ENB,
    SPEAR1340_UART0_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0000000.serial");
    clk = clk_register_aux("uart1_syn_clk", "uart1_syn_gclk",
    "vco1div2_clk", 0, SPEAR1340_UART1_CLK_SYNT, core::ptr::null_mut(),
    aux_rtbl, ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "uart1_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "uart1_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "uart1_mclk", uart1_parents,
    ARRAY_SIZE(uart1_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_UART1_CLK_SHIFT,
    SPEAR1340_UART_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "uart1_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "uart1_clk", "uart1_mclk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_UART1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b4100000.serial");
    clk = clk_register_aux("sdhci_syn_clk", "sdhci_syn_gclk",
    "vco1div2_clk", 0, SPEAR1340_SDHCI_CLK_SYNT, core::ptr::null_mut(),
    aux_rtbl, ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "sdhci_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "sdhci_syn_gclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "sdhci_clk", "sdhci_syn_gclk",
    CLK_SET_RATE_PARENT, SPEAR1340_PERIP1_CLK_ENB,
    SPEAR1340_SDHCI_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b3000000.sdhci");
    clk = clk_register_aux("cfxd_syn_clk", "cfxd_syn_gclk", "vco1div2_clk",
    0, SPEAR1340_CFXD_CLK_SYNT, core::ptr::null_mut(), aux_rtbl,
    ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "cfxd_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "cfxd_syn_gclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "cfxd_clk", "cfxd_syn_gclk",
    CLK_SET_RATE_PARENT, SPEAR1340_PERIP1_CLK_ENB,
    SPEAR1340_CFXD_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2800000.cf");
    clk_register_clkdev(clk, core::ptr::null_mut(), "arasan_xd");
    clk = clk_register_aux("c3_syn_clk", "c3_syn_gclk", "vco1div2_clk", 0,
    SPEAR1340_C3_CLK_SYNT, core::ptr::null_mut(), aux_rtbl,
    ARRAY_SIZE(aux_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "c3_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "c3_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "c3_mclk", c3_parents,
    ARRAY_SIZE(c3_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_C3_CLK_SHIFT,
    SPEAR1340_C3_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "c3_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "c3_clk", "c3_mclk", CLK_SET_RATE_PARENT,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_C3_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e1800000.c3");
// gmac
    clk = clk_register_mux(core::ptr::null_mut(), "phy_input_mclk", gmac_phy_input_parents,
    ARRAY_SIZE(gmac_phy_input_parents),
    CLK_SET_RATE_NO_REPARENT, SPEAR1340_GMAC_CLK_CFG,
    SPEAR1340_GMAC_PHY_INPUT_CLK_SHIFT,
    SPEAR1340_GMAC_PHY_INPUT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "phy_input_mclk", core::ptr::null_mut());
    clk = clk_register_aux("phy_syn_clk", "phy_syn_gclk", "phy_input_mclk",
    0, SPEAR1340_GMAC_CLK_SYNT, core::ptr::null_mut(), gmac_rtbl,
    ARRAY_SIZE(gmac_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "phy_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "phy_syn_gclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "phy_mclk", gmac_phy_parents,
    ARRAY_SIZE(gmac_phy_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_GMAC_PHY_CLK_SHIFT,
    SPEAR1340_GMAC_PHY_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "stmmacphy.0", core::ptr::null_mut());
// clcd
    clk = clk_register_mux(core::ptr::null_mut(), "clcd_syn_mclk", clcd_synth_parents,
    ARRAY_SIZE(clcd_synth_parents),
    CLK_SET_RATE_NO_REPARENT, SPEAR1340_CLCD_CLK_SYNT,
    SPEAR1340_CLCD_SYNT_CLK_SHIFT,
    SPEAR1340_CLCD_SYNT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "clcd_syn_mclk", core::ptr::null_mut());
    clk = clk_register_frac("clcd_syn_clk", "clcd_syn_mclk", 0,
    SPEAR1340_CLCD_CLK_SYNT, clcd_rtbl,
    ARRAY_SIZE(clcd_rtbl), &_lock);
    clk_register_clkdev(clk, "clcd_syn_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "clcd_pixel_mclk", clcd_pixel_parents,
    ARRAY_SIZE(clcd_pixel_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_CLCD_CLK_SHIFT,
    SPEAR1340_CLCD_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "clcd_pixel_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "clcd_clk", "clcd_pixel_mclk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_CLCD_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e1000000.clcd");
// i2s
    clk = clk_register_mux(core::ptr::null_mut(), "i2s_src_mclk", i2s_src_parents,
    ARRAY_SIZE(i2s_src_parents), CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_I2S_CLK_CFG, SPEAR1340_I2S_SRC_CLK_SHIFT,
    SPEAR1340_I2S_SRC_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2s_src_mclk", core::ptr::null_mut());
    clk = clk_register_aux("i2s_prs1_clk", core::ptr::null_mut(), "i2s_src_mclk",
    CLK_SET_RATE_PARENT, SPEAR1340_I2S_CLK_CFG,
    &i2s_prs1_masks, i2s_prs1_rtbl,
    ARRAY_SIZE(i2s_prs1_rtbl), &_lock, core::ptr::null_mut());
    clk_register_clkdev(clk, "i2s_prs1_clk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "i2s_ref_mclk", i2s_ref_parents,
    ARRAY_SIZE(i2s_ref_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_I2S_CLK_CFG, SPEAR1340_I2S_REF_SHIFT,
    SPEAR1340_I2S_REF_SEL_MASK, 0, &_lock);
    clk_register_clkdev(clk, "i2s_ref_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "i2s_ref_pad_clk", "i2s_ref_mclk", 0,
    SPEAR1340_PERIP2_CLK_ENB, SPEAR1340_I2S_REF_PAD_CLK_ENB,
    0, &_lock);
    clk_register_clkdev(clk, "i2s_ref_pad_clk", core::ptr::null_mut());
    clk = clk_register_aux("i2s_sclk_clk", "i2s_sclk_gclk", "i2s_ref_mclk",
    0, SPEAR1340_I2S_CLK_CFG, &i2s_sclk_masks,
    i2s_sclk_rtbl, ARRAY_SIZE(i2s_sclk_rtbl), &_lock,
    &clk1);
    clk_register_clkdev(clk, "i2s_sclk_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "i2s_sclk_gclk", core::ptr::null_mut());
// clock derived from ahb clk
    clk = clk_register_gate(core::ptr::null_mut(), "i2c0_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_I2C0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0280000.i2c");
    clk = clk_register_gate(core::ptr::null_mut(), "i2c1_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_I2C1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b4000000.i2c");
    clk = clk_register_gate(core::ptr::null_mut(), "dma_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_DMA_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "ea800000.dma");
    clk_register_clkdev(clk, core::ptr::null_mut(), "eb000000.dma");
    clk = clk_register_gate(core::ptr::null_mut(), "gmac_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_GMAC_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e2000000.eth");
    clk = clk_register_gate(core::ptr::null_mut(), "fsmc_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_FSMC_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b0000000.flash");
    clk = clk_register_gate(core::ptr::null_mut(), "smi_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_SMI_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "ea000000.flash");
    clk = clk_register_gate(core::ptr::null_mut(), "usbh0_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_UHC0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e4000000.ohci");
    clk_register_clkdev(clk, core::ptr::null_mut(), "e4800000.ehci");
    clk = clk_register_gate(core::ptr::null_mut(), "usbh1_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_UHC1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e5000000.ohci");
    clk_register_clkdev(clk, core::ptr::null_mut(), "e5800000.ehci");
    clk = clk_register_gate(core::ptr::null_mut(), "uoc_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_UOC_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e3800000.otg");
    clk = clk_register_gate(core::ptr::null_mut(), "pcie_sata_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_PCIE_SATA_CLK_ENB,
    0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b1000000.pcie");
    clk_register_clkdev(clk, core::ptr::null_mut(), "b1000000.ahci");
    clk = clk_register_gate(core::ptr::null_mut(), "sysram0_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_SYSRAM0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "sysram0_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "sysram1_clk", "ahb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_SYSRAM1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, "sysram1_clk", core::ptr::null_mut());
    clk = clk_register_aux("adc_syn_clk", "adc_syn_gclk", "ahb_clk",
    0, SPEAR1340_ADC_CLK_SYNT, core::ptr::null_mut(), adc_rtbl,
    ARRAY_SIZE(adc_rtbl), &_lock, &clk1);
    clk_register_clkdev(clk, "adc_syn_clk", core::ptr::null_mut());
    clk_register_clkdev(clk1, "adc_syn_gclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "adc_clk", "adc_syn_gclk",
    CLK_SET_RATE_PARENT, SPEAR1340_PERIP1_CLK_ENB,
    SPEAR1340_ADC_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0080000.adc");
// clock derived from apb clk
    clk = clk_register_gate(core::ptr::null_mut(), "ssp_clk", "apb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_SSP_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0100000.spi");
    clk = clk_register_gate(core::ptr::null_mut(), "gpio0_clk", "apb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_GPIO0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0600000.gpio");
    clk = clk_register_gate(core::ptr::null_mut(), "gpio1_clk", "apb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_GPIO1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0680000.gpio");
    clk = clk_register_gate(core::ptr::null_mut(), "i2s_play_clk", "apb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_I2S_PLAY_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2400000.i2s-play");
    clk = clk_register_gate(core::ptr::null_mut(), "i2s_rec_clk", "apb_clk", 0,
    SPEAR1340_PERIP1_CLK_ENB, SPEAR1340_I2S_REC_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "b2000000.i2s-rec");
    clk = clk_register_gate(core::ptr::null_mut(), "kbd_clk", "apb_clk", 0,
    SPEAR1340_PERIP2_CLK_ENB, SPEAR1340_KBD_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0300000.kbd");
// RAS clks
    clk = clk_register_mux(core::ptr::null_mut(), "gen_syn0_1_mclk", gen_synth0_1_parents,
    ARRAY_SIZE(gen_synth0_1_parents),
    CLK_SET_RATE_NO_REPARENT, SPEAR1340_PLL_CFG,
    SPEAR1340_GEN_SYNT0_1_CLK_SHIFT,
    SPEAR1340_GEN_SYNT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gen_syn0_1_mclk", core::ptr::null_mut());
    clk = clk_register_mux(core::ptr::null_mut(), "gen_syn2_3_mclk", gen_synth2_3_parents,
    ARRAY_SIZE(gen_synth2_3_parents),
    CLK_SET_RATE_NO_REPARENT, SPEAR1340_PLL_CFG,
    SPEAR1340_GEN_SYNT2_3_CLK_SHIFT,
    SPEAR1340_GEN_SYNT_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "gen_syn2_3_mclk", core::ptr::null_mut());
    clk = clk_register_frac("gen_syn0_clk", "gen_syn0_1_mclk", 0,
    SPEAR1340_GEN_CLK_SYNT0, gen_rtbl, ARRAY_SIZE(gen_rtbl),
    &_lock);
    clk_register_clkdev(clk, "gen_syn0_clk", core::ptr::null_mut());
    clk = clk_register_frac("gen_syn1_clk", "gen_syn0_1_mclk", 0,
    SPEAR1340_GEN_CLK_SYNT1, gen_rtbl, ARRAY_SIZE(gen_rtbl),
    &_lock);
    clk_register_clkdev(clk, "gen_syn1_clk", core::ptr::null_mut());
    clk = clk_register_frac("gen_syn2_clk", "gen_syn2_3_mclk", 0,
    SPEAR1340_GEN_CLK_SYNT2, gen_rtbl, ARRAY_SIZE(gen_rtbl),
    &_lock);
    clk_register_clkdev(clk, "gen_syn2_clk", core::ptr::null_mut());
    clk = clk_register_frac("gen_syn3_clk", "gen_syn2_3_mclk", 0,
    SPEAR1340_GEN_CLK_SYNT3, gen_rtbl, ARRAY_SIZE(gen_rtbl),
    &_lock);
    clk_register_clkdev(clk, "gen_syn3_clk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "mali_clk", "gen_syn3_clk",
    CLK_SET_RATE_PARENT, SPEAR1340_PERIP3_CLK_ENB,
    SPEAR1340_MALI_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "mali");
    clk = clk_register_gate(core::ptr::null_mut(), "cec0_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_CEC0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "spear_cec.0");
    clk = clk_register_gate(core::ptr::null_mut(), "cec1_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_CEC1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "spear_cec.1");
    clk = clk_register_mux(core::ptr::null_mut(), "spdif_out_mclk", spdif_out_parents,
    ARRAY_SIZE(spdif_out_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_SPDIF_OUT_CLK_SHIFT,
    SPEAR1340_SPDIF_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "spdif_out_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "spdif_out_clk", "spdif_out_mclk",
    CLK_SET_RATE_PARENT, SPEAR1340_PERIP3_CLK_ENB,
    SPEAR1340_SPDIF_OUT_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0000000.spdif-out");
    clk = clk_register_mux(core::ptr::null_mut(), "spdif_in_mclk", spdif_in_parents,
    ARRAY_SIZE(spdif_in_parents),
    CLK_SET_RATE_PARENT | CLK_SET_RATE_NO_REPARENT,
    SPEAR1340_PERIP_CLK_CFG, SPEAR1340_SPDIF_IN_CLK_SHIFT,
    SPEAR1340_SPDIF_CLK_MASK, 0, &_lock);
    clk_register_clkdev(clk, "spdif_in_mclk", core::ptr::null_mut());
    clk = clk_register_gate(core::ptr::null_mut(), "spdif_in_clk", "spdif_in_mclk",
    CLK_SET_RATE_PARENT, SPEAR1340_PERIP3_CLK_ENB,
    SPEAR1340_SPDIF_IN_CLK_ENB, 0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0100000.spdif-in");
    clk = clk_register_gate(core::ptr::null_mut(), "acp_clk", "ahb_clk", 0,
    SPEAR1340_PERIP2_CLK_ENB, SPEAR1340_ACP_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "acp_clk");
    clk = clk_register_gate(core::ptr::null_mut(), "plgpio_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_PLGPIO_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e2800000.gpio");
    clk = clk_register_gate(core::ptr::null_mut(), "video_dec_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_VIDEO_DEC_CLK_ENB,
    0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "video_dec");
    clk = clk_register_gate(core::ptr::null_mut(), "video_enc_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_VIDEO_ENC_CLK_ENB,
    0, &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "video_enc");
    clk = clk_register_gate(core::ptr::null_mut(), "video_in_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_VIDEO_IN_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "spear_vip");
    clk = clk_register_gate(core::ptr::null_mut(), "cam0_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_CAM0_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0200000.cam0");
    clk = clk_register_gate(core::ptr::null_mut(), "cam1_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_CAM1_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0300000.cam1");
    clk = clk_register_gate(core::ptr::null_mut(), "cam2_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_CAM2_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0400000.cam2");
    clk = clk_register_gate(core::ptr::null_mut(), "cam3_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_CAM3_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "d0500000.cam3");
    clk = clk_register_gate(core::ptr::null_mut(), "pwm_clk", "ahb_clk", 0,
    SPEAR1340_PERIP3_CLK_ENB, SPEAR1340_PWM_CLK_ENB, 0,
    &_lock);
    clk_register_clkdev(clk, core::ptr::null_mut(), "e0180000.pwm");
    }
