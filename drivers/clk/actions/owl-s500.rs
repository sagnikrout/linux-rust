//! Automatically rewritten from C to Rust
//! Source: drivers/clk/actions/owl-s500.c
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
// Actions Semi Owl S500 SoC clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
//
// Copyright (c) 2018 LSI-TEC - Caninos Loucos
// Author: Edgar Bernardi Righi <edgar.righi@lsitec.org.br>
//

    static const struct clk_pll_table clk_audio_pll_table[] = {
    { 0, 45158400 }, { 1, 49152000 },
    { /* sentinel */ }
    };
// pll clocks
    static OWL_PLL_NO_PARENT_DELAY(ethernet_pll_clk, "ethernet_pll_clk", CMU_ETHERNETPLL, 500000000, 0, 0, 0, 0, 0, OWL_S500_ETHERNETPLL_DELAY, core::ptr::null_mut(), CLK_IGNORE_UNUSED);
    static OWL_PLL_NO_PARENT_DELAY(core_pll_clk, "core_pll_clk", CMU_COREPLL, 12000000, 9, 0, 8, 4, 134, OWL_S500_COREPLL_DELAY, core::ptr::null_mut(), CLK_IGNORE_UNUSED);
    static OWL_PLL_NO_PARENT_DELAY(ddr_pll_clk, "ddr_pll_clk", CMU_DDRPLL, 12000000, 8, 0, 8, 1, 67, OWL_S500_DDRPLL_DELAY, core::ptr::null_mut(), CLK_IGNORE_UNUSED);
    static OWL_PLL_NO_PARENT_DELAY(nand_pll_clk, "nand_pll_clk", CMU_NANDPLL, 6000000, 8, 0, 7, 2, 86, OWL_S500_NANDPLL_DELAY, core::ptr::null_mut(), CLK_IGNORE_UNUSED);
    static OWL_PLL_NO_PARENT_DELAY(display_pll_clk, "display_pll_clk", CMU_DISPLAYPLL, 6000000, 8, 0, 8, 2, 126, OWL_S500_DISPLAYPLL_DELAY, core::ptr::null_mut(), CLK_IGNORE_UNUSED);
    static OWL_PLL_NO_PARENT_DELAY(dev_pll_clk, "dev_pll_clk", CMU_DEVPLL, 6000000, 8, 0, 7, 8, 126, OWL_S500_DEVPLL_DELAY, core::ptr::null_mut(), CLK_IGNORE_UNUSED);
    static OWL_PLL_NO_PARENT_DELAY(audio_pll_clk, "audio_pll_clk", CMU_AUDIOPLL, 0, 4, 0, 1, 0, 0, OWL_S500_AUDIOPLL_DELAY, clk_audio_pll_table, CLK_IGNORE_UNUSED);
    static const char * const dev_clk_mux_p[] = { "hosc", "dev_pll_clk" };
    static const char * const bisp_clk_mux_p[] = { "display_pll_clk", "dev_clk" };
    static const char * const sensor_clk_mux_p[] = { "hosc", "bisp_clk" };
    static const char * const sd_clk_mux_p[] = { "dev_clk", "nand_pll_clk" };
    static const char * const pwm_clk_mux_p[] = { "losc", "hosc" };
    static const char * const ahbprediv_clk_mux_p[] = { "dev_clk", "display_pll_clk", "nand_pll_clk", "ddr_pll_clk" };
    static const char * const nic_clk_mux_p[] = { "dev_clk", "display_pll_clk", "nand_pll_clk", "ddr_pll_clk" };
    static const char * const uart_clk_mux_p[] = { "hosc", "dev_pll_clk" };
    static const char * const de_clk_mux_p[] = { "display_pll_clk", "dev_clk" };
    static const char * const i2s_clk_mux_p[] = { "audio_pll_clk" };
    static const char * const hde_clk_mux_p[] = { "dev_clk", "display_pll_clk", "nand_pll_clk", "ddr_pll_clk" };
    static const char * const nand_clk_mux_p[] = { "nand_pll_clk", "display_pll_clk", "dev_clk", "ddr_pll_clk" };
    static struct clk_factor_table sd_factor_table[] = {
// bit0 ~ 4
    { 0, 1, 1 }, { 1, 1, 2 }, { 2, 1, 3 }, { 3, 1, 4 },
    { 4, 1, 5 }, { 5, 1, 6 }, { 6, 1, 7 }, { 7, 1, 8 },
    { 8, 1, 9 }, { 9, 1, 10 }, { 10, 1, 11 }, { 11, 1, 12 },
    { 12, 1, 13 }, { 13, 1, 14 }, { 14, 1, 15 }, { 15, 1, 16 },
    { 16, 1, 17 }, { 17, 1, 18 }, { 18, 1, 19 }, { 19, 1, 20 },
    { 20, 1, 21 }, { 21, 1, 22 }, { 22, 1, 23 }, { 23, 1, 24 },
    { 24, 1, 25 },
// bit8: /128
    { 256, 1, 1 * 128 }, { 257, 1, 2 * 128 }, { 258, 1, 3 * 128 }, { 259, 1, 4 * 128 },
    { 260, 1, 5 * 128 }, { 261, 1, 6 * 128 }, { 262, 1, 7 * 128 }, { 263, 1, 8 * 128 },
    { 264, 1, 9 * 128 }, { 265, 1, 10 * 128 }, { 266, 1, 11 * 128 }, { 267, 1, 12 * 128 },
    { 268, 1, 13 * 128 }, { 269, 1, 14 * 128 }, { 270, 1, 15 * 128 }, { 271, 1, 16 * 128 },
    { 272, 1, 17 * 128 }, { 273, 1, 18 * 128 }, { 274, 1, 19 * 128 }, { 275, 1, 20 * 128 },
    { 276, 1, 21 * 128 }, { 277, 1, 22 * 128 }, { 278, 1, 23 * 128 }, { 279, 1, 24 * 128 },
    { 280, 1, 25 * 128 },
    { /* sentinel */ }
    };
    static struct clk_factor_table de_factor_table[] = {
    { 0, 1, 1 }, { 1, 2, 3 }, { 2, 1, 2 }, { 3, 2, 5 },
    { 4, 1, 3 }, { 5, 1, 4 }, { 6, 1, 6 }, { 7, 1, 8 },
    { 8, 1, 12 },
    { /* sentinel */ }
    };
    static struct clk_factor_table hde_factor_table[] = {
    { 0, 1, 1 }, { 1, 2, 3 }, { 2, 1, 2 }, { 3, 2, 5 },
    { 4, 1, 3 }, { 5, 1, 4 }, { 6, 1, 6 }, { 7, 1, 8 },
    { /* sentinel */ }
    };
    static struct clk_div_table rmii_ref_div_table[] = {
    { 0, 4 }, { 1, 10 },
    { /* sentinel */ }
    };
    static struct clk_div_table std12rate_div_table[] = {
    { 0, 1 }, { 1, 2 }, { 2, 3 }, { 3, 4 },
    { 4, 5 }, { 5, 6 }, { 6, 7 }, { 7, 8 },
    { 8, 9 }, { 9, 10 }, { 10, 11 }, { 11, 12 },
    { /* sentinel */ }
    };
    static struct clk_div_table i2s_div_table[] = {
    { 0, 1 }, { 1, 2 }, { 2, 3 }, { 3, 4 },
    { 4, 6 }, { 5, 8 }, { 6, 12 }, { 7, 16 },
    { 8, 24 },
    { /* sentinel */ }
    };
    static struct clk_div_table nand_div_table[] = {
    { 0, 1 }, { 1, 2 }, { 2, 4 }, { 3, 6 },
    { 4, 8 }, { 5, 10 }, { 6, 12 }, { 7, 14 },
    { 8, 16 }, { 9, 18 }, { 10, 20 }, { 11, 22 },
    { /* sentinel */ }
    };
// mux clock
    static OWL_MUX(dev_clk, "dev_clk", dev_clk_mux_p, CMU_DEVPLL, 12, 1, CLK_SET_RATE_PARENT);
// gate clocks
    static OWL_GATE(gpio_clk, "gpio_clk", "apb_clk", CMU_DEVCLKEN0, 18, 0, 0);
    static OWL_GATE(dmac_clk, "dmac_clk", "h_clk", CMU_DEVCLKEN0, 1, 0, 0);
    static OWL_GATE(spi0_clk, "spi0_clk", "ahb_clk", CMU_DEVCLKEN1, 10, 0, CLK_IGNORE_UNUSED);
    static OWL_GATE(spi1_clk, "spi1_clk", "ahb_clk", CMU_DEVCLKEN1, 11, 0, CLK_IGNORE_UNUSED);
    static OWL_GATE(spi2_clk, "spi2_clk", "ahb_clk", CMU_DEVCLKEN1, 12, 0, CLK_IGNORE_UNUSED);
    static OWL_GATE(spi3_clk, "spi3_clk", "ahb_clk", CMU_DEVCLKEN1, 13, 0, CLK_IGNORE_UNUSED);
    static OWL_GATE(timer_clk, "timer_clk", "hosc", CMU_DEVCLKEN1, 27, 0, 0);
    static OWL_GATE(hdmi_clk, "hdmi_clk", "hosc", CMU_DEVCLKEN1, 3, 0, 0);
// divider clocks
    static OWL_DIVIDER(h_clk, "h_clk", "ahbprediv_clk", CMU_BUSCLK1, 2, 2, core::ptr::null_mut(), 0, 0);
    static OWL_DIVIDER(apb_clk, "apb_clk", "nic_clk", CMU_BUSCLK1, 14, 2, core::ptr::null_mut(), 0, 0);
    static OWL_DIVIDER(rmii_ref_clk, "rmii_ref_clk", "ethernet_pll_clk", CMU_ETHERNETPLL, 1, 1, rmii_ref_div_table, 0, 0);
// factor clocks
    static OWL_FACTOR(de1_clk, "de_clk1", "de_clk", CMU_DECLK, 0, 4, de_factor_table, 0, 0);
    static OWL_FACTOR(de2_clk, "de_clk2", "de_clk", CMU_DECLK, 4, 4, de_factor_table, 0, 0);
// composite clocks
    static OWL_COMP_DIV(nic_clk, "nic_clk", nic_clk_mux_p,
    OWL_MUX_HW(CMU_BUSCLK1, 4, 3),
    { 0 },
    OWL_DIVIDER_HW(CMU_BUSCLK1, 16, 2, 0, core::ptr::null_mut()),
    0);
    static OWL_COMP_DIV(ahbprediv_clk, "ahbprediv_clk", ahbprediv_clk_mux_p,
    OWL_MUX_HW(CMU_BUSCLK1, 8, 3),
    { 0 },
    OWL_DIVIDER_HW(CMU_BUSCLK1, 12, 2, 0, core::ptr::null_mut()),
    CLK_SET_RATE_PARENT);
    static OWL_COMP_FIXED_FACTOR(ahb_clk, "ahb_clk", "h_clk",
    { 0 },
    1, 1, 0);
    static OWL_COMP_FACTOR(vce_clk, "vce_clk", hde_clk_mux_p,
    OWL_MUX_HW(CMU_VCECLK, 4, 2),
    OWL_GATE_HW(CMU_DEVCLKEN0, 26, 0),
    OWL_FACTOR_HW(CMU_VCECLK, 0, 3, 0, hde_factor_table),
    0);
    static OWL_COMP_FACTOR(vde_clk, "vde_clk", hde_clk_mux_p,
    OWL_MUX_HW(CMU_VDECLK, 4, 2),
    OWL_GATE_HW(CMU_DEVCLKEN0, 25, 0),
    OWL_FACTOR_HW(CMU_VDECLK, 0, 3, 0, hde_factor_table),
    0);
    static OWL_COMP_DIV(bisp_clk, "bisp_clk", bisp_clk_mux_p,
    OWL_MUX_HW(CMU_BISPCLK, 4, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 14, 0),
    OWL_DIVIDER_HW(CMU_BISPCLK, 0, 4, 0, std12rate_div_table),
    0);
    static OWL_COMP_DIV(sensor0_clk, "sensor0_clk", sensor_clk_mux_p,
    OWL_MUX_HW(CMU_SENSORCLK, 4, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 14, 0),
    OWL_DIVIDER_HW(CMU_SENSORCLK, 0, 4, 0, std12rate_div_table),
    0);
    static OWL_COMP_DIV(sensor1_clk, "sensor1_clk", sensor_clk_mux_p,
    OWL_MUX_HW(CMU_SENSORCLK, 4, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 14, 0),
    OWL_DIVIDER_HW(CMU_SENSORCLK, 8, 4, 0, std12rate_div_table),
    0);
    static OWL_COMP_FACTOR(sd0_clk, "sd0_clk", sd_clk_mux_p,
    OWL_MUX_HW(CMU_SD0CLK, 9, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 5, 0),
    OWL_FACTOR_HW(CMU_SD0CLK, 0, 9, 0, sd_factor_table),
    0);
    static OWL_COMP_FACTOR(sd1_clk, "sd1_clk", sd_clk_mux_p,
    OWL_MUX_HW(CMU_SD1CLK, 9, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 6, 0),
    OWL_FACTOR_HW(CMU_SD1CLK, 0, 9, 0, sd_factor_table),
    0);
    static OWL_COMP_FACTOR(sd2_clk, "sd2_clk", sd_clk_mux_p,
    OWL_MUX_HW(CMU_SD2CLK, 9, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 7, 0),
    OWL_FACTOR_HW(CMU_SD2CLK, 0, 9, 0, sd_factor_table),
    0);
    static OWL_COMP_DIV(pwm0_clk, "pwm0_clk", pwm_clk_mux_p,
    OWL_MUX_HW(CMU_PWM0CLK, 12, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 23, 0),
    OWL_DIVIDER_HW(CMU_PWM0CLK, 0, 10, 0, core::ptr::null_mut()),
    0);
    static OWL_COMP_DIV(pwm1_clk, "pwm1_clk", pwm_clk_mux_p,
    OWL_MUX_HW(CMU_PWM1CLK, 12, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 24, 0),
    OWL_DIVIDER_HW(CMU_PWM1CLK, 0, 10, 0, core::ptr::null_mut()),
    0);
    static OWL_COMP_DIV(pwm2_clk, "pwm2_clk", pwm_clk_mux_p,
    OWL_MUX_HW(CMU_PWM2CLK, 12, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 25, 0),
    OWL_DIVIDER_HW(CMU_PWM2CLK, 0, 10, 0, core::ptr::null_mut()),
    0);
    static OWL_COMP_DIV(pwm3_clk, "pwm3_clk", pwm_clk_mux_p,
    OWL_MUX_HW(CMU_PWM3CLK, 12, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 26, 0),
    OWL_DIVIDER_HW(CMU_PWM3CLK, 0, 10, 0, core::ptr::null_mut()),
    0);
    static OWL_COMP_DIV(pwm4_clk, "pwm4_clk", pwm_clk_mux_p,
    OWL_MUX_HW(CMU_PWM4CLK, 12, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 11, 0),
    OWL_DIVIDER_HW(CMU_PWM4CLK, 0, 10, 0, core::ptr::null_mut()),
    0);
    static OWL_COMP_DIV(pwm5_clk, "pwm5_clk", pwm_clk_mux_p,
    OWL_MUX_HW(CMU_PWM5CLK, 12, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 0, 0),
    OWL_DIVIDER_HW(CMU_PWM5CLK, 0, 10, 0, core::ptr::null_mut()),
    0);
    static OWL_COMP_PASS(de_clk, "de_clk", de_clk_mux_p,
    OWL_MUX_HW(CMU_DECLK, 12, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 8, 0),
    0);
    static OWL_COMP_FIXED_FACTOR(i2c0_clk, "i2c0_clk", "ethernet_pll_clk",
    OWL_GATE_HW(CMU_DEVCLKEN1, 14, 0),
    1, 5, 0);
    static OWL_COMP_FIXED_FACTOR(i2c1_clk, "i2c1_clk", "ethernet_pll_clk",
    OWL_GATE_HW(CMU_DEVCLKEN1, 15, 0),
    1, 5, 0);
    static OWL_COMP_FIXED_FACTOR(i2c2_clk, "i2c2_clk", "ethernet_pll_clk",
    OWL_GATE_HW(CMU_DEVCLKEN1, 30, 0),
    1, 5, 0);
    static OWL_COMP_FIXED_FACTOR(i2c3_clk, "i2c3_clk", "ethernet_pll_clk",
    OWL_GATE_HW(CMU_DEVCLKEN1, 31, 0),
    1, 5, 0);
    static OWL_COMP_FIXED_FACTOR(ethernet_clk, "ethernet_clk", "ethernet_pll_clk",
    OWL_GATE_HW(CMU_DEVCLKEN1, 22, 0),
    1, 20, 0);
    static OWL_COMP_DIV(uart0_clk, "uart0_clk", uart_clk_mux_p,
    OWL_MUX_HW(CMU_UART0CLK, 16, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 6, 0),
    OWL_DIVIDER_HW(CMU_UART0CLK, 0, 8, CLK_DIVIDER_ROUND_CLOSEST, core::ptr::null_mut()),
    CLK_IGNORE_UNUSED);
    static OWL_COMP_DIV(uart1_clk, "uart1_clk", uart_clk_mux_p,
    OWL_MUX_HW(CMU_UART1CLK, 16, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 7, 0),
    OWL_DIVIDER_HW(CMU_UART1CLK, 0, 8, CLK_DIVIDER_ROUND_CLOSEST, core::ptr::null_mut()),
    CLK_IGNORE_UNUSED);
    static OWL_COMP_DIV(uart2_clk, "uart2_clk", uart_clk_mux_p,
    OWL_MUX_HW(CMU_UART2CLK, 16, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 8, 0),
    OWL_DIVIDER_HW(CMU_UART2CLK, 0, 8, CLK_DIVIDER_ROUND_CLOSEST, core::ptr::null_mut()),
    CLK_IGNORE_UNUSED);
    static OWL_COMP_DIV(uart3_clk, "uart3_clk", uart_clk_mux_p,
    OWL_MUX_HW(CMU_UART3CLK, 16, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 19, 0),
    OWL_DIVIDER_HW(CMU_UART3CLK, 0, 8, CLK_DIVIDER_ROUND_CLOSEST, core::ptr::null_mut()),
    CLK_IGNORE_UNUSED);
    static OWL_COMP_DIV(uart4_clk, "uart4_clk", uart_clk_mux_p,
    OWL_MUX_HW(CMU_UART4CLK, 16, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 20, 0),
    OWL_DIVIDER_HW(CMU_UART4CLK, 0, 8, CLK_DIVIDER_ROUND_CLOSEST, core::ptr::null_mut()),
    CLK_IGNORE_UNUSED);
    static OWL_COMP_DIV(uart5_clk, "uart5_clk", uart_clk_mux_p,
    OWL_MUX_HW(CMU_UART5CLK, 16, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 21, 0),
    OWL_DIVIDER_HW(CMU_UART5CLK, 0, 8, CLK_DIVIDER_ROUND_CLOSEST, core::ptr::null_mut()),
    CLK_IGNORE_UNUSED);
    static OWL_COMP_DIV(uart6_clk, "uart6_clk", uart_clk_mux_p,
    OWL_MUX_HW(CMU_UART6CLK, 16, 1),
    OWL_GATE_HW(CMU_DEVCLKEN1, 18, 0),
    OWL_DIVIDER_HW(CMU_UART6CLK, 0, 8, CLK_DIVIDER_ROUND_CLOSEST, core::ptr::null_mut()),
    CLK_IGNORE_UNUSED);
    static OWL_COMP_DIV(i2srx_clk, "i2srx_clk", i2s_clk_mux_p,
    OWL_MUX_HW(CMU_AUDIOPLL, 24, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 21, 0),
    OWL_DIVIDER_HW(CMU_AUDIOPLL, 20, 4, 0, i2s_div_table),
    0);
    static OWL_COMP_DIV(i2stx_clk, "i2stx_clk", i2s_clk_mux_p,
    OWL_MUX_HW(CMU_AUDIOPLL, 24, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 20, 0),
    OWL_DIVIDER_HW(CMU_AUDIOPLL, 16, 4, 0, i2s_div_table),
    0);
    static OWL_COMP_DIV(hdmia_clk, "hdmia_clk", i2s_clk_mux_p,
    OWL_MUX_HW(CMU_AUDIOPLL, 24, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 22, 0),
    OWL_DIVIDER_HW(CMU_AUDIOPLL, 24, 4, 0, i2s_div_table),
    0);
    static OWL_COMP_DIV(spdif_clk, "spdif_clk", i2s_clk_mux_p,
    OWL_MUX_HW(CMU_AUDIOPLL, 24, 1),
    OWL_GATE_HW(CMU_DEVCLKEN0, 23, 0),
    OWL_DIVIDER_HW(CMU_AUDIOPLL, 28, 4, 0, i2s_div_table),
    0);
    static OWL_COMP_DIV(nand_clk, "nand_clk", nand_clk_mux_p,
    OWL_MUX_HW(CMU_NANDCCLK, 8, 2),
    OWL_GATE_HW(CMU_DEVCLKEN0, 4, 0),
    OWL_DIVIDER_HW(CMU_NANDCCLK, 0, 3, 0, nand_div_table),
    CLK_SET_RATE_PARENT);
    static OWL_COMP_DIV(ecc_clk, "ecc_clk", nand_clk_mux_p,
    OWL_MUX_HW(CMU_NANDCCLK, 8, 2),
    OWL_GATE_HW(CMU_DEVCLKEN0, 4, 0),
    OWL_DIVIDER_HW(CMU_NANDCCLK, 4, 3, 0, nand_div_table),
    CLK_SET_RATE_PARENT);
    static struct owl_clk_common *s500_clks[] = {
    &ethernet_pll_clk.common,
    &core_pll_clk.common,
    &ddr_pll_clk.common,
    &dev_pll_clk.common,
    &nand_pll_clk.common,
    &audio_pll_clk.common,
    &display_pll_clk.common,
    &dev_clk.common,
    &timer_clk.common,
    &i2c0_clk.common,
    &i2c1_clk.common,
    &i2c2_clk.common,
    &i2c3_clk.common,
    &uart0_clk.common,
    &uart1_clk.common,
    &uart2_clk.common,
    &uart3_clk.common,
    &uart4_clk.common,
    &uart5_clk.common,
    &uart6_clk.common,
    &pwm0_clk.common,
    &pwm1_clk.common,
    &pwm2_clk.common,
    &pwm3_clk.common,
    &pwm4_clk.common,
    &pwm5_clk.common,
    &sensor0_clk.common,
    &sensor1_clk.common,
    &sd0_clk.common,
    &sd1_clk.common,
    &sd2_clk.common,
    &bisp_clk.common,
    &ahb_clk.common,
    &ahbprediv_clk.common,
    &h_clk.common,
    &spi0_clk.common,
    &spi1_clk.common,
    &spi2_clk.common,
    &spi3_clk.common,
    &rmii_ref_clk.common,
    &de_clk.common,
    &de1_clk.common,
    &de2_clk.common,
    &i2srx_clk.common,
    &i2stx_clk.common,
    &hdmia_clk.common,
    &hdmi_clk.common,
    &vce_clk.common,
    &vde_clk.common,
    &spdif_clk.common,
    &nand_clk.common,
    &ecc_clk.common,
    &apb_clk.common,
    &dmac_clk.common,
    &gpio_clk.common,
    &nic_clk.common,
    &ethernet_clk.common,
    };
    static struct clk_hw_onecell_data s500_hw_clks = {
    .hws = {
    [CLK_ETHERNET_PLL]	= &ethernet_pll_clk.common.hw,
    [CLK_CORE_PLL]		= &core_pll_clk.common.hw,
    [CLK_DDR_PLL]		= &ddr_pll_clk.common.hw,
    [CLK_NAND_PLL]		= &nand_pll_clk.common.hw,
    [CLK_DISPLAY_PLL]	= &display_pll_clk.common.hw,
    [CLK_DEV_PLL]		= &dev_pll_clk.common.hw,
    [CLK_AUDIO_PLL]		= &audio_pll_clk.common.hw,
    [CLK_TIMER]		= &timer_clk.common.hw,
    [CLK_DEV]		= &dev_clk.common.hw,
    [CLK_DE]		= &de_clk.common.hw,
    [CLK_DE1]		= &de1_clk.common.hw,
    [CLK_DE2]		= &de2_clk.common.hw,
    [CLK_I2C0]		= &i2c0_clk.common.hw,
    [CLK_I2C1]		= &i2c1_clk.common.hw,
    [CLK_I2C2]		= &i2c2_clk.common.hw,
    [CLK_I2C3]		= &i2c3_clk.common.hw,
    [CLK_I2SRX]		= &i2srx_clk.common.hw,
    [CLK_I2STX]		= &i2stx_clk.common.hw,
    [CLK_UART0]		= &uart0_clk.common.hw,
    [CLK_UART1]		= &uart1_clk.common.hw,
    [CLK_UART2]		= &uart2_clk.common.hw,
    [CLK_UART3]		= &uart3_clk.common.hw,
    [CLK_UART4]		= &uart4_clk.common.hw,
    [CLK_UART5]		= &uart5_clk.common.hw,
    [CLK_UART6]		= &uart6_clk.common.hw,
    [CLK_PWM0]		= &pwm0_clk.common.hw,
    [CLK_PWM1]		= &pwm1_clk.common.hw,
    [CLK_PWM2]		= &pwm2_clk.common.hw,
    [CLK_PWM3]		= &pwm3_clk.common.hw,
    [CLK_PWM4]		= &pwm4_clk.common.hw,
    [CLK_PWM5]		= &pwm5_clk.common.hw,
    [CLK_SENSOR0]		= &sensor0_clk.common.hw,
    [CLK_SENSOR1]		= &sensor1_clk.common.hw,
    [CLK_SD0]		= &sd0_clk.common.hw,
    [CLK_SD1]		= &sd1_clk.common.hw,
    [CLK_SD2]		= &sd2_clk.common.hw,
    [CLK_BISP]		= &bisp_clk.common.hw,
    [CLK_SPI0]		= &spi0_clk.common.hw,
    [CLK_SPI1]		= &spi1_clk.common.hw,
    [CLK_SPI2]		= &spi2_clk.common.hw,
    [CLK_SPI3]		= &spi3_clk.common.hw,
    [CLK_AHB]		= &ahb_clk.common.hw,
    [CLK_H]			= &h_clk.common.hw,
    [CLK_AHBPREDIV]		= &ahbprediv_clk.common.hw,
    [CLK_RMII_REF]		= &rmii_ref_clk.common.hw,
    [CLK_HDMI_AUDIO]	= &hdmia_clk.common.hw,
    [CLK_HDMI]		= &hdmi_clk.common.hw,
    [CLK_VDE]		= &vde_clk.common.hw,
    [CLK_VCE]		= &vce_clk.common.hw,
    [CLK_SPDIF]		= &spdif_clk.common.hw,
    [CLK_NAND]		= &nand_clk.common.hw,
    [CLK_ECC]		= &ecc_clk.common.hw,
    [CLK_APB]		= &apb_clk.common.hw,
    [CLK_DMAC]		= &dmac_clk.common.hw,
    [CLK_GPIO]		= &gpio_clk.common.hw,
    [CLK_NIC]		= &nic_clk.common.hw,
    [CLK_ETHERNET]		= &ethernet_clk.common.hw,
    },
    .num = CLK_NR_CLKS,
    };
    static const struct owl_reset_map s500_resets[] = {
    [RESET_DMAC]	= { CMU_DEVRST0, BIT(0) },
    [RESET_NORIF]	= { CMU_DEVRST0, BIT(1) },
    [RESET_DDR]	= { CMU_DEVRST0, BIT(2) },
    [RESET_NANDC]	= { CMU_DEVRST0, BIT(3) },
    [RESET_SD0]	= { CMU_DEVRST0, BIT(4) },
    [RESET_SD1]	= { CMU_DEVRST0, BIT(5) },
    [RESET_PCM1]	= { CMU_DEVRST0, BIT(6) },
    [RESET_DE]	= { CMU_DEVRST0, BIT(7) },
    [RESET_LCD]	= { CMU_DEVRST0, BIT(8) },
    [RESET_SD2]	= { CMU_DEVRST0, BIT(9) },
    [RESET_DSI]	= { CMU_DEVRST0, BIT(10) },
    [RESET_CSI]	= { CMU_DEVRST0, BIT(11) },
    [RESET_BISP]	= { CMU_DEVRST0, BIT(12) },
    [RESET_KEY]	= { CMU_DEVRST0, BIT(14) },
    [RESET_GPIO]	= { CMU_DEVRST0, BIT(15) },
    [RESET_AUDIO]	= { CMU_DEVRST0, BIT(17) },
    [RESET_PCM0]	= { CMU_DEVRST0, BIT(18) },
    [RESET_VDE]	= { CMU_DEVRST0, BIT(19) },
    [RESET_VCE]	= { CMU_DEVRST0, BIT(20) },
    [RESET_GPU3D]	= { CMU_DEVRST0, BIT(22) },
    [RESET_NIC301]	= { CMU_DEVRST0, BIT(23) },
    [RESET_LENS]	= { CMU_DEVRST0, BIT(26) },
    [RESET_PERIPHRESET] = { CMU_DEVRST0, BIT(27) },
    [RESET_USB2_0]	= { CMU_DEVRST1, BIT(0) },
    [RESET_TVOUT]	= { CMU_DEVRST1, BIT(1) },
    [RESET_HDMI]	= { CMU_DEVRST1, BIT(2) },
    [RESET_HDCP2TX]	= { CMU_DEVRST1, BIT(3) },
    [RESET_UART6]	= { CMU_DEVRST1, BIT(4) },
    [RESET_UART0]	= { CMU_DEVRST1, BIT(5) },
    [RESET_UART1]	= { CMU_DEVRST1, BIT(6) },
    [RESET_UART2]	= { CMU_DEVRST1, BIT(7) },
    [RESET_SPI0]	= { CMU_DEVRST1, BIT(8) },
    [RESET_SPI1]	= { CMU_DEVRST1, BIT(9) },
    [RESET_SPI2]	= { CMU_DEVRST1, BIT(10) },
    [RESET_SPI3]	= { CMU_DEVRST1, BIT(11) },
    [RESET_I2C0]	= { CMU_DEVRST1, BIT(12) },
    [RESET_I2C1]	= { CMU_DEVRST1, BIT(13) },
    [RESET_USB3]	= { CMU_DEVRST1, BIT(14) },
    [RESET_UART3]	= { CMU_DEVRST1, BIT(15) },
    [RESET_UART4]	= { CMU_DEVRST1, BIT(16) },
    [RESET_UART5]	= { CMU_DEVRST1, BIT(17) },
    [RESET_I2C2]	= { CMU_DEVRST1, BIT(18) },
    [RESET_I2C3]	= { CMU_DEVRST1, BIT(19) },
    [RESET_ETHERNET] = { CMU_DEVRST1, BIT(20) },
    [RESET_CHIPID]	= { CMU_DEVRST1, BIT(21) },
    [RESET_USB2_1]	= { CMU_DEVRST1, BIT(22) },
    [RESET_WD0RESET] = { CMU_DEVRST1, BIT(24) },
    [RESET_WD1RESET] = { CMU_DEVRST1, BIT(25) },
    [RESET_WD2RESET] = { CMU_DEVRST1, BIT(26) },
    [RESET_WD3RESET] = { CMU_DEVRST1, BIT(27) },
    [RESET_DBG0RESET] = { CMU_DEVRST1, BIT(28) },
    [RESET_DBG1RESET] = { CMU_DEVRST1, BIT(29) },
    [RESET_DBG2RESET] = { CMU_DEVRST1, BIT(30) },
    [RESET_DBG3RESET] = { CMU_DEVRST1, BIT(31) },
    };
    static struct owl_clk_desc s500_clk_desc = {
    .clks	    = s500_clks,
    .num_clks   = ARRAY_SIZE(s500_clks),
    .hw_clks    = &s500_hw_clks,
    .resets     = s500_resets,
    .num_resets = ARRAY_SIZE(s500_resets),
    };
#[no_mangle]
unsafe extern "C" fn s500_clk_probe(pdev: *mut platform_device) -> c_int {
    static int s500_clk_probe(struct platform_device *pdev)
    {
    struct owl_clk_desc *desc;
    struct owl_reset *reset;
    int ret;
    desc = &s500_clk_desc;
    owl_clk_regmap_init(pdev, desc);
    reset = devm_kzalloc(&pdev.dev, sizeof(*reset), GFP_KERNEL);
    if (!reset)
    return -ENOMEM;
    reset.rcdev.of_node = pdev.dev.of_node;
    reset.rcdev.ops = &owl_reset_ops;
    reset.rcdev.nr_resets = desc.num_resets;
    reset.reset_map = desc.resets;
    reset.regmap = desc.regmap;
    ret = devm_reset_controller_register(&pdev.dev, &reset.rcdev);
    if (ret)
    dev_err(&pdev.dev, "Failed to register reset controller\n");
    return owl_clk_probe(&pdev.dev, desc.hw_clks);
    }
    static const struct of_device_id s500_clk_of_match[] = {
    { .compatible = "actions,s500-cmu", },
    { /* sentinel */ }
    };
    static struct platform_driver s500_clk_driver = {
    .probe = s500_clk_probe,
    .driver = {
    .name = "s500-cmu",
    .of_match_table = s500_clk_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn s500_clk_init() -> int __init {
    static int __init s500_clk_init(void)
    {
    return platform_driver_register(&s500_clk_driver);
    }
    core_initcall(s500_clk_init);
