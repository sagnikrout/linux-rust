//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sophgo/clk-cv1800.h
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
// Copyright (C) 2023 Inochi Amaoto <inochiama@outlook.com>
//

pub const REG_PLL_G2_CTRL: c_uint = 0x800;
pub const REG_PLL_G2_STATUS: c_uint = 0x804;
pub const REG_MIPIMPLL_CSR: c_uint = 0x808;
pub const REG_A0PLL_CSR: c_uint = 0x80C;
pub const REG_DISPPLL_CSR: c_uint = 0x810;
pub const REG_CAM0PLL_CSR: c_uint = 0x814;
pub const REG_CAM1PLL_CSR: c_uint = 0x818;
pub const REG_PLL_G2_SSC_SYN_CTRL: c_uint = 0x840;
pub const REG_A0PLL_SSC_SYN_CTRL: c_uint = 0x850;
pub const REG_A0PLL_SSC_SYN_SET: c_uint = 0x854;
pub const REG_A0PLL_SSC_SYN_SPAN: c_uint = 0x858;
pub const REG_A0PLL_SSC_SYN_STEP: c_uint = 0x85C;
pub const REG_DISPPLL_SSC_SYN_CTRL: c_uint = 0x860;
pub const REG_DISPPLL_SSC_SYN_SET: c_uint = 0x864;
pub const REG_DISPPLL_SSC_SYN_SPAN: c_uint = 0x868;
pub const REG_DISPPLL_SSC_SYN_STEP: c_uint = 0x86C;
pub const REG_CAM0PLL_SSC_SYN_CTRL: c_uint = 0x870;
pub const REG_CAM0PLL_SSC_SYN_SET: c_uint = 0x874;
pub const REG_CAM0PLL_SSC_SYN_SPAN: c_uint = 0x878;
pub const REG_CAM0PLL_SSC_SYN_STEP: c_uint = 0x87C;
pub const REG_CAM1PLL_SSC_SYN_CTRL: c_uint = 0x880;
pub const REG_CAM1PLL_SSC_SYN_SET: c_uint = 0x884;
pub const REG_CAM1PLL_SSC_SYN_SPAN: c_uint = 0x888;
pub const REG_CAM1PLL_SSC_SYN_STEP: c_uint = 0x88C;
pub const REG_APLL_FRAC_DIV_CTRL: c_uint = 0x890;
pub const REG_APLL_FRAC_DIV_M: c_uint = 0x894;
pub const REG_APLL_FRAC_DIV_N: c_uint = 0x898;
pub const REG_MIPIMPLL_CLK_CSR: c_uint = 0x8A0;
pub const REG_A0PLL_CLK_CSR: c_uint = 0x8A4;
pub const REG_DISPPLL_CLK_CSR: c_uint = 0x8A8;
pub const REG_CAM0PLL_CLK_CSR: c_uint = 0x8AC;
pub const REG_CAM1PLL_CLK_CSR: c_uint = 0x8B0;
pub const REG_CLK_CAM0_SRC_DIV: c_uint = 0x8C0;
pub const REG_CLK_CAM1_SRC_DIV: c_uint = 0x8C4;
// top_pll_g6
pub const REG_PLL_G6_CTRL: c_uint = 0x900;
pub const REG_PLL_G6_STATUS: c_uint = 0x904;
pub const REG_MPLL_CSR: c_uint = 0x908;
pub const REG_TPLL_CSR: c_uint = 0x90C;
pub const REG_FPLL_CSR: c_uint = 0x910;
pub const REG_PLL_G6_SSC_SYN_CTRL: c_uint = 0x940;
pub const REG_DPLL_SSC_SYN_CTRL: c_uint = 0x950;
pub const REG_DPLL_SSC_SYN_SET: c_uint = 0x954;
pub const REG_DPLL_SSC_SYN_SPAN: c_uint = 0x958;
pub const REG_DPLL_SSC_SYN_STEP: c_uint = 0x95C;
pub const REG_MPLL_SSC_SYN_CTRL: c_uint = 0x960;
pub const REG_MPLL_SSC_SYN_SET: c_uint = 0x964;
pub const REG_MPLL_SSC_SYN_SPAN: c_uint = 0x968;
pub const REG_MPLL_SSC_SYN_STEP: c_uint = 0x96C;
pub const REG_TPLL_SSC_SYN_CTRL: c_uint = 0x970;
pub const REG_TPLL_SSC_SYN_SET: c_uint = 0x974;
pub const REG_TPLL_SSC_SYN_SPAN: c_uint = 0x978;
pub const REG_TPLL_SSC_SYN_STEP: c_uint = 0x97C;
// clkgen
pub const REG_CLK_EN_0: c_uint = 0x000;
pub const REG_CLK_EN_1: c_uint = 0x004;
pub const REG_CLK_EN_2: c_uint = 0x008;
pub const REG_CLK_EN_3: c_uint = 0x00C;
pub const REG_CLK_EN_4: c_uint = 0x010;
pub const REG_CLK_SEL_0: c_uint = 0x020;
pub const REG_CLK_BYP_0: c_uint = 0x030;
pub const REG_CLK_BYP_1: c_uint = 0x034;
pub const REG_DIV_CLK_A53_0: c_uint = 0x040;
pub const REG_DIV_CLK_A53_1: c_uint = 0x044;
pub const REG_DIV_CLK_CPU_AXI0: c_uint = 0x048;
pub const REG_DIV_CLK_CPU_GIC: c_uint = 0x050;
pub const REG_DIV_CLK_TPU: c_uint = 0x054;
pub const REG_DIV_CLK_EMMC: c_uint = 0x064;
pub const REG_DIV_CLK_EMMC_100K: c_uint = 0x06C;
pub const REG_DIV_CLK_SD0: c_uint = 0x070;
pub const REG_DIV_CLK_SD0_100K: c_uint = 0x078;
pub const REG_DIV_CLK_SD1: c_uint = 0x07C;
pub const REG_DIV_CLK_SD1_100K: c_uint = 0x084;
pub const REG_DIV_CLK_SPI_NAND: c_uint = 0x088;
pub const REG_DIV_CLK_ETH0_500M: c_uint = 0x08C;
pub const REG_DIV_CLK_ETH1_500M: c_uint = 0x090;
pub const REG_DIV_CLK_GPIO_DB: c_uint = 0x094;
pub const REG_DIV_CLK_SDMA_AUD0: c_uint = 0x098;
pub const REG_DIV_CLK_SDMA_AUD1: c_uint = 0x09C;
pub const REG_DIV_CLK_SDMA_AUD2: c_uint = 0x0A0;
pub const REG_DIV_CLK_SDMA_AUD3: c_uint = 0x0A4;
pub const REG_DIV_CLK_CAM0_200: c_uint = 0x0A8;
pub const REG_DIV_CLK_AXI4: c_uint = 0x0B8;
pub const REG_DIV_CLK_AXI6: c_uint = 0x0BC;
pub const REG_DIV_CLK_DSI_ESC: c_uint = 0x0C4;
pub const REG_DIV_CLK_AXI_VIP: c_uint = 0x0C8;
pub const REG_DIV_CLK_SRC_VIP_SYS_0: c_uint = 0x0D0;
pub const REG_DIV_CLK_SRC_VIP_SYS_1: c_uint = 0x0D8;
pub const REG_DIV_CLK_DISP_SRC_VIP: c_uint = 0x0E0;
pub const REG_DIV_CLK_AXI_VIDEO_CODEC: c_uint = 0x0E4;
pub const REG_DIV_CLK_VC_SRC0: c_uint = 0x0EC;
pub const REG_DIV_CLK_1M: c_uint = 0x0FC;
pub const REG_DIV_CLK_SPI: c_uint = 0x100;
pub const REG_DIV_CLK_I2C: c_uint = 0x104;
pub const REG_DIV_CLK_SRC_VIP_SYS_2: c_uint = 0x110;
pub const REG_DIV_CLK_AUDSRC: c_uint = 0x118;
pub const REG_DIV_CLK_PWM_SRC_0: c_uint = 0x120;
pub const REG_DIV_CLK_AP_DEBUG: c_uint = 0x128;
pub const REG_DIV_CLK_RTCSYS_SRC_0: c_uint = 0x12C;
pub const REG_DIV_CLK_C906_0_0: c_uint = 0x130;
pub const REG_DIV_CLK_C906_0_1: c_uint = 0x134;
pub const REG_DIV_CLK_C906_1_0: c_uint = 0x138;
pub const REG_DIV_CLK_C906_1_1: c_uint = 0x13C;
pub const REG_DIV_CLK_SRC_VIP_SYS_3: c_uint = 0x140;
pub const REG_DIV_CLK_SRC_VIP_SYS_4: c_uint = 0x144;
